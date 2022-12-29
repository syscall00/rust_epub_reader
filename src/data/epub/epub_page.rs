use druid::{widget::{Controller, Flex}, EventCtx, Event, Env, Widget, WidgetExt};

use crate::{appstate::AppState, core::{constants::commands::{INTERNAL_COMMAND, InternalUICommand}, commands::NAVIGATE_TO}, epub_page, PageType, widgets::EditWidget};


pub struct EpubPageController;

impl Controller<AppState, Flex<AppState>> for EpubPageController {
    fn event(
        &mut self,
        child: &mut Flex<AppState>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                if let Some(_) = cmd.get(INTERNAL_COMMAND) {
                    match cmd.get(INTERNAL_COMMAND).unwrap() {
                        InternalUICommand::OpenOCRDialog => {
                            ctx.new_sub_window(
                                druid::WindowConfig::default()
                                    .show_titlebar(true)
                                    .set_level(druid::WindowLevel::AppWindow),
                                epub_page::generate_ui_ocr(),
                                data.epub_data.clone(),
                                env.clone(),
                            );
                            ctx.set_handled();

                        }
                        InternalUICommand::GoToMenu => {
                            // save position in the book
                            data.epub_data.update_position();
                            ctx.submit_command(NAVIGATE_TO.with(PageType::Home));
                            ctx.set_handled();

                        }
                        InternalUICommand::OpenEditDialog => {
                            if !data.epub_data.edit_mode {
                                data.epub_data.edit_mode = true;
                                let window_config = druid::WindowConfig::default();

                                ctx.new_sub_window(
                                    window_config,
                                    EditWidget::new().lens(AppState::epub_data),
                                    data.clone(),
                                    env.clone(),
                                );
                                ctx.set_handled();

                            }
                        }
                        InternalUICommand::SaveModification(path) => {
                            data.epub_data.save_new_epub(path);
                            ctx.request_update();
                            ctx.set_handled();
                        }
                        _ => {  }
                    }
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

