
use std::ops::Range;

use druid::im::Vector;
use druid::kurbo::Line;
use druid::piet::TextLayout as textLayout;
use druid::widget::{ClipBox, Scroll, Axis, ViewSwitcher};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod, Data, Modifiers, WidgetExt, Rect,
};
use crate::appstate::{EpubData};
use crate::core::commands::{GO_TO_POS, CHANGE_PAGE, VisualizationMode, CHANGE_CHAPTER};
use crate::core::commands::{REQUEST_EDIT, CHANGE_VISUALIZATION};

use crate::appstate::NavigationDirection;

// Create a common interface for both single and multi page views
pub trait CommonInterface {
    //fn next_page(&self) -> bool;
    //fn prev_page(&self) -> bool;
    //fn goto_page(&self, page: usize) -> bool;


    fn get_next_pos(&self) -> Option<(usize, Point)>;
    fn get_prev_pos(&self) -> Option<(usize, Point)>;
    fn get_pos(&self) -> Option<(usize, Point)>;
}

//pub struct TwoView {
//    left_view: WidgetPod<EpubData, ClipBox<EpubData, MyLabel>>,
//    right_view: WidgetPod<EpubData, ClipBox<EpubData, MyLabel>>,
//}
//
//impl TwoView {
//    pub fn new() -> Self {
//        let mut left_view = ClipBox::new(
//            MyLabel::new(0)
//            .with_text_color(Color::BLACK)
//        );
//        left_view.set_constrain_horizontal(true);
//
//        let mut right_view = ClipBox::new(
//            MyLabel::new(1)
//            .with_text_color(Color::BLACK)
//        );
//        right_view.set_constrain_horizontal(true);
//        TwoView {
//            left_view: WidgetPod::new(left_view),
//            right_view: WidgetPod::new(right_view),
//        }
//    }
////}
//
//impl Widget<EpubData> for TwoView {
//    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
//                match event {
//            Event::Command(cmd) => {
//
//                //println!(" Num:{}", BOOK_POSITION );
//
//                // Text that should be displayed
//                //let t = druid::piet::TextStorage::as_str(self.label().layout.text().unwrap());
//
//                //println!(" TEXT:{}", utf8_slice::slice(t, (orig.content_size.height % (orig.view_origin.y)) as usize, 
//                //        (orig.content_size.height % orig.view_origin.y) as usize + BOOK_POSITION));
//
//                ////println!("Vh size: {}", self.clip_mut().viewport_size());
//                ////println!("Vh: {:?}", self.clip_mut().viewport());
//
//                    
//                //if cmd.is(CHANGE_PAGE) {
//                //    let pos = cmd.get_unchecked(CHANGE_PAGE);
////
//                //    // DO Next Page
//                //    if *pos  { 
//                //        let orig = self.right_view.widget().viewport();
//                //        println!("ORIG: {:?}", orig);
//                //        let pointt = Point::new(orig.view_size.width, orig.view_size.height);
//                //        let visualized_length = self.point_to_text(pointt);
//        //
//                //        data.epub_metrics.navigate(NavigationDirection::Next(visualized_length));
////
//                //        // Move to the next page if possible
//                //        let can_move = self.move_if_not_out_of_range(
//                //            self.text_to_point(data.epub_metrics.BOOK_POSITION).y);
////
//                //        // if not possible, then load the next chapter
//                //        if !can_move {
//                //            data.next_chapter();
//                //        }   
//                //    }
//                //    // DO Previous page 
//                //    else {
//                //        let orig = self.label_text_lines.widget().viewport();
//                //        println!("ORIG: {:?}", orig);
//                //        let pointt = Point::new(orig.view_size.width, orig.view_size.height);
//                //        let visualized_length = self.point_to_text(pointt);
//        //
//                //        data.epub_metrics.navigate(NavigationDirection::Prev(visualized_length));
////
//                //        println!("Visualized length: {}", visualized_length);
//                //        println!("Book Pos {}", data.epub_metrics.BOOK_POSITION);
//                //        let mut ppt=  self.text_to_point(data.epub_metrics.BOOK_POSITION);
//                //        if data.epub_metrics.BOOK_POSITION == 0 {
//                //            ppt.y -= 30.;
//                //        }
//                //        let can_move = self.clip_mut().pan_to(
//                //            ppt);
////
//                //        if !can_move {
//                //            data.previous_chapter();
//                //        }                    
//                //    }
//                //}
//            },
//            _ => {}
//        }
//        self.left_view.event(ctx, event, data, env);
//        self.right_view.event(ctx, event, data, env);
//    }
//
//    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
//        match event {
//            LifeCycle::WidgetAdded => {
//                self.left_view.lifecycle(ctx, event, data, env);
//                self.right_view.lifecycle(ctx, event, data, env);
//
//            
//                
//            
//                let orig = self.left_view.widget().viewport();
//                let pointt = Point::new(orig.view_size.width, orig.view_size.height);
//                let visualized_length = self.left_view.widget_mut().child().layout.text_position_for_point(pointt);
//
//                let apad = self.left_view.widget_mut().child().layout.point_for_text_position(data.epub_metrics.BOOK_POSITION).y;   
//                let can_move = self.left_view.widget_mut().pan_to_on_axis(Axis::Vertical, apad);
//
//                // if not possible, then load the next chapter
//                if !can_move {
//                    println!("Cant move");
//                    //data.next_chapter();
//                }   
//
//            },
//            _ => {
//                self.left_view.lifecycle(ctx, event, data, env);
//                self.right_view.lifecycle(ctx, event, data, env);        
//            }
//        }
//
//    }
//
//    fn update(&mut self, ctx: &mut UpdateCtx, do_mouse_down: &EpubData, data: &EpubData, env: &Env) {
//        self.left_view.update(ctx, data, env);
//        self.right_view.update(ctx,data, env);
//    }
//
//    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
//        let mut size = bc.max();
//        size.height -= 50.;
//        let left_size = Size::new(size.width / 2.0, size.height);
//        let right_size = Size::new(size.width / 2.0, size.height);
//        let left_origin = Point::ORIGIN;
//        let right_origin = Point::new(size.width / 2.0, 0.0);
//        self.left_view.layout(ctx, &BoxConstraints::tight(left_size), data, env);
//        self.right_view.layout(ctx, &BoxConstraints::tight(right_size), data, env);
//        self.left_view.set_origin(ctx, data, env, left_origin);
//        self.right_view.set_origin(ctx, data, env, right_origin);
//        size
//    }
//
//    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
//        let size = ctx.size();
//        ctx.fill(size.to_rect(), &Color::WHITE);
//
//        self.left_view.paint(ctx, data, env);
//        self.right_view.paint(ctx, data, env);
//    }
//}

pub struct PageSplitter {
    //text: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
    text: Vec<TextLayout<RichText>>,
    starting_point: usize,
    visualized_range: PageSplitterRanges,
    two_side : bool
}

enum PageSplitterRanges {
    OnePage(std::ops::Range<usize>),
    TwoPages(std::ops::Range<usize>, std::ops::Range<usize>),
}


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
                if page == 0 {
                    return range.clone();
                } 
            }
            PageSplitterRanges::TwoPages(range1, range2) => {
                if page == 0 {
                    return range1.clone();
                } else if page == 1 {
                    return range2.clone();
                } 
            }
        };
        Range::default()
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
impl PageSplitter {
    pub fn new() -> Self {
        Self { 
            text: Vec::new(),
            starting_point: 0,
            visualized_range: PageSplitterRanges::default(),
            two_side: false
        }
    }

    fn generate_text(&mut self, chapter: &Vector<RichText>)  {
        for label in chapter.iter() {
            let mut text_layout = TextLayout::new();
            text_layout.set_text(label.clone());
            text_layout.set_text_color(Color::BLACK);
            self.text.push(text_layout);

        }
    }

    
    fn get_current_range(&mut self, current_size: &Size, direction: bool) -> PageSplitterRanges {
        let mut cnt = 0;
        let mut y = 0.;
        
        let it :  Box<dyn Iterator<Item = (usize, &TextLayout<RichText>)>> = if direction {
            Box::new(self.text.iter().enumerate().skip(self.starting_point))
        } else {
            Box::new(self.text.iter().enumerate().filter(|(i,_)| *i < self.starting_point).rev())
        };
        
        
        //let _ = 
        //self.text.iter().enumerate().filter(|(i,_)| *i < self.starting_point).map(|(i, _)| i).rev().collect::<Vec<usize>>();
        
        //println!("it {:?} {:?}", aaa.first(), aaa.last());
        for (i, label) in it {
            cnt = i;
            if y + label.size().height > current_size.height {
                break;
            }
            y += label.size().height+ 10.;

        }

        
        //if self.two_side {
        //    y = 0.;
        //    for (i, label) in itt {
        //        cnt = i;
        //        if y + label.size().height > current_size.height {
        //            break;
        //        }
        //        y += label.size().height+ 10.;
    //
        //    }
    //
        //}
        if !direction && cnt == 0 {
                self.starting_point = cnt;
                return self.get_current_range(current_size, true);
        }
        //println!("First {} Second {} ", y, current_size.height);

        if direction {
            PageSplitterRanges::OnePage(self.starting_point..cnt)
        }
        else {
            PageSplitterRanges::OnePage(cnt..self.starting_point)
        }
    }


}

impl Widget<EpubData> for PageSplitter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            //Event::WindowSize(_) => todo!(),
            //Event::MouseDown(_) => todo!(),
            //Event::MouseUp(_) => todo!(),
            //Event::MouseMove(_) => todo!(),
            //Event::Wheel(_) => todo!(),
            //Event::KeyDown(_) => todo!(),
            //Event::KeyUp(_) => todo!(),
            //Event::Paste(_) => todo!(),
            //Event::Zoom(_) => todo!(),
            //Event::Timer(_) => todo!(),
            //Event::AnimFrame(_) => todo!(),
            ////Event::MouseMove(e) => {
            ////    if ctx.is_hot() {
            ////        // have to check if exists any label under the mouse
            ////        //println!("Have to check if exists in range {:?} {:?}", self.starting_point, self.current_length);
            ////        self.text.iter().enumerate().filter(|(i,_)| *i >= self.starting_point && *i < self.current_length).for_each(|(i, label)| {
            ////            println!("Checking pos {:?} in label {} {:?}", e.pos, i, label.size());
            ////            if label.layout().unwrap().hit_test_point(e.pos).is_inside {
            ////                println!("Found label {}", i);
            ////            }
            ////        });
            ////    }
            ////}
            Event::Command(cmd) => {
                if cmd.is(CHANGE_PAGE) {
                    let direction = cmd.get_unchecked(CHANGE_PAGE).clone();
                    //println!("starting point: {}, current_length: {}", self.starting_point, data.get_current_chap().len());

                    if direction {
                        self.starting_point = self.visualized_range.end();
                        self.visualized_range = self.get_current_range(&ctx.size(), direction);

                        if (self.visualized_range.is_empty() || self.visualized_range.end() >= data.get_current_chap().len()-1) && data.has_next_chapter(){
                            //println!("Visualized empty range {:?}", self.visualized_range);
                            data.next_chapter();

                            self.text.clear();
                            self.generate_text(data.get_current_chap());

                            self.starting_point = 0;
                            
                        }
                            //self.starting_point = self.visualized_range.end;
                    }
                    else {
                        self.starting_point = self.visualized_range.start();
                        self.visualized_range = self.get_current_range(&ctx.size(), direction);

                        if (self.visualized_range.is_empty() || self.visualized_range.start() == 0) && data.has_prev_chapter() {
                            println!("Empty");
                            data.previous_chapter();

                            self.text.clear();
                            self.generate_text(data.get_current_chap());

                            self.starting_point = data.get_current_chap().len()-1;

                        }

                    }
                    self.visualized_range = self.get_current_range(&ctx.size(), direction);

                    //data.epu_metrics.navigate(NavigationDirection::Page(page));

                }
                else if cmd.is(CHANGE_VISUALIZATION) {
                    let v = cmd.get_unchecked(CHANGE_VISUALIZATION);
                    match v {
                        VisualizationMode::Single => self.two_side = false,
                        VisualizationMode::Two => self.two_side = true,
                        VisualizationMode::Scroll => self.two_side = false,
                    }
                }
                //if cmd.is(selector::NAVIGATE) {
                //    let nav = cmd.get_unchecked(selector::NAVIGATE);
                //    match nav {
                //        NavigationDirection::Next(_) => {
                //            data.next_chapter();
                //        },
                //        NavigationDirection::Prev(_) => {
                //            data.previous_chapter();
                //        }
                //    }
                //}

            },
            //Event::Notification(_) => todo!(),
            //Event::ImeStateChange => todo!(),
            _ => {}
        }
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.generate_text(data.get_current_chap());
                //self.visualized_range = 0..self.text.len();
        
            },
            LifeCycle::Size(new_size) => {
                self.starting_point = self.visualized_range.start();

                self.visualized_range = self.get_current_range(&new_size, true);
                //println!("Changin size. New range is {:?} ", self.visualized_range);

            }
            _ => {}
        }
    }


    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {

        let width = if self.two_side {
            bc.max().width/2.-20.
        } else {
            bc.max().width-20.
        };
        for t in self.text.iter_mut() {
            t.set_wrap_width(width);
            t.rebuild_if_needed(ctx.text(), env);
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        let mut y = 0.0;
        //let range = self.get_current_range(&size);
        //let mut range2 = self.get_current_range(&ctx.size());
        //if self.two_side {
        //    self.starting_point = range.end;
        //    range2 = self.get_current_range(&ctx.size());
        //}
        //self.prev_length = range.start;
        
        //println!("Im painting in this range {:?}", self.visualized_range);
        for i in self.visualized_range.get_page(0) {
            let label = &self.text[i];
            label.draw(ctx, Point::new(10., y));
            y += label.size().height+ 10.;
        }
        //if self.two_side {
        //    y = 0.;
        //    for i in range2 {
        //        let label = &self.text[i];
        //        label.draw(ctx, Point::new(ctx.size().width/2.+10., y));
        //        y += label.size().height+ 10.;
        //    }
        //    self.starting_point = range.start;
        //}
        
    }
}




pub struct TextContainer {   
    //label_text_lines: WidgetPod<EpubData, Scroll<EpubData, MyLabel>>,
    //label_text_lines: WidgetPod<EpubData, ClipBox<EpubData, MyLabel>>,
    label_text_lines: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    side : usize
}




impl TextContainer {
    pub fn new(_data : EpubData) -> Self {
        let a = PageSplitter::new();
        //let a = //Scroll::new(
        //    //MyList::new(|| {
        //    druid::widget::List::new(|| {
//
        //        
        //        druid::widget::RawLabel::new()
        //        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        //        ////MyLabel::new(0)
        //        .with_text_color(Color::BLACK)
        //})
        //.with_spacing(20.)
        //.lens(EpubData::visualized_page);//).vertical();
        //let mut label = ClipBox::new(a);
        //label.set_constrain_horizontal(true);

        //// SCROLL
        //  let mut label = ClipBox::new(
        //      MyLabel::new()
        //     .with_text_color(Color::BLACK)
        //  ).disable_scrollbars();
        //label.set_vertical_scroll_enabled(false);
        //label.set_horizontal_scroll_enabled(false);
        


        //let visualization_mode_switcher = ViewSwitcher::new(
        //    |data: &EpubData, _env: &Env| data.visualization_mode.clone(),
        //    |visualization_mode, data, _env| {
        //        match *visualization_mode {
        //            VisualizationMode::Single => label.boxed(),
        //            VisualizationMode::Two => todo!(),
        //            VisualizationMode::Scroll => todo!(),
        //        }
        //    }
        //);


        Self {
            label_text_lines : WidgetPod::new(a.boxed()),
            side: 0
        }
    }



    //fn clip_widget (&mut self) -> &mut Scroll<EpubData, MyLabel> {
    //fn clip_widget (&self) -> &ClipBox<EpubData, MyLabel> {
    //    self.label_text_lines.widget() 
    //}  
//
    //fn clip_mut (&mut self) -> &mut ClipBox<EpubData, MyLabel> {
    //    self.label_text_lines.widget_mut() 
    //}

    //fn label (&self) -> &MyLabel {
    //    self.clip_widget().child()
    //}
    //fn label_mut (&mut self) -> &mut MyLabel {
    //    self.clip_mut().child_mut()
    //}
//
    //fn text_to_point(&self, text_pos: usize) -> Point {
    //    self.label().layout.point_for_text_position(text_pos)
    //}
//
    //fn point_to_text(&self, point: Point) -> usize {
    //    self.label().layout.text_position_for_point(point)
    //}
//
    //pub fn move_if_not_out_of_range(&mut self, position : f64) -> bool {
    //    self.clip_mut().pan_to_on_axis(Axis::Vertical, position)
    //    //self.clip_widget().scroll_to_on_axis(Axis::Vertical, position)
    //}
}


impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::Command(cmd) => {

                //let orig = self.label_text_lines.widget().viewport();
                //println!("ORIG: {:?}", orig);
                //let pointt = Point::new(orig.view_size.width, orig.view_size.height);
                //let visualized_length = self.point_to_text(pointt);
                ////println!(" Num:{}", BOOK_POSITION );
//
                //// Text that should be displayed
                ////let t = druid::piet::TextStorage::as_str(self.label().layout.text().unwrap());
//
                ////println!(" TEXT:{}", utf8_slice::slice(t, (orig.content_size.height % (orig.view_origin.y)) as usize, 
                ////        (orig.content_size.height % orig.view_origin.y) as usize + BOOK_POSITION));
//
                //////println!("Vh size: {}", self.clip_mut().viewport_size());
                //////println!("Vh: {:?}", self.clip_mut().viewport());
//
                //    
                //if cmd.is(CHANGE_PAGE) {
                //    let pos = cmd.get_unchecked(CHANGE_PAGE);
//
                //    // DO Next Page
                //    if *pos  { 
                //        data.epub_metrics.navigate(NavigationDirection::Next(visualized_length));
//
                //        // Move to the next page if possible
                //        let can_move = self.move_if_not_out_of_range(
                //            self.text_to_point(data.epub_metrics.BOOK_POSITION).y);
//
                //        // if not possible, then load the next chapter
                //        if !can_move {
                //            data.next_chapter();
                //        }   
                //    }
                //    // DO Previous page 
                //    else {
                //        data.epub_metrics.navigate(NavigationDirection::Prev(visualized_length));
//
                //        //println!("Visualized length: {}", visualized_length);
                //        //println!("Book Pos {}", data.epub_metrics.BOOK_POSITION);
                //        let mut ppt=  self.text_to_point(data.epub_metrics.BOOK_POSITION);
                //        //if data.epub_metrics.BOOK_POSITION == 0 {
                //        //    ppt.y -= 30.;
                //        //}
                //        let can_move = self.clip_mut().pan_to(
                //            ppt);
//
                //        if !can_move {
                //            data.previous_chapter();
                //        }                    
                //    }
                //}
//
                //else if cmd.is(GO_TO_POS) {
                //    let pos = cmd.get_unchecked(GO_TO_POS);
//
                //    
                //    data.move_to_pos(pos);
                //    data.epub_metrics.navigate(NavigationDirection::Goto(pos.page));
                //    ctx.request_update();
//
                //    self.label_mut().set_selection(pos.page+10.. pos.page+ data.search_input.len()+10);
                //    let ppt = self.text_to_point(pos.page);
                //    self.clip_mut().pan_to(ppt);
                //    
                //}
                ctx.request_update();
                ctx.request_layout();
                ctx.request_paint();

            }
            ///// Event::Wheel(wheel) => {
            /////     // if scrolling down, next page; if scrolling up, previous page
            /////     if wheel.wheel_delta.y > 0. {
            /////         data.epub_metrics.current_page_in_chapter += 1;
            /////         if !self.move_if_not_out_of_range(
            /////             ctx.size().height * data.epub_metrics.current_page_in_chapter as f64) {
            /////                 data.next_chapter();
            /////                 data.epub_metrics.current_page_in_chapter = 0;
            /////             }
            /////     }
            /////     else {
            /////         
            /////         data.epub_metrics.current_page_in_chapter -= 1;
///// 
            /////         if !self.move_if_not_out_of_range(
            /////             ctx.size().height * data.epub_metrics.current_page_in_chapter as f64) {
            /////                 data.previous_chapter();
            /////                 data.epub_metrics.current_page_in_chapter = 0;
            /////             }                    
            /////         }
///// 
            /////         ctx.request_layout();
///// 

            ////}
            _ => { } 
        }

        self.label_text_lines.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.label_text_lines.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EpubData, data: &EpubData, env: &Env) {
        self.label_text_lines.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        
        let size = self.label_text_lines.layout(ctx, &BoxConstraints::tight(Size::new(bc.max().width, bc.max().height-50.)), data, env);
        self.label_text_lines.set_origin(ctx, data, env, Point::ORIGIN);
        
        size
    
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {

        let size = ctx.size();
        ctx.fill(size.to_rect(), &Color::WHITE);

        ctx.clip(size.to_rect());
        self.label_text_lines.paint(ctx, data, env);

    }
}





pub struct MyLabel {
    layout: TextLayout<RichText>,
    selection : Selection,
}


impl MyLabel {
    /// Create a new `MyLabel`.
    pub fn new(side : usize) -> Self {
        Self {
            layout: TextLayout::new(),
            selection : Selection::default(),
        }
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.set_text_color(color);
        self
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.layout.set_text_color(color);
    }

    fn do_mouse_down(&mut self, point: Point, mods: Modifiers) {
        let point = point - Vec2::new(LABEL_X_PADDING, 0.0);
        let pos = self.layout.text_position_for_point(point);
        if mods.shift() {
            self.selection.active = pos;
        } else {
            let Range { start, end } = pos..pos;
            self.selection = Selection::new(start, end);

        }
    }

    fn set_selection(&mut self, range: Range<usize>) {
        self.selection = Selection::new(range.start, range.end);
    }


    fn do_drag(&mut self, point: Point) {
        let point = point - Vec2::new(LABEL_X_PADDING, 0.0);
        //FIXME: this should behave differently if we were double or triple clicked
        let pos = self.layout.text_position_for_point(point);
        let _text = match self.layout.text() {
            Some(text) => text,
            None => return,
        };
        self.selection = Selection::new(self.selection.anchor, pos);
    }


    fn paint_selection(&mut self, ctx : &mut PaintCtx, lines : Vec<Rect>, color: &Color) {
        for region in lines {
            let y = region.max_y().floor();
            let line = Line::new((region.min_x(), y-10.), (region.max_x(), y-10.));
            ctx.stroke(line, color, 13.0);
        }
    }

}





use druid::text::{RichText, Selection};
use druid::{Vec2, TextLayout, Cursor};

const LABEL_X_PADDING: f64 = 25.0;
const LABEL_Y_PADDING: f64 = 25.0;
impl Widget<EpubData> for MyLabel {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, _env: &Env) {
        match event {
            Event::MouseUp(event) => {
                // Account for the padding
                let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);
                if let Some(link) = self.layout.link_for_pos(pos) {
                    ctx.submit_command(link.command.clone());
                }
                if event.button.is_left() && self.selection.active != self.selection.anchor {
                    if data.selected_tool == crate::tool::Tool::Marker {
                        data.add_book_highlight(self.selection.anchor, self.selection.active);
                        self.selection.active = 0;
                        self.selection.anchor = 0;    
                    }
                    ctx.request_update();
                    ctx.request_paint();
                }
                ctx.set_handled();

            }
            Event::MouseDown(event) => {
                if event.button.is_left() {
                    self.do_mouse_down(event.pos, event.mods);
                    ctx.request_paint();
                }
                else if event.button.is_right() {
                    // Open a context menu!

                    for (i, hightlight) in data.sidebar_data.book_highlights.iter().enumerate() {
                        let r = hightlight.value.slice.0..hightlight.value.slice.1;
                        let rect = self.layout.rects_for_range(r);
                        for r in rect {
                            if r.contains(event.pos) {
                                println!("RIMUOVO {}", i);
                                break;
                                // Should open a context menu
                            }
                        }

                    }

                    ctx.set_handled();
                    //}
                    
                }
            }
            Event::MouseMove(event) => {
                // IF Selected tool is Highlighter
                ctx.set_cursor(&Cursor::IBeam);
                
                
                if event.buttons.contains(druid::MouseButton::Left)  {
                        self.do_drag(event.pos);
                        ctx.request_paint();
                    }

                //}
                                
                // Account for the padding
                let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);
                if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&Cursor::Pointer);
                }
                ctx.set_handled();

            }

            _ => {  }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
              //  self.layout.set_text(data.visualized_page.to_owned());
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, _env: &Env) {
        if !old_data.same(data) {
           // self.layout.set_text(data.visualized_page.clone());
            ctx.request_layout();
            
        }
        if self.layout.needs_rebuild_after_update(ctx) {
            ctx.request_layout();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EpubData, env: &Env) -> Size {

        let width = bc.max().width - LABEL_X_PADDING * 2.0;

        self.layout.set_wrap_width(width);
        self.layout.rebuild_if_needed(ctx.text(), env);

        let text_metrics = self.layout.layout_metrics();
        ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        let size = bc.constrain(Size::new(
            text_metrics.size.width + 2. * LABEL_X_PADDING,
            text_metrics.size.height + LABEL_Y_PADDING,
        ));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        
        for selection in data.sidebar_data.book_highlights.iter() {
            if selection.value.chapter != data.epub_metrics.current_chapter {
                continue;
            }
            let rects = self.layout.rects_for_range(selection.value.slice.0..selection.value.slice.1);
            self.paint_selection(ctx, rects, &Color::YELLOW);
        }

        let selection_lines = self.layout.rects_for_range(self.selection.range());
        if !(data.selected_tool == crate::tool::Tool::Marker) {
            self.paint_selection(ctx, selection_lines, &env.get(druid::theme::SELECTED_TEXT_BACKGROUND_COLOR));
        }
        else {
            self.paint_selection(ctx, selection_lines,&Color::YELLOW);
        }


        let origin = Point::new(LABEL_X_PADDING, LABEL_Y_PADDING);
        let label_size = ctx.size();
        ctx.clip(label_size.to_rect());
        self.layout.draw(ctx, origin);

    }


    
}



