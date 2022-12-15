use druid::widget::{Controller, Flex, TextBox};
use druid::{
    Env, Event, EventCtx, Widget,
};
use druid::{Code, WidgetExt};

use crate::appstate::EpubData;

use crate::core::constants::commands::{INTERNAL_COMMAND, InternalUICommand};



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

pub fn _generate_ui_edit() -> impl Widget<EpubData> {
    Flex::column()
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
                .with_child(druid::widget::RawLabel::new().lens(EpubData::chapter_title))
                .with_child(druid::widget::Button::new("Choose a picture")).on_click(|ctx, _data, _| {
                    // open a file dialog
                    let filedialog = druid::FileDialogOptions::new();

                    ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
                        filedialog.allowed_types(vec![druid::FileSpec::new("Image (.jpg, png)", &["jpg", "png"])]),
                    ));
                            
                }))
        .with_child(
            Flex::row()
                //.with_child(druid::widget::Image::new(|data: &EpubData, _env: &Env| data.image.clone()))
                .with_child(
                    Flex::row().with_child(druid::widget::Label::new("Choose a text")), //.with_child(
                ),
        )
    .controller(EditWindowController{})
}

