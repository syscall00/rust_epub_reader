use druid::widget::{Controller, Flex, TextBox};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget,
};
use druid::{Code, Data, Key, WidgetExt, WidgetPod};

use crate::PageType;
use crate::appstate::EpubData;

use crate::core::commands::{NAVIGATE_TO};
use crate::core::constants::commands::{INTERNAL_COMMAND, InternalUICommand};
use crate::widgets::epub_page::textcontainer::TextContainer;
use crate::widgets::epub_page::toolbar::Toolbar;



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
                                ).to(druid::Target::Global)
                            );
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
                        InternalUICommand::GoToMenu => {
                            ctx.submit_command(NAVIGATE_TO.with(PageType::Home));
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
                                        generate_ui_edit(),
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
