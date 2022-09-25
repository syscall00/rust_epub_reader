use druid::{Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Size, PaintCtx, Color, RenderContext};

use crate::application_state::AppState;



// topbar will have a label with Application name and a button to open a file
pub struct Topbar {
    
}


impl Topbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget<AppState> for Topbar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            _ => {
                println!("event {:?}", event);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        match event {
            _ => {
                println!("lifecycle {:?}", event);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {

    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {

        let size = ctx.size();
        

        ctx.fill(
            size.to_rect(),
            &Color::rgb8(0x00, 0x15, 0x10),
        );
    }
}