use druid::im::Vector;
use druid::piet::{CairoText, Text, TextLayoutBuilder};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, LayoutCtx,
    LifeCycle, LifeCycleCtx, LinearGradient, PaintCtx, Point, Rect, RenderContext, Size,
    TextLayout, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

use crate::{
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND},
    data::epub::{EpubData,
        settings::{EpubSettings, VisualizationMode},
    },
    dom::Renderable,
    widgets::RoundButton,
};

use druid::text::{RichText, Selection};

#[derive(Debug, Clone)]
enum PageSplitterRanges {
    OnePage(std::ops::Range<usize>),
    TwoPages(std::ops::Range<usize>, std::ops::Range<usize>),
}

const TEXT_Y_PADDING: f64 = 15.0;
//const TEXT_BOTTOM_PADDING: f64 = 30.;

// constants for Page Label in PageSplitter
const PAGE_LABEL_DISTANCE_FROM_CENTER: f64 = 15.;
const PAGE_LABEL_Y_PADDING: f64 = 20.;

use druid_material_icons::normal::action::{ARROW_CIRCLE_LEFT, ARROW_CIRCLE_RIGHT};

impl PageSplitterRanges {
    pub fn is_empty(&self) -> bool {
        match self {
            PageSplitterRanges::OnePage(range) => range.is_empty(),
            PageSplitterRanges::TwoPages(range1, _) => range1.is_empty(),
        }
    }

    pub fn get_page(&self, page: usize) -> std::ops::Range<usize> {
        match self {
            PageSplitterRanges::OnePage(range) => range.clone(),
            PageSplitterRanges::TwoPages(range1, range2) => {
                if page == 0 {
                    range1.clone()
                } else {
                    range2.clone()
                }
            }
        }
    }

    pub fn start(&self) -> usize {
        match self {
            PageSplitterRanges::OnePage(range) => range.start,
            PageSplitterRanges::TwoPages(range1, _) => range1.start,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            PageSplitterRanges::OnePage(range) => range.end,
            PageSplitterRanges::TwoPages(_, range2) => range2.end,
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        match self {
            PageSplitterRanges::OnePage(range) => range.contains(&index),
            PageSplitterRanges::TwoPages(range1, range2) => {
                range1.contains(&index) || range2.contains(&index)
            }
        }
    }
}
impl Default for PageSplitterRanges {
    fn default() -> Self {
        PageSplitterRanges::OnePage(0..0)
    }
}

pub struct PageSplitter {
    text: Vec<TextLayout<RichText>>,
    //text_pos: Vec<f64>,
    visualized_range: PageSplitterRanges,
    search_selection: Option<(usize, Selection)>,
}

impl PageSplitter {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            //text_pos: Vec::new(),
            visualized_range: PageSplitterRanges::default(),
            //selection : None,
            search_selection: None,
        }
    }
    fn generate_text(&mut self, chapter: &Vector<Renderable>, font_size: f64) {
        self.text.clear();

        //self.text_pos.clear();

        for renderable in chapter.iter() {
            match renderable {
                Renderable::Image(_) => {}
                Renderable::Text(text) => {
                    let mut text_layout = TextLayout::new();
                    text_layout.set_text(text.clone());
                    text_layout.set_font(FontDescriptor::new(FontFamily::SERIF));
                    text_layout.set_text_size(font_size);
                    text_layout.set_text_color(Color::BLACK);
                    self.text.push(text_layout);
                }
            }
        }
    }

    fn wrap_label_size(
        &mut self,
        size: &Size,
        text: &mut CairoText,
        margin: f64,
        env: &Env,
    ) -> Size {
        let mut ret_size = Size::ZERO;
        // wrap text as half of the page width
        let width = size.width / 2.;

        for t in self.text.iter_mut() {
            t.set_wrap_width(width - margin * 2.);
            t.rebuild_if_needed(text, env);
            ret_size += t.size();
        }
        ret_size
    }

    fn range(
        &mut self,
        mut current_height: f64,
        direction: bool,
        starting_point: usize,
        paragraph_spacing: f64,
    ) -> std::ops::Range<usize> {
        let mut count = starting_point;

        // if going forward, get all texts starting from starting_point to the end
        let it: Box<dyn Iterator<Item = (usize, &TextLayout<RichText>)>> = if direction {
            Box::new(self.text.iter().enumerate().skip(starting_point))
        }
        // otherwise, get all texts starting from starting_point to the beginning
        else {
            let value_to_skip = if (self.text.len() as isize) - starting_point as isize > 0 {
                self.text.len() - starting_point
            } else {
                0
            };
            Box::new(self.text.iter().enumerate().rev().skip(value_to_skip))
        };

        for (i, label) in it {
            current_height -= label.size().height + paragraph_spacing;
            if current_height <= 0. {
                break;
            }
            count = i;
        }

        if direction {
            starting_point..count
        } else {
            count..starting_point
        }
    }
    fn get_current_range(
        &mut self,
        current_height: f64,
        direction: bool,
        starting_point: usize,
        epub_settings: &EpubSettings,
    ) -> PageSplitterRanges {
        //current_height-= TEXT_BOTTOM_PADDING+TEXT_Y_PADDING;
        let page_1 = self.range(
            current_height,
            direction,
            starting_point,
            epub_settings.paragraph_spacing,
        );

        if !direction && page_1.start == 0 {
            return self.get_current_range(current_height, true, 0, epub_settings);
        }

        if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            let stg = if direction { page_1.end } else { page_1.start };
            let page_2 = self.range(
                current_height,
                direction,
                stg,
                epub_settings.paragraph_spacing,
            );
            if direction {
                return PageSplitterRanges::TwoPages(page_1, page_2);
            } else {
                return PageSplitterRanges::TwoPages(page_2, page_1);
            }
        }

        PageSplitterRanges::OnePage(page_1)
    }

    fn next_page(&mut self, mut current_height: f64, epub_settings: &EpubSettings) -> bool {
        let mut i = self.visualized_range.start();
        current_height = if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            current_height * 2.
        } else {
            current_height
        };
        while i < self.text.len()
            && current_height - self.text[i].size().height + epub_settings.paragraph_spacing > 0.
        {
            current_height -= self.text[i].size().height + epub_settings.paragraph_spacing;
            i += 1;
        }

        // If we reached the end of the text elements, don't update the start_range
        if i >= self.text.len() {
            return false;
        }
        // Update the start_range to the next page
        self.visualized_range = PageSplitterRanges::OnePage(i..i);
        true
    }

    fn prev_page(&mut self, mut current_height: f64, epub_settings: &EpubSettings) -> bool {
        if self.visualized_range.start() == 0 {
            return false;
        }
        current_height = if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            current_height * 2.
        } else {
            current_height
        };
        // Calculate the total size of the text elements starting from the current start_range
        let mut i = self.visualized_range.start();
        while i > 0
            && current_height - self.text[i].size().height + epub_settings.paragraph_spacing > 0.
        {
            current_height -= self.text[i].size().height + epub_settings.paragraph_spacing;
            i -= 1;
        }

        // Update the start_range to the previous page
        self.visualized_range = PageSplitterRanges::OnePage(i..i);
        return true;
    }

    fn get_visible_elements(
        &mut self,
        window_size: f64,
        epub_settings: &EpubSettings,
    ) -> (Vec<TextLayout<RichText>>, Vec<TextLayout<RichText>>) {
        let mut visible_elements = Vec::new();
        let mut second_page = Vec::new();
        let mut total_size = 0.0;
        let mut i = self.visualized_range.start();
        for element in self.text[self.visualized_range.start()..].iter() {
            if total_size + element.size().height + epub_settings.paragraph_spacing > window_size {
                break;
            }
            visible_elements.push(element.clone());
            total_size += element.size().height + epub_settings.paragraph_spacing;
            i += 1;
        }

        total_size = 0.0;
        if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            for element in self.text[i..].iter() {
                if total_size + element.size().height + epub_settings.paragraph_spacing
                    > window_size
                {
                    break;
                }
                second_page.push(element.clone());
                total_size += element.size().height + epub_settings.paragraph_spacing;
                i += 1;
            }
        }

        self.visualized_range = PageSplitterRanges::OnePage(self.visualized_range.start()..i);

        (visible_elements, second_page)
    }
}

impl Widget<EpubData> for PageSplitter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, _: &Env) {
        // why cannot get key down?
        match event {
            Event::Command(cmd) => {
                if let Some(internal) = cmd.get(INTERNAL_COMMAND) {
                    match internal {
                        InternalUICommand::EpubNavigate(direction) => {
                            let mut page_position = data.page_position.clone();

                            if *direction {
                                let next =
                                    self.next_page(ctx.size().height - 50., &data.epub_settings);
                                if !next {
                                    if data.has_next_chapter() {
                                        println!("Going forward");
                                        page_position.set_chapter(data.page_position.chapter() + 1);
                                        page_position.set_richtext_number(0);
                                        self.visualized_range = PageSplitterRanges::OnePage(0..0);
                                    }
                                }
                            } else {
                                let prev =
                                    self.prev_page(ctx.size().height - 50., &data.epub_settings);
                                if !prev {
                                    if data.has_prev_chapter() {
                                        println!("Going backward");
                                        page_position.set_chapter(data.page_position.chapter() - 1);
                                        page_position.set_richtext_number(
                                            data.get_chap_len(data.page_position.chapter() - 1) - 1,
                                        );
                                        // TODO: set the visualized range to the last page of the previous chapter
                                        self.visualized_range = PageSplitterRanges::OnePage(
                                            page_position.richtext_number()
                                                ..page_position.richtext_number(),
                                        );
                                    }
                                }
                            }
                            //if direction {
                            //    if (data.get_current_chap().len() == 0 || (self.visualized_range.end() >= data.get_current_chap().len()-1)) && data.has_next_chapter() {
                            //        page_position.set_chapter(data.epub_metrics.current_chapter+1);
                            //        page_position.set_richtext_number(0);
                            //        //data.change_position(PagePosition::new(data.epub_metrics.current_chapter+1, 0))
                            //    }
                            //    else {
                            //        page_position.set_richtext_number(self.visualized_range.end());
                            //        //data.page_position.set_richtext_number(self.visualized_range.end());
                            //    }
                            //}
                            //else {
                            //    if (self.visualized_range.start() == 0) && data.has_prev_chapter() {
                            //        //data.change_position(PagePosition::new(data.epub_metrics.current_chapter-1, data.get_chap_len(data.epub_metrics.current_chapter-1)-1));
                            //        page_position.set_chapter(data.epub_metrics.current_chapter-1);
                            //        page_position.set_richtext_number(data.get_chap_len(data.epub_metrics.current_chapter-1)-1);
                            //    }
                            //    else {
                            //        //data.page_position.set_richtext_number(self.visualized_range.start());
                            //        page_position.set_richtext_number(self.visualized_range.start());
                            //    }
                            //}
                            data.change_position(page_position);
                            ctx.request_update();
                            ctx.request_layout();
                            ctx.request_paint();
                        }

                        InternalUICommand::EpubGoToPos(pos) => {
                            if data.page_position.chapter() != pos.chapter()
                                || !self.visualized_range.contains(pos.richtext_number())
                            {
                                data.change_position(pos.clone());
                            }

                            if let Some(range) = pos.range() {
                                self.search_selection = Some((
                                    pos.richtext_number(),
                                    Selection::new(range.start, range.end),
                                ));
                            }
                            ctx.request_update();
                            ctx.request_layout();
                            ctx.request_paint();
                        }
                        _ => {}
                    }
                }
            }
            // when the window is going to be closed, save the current position
            Event::WindowDisconnected => {
                ctx.submit_command(
                    INTERNAL_COMMAND
                        .with(InternalUICommand::UpdateBookInfo(data.get_epub_path())),
                );
            }

            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.generate_text(&data.get_current_chap(), data.epub_settings.font_size);
                self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
            }
            LifeCycle::Size(new_size) => {

                //self.visualized_range =
                //    self.get_current_range(new_size.height, true, data.page_position.richtext_number(), &data.epub_settings);
                //
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        if !(data.same(&old_data)) {
            println!("Updating settings");
            if !data
                .edit_data
                .edited_chapter()
                .same(&old_data.edit_data.edited_chapter())
            {
                println!("Updating chapter");
                self.generate_text(
                    &crate::dom::generate_renderable_tree(
                        data.edit_data.edited_chapter(),
                        data.epub_settings.font_size,
                    ),
                    data.epub_settings.font_size,
                );
                //self.text.clear();
            }
            if !data
                .page_position
                .chapter()
                .same(&old_data.page_position.chapter())
            {
                println!("Updating chapter");
                self.generate_text(&data.get_current_chap(), data.epub_settings.font_size);
                //self.text.clear();
                //let renderable = crate::dom::generate_renderable_tree(&data.edit_data.visualized_chapter, data.epub_settings.font_size);
                //
                //for label in renderable.iter() {
                //
                //    match label {
                //        Renderable::Image(_) => todo!(),
                //        Renderable::Text(text) => {
                //            let mut text_layout = TextLayout::new();
                //            text_layout.set_text(text.clone());
                //            text_layout.set_font(FontDescriptor::new(FontFamily::SERIF) );
                //            text_layout.set_text_size(data.epub_settings.font_size);
                //            text_layout.set_text_color(Color::BLACK);
                //            self.text.push(text_layout);
                //        }
                //    }
                //}

                //self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
                //self.visualized_range =
                //    self.get_current_range(ctx.size().height, true, data.page_position.richtext_number(), &data.epub_settings);
                //
            }

            // if text changed, update the text

            // I have to regenerate the text only if the font size has changed
            // Could be possible to change the font size without regenerating the text
            // but using the set_font_size method of TextLayout, but Header have fixed font size
            // calculated from the starting font size of Paragraph elements

            // must regenerate text if font size has changed or if the chapter has changed
            //if data.epub_metrics.current_chapter != old_data.epub_metrics.current_chapter ||
            //    data.epub_settings.font_size != old_data.epub_settings.font_size {
            //    self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
            //}

            //if !data.epub_settings.same(&old_data.epub_settings) {
            self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
            ////}
            //
            //
            //if !(data.page_position.same(&old_data.page_position)) {
            //    // if true, going forward; if false, going backward
            //    let direction = data.page_position.richtext_number() > old_data.page_position.richtext_number() || data.page_position.chapter() > old_data.page_position.chapter();
            //    //println!("Direction is {:?}", direction);
            //    //
            //    //self.visualized_range = self.get_current_range(ctx.size().height, direction, data.page_position.richtext_number(), &data.epub_settings);
            //    //
            //    if direction {
            //        self.next_page(0, ctx.size().height, &data.epub_settings)
            //    }
            //    else {
            //        println!("Going backward");
            //        self.prev_page(0, ctx.size().height, &data.epub_settings)
            //    }
            //}//
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        //
        println!("Layout");
        self.wrap_label_size(&bc.max(), ctx.text(), data.epub_settings.margin, env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, _: &Env) {
        let size = ctx.size();
        let mut y = 0.0;
        //self.text_pos.clear();

        // draw text in this way:
        // if two side, draw two pages of size (size.width/2, size.height)
        // if one side, draw one page of size (size.width/2, size.height) and center it
        // if scroll, draw one page of size (size.width/2, size.height) and center it

        // draw a white background
        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            let rect = Rect::ZERO.with_size(size);
            ctx.fill(rect, &Color::WHITE);
        } else {
            // draw only the part of the page that is visible
            let rect = Rect::from_origin_size(
                Point::new(size.width * 0.25, 0.),
                Size::new(size.width / 2., size.height),
            );
            ctx.fill(rect, &Color::WHITE);
        }

        let x = if !(data.epub_settings.visualization_mode == VisualizationMode::TwoPage) {
            size.width * 0.25
        } else {
            0.0
        };

        for label in self
            .get_visible_elements(ctx.size().height - 50., &data.epub_settings)
            .0
        {
            label.draw(
                ctx,
                Point::new(x + data.epub_settings.margin, y + TEXT_Y_PADDING),
            );
            y += label.size().height + data.epub_settings.paragraph_spacing;
        }

        //for i in self.visualized_range.get_page(0) {
        //    let label = &self.text[i];
        //    //self.text_pos.push(y);
        //    // if self.selection exists, draw it
        //    if let Some((richtext, selection)) = &self.search_selection {
        //        if *richtext == i {
        //            let label = &self.text[i];
        //            label.rects_for_range(selection.range()).iter().for_each(|rect| {
        //                ctx.fill(*rect+druid::Vec2::new(x+data.epub_settings.margin, TEXT_Y_PADDING+y), &Color::YELLOW);
        //            });
        //
        //        }
        //    }
        //    label.draw(ctx, Point::new(x+data.epub_settings.margin, y+TEXT_Y_PADDING));
        //
        //    // label is the first label of the selection
        //    y += label.size().height+  data.epub_settings.paragraph_spacing;
        //}

        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            y = 0.0;
            println!("Drawing second page");
            for label in self
                .get_visible_elements(ctx.size().height - 50., &data.epub_settings)
                .1
            {
                label.draw(
                    ctx,
                    Point::new(
                        size.width / 2. + data.epub_settings.margin,
                        y + TEXT_Y_PADDING,
                    ),
                );
                y += label.size().height + data.epub_settings.paragraph_spacing;
            }

            //for i in self.visualized_range.get_page(1) {
            //    let label = &self.text[i];
            //    if let Some((richtext, selection)) = &self.search_selection {
            //        if *richtext == i {
            //            let label = &self.text[i];
            //            label.rects_for_range(selection.range()).iter().for_each(|rect| {
            //                ctx.fill(*rect+druid::Vec2::new(size.width/2.+data.epub_settings.margin, TEXT_Y_PADDING+y), &Color::rgb8(255, 255, 0));
            //            });
            //
            //        }
            //    }
            //    label.draw(ctx, Point::new(size.width/2.+data.epub_settings.margin, y+TEXT_Y_PADDING));
            //    //self.text_pos.push(y);
            //    y += label.size().height+  data.epub_settings.paragraph_spacing;
            //}
        } //

        // draw a frame for the page
        // if two side, draw two frames with a shadow in the middle
        // if one side, draw one frame with a shadow in the left
        // if scroll, draw one frame with a shadow in the left
        let stops = (
            (Color::BLACK.with_alpha(0.)),
            (Color::BLACK.with_alpha(0.1)),
            (Color::BLACK.with_alpha(0.2)),
            (Color::BLACK.with_alpha(0.3)),
            (Color::BLACK.with_alpha(0.5)),
            (Color::BLACK.with_alpha(0.8)),
        );

        let shadow = LinearGradient::new(UnitPoint::RIGHT, UnitPoint::LEFT, stops.clone());

        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            let rect = Rect::from_origin_size(
                Point::new(size.width / 2., 0.),
                Size::new(15., size.height),
            );
            ctx.fill(rect, &shadow);

            let shadow = LinearGradient::new(UnitPoint::LEFT, UnitPoint::RIGHT, stops);
            let rect = Rect::from_origin_size(
                Point::new(size.width / 2. - 15.5, 0.),
                Size::new(15., size.height),
            );
            ctx.fill(rect, &shadow);
            let rect = Rect::from_origin_size(
                Point::new(size.width / 2., 0.),
                Size::new(size.width / 2., size.height),
            );
            ctx.stroke(rect, &Color::BLACK, 1.0);
        } else {
            let rect = Rect::from_origin_size(
                Point::new(size.width * 0.25, 0.),
                Size::new(15., size.height),
            );
            ctx.fill(rect, &shadow);
        }

        // create a rectangular frame for the page
        let rect =
            Rect::from_origin_size(Point::new(x, 0.), Size::new(size.width / 2., size.height));
        ctx.stroke(rect, &Color::BLACK, 1.0);
    }
}

pub struct TextContainer {
    label_text_lines: WidgetPod<EpubData, PageSplitter>,
    navigation_buttons: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
}
impl TextContainer {
    pub fn new() -> Self {
        let navigation_buttons = vec![
            WidgetPod::new(
                RoundButton::new(ARROW_CIRCLE_LEFT)
                    .with_click_handler(|ctx, _, _| {
                        ctx.submit_command(
                            INTERNAL_COMMAND.with(InternalUICommand::EpubNavigate(false)),
                        );
                    })
                    .with_color(crate::core::style::get_color_unchecked(
                        crate::core::style::PRIMARY_LIGHT,
                    ))
                    .boxed(),
            ),
            WidgetPod::new(
                RoundButton::new(ARROW_CIRCLE_RIGHT)
                    .with_click_handler(|ctx, _, _| {
                        ctx.submit_command(
                            INTERNAL_COMMAND.with(InternalUICommand::EpubNavigate(true)),
                        );
                    })
                    .with_color(crate::core::style::get_color_unchecked(
                        crate::core::style::PRIMARY_LIGHT,
                    ))
                    .boxed(),
            ),
        ];
        Self {
            label_text_lines: WidgetPod::new(PageSplitter::new()),
            navigation_buttons,
        }
    }
}

impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        self.label_text_lines.event(ctx, event, data, env);
        for nav_button in self.navigation_buttons.iter_mut() {
            nav_button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            //LifeCycle::BuildFocusChain => ctx.register_for_focus(),
            _ => {}
        }
        self.label_text_lines.lifecycle(ctx, event, data, env);
        for nav_button in self.navigation_buttons.iter_mut() {
            nav_button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EpubData, data: &EpubData, env: &Env) {
        self.label_text_lines.update(ctx, data, env);
        for nav_button in self.navigation_buttons.iter_mut() {
            nav_button.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        let size = self.label_text_lines.layout(
            ctx,
            &BoxConstraints::tight(Size::new(bc.max().width, bc.max().height)),
            data,
            env,
        );
        self.label_text_lines
            .set_origin(ctx, data, env, Point::ORIGIN);

        let mut x = 10.0;
        for nav_button in self.navigation_buttons.iter_mut() {
            nav_button.layout(ctx, bc, data, env);
            nav_button.set_origin(ctx, data, env, Point::new(x, size.height - 100.));
            x = size.width - 50.;
        }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        self.label_text_lines.paint(ctx, data, env);

        for nav_button in self.navigation_buttons.iter_mut() {
            nav_button.paint(ctx, data, env);
        }
        let label_text = self.label_text_lines.widget_mut();
        let number_of_labels = label_text.text.len() as isize - 1;
        let range = label_text.visualized_range.get_page(0);
        let text = range.start.to_string()
            + "-"
            + &range.end.to_string()
            + "/"
            + &number_of_labels.to_string();
        let layout = ctx.text().new_text_layout(text).build().unwrap();

        let mut origin = Point::new(
            size.width / 2. - PAGE_LABEL_DISTANCE_FROM_CENTER,
            size.height - PAGE_LABEL_Y_PADDING,
        );
        if data.epub_settings.visualization_mode == VisualizationMode::SinglePage {
            ctx.draw_text(&layout, origin);
        } else {
            origin.x = size.width / 2. - size.width / 4. - PAGE_LABEL_DISTANCE_FROM_CENTER;
            ctx.draw_text(&layout, origin);

            let range = label_text.visualized_range.get_page(1);
            let text = range.start.to_string()
                + "-"
                + &range.end.to_string()
                + "/"
                + &number_of_labels.to_string();
            let layout = ctx.text().new_text_layout(text).build().unwrap();

            origin.x = size.width / 2. + size.width / 4. - PAGE_LABEL_DISTANCE_FROM_CENTER;
            ctx.draw_text(&layout, origin);
        }
    }
}
