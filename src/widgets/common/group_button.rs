use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, RenderContext, Size, UpdateCtx, Widget, WidgetPod,
};

/**
 * GroupButton is a widget that can be used to create a group of buttons that can be toggled on and off.
 */
pub struct GroupButton<T> {
    buttons: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    active: usize,

    active_closure: Option<Box<dyn Fn(&T, &Env) -> usize>>,

}


impl<T: Data> GroupButton<T> {
    pub fn new(buttons: Vec<Box<dyn Widget<T>>>) -> Self {
        Self {
            buttons: buttons
                .into_iter()
                .map(|button| WidgetPod::new(button))
                .collect(),
            active: 0,
            active_closure: None,
        }
    }

    pub fn with_active_closure<F>(mut self, closure: F) -> Self
    where
        F: Fn(&T, &Env) -> usize + 'static,
    {
        self.active_closure = Some(Box::new(closure));
        self
    }


}

impl<T: Data> Widget<T> for GroupButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button.is_left() {
                    let rect = ctx.size().to_rect();
                    if rect.contains(mouse_event.pos) {
                        let mut index = 0;
                        for button in &mut self.buttons {
                            if button.layout_rect().contains(mouse_event.pos) {
                                self.active = index;
                                ctx.request_paint();
                                break;
                            }
                            index += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        for button in &mut self.buttons {
            button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                if let Some(closure) = &self.active_closure {
                    self.active = closure(data, env);
                }
            }
            _ => {}
        }
        for button in &mut self.buttons {
            button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, data: &T, env: &Env) {
        for button in &mut self.buttons {
            button.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        const PADDING_BETWEEN_BUTTONS: f64 = 8.;
        let mut x = 0.;
        let y = 0.;
        let mut max_height: f64 = 0.;
        for button in &mut self.buttons {
            let size = button.layout(ctx, bc, data, env);
            button.set_origin(ctx, data, env, Point::new(x, y));
            x += size.width + PADDING_BETWEEN_BUTTONS;
            max_height = max_height.max(size.height);
        }
        Size::new(x, max_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for (index, button) in &mut self.buttons.iter_mut().enumerate() {
            // create a shadow for active button
            if index == self.active {
                let shadow = ctx
                    .render_ctx
                    .solid_brush(Color::rgb8(0, 0, 0).with_alpha(0.2));

                ctx.render_ctx.fill(button.layout_rect(), &shadow);
            }

            button.paint(ctx, data, env);
        }
    }
}
