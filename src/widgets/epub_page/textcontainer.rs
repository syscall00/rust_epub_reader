use std::ops::Range;

use druid::im::Vector;
use druid::piet::{CairoText, Text, TextLayoutBuilder};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, LayoutCtx,
    LifeCycle, LifeCycleCtx, LinearGradient, PaintCtx, Point, Rect, RenderContext, Size,
    TextLayout, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

use crate::{
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND},
    data::epub::{
        settings::{EpubSettings, VisualizationMode},
        EpubData,
    },
    dom::Renderable,
    widgets::RoundButton,
};

use druid::text::{RichText, Selection};

const TEXT_Y_PADDING: f64 = 15.0;

// constants for Page Label in PageSplitter
const PAGE_LABEL_DISTANCE_FROM_CENTER: f64 = 15.;
const PAGE_LABEL_Y_PADDING: f64 = 20.;

use druid_material_icons::normal::action::{ARROW_CIRCLE_LEFT, ARROW_CIRCLE_RIGHT};

pub struct PageSplitter {
    text: Vec<TextLayout<RichText>>,
    visualized_range: Range<usize>,
    search_selection: Option<(usize, Selection)>,
}

impl PageSplitter {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            visualized_range: 0..0,
            search_selection: None,
        }
    }
    fn generate_text(&mut self, chapter: &Vector<Renderable>, font_size: f64) {
        self.text.clear();

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

    fn next_page(&mut self, mut current_height: f64, epub_settings: &EpubSettings) -> bool {
        let mut i = self.visualized_range.start;
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
        self.visualized_range = i..i;
        true
    }

    fn prev_page(&mut self, mut current_height: f64, epub_settings: &EpubSettings) -> bool {
        if self.visualized_range.start == 0 {
            return false;
        }
        current_height = if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            current_height * 2.
        } else {
            current_height
        };
        // Calculate the total size of the text elements starting from the current start_range
        let mut i = self.visualized_range.start;
        while i > 0
            && current_height - self.text[i].size().height + epub_settings.paragraph_spacing > 0.
        {
            current_height -= self.text[i].size().height + epub_settings.paragraph_spacing;
            i -= 1;
        }

        // Update the start_range to the previous page
        self.visualized_range = i..i;
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
        let mut i = self.visualized_range.start;
        for element in self.text[self.visualized_range.start..].iter() {
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

        self.visualized_range = self.visualized_range.start..i;

        (visible_elements, second_page)
    }
}

impl Widget<EpubData> for PageSplitter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, _: &Env) {
        match event {
            Event::Command(cmd) => {
                if let Some(internal) = cmd.get(INTERNAL_COMMAND) {
                    match internal {
                        InternalUICommand::EpubNavigate(direction) => {
                            if *direction {
                                let can_get_next_page =
                                    self.next_page(ctx.size().height - 50., &data.epub_settings);
                                if !can_get_next_page && data.next_chapter() {
                                    self.visualized_range = 0..0;
                                }
                            } else {
                                let can_get_prev_page =
                                    self.prev_page(ctx.size().height - 50., &data.epub_settings);
                                if !can_get_prev_page && data.prev_chapter() {
                                    let last_pos = data.get_current_chap().len() - 1;
                                    self.visualized_range = last_pos..last_pos;
                                }
                            }
                            data.set_position_in_page(self.visualized_range.start);
                            ctx.request_update();
                            ctx.request_layout();
                            ctx.request_paint();
                        }

                        InternalUICommand::EpubGoToPos(pos) => {
                            if data.page_position.chapter() != pos.chapter()
                                || !self.visualized_range.contains(&pos.richtext_number())
                            {
                                data.change_position(pos.clone());
                                self.visualized_range =
                                    pos.richtext_number()..pos.richtext_number();
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
                    INTERNAL_COMMAND.with(InternalUICommand::UpdateBookInfo(data.get_epub_path())),
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

                // If the position is not 0, then we need to go to the position
                if data.page_position.richtext_number() != 0 {
                    self.visualized_range =
                        data.page_position.richtext_number()..data.page_position.richtext_number();
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        let mut should_update_chap = false;
        if !(data.same(&old_data)) {
            println!("Updating settings");
            should_update_chap = should_update_chap
                || !data
                    .edit_data
                    .edited_chapter()
                    .same(&old_data.edit_data.edited_chapter())
                || !data
                    .page_position
                    .chapter()
                    .same(&old_data.page_position.chapter())
                || data.epub_settings.font_size != old_data.epub_settings.font_size;

            //if !data.epub_settings.same(&old_data.epub_settings) {
        }

        if should_update_chap {
            println!("Updating chapter1");
            self.generate_text(
                &crate::dom::generate_renderable_tree(
                    data.edit_data.edited_chapter(),
                    data.epub_settings.font_size,
                ),
                data.epub_settings.font_size,
            );
        self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
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
        // println!("Layout");
        self.wrap_label_size(&bc.max(), ctx.text(), data.epub_settings.margin, env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, _: &Env) {
        let size = ctx.size();
        let mut y = 0.0;
        //self.text_pos.clear();

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

        let (page_1, page_2) = self.get_visible_elements(size.height - 50., &data.epub_settings);

        for (i, label) in page_1.iter().enumerate() {
            if let Some((richtext, selection)) = &self.search_selection {
                let i = i + self.visualized_range.start;
                if *richtext == i {
                    let label = &self.text[i];
                    label
                        .rects_for_range(selection.range())
                        .iter()
                        .for_each(|rect| {
                            ctx.fill(
                                *rect
                                    + druid::Vec2::new(
                                        x + data.epub_settings.margin,
                                        TEXT_Y_PADDING + y,
                                    ),
                                &Color::YELLOW,
                            );
                        });
                }
            }
            label.draw(
                ctx,
                Point::new(x + data.epub_settings.margin, y + TEXT_Y_PADDING),
            );
            y += label.size().height + data.epub_settings.paragraph_spacing;
        }

        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            y = 0.0;
            for (i, label) in page_2.iter().enumerate() {
                if let Some((richtext, selection)) = &self.search_selection {
                    let i = i + self.visualized_range.start + page_1.len();
                    if *richtext == i {
                        let label = &self.text[i];
                        label
                            .rects_for_range(selection.range())
                            .iter()
                            .for_each(|rect| {
                                ctx.fill(
                                    *rect
                                        + druid::Vec2::new(
                                            size.width / 2. + data.epub_settings.margin,
                                            TEXT_Y_PADDING + y,
                                        ),
                                    &Color::YELLOW,
                                );
                            });
                    }
                }
                label.draw(
                    ctx,
                    Point::new(
                        size.width / 2. + data.epub_settings.margin,
                        y + TEXT_Y_PADDING,
                    ),
                );
                y += label.size().height + data.epub_settings.paragraph_spacing;
            }
        }

        // draw a frame for the page
        // if two side, draw two frames with a shadow in the middle
        // if one side, draw one frame with a shadow in the left
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
        let range = &label_text.visualized_range;
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

            let range = &label_text.visualized_range;
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
