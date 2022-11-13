use druid::widget::{Axis, Controller, Flex, TextBox, ViewSwitcher};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget,
};
use druid::{Code, Data, Key, WidgetExt, WidgetPod};

use crate::appstate::EpubData;

use crate::core::commands::{self, INTERNAL_COMMAND, REQUEST_EDIT, SAVE_EPUB};
use crate::sidebar::InternalUICommand;
use crate::widgets::epub_page::navbar::NavigationBar;
use crate::widgets::epub_page::textcontainer::TextContainer;
use crate::widgets::epub_page::toolbar::Toolbar;

pub struct Container<T> {
    widgets: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    widget_origins: Vec<Point>,
    widget_size: Vec<f64>,
    axis: Axis,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            widget_origins: Vec::new(),
            widget_size: Vec::new(),
            axis: Axis::Horizontal,
        }
    }

    fn for_axis(axis: Axis) -> Self {
        Self {
            widgets: Vec::new(),
            widget_origins: Vec::new(),
            widget_size: Vec::new(),
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
        self.widget_size.push(1.);
        self
    }

    pub fn with_child_and_size(mut self, child: impl Widget<T> + 'static, size: f64) -> Self {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(Point::ORIGIN);
        self.widget_size.push(size);

        self
    }
    pub fn with_widget_and_origin(
        mut self,
        child: impl Widget<T> + 'static,
        origin: Point,
    ) -> Self {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(origin);
        self.widget_size.push(1.);

        self
    }
    pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(Point::ORIGIN);
        self.widget_size.push(1.);
    }
    pub fn add_widget_and_origin(&mut self, child: impl Widget<T> + 'static, origin: Point) {
        self.widgets.push(WidgetPod::new(Box::new(child)));
        self.widget_origins.push(origin);
        self.widget_size.push(1.);
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
        for (widget, (origin, wid_size)) in self
            .widgets
            .iter_mut()
            .zip((self.widget_origins.iter()).zip(self.widget_size.iter()))
        {
            let siz = Size::new(size.width * wid_size, size.height) - zero_origin;
            let widget_size = widget.layout(ctx, &BoxConstraints::tight(siz), data, env);
            
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
                        widget.set_origin(ctx, data, env, Point::new(zero_origin.width, 0.));
                        zero_origin.width += widget_size.width;
                    }
                    Axis::Horizontal => {
                        widget.set_origin(ctx, data, env, Point::new(0., zero_origin.height));
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
    text_field: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
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
            _ => {}
        }
        self.text_field.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.text_field.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &EpubData, data: &EpubData, env: &Env) {
        self.text_field.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
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
pub struct EditWindowController;

impl Controller<EpubData, Flex<EpubData>> for EditWindowController {
    fn event(
        &mut self,
        child: &mut Flex<EpubData>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EpubData,
        env: &Env,
    ) {
        match event {
            Event::KeyDown(k) => {
                match k.code {
                    Code::Escape => {
                        println!("Exiting");
                        //ctx.submit_command(commands::CLOSE_WINDOW.to(data.window_id));
                    }
                    // If crtl + s is pressed, save the file
                    Code::KeyS => {
                        if k.mods.ctrl() {
                            println!("Saving");
                            ctx.submit_command(INTERNAL_COMMAND.with(
                                InternalUICommand::SaveModification(
                                    data.visualized_chapter.clone(),
                                )
                            ));
                            ctx.request_update();
                        }
                    }
                    _ => {}
                }
            }

            Event::WindowCloseRequested => {
                println!("Exiting");
                //ctx.submit_command(commands::CLOSE_WINDOW.to(data.window_id));
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

pub fn generate_ui_edit() -> impl Widget<EpubData> {
    Flex::column()
        .with_child(Toolbar::new())
        .with_flex_child(
            TextBox::multiline()
                .expand()
                .lens(EpubData::visualized_chapter),
            1.,
        )
        .controller(EditWindowController {})
}

pub fn generate_ui_ocr() -> impl Widget<EpubData> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(druid::widget::Label::new("Loaded picture: "))
                .with_child(druid::widget::Button::new("Choose a picture")),
        )
        .with_child(
            Flex::row()
                //.with_child(druid::widget::Image::new(|data: &EpubData, _env: &Env| data.image.clone()))
                .with_child(
                    Flex::row().with_child(druid::widget::Label::new("Choose a text")), //.with_child(
                                                                                        //    druid::widget::List::new(|| druid::widget::Label::new("test"))
                                                                                        //)
                ),
        )
    .controller(EditWindowController{})
}

impl EpubPage {
    pub fn new(_data: EpubData) -> Self {
        let view_switcher = WidgetPod::new(ViewSwitcher::new(
            |data: &EpubData, _env: &Env| data.edit_mode,
            |edit_mode, _, _env| {
                if true {
                    let visualization_mode_switcher = TextContainer::new().expand().boxed();

                    let c = Container::new().with_child(visualization_mode_switcher);
                    //if !(false) {
                    //    c.with_widget_and_origin(NavigationBar::new(), Point::new(0.0, -50.0))
                    //} else {
                    //    c
                    //}
                    c.boxed()
                } else {
                    //Container::new()
                    //.with_child(Toolbar::new())
                    //.with_child(generate_ui_edit())
                    generate_ui_edit().boxed()
                }
            },
        ))
        .boxed();
        let switcher = Flex::column().with_flex_child(TextContainer::new().expand(), 1.);
        //.with_child(NavigationBar::new().with_height(50.));
        EpubPage {
            view_switcher: WidgetPod::new(switcher.boxed()),
        }
    }
}

impl Widget<EpubData> for EpubPage {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::Command(cmd) => {
                if let Some(_) = cmd.get(INTERNAL_COMMAND) {
                    match cmd.get(INTERNAL_COMMAND).unwrap() {
                        InternalUICommand::OpenOCRDialog => {
                            ctx.new_sub_window(
                                druid::WindowConfig::default()
                                    .show_titlebar(true)
                                    .set_level(druid::WindowLevel::AppWindow),
                                generate_ui_ocr(),
                                data.clone(),
                                env.clone(),
                            );
                        }
                        InternalUICommand::OpenEditDialog => {
                            if !data.edit_mode {
                                data.edit_mode = true;
                                // share data between current window and subwindow

                                //ctx.new_window(druid::WindowDesc::new(generate_ui_edit()));
                                let tb = Flex::column().with_child(
                                    TextBox::new().lens(EpubData::visualized_chapter))
                                    .controller(EditWindowController); //lens(SubState::my_stuff);

                                ctx.new_sub_window(
                                    druid::WindowConfig::default()
                                        .show_titlebar(true)
                                        .set_level(druid::WindowLevel::AppWindow),
                                        tb,
                                    data.clone(),
                                    env.clone(),
                                );

                                //ctx.new_sub_window(
                                //        druid::WindowConfig::default()
                                //        .set_level(druid::WindowLevel::Tooltip(ctx.window().clone()))
                                //         //.window_size_policy(druid::WindowSizePolicy::Content)
                                //         //.set_level(druid::WindowLevel::Tooltip(ctx.window().clone()))
                                //         .show_titlebar(true),
                                //
                                //         generate_ui_edit(),
                                //     data.clone(),
                                //     env.clone(),
                                //);

                                //    ctx.new_sub_window(
                                //        druid::WindowConfig::default()
                                //        .set_level(druid::WindowLevel::Tooltip(ctx.window().clone()))
                                //
                                //    , generate_ui_edit(),
                                //        data.clone(), env.clone());
                            }
                            //data.edit_mode = !data.edit_mode;
                            //ctx.request_layout();
                        }
                        InternalUICommand::SaveModification(edit_data) => {
                            println!("saving modification");
                            data.visualized_chapter = edit_data.clone();
                            data.save_new_epub();
                            data.edit_mode = !data.edit_mode;
                            ctx.request_update();
                            ctx.set_handled();
                        }
                        _ => {}
                    }
                    ctx.set_handled();
                } else if cmd.is(SAVE_EPUB) {
                    // first, save epup; then go to visualization mode
                    data.save_new_epub();
                    data.edit_mode = !data.edit_mode;
                    ctx.request_update();
                    ctx.set_handled();
                }
            }
            _ => {}
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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        self.view_switcher
            .layout(ctx, &BoxConstraints::new(bc.min(), bc.max()), data, env);
        self.view_switcher.set_origin(ctx, data, env, Point::ORIGIN);

        return bc.max();
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        self.view_switcher.paint(ctx, data, env);
    }
}
