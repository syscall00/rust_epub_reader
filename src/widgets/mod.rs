pub mod home_page;
pub mod epub_page;
pub mod tooltip;


pub mod widgets {
    use druid::{WidgetPod, EventCtx, Env, Data, Event, Widget, LifeCycleCtx, Cursor, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, Point, PaintCtx, Rect, Color, RenderContext};
    use druid_material_icons::IconPaths;

    pub struct RoundButton<T> {
        icon: WidgetPod<T, Icon>,
        radius : f64,
        click_handler: Option<Box<dyn Fn(&mut EventCtx, &mut T, &Env)>>,

    }
    
    impl <T : Data> RoundButton<T> {
        pub fn new(icon: IconPaths) -> Self {
            Self {
                icon: WidgetPod::new(Icon::new(icon)),
                radius : 20.,
                click_handler: None,
            }
        }
        pub fn with_radius(mut self, radius : f64) -> Self {
            self.radius = radius;
            self
        }

        pub fn with_color(mut self, color : Color) -> Self {
            self.icon.widget_mut().set_color(&color);
            self
        }
    
        pub fn with_click_handler(mut self, click_handler: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
            self.click_handler = Some(Box::new(click_handler));
            self
        }
    }
    
    
    impl <T : Data> Widget<T> for RoundButton<T> {
        fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
            match event {
                Event::MouseUp(mouse_event) => {
                    if mouse_event.button.is_left() {
                        if ctx.is_active() {
                            ctx.set_active(false);
                            let rect = ctx.size().to_rect();
                            if rect.contains(mouse_event.pos) {
                                if let Some(click_handler) = &self.click_handler {
                                    click_handler(ctx, data, env);
                                }
                                ctx.set_handled();
                            }
                        }
                    }
                }
                Event::MouseDown(mouse_event) => {
                    if mouse_event.button.is_left() {
                        ctx.set_active(true);
                        ctx.set_handled();
                    }
                }
                Event::MouseMove(mouse_event) => {
                    //if ctx.is_hot() {
                        let rect = ctx.size().to_rect();
                        if rect.contains(mouse_event.pos) {
                            ctx.set_cursor(&Cursor::Pointer);
                        } else {
                            ctx.set_cursor(&Cursor::Arrow);
                        }
                    //}
                }
                _ => {}
            }
            self.icon.event(ctx, event, data, env);
        }
    
        fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
            self.icon.lifecycle(ctx, event, data, env);
        }
    
        fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
            self.icon.update(ctx, data, env);
        }
    
        fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
            // size based of radius
            let size = Size::new(self.radius*2., self.radius*2.);
            
            self.icon.layout(ctx, &BoxConstraints::tight(size), data, env);
            self.icon.set_origin(ctx, data, env, Point::ORIGIN);
            size
        }
    
        fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
            let size = ctx.size();
            let alpha = if ctx.is_hot() { 1.0 } else { 0.5 };
    
            //let r = Rect::from_origin_size(Point::new(0., 0.), ctx.size()).to_rounded_rect(self.radius);
            //ctx.fill(r, &Color::BLACK.with_alpha(alpha));
            self.icon.paint(ctx, data, env);
        }
    }
    






pub use druid_material_icons::{normal};

use druid::{
    kurbo::{Affine}, KeyOrValue,
};

#[derive(Debug, Clone)]
pub struct Icon {
    paths: IconPaths,
    color: Color,
}

impl Icon {
    #[inline]
    pub fn new(paths: IconPaths) -> Self {
        Self {
            paths,
            color: Color::WHITE
        }
    }


    pub fn set_color(&mut self, color: &Color) {
        self.color = color.clone();
    }
}

impl<T: Data> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
        // no events
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {
        // no lifecycle
    }
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
        // no update
    }
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        // Try to preserve aspect ratio if possible, but if not then allow non-uniform scaling.
        bc.constrain_aspect_ratio(self.paths.size.aspect_ratio(), self.paths.size.width)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let Size { width, height } = ctx.size();
        let Size {
            width: icon_width,
            height: icon_height,
        } = self.paths.size;
        ctx.transform(Affine::scale_non_uniform(
            width * icon_width.recip(),
            height * icon_height.recip(),
        ));
        for path in self.paths.paths {
            ctx.fill(path, &self.color);
        }
    }
}
}