

use druid::widget::{Button, Flex};
use druid::{
    Point, RenderContext, Size,
    Widget, WidgetPod,
};

use crate::appstate::{EpubData};
use crate::core::style::{self, SECONDARY_DARK};



/*
In toolbar will be items for:
- button to edit text [ ]
- button to change reading mode (single page, double page, scroll) [ ]
- font size (button + textbox) [ ]
- button for expand or collapse toolbar [ ]
- button to exit from reading mode to home page [ ]
*/
pub struct Toolbar {
    tools : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}

impl Toolbar {
    pub fn new() -> Self {
        // default expanded 
        let tools = Flex::row()

        .with_flex_child(Button::new("Save".to_string()).on_click(|ctx, _, _| {
            ctx.submit_command(crate::core::commands::SAVE_EPUB.with(()));
        }), 0.1)
        .with_default_spacer();
                
        Self {
            tools: WidgetPod::new(tools).boxed(),
        }
    }
}


impl Widget<EpubData> for Toolbar {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut EpubData, env: &druid::Env) {
        

        self.tools.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &EpubData, env: &druid::Env) {
        
        
        self.tools.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _: &EpubData, data: &EpubData, env: &druid::Env) {

        self.tools.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &EpubData, env: &druid::Env) -> druid::Size {


        self.tools.set_origin(ctx, data, env, Point::new(25., 0.));


        Size::new(bc.max().width, 30.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &style::get_color_unchecked(SECONDARY_DARK));


        self.tools.paint(ctx, data, env);

    }
}
