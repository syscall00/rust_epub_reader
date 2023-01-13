use druid::{
    widget::{Container, Controller, Flex, Label, Spinner},
    Env, Event, EventCtx, Insets, Widget, WidgetExt,
};

use crate::{
    core::{
        constants::commands::{
            InternalUICommand, INTERNAL_COMMAND, OPEN_OCR_FILE, OPEN_REVERSE_OCR_1,
            OPEN_REVERSE_OCR_2,
        },
        style,
    },
    data::{
        epub::ocr_data::{OcrData, OcrMode, EMPTY_STRING},
        PagePosition,
    },
    widgets::RoundButton,
};
/**
 * Ocr is a widget that displays the popup for OCR.
 * It can be used for both the search and the reverse search.
 * 
 */
pub fn build_ocr_ui() -> impl Widget<OcrData> {
    let main_tabs = druid::widget::Tabs::new()
        .with_axis(druid::widget::Axis::Horizontal)
        .with_edge(druid::widget::TabsEdge::Leading)
        .with_transition(Default::default())
        .with_tab("OCR Search", find_by_photo())
        .with_tab("Reverse OCR", find_by_virtual())
        .with_tab_index(0)
        .background(style::get_color_unchecked(style::PRIMARY_DARK));

    main_tabs.controller(OcrController)
}

fn open_choose_dialog(ctx: &mut druid::EventCtx, _: &mut OcrData, _env: &druid::Env) {
    let accept_command = OPEN_OCR_FILE;

    let filedialog = druid::FileDialogOptions::new().accept_command(accept_command);

    ctx.submit_command(
        druid::commands::SHOW_OPEN_PANEL.with(filedialog.allowed_types(vec![
            druid::FileSpec::new("Image (.jpg, png)", &["jpg", "png"]),
        ])),
    );
}

fn find_by_photo() -> impl Widget<OcrData> {
    let choose_pic = Flex::row()
        .with_child(
            (RoundButton::new(druid_material_icons::normal::image::ADD_PHOTO_ALTERNATE)
                .with_radius(15.)
                .with_border_color(druid::Color::RED))
            .on_click(open_choose_dialog)
            .padding(10.0),
        )
        .with_child(druid::widget::RawLabel::new().lens(OcrData::image_to_pos))
        .expand_width();

    let confirm_button = druid::widget::Button::new("Start OCR")
        .on_click(|ctx, data: &mut OcrData, _| {
            ctx.submit_command(
                INTERNAL_COMMAND
                    .with(InternalUICommand::RequestOCRSearch(
                        data.image_to_pos.clone(),
                    ))
                    .to(druid::Target::Global),
            );
            data.processing = true;
            ctx.request_update();
        })
        .disabled_if(|data: &OcrData, _| data.image_to_pos == EMPTY_STRING)
        .fix_size(100., 35.)
        .padding(Insets::new(0., 0., 0., 10.));

    let result = Flex::row()
        .with_child(druid::widget::Label::new(
            |data: &OcrData, _env: &druid::Env| data.ocr_result.to_string(),
        ))
        .with_child(
            (RoundButton::new(druid_material_icons::normal::action::ARROW_RIGHT_ALT)
                .with_radius(15.)
                .with_border_color(druid::Color::GRAY))
            .on_click(|ctx, data: &mut OcrData, _| {
                // go to pos
                ctx.submit_command(
                    INTERNAL_COMMAND
                        .with(InternalUICommand::EpubGoToPos(data.ocr_result.clone()))
                        .to(druid::Target::Global),
                );
                ctx.request_update();
            })
            .padding(10.0),
        )
        .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
        .expand_width();

    let either = druid::widget::Either::new(
        |data: &OcrData, _env: &druid::Env| data.processing,
        Flex::column()
            .with_child(Label::new("Processing...").padding(5.0))
            .with_child(Spinner::new())
            .center(),
        result,
    );

    let either_result = druid::widget::Either::new(
        |data: &OcrData, _env: &druid::Env| {
            data.ocr_result != PagePosition::default() || data.processing
        },
        either,
        druid::widget::Label::new(""),
    );
    Flex::column()
        .with_child(explanation_text(&OcrMode::FindByPhoto))
        .with_child(choose_pic)
        .with_child(confirm_button)
        .with_child(either_result)
        .boxed()
}

fn find_by_virtual() -> impl Widget<OcrData> {
    let choose_pic = Flex::row()
        .with_child(
            (RoundButton::new(druid_material_icons::normal::image::ADD_PHOTO_ALTERNATE)
                .with_radius(15.)
                .with_border_color(druid::Color::RED))
            .on_click(|ctx, _: &mut OcrData, _| {
                let filedialog = druid::FileDialogOptions::new().accept_command(OPEN_REVERSE_OCR_1);

                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
                    filedialog.allowed_types(vec![druid::FileSpec::new(
                        "Image (.jpg, png)",
                        &["jpg", "png"],
                    )]),
                ));
                ctx.request_update();
            })
            .padding(10.0),
        )
        .with_child(druid::widget::RawLabel::new().lens(OcrData::image_for_pos_1))
        .expand_width();

    let choose_pic2 = Flex::row()
        .with_child(
            (RoundButton::new(druid_material_icons::normal::image::ADD_PHOTO_ALTERNATE)
                .with_radius(15.)
                .with_border_color(druid::Color::RED))
            .on_click(|ctx, _: &mut OcrData, _| {
                let filedialog = druid::FileDialogOptions::new().accept_command(OPEN_REVERSE_OCR_2);

                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
                    filedialog.allowed_types(vec![druid::FileSpec::new(
                        "Image (.jpg, png)",
                        &["jpg", "png"],
                    )]),
                ));
                ctx.request_update();
            })
            .padding(10.0),
        )
        .with_child(druid::widget::RawLabel::new().lens(OcrData::image_for_pos_2))
        .expand_width();

    let confirm_button = druid::widget::Button::new("Reverse OCR")
        .on_click(|ctx, data: &mut OcrData, _| {
            ctx.submit_command(
                INTERNAL_COMMAND
                    .with(InternalUICommand::RequestReverseOCR((
                        data.image_for_pos_1.clone(),
                        data.image_for_pos_2.clone(),
                    )))
                    .to(druid::Target::Global),
            );
            data.processing = true;
            ctx.request_update();
        })
        .disabled_if(|data: &OcrData, _| {
            data.image_for_pos_1 == EMPTY_STRING || data.image_for_pos_2 == EMPTY_STRING
        })
        .fix_size(120., 35.)
        .padding(Insets::new(0., 0., 0., 10.));

    let result = Flex::row()
        .with_child(druid::widget::Label::new(
            |data: &OcrData, _env: &druid::Env| format!("Current virtual page is at physical page {} ", data.reverse_ocr_result)
        ))
        .center();

    let either = druid::widget::Either::new(
        |data: &OcrData, _env: &druid::Env| data.processing,
        Flex::column()
            .with_child(Label::new("Processing...").padding(5.0))
            .with_child(Spinner::new())
            .center(),
        result,
    );

    let either_result = druid::widget::Either::new(
        |data: &OcrData, _env: &druid::Env| {
            data.reverse_ocr_result != usize::MAX || data.processing
        },
        either,
        druid::widget::Label::new(""),
    );
    Flex::column()
        .with_child(explanation_text(&OcrMode::FindByVirtual))
        .with_child(choose_pic)
        .with_child(choose_pic2)
        .with_child(confirm_button)
        .with_child(either_result)
}

fn explanation_text(mode: &OcrMode) -> impl Widget<OcrData> {
    match mode {
        OcrMode::FindByPhoto => druid::widget::Label::new("OCR Search"),
        OcrMode::FindByVirtual => druid::widget::Label::new("OCR Reverse Search"),
    }
    .with_text_size(22.)
    .expand_width()
    .padding(druid::Insets::new(5., 5., 5., 15.))
}



pub struct OcrController;

impl Controller<OcrData, Container<OcrData>> for OcrController {
    fn event(
        &mut self,
        child: &mut Container<OcrData>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut OcrData,
        env: &Env,
    ) {
        match event {
            druid::Event::Command(cmd) => {
                if let Some(file_info) = cmd.get(OPEN_OCR_FILE) {
                    data.image_to_pos = file_info.path().to_str().unwrap().to_string();
                    ctx.request_update();
                    ctx.set_handled();
                } else if let Some(internal) = cmd.get(INTERNAL_COMMAND) {
                    match internal {
                        InternalUICommand::OCRSearchCompleted(pos) => {
                            data.ocr_result = pos.to_owned();
                            data.processing = false;
                        }
                        InternalUICommand::ReverseOCRCompleted(pos) => {
                            data.reverse_ocr_result = *pos;
                            data.processing = false;
                        }
                        _ => {}
                    }
                    ctx.request_update();
                } else if let Some(file_info) = cmd.get(OPEN_REVERSE_OCR_1) {
                    data.image_for_pos_1 = file_info.path().to_str().unwrap().to_string();
                    ctx.request_update();
                    ctx.set_handled();
                } else if let Some(file_info) = cmd.get(OPEN_REVERSE_OCR_2) {
                    data.image_for_pos_2 = file_info.path().to_str().unwrap().to_string();
                    ctx.request_update();
                    ctx.set_handled();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
