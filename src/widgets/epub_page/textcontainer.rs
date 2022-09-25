
use std::ops::Range;

use druid::kurbo::Line;
use druid::piet::{Text, TextLayoutBuilder};
use druid::widget::{Label, RawLabel, LineBreaking, Scroll, ClipBox, Axis};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod, WidgetExt, Selector, Data, Modifiers, Rect,
};

use crate::application_state::{EpubData};


pub struct TextContainer {
    // text: TextLayout,
    // selection: Selection,
    // tool: Tool,

    // List of text lines
    label_text_lines: WidgetPod<EpubData, ClipBox<EpubData, MyLabel>>,
    
}

impl TextContainer {
    pub fn new(data : EpubData) -> Self {

        let mut label = 
        
        ClipBox::new(
            MyLabel::new()
            .with_text_color(Color::BLACK)
        );
        label.set_constrain_horizontal(true);

        Self {
            label_text_lines : WidgetPod::new(label),
        }
    }
}

impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {

            Event::KeyDown(key_event) => {
                println!("Ev {:?}", key_event);
                if key_event.key == druid::keyboard_types::Key::ArrowUp {
                    data.epub_metrics.percentage_page_in_chapter += 1.;
                    if !self.label_text_lines
                    .widget_mut()
                    .pan_to_on_axis(
                        Axis::Vertical, ctx.size().height
                        * data.epub_metrics.percentage_page_in_chapter) {
                            data.next_chapter();
                            data.epub_metrics.percentage_page_in_chapter = 0.;
                        }
                    }


                if key_event.key == druid::keyboard_types::Key::ArrowDown {
                    data.epub_metrics.percentage_page_in_chapter -= 1.;

                    if !self.label_text_lines
                    .widget_mut()
                    .pan_to_on_axis(
                        Axis::Vertical, ctx.size().height
                        * data.epub_metrics.percentage_page_in_chapter) {
                            data.previous_chapter();
                            data.epub_metrics.percentage_page_in_chapter = 0.;
                        }                    
                }
            }
            
            Event::Command(cmd) => {
                if cmd.is(CHANGE_PAGE) {

                    let pos = cmd.get_unchecked(CHANGE_PAGE);

                    if *pos  {
                        data.epub_metrics.percentage_page_in_chapter += 1.;
                        if !self.label_text_lines
                        .widget_mut()
                        .pan_to_on_axis(
                            Axis::Vertical, ctx.size().height
                            * data.epub_metrics.percentage_page_in_chapter) {
                                data.next_chapter();
                                data.epub_metrics.percentage_page_in_chapter = 0.;
                            }
                    }
                    else {
                        
                        data.epub_metrics.percentage_page_in_chapter -= 1.;

                        if !self.label_text_lines
                        .widget_mut()
                        .pan_to_on_axis(
                            Axis::Vertical, ctx.size().height
                            * data.epub_metrics.percentage_page_in_chapter) {
                                data.previous_chapter();
                                data.epub_metrics.percentage_page_in_chapter = 0.;
                            }                    
                        }
                }
                else if cmd.is(GO_TO_POS) {
                    let pos = cmd.get_unchecked(GO_TO_POS);
                    let label = self.label_text_lines.widget_mut().child_mut();
                    let mut  ppt = label.layout.point_for_text_position(*pos);
                    ppt.y -= 15.;
                    self.label_text_lines
                        .widget_mut()
                        .pan_to(ppt);
                }
            }
            Event::Wheel(wheel) => {
                // if scrolling down, next page; if scrolling up, previous page
                if wheel.wheel_delta.y > 0. {
                    data.epub_metrics.percentage_page_in_chapter += 1.;
                    if !self.label_text_lines
                    .widget_mut()
                    .pan_to_on_axis(
                        Axis::Vertical, ctx.size().height
                        * data.epub_metrics.percentage_page_in_chapter) {
                            data.next_chapter();
                            data.epub_metrics.percentage_page_in_chapter = 0.;
                        }
                }
                else {
                    
                    data.epub_metrics.percentage_page_in_chapter -= 1.;

                    if !self.label_text_lines
                    .widget_mut()
                    .pan_to_on_axis(
                        Axis::Vertical, ctx.size().height
                        * data.epub_metrics.percentage_page_in_chapter) {
                            data.previous_chapter();
                            data.epub_metrics.percentage_page_in_chapter = 0.;
                        }                    
                    }



                
            }
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
        ctx.clip(size.to_rect());
        self.label_text_lines.paint(ctx, data, env);

    }
}





pub struct MyLabel {
    layout: TextLayout<RichText>,
    selection : Selection,
    sel_rects: Vec<Selection>,
}


impl MyLabel {
    /// Create a new `MyLabel`.
    pub fn new() -> Self {
        Self {
            layout: TextLayout::new(),
            selection : Selection::default(),
            sel_rects: Vec::new(),
        }
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.set_text_color(color);
        self
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.layout.set_text_color(color);
    }

    fn do_mouse_down(&mut self, point: Point, mods: Modifiers, count: u8) {
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
        let text = match self.layout.text() {
            Some(text) => text,
            None => return,
        };
        self.selection = Selection::new(self.selection.anchor, pos);
    }


}

const CHANGE_PAGE: Selector<bool> = Selector::new("change_page");
const GO_TO_POS: Selector<usize> = Selector::new("go_to_pos");




use druid::text::{RichText, Selection};
use druid::{Vec2, TextLayout, Cursor, TextAlignment};

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
                self.sel_rects.push(self.selection);
                ctx.set_active(false);
            }
            Event::MouseDown(event) => {
                self.do_mouse_down(event.pos, event.mods, event.count);
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseMove(event) => {
                // IF Selected tool is Highlighter
                if !ctx.is_disabled() {
                    ctx.set_cursor(&Cursor::IBeam);
                    if ctx.is_active() {
                        self.do_drag(event.pos);
                        ctx.request_paint();
                    }
                    /*
                    if !ctx.is_disabled() {
                    ctx.set_cursor(&Cursor::IBeam);
                    if ctx.is_active() {
                        let pre_sel = self.borrow().selection();
                        self.borrow_mut().do_drag(mouse.pos);
                        if self.borrow().selection() != pre_sel {
                            self.borrow_mut()
                                .update_pending_invalidation(ImeInvalidation::SelectionChanged);
                            ctx.request_update();
                            ctx.request_paint();
                        }
                    }   
                    */
                }
                else {
                    // Account for the padding
                    let pos = event.pos - Vec2::new(LABEL_X_PADDING, 0.0);

                    if self.layout.link_for_pos(pos).is_some() {
                        ctx.set_cursor(&Cursor::Pointer);
                    } else {
                        ctx.clear_cursor();
                    }
                }
            }

            _ => {  }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, _env: &Env) {
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
        bc.debug_check("Label");

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

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &EpubData, _env: &Env) {


        for selection in self.sel_rects.iter() {
            let rects = self.layout.rects_for_range(selection.range());
            for region in rects {
                let y = region.max_y().floor();
                let line = Line::new((region.min_x(), y), (region.max_x(), y));//- Vec2::new(LABEL_X_PADDING, 0.);
                ctx.stroke(line, &Color::YELLOW, 10.0);
            }
            }

        let origin = Point::new(LABEL_X_PADDING, 0.0);
        let label_size = ctx.size();
        ctx.clip(label_size.to_rect());
        self.layout.draw(ctx, origin);

    }
}