
use std::ops::Range;

use druid::kurbo::Line;
use druid::widget::{ClipBox, Scroll, Axis, ViewSwitcher};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod, Data, Modifiers, WidgetExt, Rect,
};
use crate::appstate::{EpubData};
use crate::core::commands::{GO_TO_POS, CHANGE_PAGE, VisualizationMode};


pub struct TextContainer {
    // text: TextLayout,
    // selection: Selection,
    // tool: Tool,

    // List of text lines
    label_text_lines: WidgetPod<EpubData, Scroll<EpubData, MyLabel>>,
    
}

impl TextContainer {
    pub fn new(_data : EpubData) -> Self {

        let mut label = Scroll::new(
            MyLabel::new()
            .with_text_color(Color::BLACK)
        ).disable_scrollbars();
        //label.set_vertical_scroll_enabled(false);
        label.set_horizontal_scroll_enabled(false);
        
        //// label.set_constrain_horizontal(true);


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
            label_text_lines : WidgetPod::new(label),
        }
    }


    fn clip_widget (&mut self) -> &mut Scroll<EpubData, MyLabel> {
        self.label_text_lines.widget_mut()
        
    }  
    pub fn move_if_not_out_of_range(&mut self, position : f64) -> bool {
        //self.clip_widget().pan_to_on_axis(Axis::Vertical, position)
        self.clip_widget().scroll_to_on_axis(Axis::Vertical, position)
    }
}

impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(CHANGE_PAGE) {
                    let pos = cmd.get_unchecked(CHANGE_PAGE);

                    // DO Next Page
                    if *pos  {
                        let can_move = self.move_if_not_out_of_range(
                            ctx.size().height * (data.epub_metrics.get_next_page_in_chap()) as f64);
                        data.next(can_move);
                    }
                    // DO Previous page 
                    else {
                        
                        data.epub_metrics.current_page_in_chapter -= 1;

                        if !self.move_if_not_out_of_range(
                            ctx.size().height * data.epub_metrics.get_previous_page_in_chap() as f64) {
                                data.previous_chapter();
                                data.epub_metrics.current_page_in_chapter = 0;
                            }                    
                        }
                        ctx.request_layout();
                }



                else if cmd.is(GO_TO_POS) {
                    let pos = cmd.get_unchecked(GO_TO_POS);
                    println!("Pos: {:?}", self.label_text_lines.widget_mut().offset());
                    data.move_to_pos(pos);
                    let label = self.label_text_lines.widget_mut().child_mut();
                    let mut ppt = label.layout.point_for_text_position(pos.page);
                /////     self.label_text_lines
                /////         .widget_mut()
                /////         .scroll_to_on_axis(Axis::Vertical, ppt.x);
                }
                ///// ctx.request_layout();
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
    pub fn new() -> Self {
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

const LABEL_X_PADDING: f64 = 2.0;
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

                    for (i, hightlight) in data.book_highlights.iter().enumerate() {
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
                        println!("mouse move");
                        self.do_drag(event.pos);
                        ctx.request_paint();
                    }

                //}
                                
                // Account for the padding
                let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);
                if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&Cursor::Pointer);
                }
                else {
                    
                }
                ctx.set_handled();

            }

            _ => {  }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.layout.set_text(data.visualized_page.to_owned());
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, _env: &Env) {
        if !old_data.same(data) {
            self.layout.set_text(data.visualized_page.clone());
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
            text_metrics.size.height,
        ));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        
        for selection in data.book_highlights.iter() {
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


        let origin = Point::new(LABEL_X_PADDING, 0.0);
        let label_size = ctx.size();
        ctx.clip(label_size.to_rect());
        self.layout.draw(ctx, origin);

    }


    
}