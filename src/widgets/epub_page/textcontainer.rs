


use druid::im::Vector;
use druid::piet::{TextLayout as textLayout, CairoText};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod, TextLayout, WidgetExt, Rect, LinearGradient, UnitPoint, FontDescriptor, FontFamily, Data,
};
use crate::appstate::{EpubData, PageIndex, EpubSettings};
use crate::core::commands::{GO_TO_POS, CHANGE_PAGE, VisualizationMode};


use druid::text::{RichText, Selection};

#[derive(Debug, Clone)]
enum PageSplitterRanges {
    OnePage(std::ops::Range<usize>),
    TwoPages(std::ops::Range<usize>, std::ops::Range<usize>),
}
const TEXT_Y_PADDING: f64 = 15.0;

impl PageSplitterRanges {
    pub fn is_empty(&self) -> bool {
        match self {
            PageSplitterRanges::OnePage(range) => range.is_empty(),
            PageSplitterRanges::TwoPages(range1, _) => range1.is_empty(),
        }
    }

    pub fn get_page(&self, page: usize) -> std::ops::Range<usize> {
        match self {
            PageSplitterRanges::OnePage(range) => {
                range.clone()
            }
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


}
impl Default for PageSplitterRanges {
    fn default() -> Self {
        PageSplitterRanges::OnePage(0..0)
    }
}


pub struct PageSplitter {
    text: Vec<TextLayout<RichText>>,
    text_pos: Vec<f64>,
    visualized_range: PageSplitterRanges,
    selection : Option<(TextLayout<RichText>, Selection, f64)>,
    search_selection : Selection
}



impl PageSplitter {
    const PAGE_MARGIN: f64 = 20.0;
    const LABEL_MARGIN: f64 = 10.0;
    pub fn new() -> Self {
        Self { 
            text: Vec::new(),
            text_pos: Vec::new(),
            visualized_range: PageSplitterRanges::default(),
            selection : None, 
            search_selection : Selection::new(0, 0)
        }
    }
}
impl PageSplitter {
    fn generate_text(&mut self, chapter: &Vector<RichText>, font_size: f64)  {
        self.text.clear();

        self.text_pos.clear();

        for label in chapter.iter() {
            let mut text_layout = TextLayout::new();
            text_layout.set_text(label.clone());
            text_layout.set_font(FontDescriptor::new(FontFamily::SERIF) );
            text_layout.set_text_size(font_size);
            text_layout.set_text_color(Color::BLACK);
            self.text.push(text_layout);
        }
    }
    
    fn wrap_label_size(&mut self, size: &Size, text: &mut CairoText, margin: f64, env: &Env) -> Size {
        let mut ret_size = Size::ZERO;
        // wrap text as half of the page width
        let width = size.width/2.;

        for t in self.text.iter_mut() {
            t.set_wrap_width(width-margin*2.);
            t.rebuild_if_needed(text, env);
            ret_size += t.size();

        }
        ret_size
    }

    
    fn range(&mut self, mut current_height: f64, direction : bool, starting_point : usize, paragraph_spacing: f64) -> std::ops::Range<usize> {
        let mut count = starting_point;

        // if going forward, get all texts starting from starting_point to the end
        let it :  Box<dyn Iterator<Item = (usize, &TextLayout<RichText>)>> = if direction {
            Box::new(self.text.iter().enumerate().skip(starting_point))
        } 
        // otherwise, get all texts starting from starting_point to the beginning
        else {
            let value_to_skip = if (self.text.len() as isize) - starting_point as isize  > 0 { self.text.len()-starting_point } else { 0 };
            Box::new(self.text.iter().enumerate().rev().skip(value_to_skip))
        };
        //else {
        //    Box::new(self.text.iter().enumerate().filter(|(i,_)| *i < starting_point).rev())
        //};
        
        for (i, label) in it {
            
            if current_height <= 0. {
                break;
            }
            current_height -= label.size().height + paragraph_spacing;//PageSplitter::LABEL_MARGIN;
            count = i;

        }

        if direction {
            starting_point..count
        } else {
            count..starting_point
        }

    }
    fn get_current_range(&mut self, current_height: f64, direction: bool, starting_point : usize, epub_settings: &EpubSettings) -> PageSplitterRanges {

        let page_1 = self.range(current_height, direction, starting_point, epub_settings.paragraph_spacing);

        if !direction && page_1.start == 0 {
            return self.get_current_range(current_height, true, 0, epub_settings);
        }

        //if self.two_side {
        //    println!("First page: {:?}", page_1);
        //}
        if epub_settings.visualization_mode == VisualizationMode::TwoPage {
            let stg = if direction {
                page_1.end
            } else {
                page_1.start
            };
            let page_2 = self.range(current_height, direction, stg, epub_settings.paragraph_spacing);
            //println!("Second page: {:?}", page_2);
            if direction {
                return PageSplitterRanges::TwoPages(page_1, page_2);
            } else {

                return PageSplitterRanges::TwoPages(page_2, page_1);
            }
        }
        //println!("page 1: {:?}", page_1);
        
        PageSplitterRanges::OnePage(page_1)
    }




    


}

impl Widget<EpubData> for PageSplitter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {

            Event::Command(cmd) => {
                if cmd.is(CHANGE_PAGE) {
                    let direction = cmd.get_unchecked(CHANGE_PAGE).clone();
                    let starting_point;
                    if direction {
                        if (self.visualized_range.is_empty() || self.visualized_range.end() >= data.get_current_chap().len()) && data.has_next_chapter() {

                            data.next_chapter();
                            
                            self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
                            self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
                            starting_point = 0;
                            
                        }
                        else {
                            starting_point = self.visualized_range.end();
                        }
                    }
                    else {
                        if (self.visualized_range.is_empty() || self.visualized_range.start() == 0) && data.has_prev_chapter() {

                            data.previous_chapter();


                            self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
                            self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
                            starting_point = data.get_current_chap().len();
                        }
                        else {
                            starting_point = self.visualized_range.start();
                        }
                    }
                    
                    self.visualized_range = self.get_current_range(ctx.size().height, direction, starting_point, &data.epub_settings);


                }


                else if cmd.is(GO_TO_POS) {
                    let pos = cmd.get_unchecked(GO_TO_POS).clone();
                    match pos {
                        PageIndex::IndexPosition { chapter, richtext_number } | 
                        PageIndex::RangePosition { chapter, richtext_number, range: _ } => {
                            data.epub_metrics.change_chapter(chapter);

                            self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
                            self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
        
                            self.visualized_range = self.get_current_range(ctx.size().height, true, richtext_number, &data.epub_settings);
                        }                        
                    }
                    if let PageIndex::RangePosition { chapter: _, richtext_number: _, range } = pos {
                        self.search_selection = Selection::new(range.start, range.end);
                    }
                    ctx.request_update();
                }

            },
            _ => {}
        }
    }



    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
                self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
                
            },
            LifeCycle::Size(new_size) => {

                self.visualized_range = 
                    self.get_current_range(new_size.height, true, self.visualized_range.start(), &data.epub_settings);

            }
            _ => {}
        }
    }


    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) { 
        if !(data.epub_settings.same(&old_data.epub_settings)) {
            // I have to regenerate the text only if the font size has changed
            // Could be possible to change the font size without regenerating the text
            // but using the set_font_size method of TextLayout, but Header have fixed font size
            // calculated from the starting font size of Paragraph elements
            
            if data.epub_settings.font_size != old_data.epub_settings.font_size {
                self.generate_text(data.get_current_chap(), data.epub_settings.font_size);
            }
            self.wrap_label_size(&ctx.size(), ctx.text(), data.epub_settings.margin, env);
            self.visualized_range = self.get_current_range(ctx.size().height, true, self.visualized_range.start(), &data.epub_settings);


        }
        
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        self.wrap_label_size(&bc.max(), ctx.text(), data.epub_settings.margin, env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {

        let size = ctx.size();
        let mut y = 0.0;


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
            let rect = Rect::from_origin_size(Point::new(size.width*0.25, 0.), Size::new(size.width/2.,  size.height));
            ctx.fill(rect, &Color::WHITE);
        }


        let x = if !(data.epub_settings.visualization_mode == VisualizationMode::TwoPage) {
            size.width*0.25
        } else {
            0.0
        };

        for i in self.visualized_range.get_page(0) {
            let label = &self.text[i];
            label.draw(ctx, Point::new(x+data.epub_settings.margin, y+TEXT_Y_PADDING));
            self.text_pos.push(y);
            y += label.size().height+  data.epub_settings.paragraph_spacing;
        }
        
        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            y = 0.0;
            for i in self.visualized_range.get_page(1) {
                let label = &self.text[i];
                label.draw(ctx, Point::new(size.width/2.+data.epub_settings.margin, y+TEXT_Y_PADDING));
                self.text_pos.push(y);
                y += label.size().height+  data.epub_settings.paragraph_spacing;
            }                    
        }


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
            (Color::BLACK.with_alpha(0.8))
        );

        let shadow = LinearGradient::new(UnitPoint::RIGHT, UnitPoint::LEFT, stops.clone());
        
        if data.epub_settings.visualization_mode == VisualizationMode::TwoPage {
            let rect = Rect::from_origin_size(Point::new(size.width/2., 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);

            let shadow = LinearGradient::new(UnitPoint::LEFT, UnitPoint::RIGHT, stops);
            let rect = Rect::from_origin_size(Point::new(size.width/2.-15.5, 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);
            let rect = Rect::from_origin_size(Point::new(size.width/2., 0.), Size::new(size.width/2., size.height));
            ctx.stroke(rect, &Color::BLACK, 1.0);
    
        }
        else {
            let rect = Rect::from_origin_size(Point::new(size.width*0.25, 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);
        }

        // create a rectangular frame for the page
        let rect = Rect::from_origin_size(Point::new(x, 0.), Size::new(size.width/2., size.height));
        ctx.stroke(rect, &Color::BLACK, 1.0);
        
        
    }
}













pub struct TextContainer {   
    label_text_lines: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}
impl TextContainer {
    pub fn new() -> Self {
        Self {
            label_text_lines : WidgetPod::new(PageSplitter::new().boxed()),
        }
    }

}
impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        self.label_text_lines.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.label_text_lines.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EpubData, data: &EpubData, env: &Env) {
        self.label_text_lines.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        
        let size = self.label_text_lines.layout(ctx, 
            &BoxConstraints::tight(Size::new(bc.max().width, bc.max().height-50.)), data, env);
        self.label_text_lines.set_origin(ctx, data, env, Point::ORIGIN);
        
        size
    
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {

        //let size = ctx.size();
        //ctx.fill(size.to_rect(), &Color::WHITE);

        //ctx.clip(size.to_rect());
        self.label_text_lines.paint(ctx, data, env);

    }
}







/*PAINT METHOD

        // paint selection
        //if let Some(tuple) = self.selection.clone() {
        //    let selection_lines = tuple.0.rects_for_range(tuple.1.range());
        //    if !(data.selected_tool == crate::tool::Tool::Marker) {
        //        self.paint_selection(ctx, selection_lines, &env.get(druid::theme::SELECTED_TEXT_BACKGROUND_COLOR), tuple.2);
        //    }
        //    else {
        //        self.paint_selection(ctx, selection_lines,&Color::YELLOW, tuple.2);
        //    }
    //
        //}
        //for selection in data.sidebar_data.book_highlights.iter() {
        //    if selection.value.chapter != data.epub_metrics.current_chapter {
        //        continue;
        //    }
        //    for i in self.visualized_range.get_page(0) {
        //        let text = &self.text[i];
        //        let layout = text.layout().unwrap();
        //        let bounds = layout.image_bounds();
        //        let rect = Rect::from_origin_size(Point::new(0., y), Size::new(size.width, bounds.height()));
        //        let rect = rect.inset(-5.);
        //        let rect = rect.to_rounded_rect(5.);
        //        ctx.fill(rect, &Color::YELLOW);
        //        y += bounds.height();
        //    }
        //}

*/

/*
Handle selection
            //Event::MouseDown(e) => {
            //    // found which richtext was clicked, then found the text position
            //    // and set the selection
            //    for i in self.visualized_range.start()..self.visualized_range.end() {
            //        let label = &self.text[i];
            //        let label_size = label.size();
            //        let label_pos = self.text_pos[i];
            //        let label_rect = Rect::from_origin_size(Point::new(0., label_pos), label_size);
            //        if label_rect.contains(e.pos) {
            //            let point = e.pos - Vec2::new(0., label_pos);
            //            let pos = label.text_position_for_point(point);
            //            let point = point - Vec2::new(LABEL_X_PADDING, 0.0);
        //
            //            let pos = label.text_position_for_point(point);
            //            self.selection = Some((label.clone(), Selection::caret(pos), e.pos.y));
            //            //if e.mods.shift() {
            //            //    self.selection.active = pos;
            //            //} else {
            //            //    let Range { start, end } = pos..pos;
            //            //    self.selection = Selection::new(start, end);
            //            //}
//
            //            println!("Clicked on label {} at position {}", i, e.pos);
            //            break;
            //        }
            //    }
            //}
            //Event::MouseMove(e) => {
            //    if e.buttons.contains(druid::MouseButton::Left) {
            //        for i in self.visualized_range.start()..self.visualized_range.end() {
            //            let label = &self.text[i];
            //            let label_size = label.size();
            //            let label_pos = self.text_pos[i];
            //            let label_rect = Rect::from_origin_size(Point::new(0., label_pos), label_size);
            //            if label_rect.contains(e.pos) {
            //                let point = e.pos - Vec2::new(0., label_pos);
            //                let pos = label.text_position_for_point(point);
            //                let _text = match label.text() {
            //                    Some(text) => text,
            //                    None => return,
            //                };
            //                if let Some((_, selection,_)) = &mut self.selection {
            //                    selection.active = pos;
            //                }
            //                break;
            //            }
            //        }
            //    }
            //}
            //Event::KeyUp(k) => {
            //    print!("Key:{:?}", k);
            //}


*/