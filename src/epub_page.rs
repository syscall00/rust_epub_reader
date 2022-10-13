

use druid::widget::{ViewSwitcher, Flex, TextBox, Split, Axis};
use druid::{WidgetPod, WidgetExt, Data};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget,
};

use crate::appstate::{EpubData, AppState};


use crate::core::commands::{REQUEST_EDIT, CHANGE_VISUALIZATION, VisualizationMode};
use crate::widgets::epub_page::navbar::NavigationBar;
use crate::widgets::epub_page::textcontainer::{TextContainer, TwoView};
use crate::widgets::epub_page::toolbar::Toolbar;

pub struct Container<T>  {
    widgets: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    widget_origins: Vec<Point>,  
    axis: Axis,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            widget_origins: Vec::new(),
            axis: Axis::Horizontal,
        }
    }

    fn for_axis(axis: Axis) -> Self {
        Self {
            widgets: Vec::new(),
            widget_origins: Vec::new(),
            axis,
        }
    }

    pub fn column() -> Self {
        Self::for_axis(Axis::Vertical)
    }
    pub fn row() -> Self {
        Self::for_axis(Axis::Horizontal)
    }

    pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(Point::ORIGIN);
        self
    }

    pub fn with_widget_and_origin(mut self, child: impl Widget<T> + 'static, origin: Point) -> Self {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(origin);
        self
    }
    pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(Point::ORIGIN);
    }
    pub fn add_widget_and_origin(&mut self, child: impl Widget<T> + 'static, origin: Point) {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(origin);
    }

}


impl<T: Data> Widget<T> for Container<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old: &T, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = bc.max();
        
        let mut zero_origin = Size::ZERO;
        for (widget, origin) in self.widgets.iter_mut().zip(self.widget_origins.iter()) {
            let widget_size = widget.layout(ctx, &BoxConstraints::tight(size-zero_origin), data, env);
            let mut orig = *origin;

            if orig.x < 0. {
                orig.x = size.width + orig.x;
            }
            if origin.y < 0. {
                orig.y = size.height + origin.y;
            } 

            // If the widget has origin 0, place it accounting others 0-origin widgets
            if *origin == Point::ORIGIN {
                match self.axis {
                    Axis::Vertical => {
                        widget.set_origin(ctx, data, env,Point::new(zero_origin.width, 0.));
                        zero_origin.width += widget_size.width;        
                   },
                    Axis::Horizontal => {
                        widget.set_origin(ctx, data, env,Point::new(0., zero_origin.height));
                        zero_origin.height += widget_size.height;
                    }
                }

            } else {
                widget.set_origin(ctx, data, env, orig);
            }
        }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.paint(ctx, data, env);
        }
    }
}


pub struct EditPage {
    text_field : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,

}

impl EditPage {
    pub fn new() -> Self {
        Self {
            text_field: WidgetPod::new(TextBox::new().lens(EpubData::visualized_chapter).boxed()),
        }
    }
}

impl Widget<EpubData> for EditPage {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            _ => {
            }
        }
        self.text_field.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.text_field.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        self.text_field.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let s = self.text_field.layout(ctx, bc, data, env);
        self.text_field.set_origin(ctx, data, env, Point::ORIGIN);
        s
        
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        self.text_field.paint(ctx, data, env);
    }
}



pub struct EpubPage {
    view_switcher: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}

impl EpubPage {
    pub fn new(_data : EpubData) -> Self {

        let view_switcher = WidgetPod::new(ViewSwitcher::new(
            |data: &EpubData, _env: &Env| data.edit_mode,
            |edit_mode, data, _env| {
                if !*edit_mode {

                    let visualization_mode_switcher = ViewSwitcher::new(
                        |data: &EpubData, _env: &Env| data.visualization_mode.clone(),
                        |visualization_mode, data, _env| {
                            match *visualization_mode {
                                VisualizationMode::Single => TextContainer::new(data.clone()).expand().boxed(),
                                VisualizationMode::Two => TwoView::new().boxed(),
                                VisualizationMode::Scroll => todo!(),
                            }
                        }
                    );

                    let c = Container::new()
                    .with_child(Toolbar::new())
                    .with_child(visualization_mode_switcher);
                    if !(data.visualization_mode == VisualizationMode::Scroll) {
                        c.with_widget_and_origin(NavigationBar::new(), Point::new(0.0, -50.0))
                    } else {
                        c
                    }
                    .boxed()

                } else {
                    Container::new()
                    .with_child(Toolbar::new())
                    .with_child(EditPage::new())
                    .boxed()
                    
                }
            },
        )).boxed();
        EpubPage {
            view_switcher
        }
    }
}

impl Widget<EpubData> for EpubPage {
    
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {

        match event {
            Event::Command(cmd) => {
                if let Some(_) = cmd.get(REQUEST_EDIT) {
                    data.edit_mode = !data.edit_mode;
                    ctx.request_update();
                    ctx.set_handled();
                }

                if let Some(visualization) = cmd.get(CHANGE_VISUALIZATION) {
                    //data.visualized_chapter = visualization.clone();
                    data.visualization_mode = visualization.clone();
                    println!("Visualization mode changed to {:?}", data.visualization_mode);
                    ctx.request_update();
                    ctx.set_handled();
                }
            },
            _ => { }
        }

        self.view_switcher.event(ctx, event, data, env);



   }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.view_switcher.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        if !old_data.same(data) {
            self.view_switcher.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        self.view_switcher.layout(ctx, &BoxConstraints::new(bc.min(), bc.max()), data, env);
        self.view_switcher.set_origin(ctx, data, env, Point::ORIGIN);

        return bc.max();
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        

        self.view_switcher.paint(ctx, data, env);
    }



}
