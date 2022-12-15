use druid::{
    widget::TextBox, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
    WindowId, WindowSizePolicy,
};
use druid_material_icons::IconPaths;

use crate::{
    appstate::EpubData,
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND, MODIFY_EPUB_PATH},
    sidebar::CustomButton,
};

pub struct EditWidget {
    dirty: bool,
    new_path: String,

    text: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    toolbar: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}

impl EditWidget {
    pub fn new() -> Self {
        let text = TextBox::multiline()
            .lens(EpubData::visualized_chapter)
            .boxed();

        EditWidget {
            dirty: false,
            new_path: String::new(),

            text: WidgetPod::new(text),
            toolbar: WidgetPod::new(Toolbar::new().boxed()),
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

fn dialog_ui(parent_id: WindowId) -> impl Widget<()> {
    let chooses = vec![PromptOption::Yes, PromptOption::No];

    let mut widget = druid::widget::Flex::column().with_child(druid::widget::Label::new(
        "Do you want to save the changes?",
    ));
    chooses.into_iter().for_each(|c| {
        widget.add_child(druid::widget::Button::new(c.to_string()).on_click(
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
        widget.add_default_spacer()
    });
    widget
}

impl Widget<EpubData> for EditWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            // if window is closing, save the if it's dirty
            Event::WindowCloseRequested => {
                data.edit_mode = false;
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
                        //.set_level(WindowLevel::Tooltip(ctx.window().clone()))
                        .set_position(dialog_pos);

                    let widget = dialog_ui(ctx.window_id());
                    ctx.new_sub_window(window_config, widget, (), env.clone());
                    ctx.set_handled();
                } else {
                    data.edit_mode = false;
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

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                //ctx.request_focus();
            }
            _ => {}
        }
        self.text.lifecycle(ctx, event, data, env);
        self.toolbar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EpubData, data: &EpubData, env: &Env) {
        ctx.request_paint();
        self.text.update(ctx, data, env);
        self.toolbar.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        let mut size = bc.max();
        ctx.set_paint_insets((0.0, 0.0, 0.0, 0.0));

        let toolbar_size = self
            .toolbar
            .layout(ctx, &BoxConstraints::tight(size), data, env);

        size.height -= toolbar_size.height;
        self.toolbar.set_origin(ctx, data, env, Point::ORIGIN);

        self.text
            .layout(ctx, &BoxConstraints::tight(size), data, env);
        self.text
            .set_origin(ctx, data, env, (0.0, toolbar_size.height).into());

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::rgb8(0x00, 0x00, 0x00));
        self.text.paint(ctx, data, env);
        self.toolbar.paint(ctx, data, env);
    }
}

// toolbar widget
pub struct Toolbar {
    pub buttons: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
}

impl Toolbar {
    pub fn new() -> Self {
        let mut buttons = Vec::new();
        for kind in vec![
            ToolbarButton::Save,
            ToolbarButton::SaveAs,
            ToolbarButton::Exit,
        ] {
            buttons.push(WidgetPod::new(CustomButton::new(kind).boxed()));
        }

        Toolbar { buttons }
    }
}

impl Widget<EpubData> for Toolbar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        for button in self.buttons.iter_mut() {
            button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        for button in self.buttons.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EpubData, data: &EpubData, env: &Env) {
        for button in self.buttons.iter_mut() {
            button.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        let mut size = Size::ZERO;
        for button in self.buttons.iter_mut() {
            let button_size = button.layout(ctx, bc, data, env);
            button.set_origin(ctx, data, env, Point::ORIGIN + (size.width, 0.0));
            size.width += button_size.width;
            size.height = button_size.height;
        }

        println!("Toolbar size: {:?}", size);
        Size::new(bc.max().width, size.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        for button in self.buttons.iter_mut() {
            button.paint(ctx, data, env);
        }
    }
}

pub enum ToolbarButton {
    Save,
    SaveAs,
    Exit,
}

impl crate::sidebar::ButtonTrait for ToolbarButton {
    fn icon(&self) -> IconPaths {
        match self {
            ToolbarButton::Save => druid_material_icons::normal::communication::LIST_ALT,
            ToolbarButton::SaveAs => druid_material_icons::normal::communication::LIST_ALT,
            ToolbarButton::Exit => druid_material_icons::normal::communication::LIST_ALT,
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
