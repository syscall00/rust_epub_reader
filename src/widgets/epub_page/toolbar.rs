use druid::{Widget, RenderContext, Color, Size, WidgetPod};

use crate::application_state::EpubData;



/*
In toolbar will be items for:
- button to edit text [ ]
- button to change reading mode (single page, double page, scroll) [ ]
- font size (button + textbox) [ ]
- button for expand or collapse toolbar [ ]
- button to exit from reading mode to home page [ ]
*/
pub struct Toolbar {
    expanded: bool,
    tools : Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
}


impl Toolbar {
    pub fn new() -> Self {
        // default expanded 
        let expanded = true;
        let tools = Vec::new();
        Self {
            expanded, 
            tools
        }
    }
}


impl Widget<EpubData> for Toolbar {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut EpubData, env: &druid::Env) {
        


        for tool in self.tools.iter_mut() {
            tool.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &EpubData, env: &druid::Env) {
        
        
        for tool in self.tools.iter_mut() {
            tool.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &EpubData, data: &EpubData, env: &druid::Env) {

        for tool in self.tools.iter_mut() {
            tool.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &EpubData, env: &druid::Env) -> druid::Size {


        for tool in self.tools.iter_mut() {
            tool.layout(ctx, bc, data, env);
        }
        
        Size::new(bc.max().width, 30.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::BLUE);


        for tools in self.tools.iter_mut() {
            tools.paint(ctx, data, env);
        }

    }
}
