


use druid::im::Vector;
use druid::piet::{TextLayout as textLayout, CairoText};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod, TextLayout, WidgetExt, Rect, LinearGradient, UnitPoint,
};
use crate::appstate::{EpubData};
use crate::core::commands::{GO_TO_POS, CHANGE_PAGE, VisualizationMode};
use crate::core::commands::{CHANGE_VISUALIZATION};

use druid::text::{RichText, Selection};

#[derive(Debug, Clone)]
enum PageSplitterRanges {
    OnePage(std::ops::Range<usize>),
    TwoPages(std::ops::Range<usize>, std::ops::Range<usize>),
}
const TEXT_X_PADDING: f64 = 20.0;
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
    two_side : bool,
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
            two_side: false,
            selection : None, 
            search_selection : Selection::new(0, 0)
        }
    }
}
impl PageSplitter {
    fn generate_text(&mut self, chapter: &Vector<RichText>)  {
        self.text.clear();

        self.text_pos.clear();

        for label in chapter.iter() {
            let mut text_layout = TextLayout::new();
            text_layout.set_text(label.clone());
            text_layout.set_text_color(Color::BLACK);
            self.text.push(text_layout);
        }
    }
    
    fn wrap_label_size(&mut self, size: &Size, text: &mut CairoText, env: &Env) -> Size {
        let mut ret_size = Size::ZERO;
        // wrap text as half of the page width
        let width = size.width/2.;

        for t in self.text.iter_mut() {
            t.set_wrap_width(width-TEXT_X_PADDING-10.);
            t.rebuild_if_needed(text, env);
            ret_size += t.size();

        }
        ret_size
    }

    
    fn range(&mut self, mut current_height: f64, direction : bool, starting_point : usize) -> std::ops::Range<usize> {
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
            current_height -= label.size().height+ PageSplitter::LABEL_MARGIN;
            count = i;

        }

        if direction {
            starting_point..count
        } else {
            count..starting_point
        }

    }
    fn get_current_range(&mut self, current_height: f64, direction: bool, starting_point : usize) -> PageSplitterRanges {

        let page_1 = self.range(current_height, direction, starting_point);

        if !direction && page_1.start == 0 {
            return self.get_current_range(current_height, true, 0);
        }

        //if self.two_side {
        //    println!("First page: {:?}", page_1);
        //}
        if self.two_side {
            let stg = if direction {
                page_1.end
            } else {
                page_1.start
            };
            let page_2 = self.range(current_height, direction, stg);
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
                            
                            self.generate_text(data.get_current_chap());
                            self.wrap_label_size(&ctx.size(), ctx.text(), env);
                            starting_point = 0;
                            
                        }
                        else {
                            starting_point = self.visualized_range.end();
                        }
                    }
                    else {
                        if (self.visualized_range.is_empty() || self.visualized_range.start() == 0) && data.has_prev_chapter() {

                            data.previous_chapter();


                            self.generate_text(data.get_current_chap());
                            self.wrap_label_size(&ctx.size(), ctx.text(), env);
                            starting_point = data.get_current_chap().len();
                        }
                        else {
                            starting_point = self.visualized_range.start();
                        }
                    }
                    
                    self.visualized_range = self.get_current_range(ctx.size().height, direction, starting_point);


                }

                else if cmd.is(CHANGE_VISUALIZATION) {
                    let v = cmd.get_unchecked(CHANGE_VISUALIZATION);
                    match v {
                        VisualizationMode::Single => self.two_side = false,
                        VisualizationMode::Two => self.two_side = true,
                        VisualizationMode::Scroll => self.two_side = false,
                    }
                    self.wrap_label_size(&ctx.size(), ctx.text(), env);
        
                    self.visualized_range = self.get_current_range(ctx.size().height, true, self.visualized_range.start());

                }

                else if cmd.is(GO_TO_POS) {
                    let pos = cmd.get_unchecked(GO_TO_POS).clone();
                    match pos {
                        crate::appstate::PageIndex::IndexPosition { chapter, richtext_number } => {
                            data.epub_metrics.change_chapter(chapter);

                            self.generate_text(data.get_current_chap());
                            self.wrap_label_size(&ctx.size(), ctx.text(), env);
        
                            self.visualized_range = self.get_current_range(ctx.size().height, true, richtext_number);
        
                        }
                        crate::appstate::PageIndex::RangePosition { chapter, richtext_number, range } => {
                            data.epub_metrics.change_chapter(chapter);
                            
                            self.generate_text(data.get_current_chap());
                            self.wrap_label_size(&ctx.size(), ctx.text(), env);
        
                            self.visualized_range = self.get_current_range(ctx.size().height, true, richtext_number);
    
                            self.search_selection = Selection::new(range.start, range.end);
                        },
                        
                    }
                    ctx.request_layout();
                }

            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.generate_text(data.get_current_chap());
                self.wrap_label_size(&ctx.size(), ctx.text(), env);
            },
            LifeCycle::Size(new_size) => {

                self.visualized_range = 
                    self.get_current_range(new_size.height, true, self.visualized_range.start());

            }
            _ => {}
        }
    }


    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) { }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let size = self.wrap_label_size(&bc.max(), ctx.text(), env);
        if !bc.is_height_bounded() {
            Size::new(bc.max().width, size.height)
        }
        else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {

        let size = ctx.size();
        let mut y = 0.0;


        // draw text in this way:
        // if two side, draw two pages of size (size.width/2, size.height)
        // if one side, draw one page of size (size.width/2, size.height) and center it
        // if scroll, draw one page of size (size.width/2, size.height) and center it

        // draw a white background
        if self.two_side {
            let rect = Rect::ZERO.with_size(size);
            ctx.fill(rect, &Color::WHITE);
    
        } else {
            // draw only the part of the page that is visible
            let rect = Rect::from_origin_size(Point::new(size.width*0.25, 0.), Size::new(size.width/2.,  size.height));
            ctx.fill(rect, &Color::WHITE);
        }


        let x = if !self.two_side {
            size.width*0.25
        } else {
            0.0
        };

        for i in self.visualized_range.get_page(0) {
            let label = &self.text[i];
            label.draw(ctx, Point::new(x+TEXT_X_PADDING, y+TEXT_Y_PADDING));
            self.text_pos.push(y);
            y += label.size().height+ PageSplitter::LABEL_MARGIN;
        }
        
        if self.two_side {
            y = 0.0;
            for i in self.visualized_range.get_page(1) {
                let label = &self.text[i];
                label.draw(ctx, Point::new(size.width/2.+TEXT_X_PADDING, y+TEXT_Y_PADDING));
                self.text_pos.push(y);
                y += label.size().height+ PageSplitter::LABEL_MARGIN;
            }                    
        }


        // draw a frame for the page
        // if two side, draw two frames with a shadow in the middle
        // if one side, draw one frame with a shadow in the left
        // if scroll, draw one frame with a shadow in the left
        let shadow_color = Color::rgba8(0, 0, 0, 0x33);
        if self.two_side {
            // create a shadow with gradient
            let shadow = LinearGradient::new(
                UnitPoint::RIGHT,
                UnitPoint::LEFT,
                (
                    (Color::BLACK.with_alpha(0.)),
                    (Color::BLACK.with_alpha(0.1)),
                    (Color::BLACK.with_alpha(0.2)),
                    (Color::BLACK.with_alpha(0.3)),
                    (Color::BLACK.with_alpha(0.5)),
                    (Color::BLACK.with_alpha(0.8)),
                ),
            );

            let rect = Rect::from_origin_size(Point::new(size.width/2., 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);

            let shadow = LinearGradient::new(
                UnitPoint::LEFT,
                UnitPoint::RIGHT,
                (
                    (Color::BLACK.with_alpha(0.)),
                    (Color::BLACK.with_alpha(0.1)),
                    (Color::BLACK.with_alpha(0.2)),
                    (Color::BLACK.with_alpha(0.3)),
                    (Color::BLACK.with_alpha(0.5)),
                    (Color::BLACK.with_alpha(0.8)),
                ),
            );


            let rect = Rect::from_origin_size(Point::new(size.width/2.-15.5, 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);
        }
        else {
            let shadow = LinearGradient::new(
                UnitPoint::RIGHT,
                UnitPoint::LEFT,
                (
                    (Color::BLACK.with_alpha(0.)),
                    (Color::BLACK.with_alpha(0.1)),
                    (Color::BLACK.with_alpha(0.2)),
                    (Color::BLACK.with_alpha(0.3)),
                    (Color::BLACK.with_alpha(0.5)),
                    (Color::BLACK.with_alpha(0.8)),
                ),
            );

            let rect = Rect::from_origin_size(Point::new(size.width*0.25, 0.), Size::new(15., size.height));
            ctx.fill(rect, &shadow);
        }

        // create a rectangular frame for the page
        let rect = Rect::from_origin_size(Point::new(x, 0.), Size::new(size.width/2., size.height));
        ctx.stroke(rect, &Color::BLACK, 1.0);
        
        
        //if self.two_side {
        //    
        //    let gradient = druid::LinearGradient::new(
        //        druid::UnitPoint::RIGHT,
        //        druid::UnitPoint::LEFT,
        //        (
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.0)),//, 0.0),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.5)),//, 0.5),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.5)),//, 0.5),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(1.0)),//, 1.0),
        //        ),
        //    );
        //    let rect = Rect::from_origin_size(Point::new(size.width/2., 0.), Size::new(PageSplitter::PAGE_MARGIN, size.height));
        //    ctx.fill(rect, &gradient);
        //    let gradient_right_to_left = druid::LinearGradient::new(
        //        druid::UnitPoint::LEFT,
        //        druid::UnitPoint::RIGHT,
        //        (
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.0)),//, 0.0),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.5)),//, 0.5),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(0.5)),//, 0.5),
        //            (Color::rgb8(0x00, 0x00, 0x00).with_alpha(1.0)),//, 1.0),
        //        ),
        //    );
        //    let rect = Rect::from_origin_size(Point::new(size.width/2.-PageSplitter::PAGE_MARGIN, 0.), Size::new(PageSplitter::PAGE_MARGIN, size.height));
        //    ctx.fill(rect, &gradient_right_to_left);
        //    
        //    let frame = Rect::from_origin_size(Point::new(0., 0.), Size::new(size.width/2., size.height));
        //    ctx.stroke(frame, &Color::rgb8(0x00, 0x00, 0x00), 1.);
        //    let frame = Rect::from_origin_size(Point::new(size.width/2., 0.), Size::new(size.width/2., size.height));
        //    ctx.stroke(frame, &Color::rgb8(0x00, 0x00, 0x00), 1.);
        //}
        //else {
        //    // create a rectangle 
        //    let rect = Rect::from_origin_size(Point::new(0., 0.), Size::new(size.width/2., size.height));
        //    let rect = rect.inset(-5.);
        //    ctx.stroke(rect, &Color::BLACK, 2.);
        //}

    }
}













pub struct TextContainer {   
    label_text_lines: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}
impl TextContainer {
    pub fn new(_data : EpubData) -> Self {
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