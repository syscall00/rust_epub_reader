use druid::{WidgetPod, Widget, widget::ControllerHost, EventCtx, Event, Env, UpdateCtx, LifeCycle, LifeCycleCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Point, WidgetExt, Data};
use druid_material_icons::IconPaths;

use crate::{widgets::{Icon}, core::constants::commands::{INTERNAL_COMMAND, InternalUICommand}};

use super::TooltipController;


pub trait ButtonTrait {
    fn icon(&self) -> IconPaths;
    fn hint(&self) -> String;
    fn command(&self) -> InternalUICommand;
}
pub enum ActionButton {
    CloseBook,
    EditBook,
    OCROpen,
}

impl ButtonTrait for ActionButton {
    fn icon(&self) -> IconPaths {
        match self {
            // Check if can rotate
            ActionButton::CloseBook => druid_material_icons::normal::action::EXIT_TO_APP,
            ActionButton::EditBook => druid_material_icons::normal::editor::EDIT_NOTE,
            ActionButton::OCROpen => druid_material_icons::normal::image::IMAGE_SEARCH,
        }
    }
    fn hint(&self) -> String {
        match self {
            ActionButton::CloseBook => "Close Book".to_string(),
            ActionButton::EditBook => "Edit Book".to_string(),
            ActionButton::OCROpen => "Search using OCR".to_string(),
        }
    }
    fn command(&self) -> InternalUICommand {
        match self {
            ActionButton::CloseBook => InternalUICommand::GoToMenu,
            ActionButton::EditBook => InternalUICommand::OpenEditDialog,
            ActionButton::OCROpen => InternalUICommand::OpenOCRDialog,
        }
    }
}





pub struct IconButton<T: ButtonTrait, D: Data> {
    kind: T,
    icon: WidgetPod<D, Box<dyn Widget<D>>>,
    open: bool,
    clickable: bool,
}
const ICON_SIZE: f64 = 32.;

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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _: &BoxConstraints,
        data: &D,
        env: &Env,
    ) -> Size {
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

