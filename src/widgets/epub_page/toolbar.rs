
use std::sync::Arc;

use druid::widget::{Button, Flex};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget, WidgetPod,
};

use crate::appstate::{EpubData};
use crate::core::commands::{REQUEST_EDIT};
use crate::core::style::{self, SECONDARY_DARK};
use crate::tool::Tool;


const TOOLBAR_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#7EA0B7");//.unwrap();

// Create a base widget for styling toolbar buttons
pub enum _ToolbarWidget {
    EditText,
    EditRender,
    EditFontSize,

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
    fn event(&mut self, _: &mut EventCtx, event: &Event, _: &mut EpubData, _: &Env) {
        match event {
            _ => {
            }
        }
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &EpubData, _: &Env) {
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &EpubData, _: &EpubData, _: &Env) {
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &EpubData, _: &Env) -> Size {
        let s = bc.max();
        s
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &EpubData, _: &Env) {
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

        .with_flex_child(Button::new("Save".to_string()).on_click(|ctx, _, _| {
            ctx.submit_command(crate::core::commands::SAVE_EPUB.with(()));
        }), 0.1)
        .with_default_spacer()
        .with_flex_child(Button::new("Marker".to_string()).on_click(|_, data: &mut EpubData, _| {
            if data.selected_tool == Tool::Marker {
                data.selected_tool = Tool::default();
            }
            else {
                data.selected_tool = Tool::Marker;
            }
            println!("Data sel: {:?}", data.selected_tool);
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

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _: &EpubData, data: &EpubData, env: &druid::Env) {

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
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &style::get_color_unchecked(SECONDARY_DARK));


        self.tools.paint(ctx, data, env);
        self.toolbar_controller.paint(ctx, data, env);
        

    }
}
