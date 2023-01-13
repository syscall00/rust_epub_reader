use super::icon::Icon;
use druid::{widget::prelude::*, Color, Cursor, Point, WidgetPod};
use druid_material_icons::IconPaths;

pub struct RoundButton<T> {
    icon: WidgetPod<T, Icon>,
    radius: f64,
    click_handler: Option<Box<dyn Fn(&mut EventCtx, &mut T, &Env)>>,
    border_color: Color,
}
/**
 * A more generic button to display an icon and trigger
 * a click handler. Different from icon_button because
 * it is not tied to a specific trait implementation.
 */

impl<T: Data> RoundButton<T> {
    pub fn new(icon: IconPaths) -> Self {
        Self {
            icon: WidgetPod::new(Icon::new(icon)),
            radius: 20.,
            click_handler: None,
            border_color: Color::TRANSPARENT,
        }
    }
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.icon.widget_mut().set_color(&color);
        self
    }

    pub fn with_click_handler(
        mut self,
        click_handler: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> Self {
        self.click_handler = Some(Box::new(click_handler));
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }
}

impl<T: Data> Widget<T> for RoundButton<T> {
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
                let rect = ctx.size().to_rect();
                if rect.contains(mouse_event.pos) {
                    ctx.set_cursor(&Cursor::Pointer);
                } else {
                    ctx.set_cursor(&Cursor::Arrow);
                }
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

    fn layout(&mut self, ctx: &mut LayoutCtx, _: &BoxConstraints, data: &T, env: &Env) -> Size {
        // size based of radius
        let size = Size::new(self.radius * 2., self.radius * 2.);

        self.icon
            .layout(ctx, &BoxConstraints::tight(size), data, env);
        self.icon.set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // color background of button with radius based on size
        let rect = ctx.size().to_rect();
        let rounded_rect = rect.to_rounded_rect(self.radius * 0.3);
        ctx.stroke(rounded_rect, &self.border_color, 1.);

        self.icon.paint(ctx, data, env);
    }
}
