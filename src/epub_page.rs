use druid::widget::{Controller, Flex};
use druid::{
    Env, Event, EventCtx, Widget,
};
use druid::Code;
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



