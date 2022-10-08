
use std::sync::Arc;

use druid::widget::{TextBox, Button, Flex};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod,
};

use crate::appstate::EpubData;
use crate::core::commands::{REQUEST_EDIT, CHANGE_VISUALIZATION, VisualizationMode};


// Create a base widget for styling toolbar buttons
pub enum ToolbarWidget {
    EditText,
    EditRender,
    EditFontSize,

}

pub struct InputWithButtons {
    text: WidgetPod<String, TextBox<String>>,
    //buttons: ,
}
impl InputWithButtons {
    pub fn new() -> Self {
        Self {
            text: WidgetPod::new(TextBox::new()),
            //buttons: Flex::row(),
        }
    }
}

impl Widget<EpubData> for InputWithButtons {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            _ => {
            }
        }
        self.text.event(ctx, event, &mut data.font_size.to_string(), env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.text.lifecycle(ctx, event, &data.font_size.to_string(), env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        self.text.update(ctx,  &data.font_size.to_string(), env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let s = self.text.layout(ctx, bc, &data.font_size.to_string(), env);
        self.text.set_origin(ctx, &data.font_size.to_string(), env, Point::ORIGIN);
        s
        
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        self.text.paint(ctx, &data.font_size.to_string(), env);
    }
}



// ToolbarController is a small widget that controls the toolbar. 
// Can extend toolbar or close epub page and pop back to main page.
pub struct ToolbarController {
    toolbar_status : Arc<bool>,
}
impl ToolbarController {    
    pub fn new(toolbar_status : Arc<bool>) -> Self {
        Self {
            toolbar_status
        }
    }
}

impl Widget<EpubData> for ToolbarController {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            _ => {
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let s = bc.max();
        s
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::rgb8(0x00, 0x00, 0x00));
    }
}


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
    tools : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,

    toolbar_controller: WidgetPod<EpubData, ToolbarController>,
}

// Create a controller
/*pub struct btnController;
impl btnController {
    pub fn new() -> Self {
        Self {}
    }
}*/

impl Toolbar {
    pub fn new() -> Self {
        // default expanded 
        let expanded = true;
        //let tools = Vec::new();
        let tools = Flex::row()
        .with_flex_child(Button::new("Edit Page".to_string())            
        .on_click(|ctx, data: &mut EpubData, env| {
            ctx.submit_command(REQUEST_EDIT.with(()));
        }), 1.)
        .with_default_spacer()
        //.with_flex_child(Button::new("GOTO".to_string())
        //    .on_click(|ctx, data, env| {
        //        ctx.submit_command(GO_TO_POS.with(15));
        //    }), 0.2)
        .with_flex_child(Button::new("Save".to_string()), 1.)
        .with_default_spacer()
        .with_flex_child(Button::new("Single Page".to_string()).on_click(|ctx, data: &mut EpubData, env| {
            ctx.submit_command(CHANGE_VISUALIZATION.with(VisualizationMode::Single));
        }), 1.)
        .with_default_spacer()
        .with_flex_child(Button::new("Two Page".to_string()).on_click(|ctx, data: &mut EpubData, env| {
            ctx.submit_command(CHANGE_VISUALIZATION.with(VisualizationMode::Two));
        }), 1.)
        .with_default_spacer()
        .with_flex_child(Button::new("Scroll".to_string()).on_click(|ctx, data: &mut EpubData, env| {
            ctx.submit_command(CHANGE_VISUALIZATION.with(VisualizationMode::Scroll));
        }), 1.);
        let toolbar_controller = 
        WidgetPod::new(ToolbarController::new(Arc::new(expanded)));
        Self {
            expanded, 
            tools: WidgetPod::new(tools).boxed(),
            toolbar_controller,
        }
    }
}


impl Widget<EpubData> for Toolbar {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut EpubData, env: &druid::Env) {
        

        self.tools.event(ctx, event, data, env);
        self.toolbar_controller.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &EpubData, env: &druid::Env) {
        
        
        self.tools.lifecycle(ctx, event, data, env);
        self.toolbar_controller.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &EpubData, data: &EpubData, env: &druid::Env) {

        self.tools.update(ctx, data, env);
        self.toolbar_controller.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &EpubData, env: &druid::Env) -> druid::Size {


        let tools_size = self.tools.layout(ctx, &BoxConstraints::tight(Size::new(bc.max().width-50., 30.)), data, env);
        // set origin to +25 to center toolbar
        self.tools.set_origin(ctx, data, env, Point::new(25., 0.));

        let controller_size = self.toolbar_controller.layout(ctx, &BoxConstraints::tight(Size::new(25., 15.)), data, env);
        self.toolbar_controller.set_origin(ctx, data, env, Point::new(tools_size.width + controller_size.width, 0.));

        Size::new(bc.max().width, 30.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        let TOOLBAR_COLOR : Color = Color::from_hex_str("#7EA0B7").unwrap();
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &TOOLBAR_COLOR);

        self.tools.paint(ctx, data, env);
        self.toolbar_controller.paint(ctx, data, env);
        

    }
}
