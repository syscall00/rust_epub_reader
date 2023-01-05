use druid::{
    widget::{TextBox, Controller, Flex}, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
    WindowId, WindowSizePolicy, Code,
};
use druid_material_icons::IconPaths;

use crate::{
    core::{constants::commands::{InternalUICommand, INTERNAL_COMMAND, MODIFY_EPUB_PATH}, style}, data::epub::edit_data::EditData,
};

use crate::widgets::common::icon_button::{IconButton, ButtonTrait};


pub struct EditWidget {
    dirty: bool,
    new_path: String,

    text: WidgetPod<EditData, Box<dyn Widget<EditData>>>,
    toolbar: WidgetPod<EditData, Box<dyn Widget<EditData>>>,
}

fn toolbar() ->  impl Widget<EditData> {

    Flex::row()
    .with_child(IconButton::new(ToolbarButton::Save))
    .with_child(IconButton::new(ToolbarButton::SaveAs))
    .with_child(IconButton::new(ToolbarButton::Exit))
    
}

impl EditWidget {
    pub fn new() -> Self {
        let text = TextBox::multiline()
            .lens(EditData::visualized_chapter)
            .boxed();

        EditWidget {
            dirty: false,
            new_path: String::new(),

            text: WidgetPod::new(text),
            toolbar: WidgetPod::new(toolbar().boxed()),
        }
    }

    fn save_command(&mut self, ctx: &mut EventCtx) {
        if self.dirty {
            if self.new_path.is_empty() {
                self.open_save_dialog(ctx);
            } else {
                self.send_save_modification_command(ctx);
            }
        }
    }

    fn open_save_dialog(&mut self, ctx: &mut EventCtx) {
        let filedialog = druid::FileDialogOptions::default()
            .accept_command(MODIFY_EPUB_PATH)
            .title("Save as ...");

        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL.with(
                filedialog.allowed_types(vec![druid::FileSpec::new("Epub (.epub)", &["epub"])]),
            ),
        );
    }

    fn send_save_modification_command(&mut self, ctx: &mut EventCtx) {
        self.dirty = false;

        ctx.submit_command(
            INTERNAL_COMMAND
                .with(InternalUICommand::SaveModification(self.new_path.clone()))
                .to(druid::Target::Global),
        );
    }
}

#[derive(Debug, Clone)]
pub enum PromptOption {
    Yes,
    No,
    #[allow(dead_code)]
    Cancel,
    #[allow(dead_code)]
    Abort,
}
impl PromptOption {
    fn to_string(&self) -> String {
        match self {
            PromptOption::Yes => "Yes",
            PromptOption::No => "No",
            PromptOption::Cancel => "Cancel",
            PromptOption::Abort => "Abort",
        }
        .to_string()
    }
}

fn dialog_ui(message: String, parent_id: WindowId) -> impl Widget<()> {
    let chooses = vec![PromptOption::Yes, PromptOption::No];

    let mut widget = druid::widget::Flex::column().with_child(druid::widget::Label::new(
        message,
    ));
    let mut row = druid::widget::Flex::row();

    chooses.into_iter().for_each(|c| {
        row.add_child(druid::widget::Button::new(c.to_string()).on_click(
            move |ctx, &mut (), _| {
                ctx.submit_command(
                    INTERNAL_COMMAND
                        .with(InternalUICommand::PromptEditSave(c.clone()))
                        .to(druid::Target::Window(parent_id.clone())),
                );

                // self close
                ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
            },
        ));
        row.add_default_spacer()
    });
    widget = widget.with_default_spacer().with_child(row);
    widget
}

impl Widget<EditData> for EditWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditData, env: &Env) {
        match event {
            // if window is closing, save the if it's dirty
            Event::WindowCloseRequested => {
                data.set_editing(false);
                if self.dirty {
                    // position of the window at the center of the current window
                    let window_pos = ctx.window().get_position();
                    let window_size = ctx.window().get_size();
                    let dialog_pos = Point::new(
                        window_pos.x + window_size.width / 2.0,
                        window_pos.y + window_size.height / 2.0,
                    );
                    let window_config = druid::WindowConfig::default()
                        .window_size_policy(WindowSizePolicy::Content)
                        .set_position(dialog_pos);

                    let widget = dialog_ui("Do you want to save the changes?".to_string(), ctx.window_id());
                    ctx.new_sub_window(window_config, widget, (), env.clone());
                    ctx.set_handled();
                } else {
                    data.set_editing(false);
                }
            }

            Event::Command(cmd) => {
                if let Some(file_info) = cmd.get(MODIFY_EPUB_PATH) {
                    self.new_path = file_info.path().to_str().unwrap().to_owned();
                    self.send_save_modification_command(ctx);
                } else if let Some(cmd) = cmd.get(INTERNAL_COMMAND) {
                    match cmd {
                        InternalUICommand::RequestSaveEdit => {
                            self.save_command(ctx);
                        }
                        InternalUICommand::SaveEditAs => {
                            self.open_save_dialog(ctx);
                        }
                        InternalUICommand::CloseEdit => {
                            ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
                        }
                        InternalUICommand::PromptEditSave(prompt_option) => {
                            let should_close = match prompt_option {
                                PromptOption::Yes => {
                                    self.save_command(ctx);
                                    true
                                }
                                PromptOption::No => true,
                                _ => false,
                            };
                            if should_close {
                                self.dirty = false;

                                ctx.submit_command(
                                    druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }

            Event::KeyUp(key_event) => match key_event.key {
                druid::keyboard_types::Key::Character(_)
                | druid::keyboard_types::Key::Delete
                | druid::keyboard_types::Key::Symbol
                | druid::keyboard_types::Key::Enter
                | druid::keyboard_types::Key::Backspace
                | druid::keyboard_types::Key::Clear
                | druid::keyboard_types::Key::Paste
                | druid::keyboard_types::Key::Cancel => {
                    self.dirty = true;
                }

                _ => {}
            },

            _ => {}
        }

        self.text.event(ctx, event, data, env);
        self.toolbar.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EditData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                //ctx.request_focus();
            }
            _ => {}
        }
        self.text.lifecycle(ctx, event, data, env);
        self.toolbar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EditData, data: &EditData, env: &Env) {
        ctx.request_paint();
        self.text.update(ctx, data, env);
        self.toolbar.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EditData,
        env: &Env,
    ) -> Size {
        let mut size = bc.max();

        let toolbar_size = self
            .toolbar
            .layout(ctx, &BoxConstraints::tight(Size::new(bc.max().width, 30.)), data, env);

        size.height -= toolbar_size.height;
        self.toolbar.set_origin(ctx, data, env, Point::ORIGIN);

        self.text
            .layout(ctx, &BoxConstraints::tight(size), data, env);
        self.text
            .set_origin(ctx, data, env, (0.0, toolbar_size.height).into());

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditData, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &style::get_color_unchecked(style::PRIMARY_LIGHT));
        self.toolbar.paint(ctx, data, env);
        self.text.paint(ctx, data, env);

    }
}


pub enum ToolbarButton {
    Save,
    SaveAs,
    Exit,
}

impl ButtonTrait for ToolbarButton {
    fn icon(&self) -> IconPaths {
        match self {
            ToolbarButton::Save => druid_material_icons::normal::content::SAVE,
            ToolbarButton::SaveAs => druid_material_icons::normal::content::SAVE_AS,
            ToolbarButton::Exit => druid_material_icons::normal::content::CLEAR,
        }
    }

    fn hint(&self) -> String {
        match self {
            ToolbarButton::Save => "Save".to_owned(),
            ToolbarButton::SaveAs => "Save As".to_owned(),
            ToolbarButton::Exit => "Exit".to_owned(),
        }
    }

    fn command(&self) -> InternalUICommand {
        match self {
            ToolbarButton::Save => InternalUICommand::RequestSaveEdit,
            ToolbarButton::SaveAs => InternalUICommand::SaveEditAs,
            ToolbarButton::Exit => InternalUICommand::CloseEdit,
        }
    }
}


pub struct EditWindowController;


impl Controller<EditData, Flex<EditData>> for EditWindowController {
    fn event(
        &mut self,
        child: &mut Flex<EditData>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditData,
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



