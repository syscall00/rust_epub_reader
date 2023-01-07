use druid::{
    widget::ControllerHost, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use druid_material_icons::IconPaths;

use crate::{
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND},
    widgets::Icon,
};

use super::TooltipController;
/**
 * A widget used in sidebar and toolbar to display an icon and trigger
 * a specific command when clicked.
 *
 * Text, tooltip and icon are defined implementing the ButtonTrait trait.
 */
pub trait ButtonTrait {
    fn icon(&self) -> IconPaths;
    fn hint(&self) -> String;
    fn command(&self) -> InternalUICommand;
}

pub struct IconButton<T: ButtonTrait, D: Data> {
    kind: T,
    icon: WidgetPod<D, Box<dyn Widget<D>>>,
    open: bool,
    clickable: bool,
}
pub const ICON_SIZE: f64 = 32.;

impl<T: ButtonTrait, D: Data> IconButton<T, D> {
    pub fn new(kind: T) -> Self {
        let hint = kind.hint();
        let icon_data = kind.icon();
        Self {
            kind,
            icon: WidgetPod::new(
                ControllerHost::new(Icon::new(icon_data), TooltipController::new(hint)).boxed(),
            ),
            open: false,
            clickable: true,
        }
    }
}

impl<T: ButtonTrait, D: Data> Widget<D> for IconButton<T, D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if self.clickable {
                    self.open = !self.open;
                    ctx.request_paint();
                }
                ctx.submit_command(INTERNAL_COMMAND.with(self.kind.command()));
            }
            // set cursors hand on hover
            Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
            }
            Event::WindowConnected => {}
            _ => {}
        }

        self.icon.event(ctx, event, data, env);
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        self.icon.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &D, _: &D, _: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, _: &BoxConstraints, data: &D, env: &Env) -> Size {
        self.icon.layout(
            ctx,
            &BoxConstraints::tight(Size::new(ICON_SIZE, ICON_SIZE)),
            data,
            env,
        );
        self.icon.set_origin(ctx, data, env, Point::ORIGIN);
        Size::new(ICON_SIZE, ICON_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        self.icon.paint(ctx, data, env);
    }
}
