use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, MouseButton, PaintCtx, Point, Rect, RenderContext, Size, TextLayout, UpdateCtx,
    Widget, widget::{RawLabel, Label}, WidgetExt,
};

use crate::commands::SCROLL_TO_VIEW;

use crate::tool::Tool;

const SELECTED_TOOL: druid::Key<u64> = druid::Key::new("org.linebender.example.important-label-color");

pub struct EpubPage<T: druid::text::TextStorage> {
    page_num: u32,
    layout: TextLayout<T>,
    mark_points: Vec<(Point, Point)>,
    pen_points: Vec<(Point, Point)>,
    set : bool,
    set2 : bool

}

impl<T: druid::text::TextStorage> EpubPage<T> {
    pub fn new(page_num: u32) -> Self {
        EpubPage {
            page_num,
            layout: TextLayout::new(),
            mark_points: Vec::new(),
            pen_points: Vec::new(),
            set: false,
            set2: false
        }
    }
}

impl<T: druid::text::TextStorage> Widget<T> for EpubPage<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        
        let selected_tool : Tool = env.get(SELECTED_TOOL).into();
        
        match event {
            Event::MouseDown(e) => {

                if selected_tool.should_be_written() && e.buttons.contains(MouseButton::Left) {

                    match selected_tool {
                        Tool::Pen | Tool::Marker => self.mark_points.push((e.pos, e.pos)),
                        _ => {}
                    }
                }
            }

            Event::MouseUp(e) => {

                if selected_tool.should_be_written() {
                    // Controllo sul unwrap;
                    let p = self.mark_points.last().unwrap();
                    if p.0 == p.1 { // if equal and not write nothing
                        self.mark_points.pop();

                    } else {
                        let l = self.mark_points.len();
                        // set last point to the current point
                        self.mark_points[l - 1].1 = e.pos;
                        ctx.request_paint();
                    }
                }
                else if selected_tool == Tool::Eraser {
                    
                    self.mark_points = self.mark_points.iter().filter(|(start, end)| {
                        let r = Rect::from_points(*start, *end);
                        println!("Pos {} Contains : {}", e.pos, r.contains(e.pos));
                        !r.contains(e.pos)
                    })
                    .map(|(p, s)| (*p, *s))
                    .collect();
                ctx.request_paint();
                }
                // Account for the padding
                let pos = e.pos;// - Vec2::new(10, 0.0);
                if let Some(link) = self.layout.link_for_pos(pos) {
                    ctx.submit_command(link.command.clone());
                }

            }
            Event::MouseMove(e) => {
                if selected_tool.should_be_written() && e.buttons.contains(MouseButton::Left) {
                    // and Mark is selected
                    //println!("{:?}", event);

                    ctx.request_paint();
                    let l = self.mark_points.len();

                    // set last point to the current point
                    self.mark_points[l - 1].1 = e.pos;
                }

                // Account for the padding
                let pos = e.pos;// - Vec2::new(LABEL_X_PADDING, 0.0);
                if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                } else {
                    ctx.clear_cursor();
                }
            }
            _ => {
                //println!("Unhandled event: {:?}", event);
            }
        }
        // let pre_data = data.raw.to_owned();
        // child.event(ctx, event, data, env);
        // if !data.raw.same(&pre_data) {
        //     data.rendered = rebuild_rendered_text(&data.raw);
        // }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.layout.set_text(data.clone());
                
            }
            _ => {} 
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
        //println!("Update???");
        // TODO Check the data has changed and rebuild the rendered text if it has changed
        //if _old_data != _data {
        if !self.set {
            self.layout.set_text(_data.clone());
            self.set = true;
        }
        //}
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {

        self.layout.set_wrap_width(bc.max().width-150.);
        self.layout.set_text_color(Color::BLACK);
        self.layout.layout();
        self.layout.rebuild_if_needed(ctx.text(), env);

        Size::new(bc.max().width, self.layout.size().height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        let size = Size::new(ctx.size().width, ctx.size().height);
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE); // bck
        
        let mut ep = epub::doc::EpubDoc::new("/home/drivesec/Downloads/I sette mariti.epub".to_string()).unwrap();

        if false && !self.set2 {
            self.set2 = true;
        let image_data = druid::ImageBuf::from_data(&ep.get_cover().unwrap()).unwrap();
        let image = image_data.to_image(ctx.render_ctx);
        ctx.draw_image(&image, image_data.size().to_rect(), druid::piet::InterpolationMode::Bilinear);

        
        for (start_point, end_point) in &self.mark_points {
            let rect = Rect::from_origin_size(
                (start_point.x, start_point.y),
                (end_point.x - start_point.x, 15.0),
            );

            let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
            ctx.fill(rect, &fill_color);
        }
    }

        // Text is easy; in real use TextLayout should either be stored in the
        // widget and reused, or a label child widget to manage it all.
        // This is one way of doing it, you can also use a builder-style way.
        // When we exit with_save, the original context's rotation is restored
        let origin = Point::new(10.+15.0, 10.0);

        // This is the builder-style way of drawing text.
        self.layout.draw(ctx, origin);
        /*let text = ctx.text();

        let layout = text.new_text_layout(rebuild_rendered_text(&data.raw))
        .max_width(ctx.size().width - 45.0)
        .build().unwrap();

        self.layout.draw(ctx, origin);
        */
    }



}

/*impl<W: Widget<AppState>> Controller<AppState, W> for EpubPage<T: TextStorage + druid::text::TextStorage> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        println!("{:?}", event);
        let pre_data = data.raw.to_owned();
        child.event(ctx, event, data, env);
        if !data.raw.same(&pre_data) {
            data.rendered = rebuild_rendered_text(&data.raw);
        }
    }
}*/
