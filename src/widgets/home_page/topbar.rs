use druid::{Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext};

use crate::appstate::AppState;



// topbar will have a label with Application name and a button to open a file
pub struct Topbar {
    
}


impl Topbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget<AppState> for Topbar {
    fn event(&mut self, _: &mut EventCtx, event: &Event, _: &mut AppState, _: &Env) {
        match event {
            _ => {
            }
        }
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, event: &LifeCycle, _: &AppState, _: &Env) {
        match event {
            _ => {
            }
        }
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &AppState, _: &AppState, _: &Env) {

    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &AppState, _: &Env) -> Size {

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &AppState, _: &Env) {

        let size = ctx.size();
        

        ctx.fill(
            size.to_rect(),
            &Color::rgb8(0x00, 0x15, 0x10),
        );
    }
}