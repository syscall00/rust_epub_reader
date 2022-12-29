
use crate::core::commands::NAVIGATE_TO;
use crate::core::style;

use appstate::AppState;
use data::{home::HomePageData, epub::EpubPageController};
use druid::{
    widget::{Controller, Flex, List, Scroll, ViewSwitcher},
    AppLauncher, Color, Data, Env, Event, EventCtx, WidgetExt, WindowDesc,
};

use druid::Widget;

mod appstate;
mod core;
mod data;
mod epub_page;
mod widgets;

mod dom;


use widgets::{recent_item::RecentWidget, epub_page::sidebar::Sidebar};
use widgets::RoundButton;

//use druid_material_icons::normal::action::*;

#[derive(Data, PartialEq, Clone, Copy)]
pub enum PageType {
    Home,
    Reader,
}

fn main() {
    // starting from icon_action file, create a widget list with all the icons
    // read file icons_action

    // home, VERTICAL_SPLIT
    // communicationh IMPORT_EXPORT / HUB
    // device::SUMMARIZE
    // editor::FORMAT_LIST_BULLETED (toc) ;

    // druid_material_icons::normal::editor::TEXT_DECREASE
    // druid_material_icons::normal::editor::TEXT_FIELDS
    // druid_material_icons::normal::editor::TEXT_INCREASE

    // druid_material_icons::normal::editor::FORMAT_INDENT_DECREASE
    // druid_material_icons::normal::editor::FORMAT_INDENT_INCREASE
    // druid_material_icons::normal::editor::FORMAT_LINE_SPACING

    //druid_material_icons::normal::editor::VERTICAL_ALIGN_BOTTOM
    //druid_material_icons::normal::editor::VERTICAL_ALIGN_CENTER
    //druid_material_icons::normal::editor::VERTICAL_ALIGN_TOP

    // two book druid_material_icons::normal::image::AUTO_STORIES
    // one book? druid_material_icons::normal::image::CROP_PORTRAIT

    // gear druid_material_icons::normal::image::TUNE

    //return;
    let data = AppState::new();
    let window = WindowDesc::new(navigator())
        .title("Epub Rust Reader")
        .window_size((1000.0, 800.0));

    //let window = WindowDesc::new(build_ui());
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(appstate::Delegate)
        .launch(data)
        .unwrap();
}

//pub fn build_ui() -> impl Widget<()> {
//    let mut flex = Flex::column();
//
//    //let mut icons_from_file =
//    let v = vec![druid_material_icons::normal::toggle::CHECK_BOX, druid_material_icons::normal::toggle::CHECK_BOX_OUTLINE_BLANK, druid_material_icons::normal::toggle::INDETERMINATE_CHECK_BOX, druid_material_icons::normal::toggle::RADIO_BUTTON_CHECKED, druid_material_icons::normal::toggle::RADIO_BUTTON_UNCHECKED, druid_material_icons::normal::toggle::STAR, druid_material_icons::normal::toggle::STAR_BORDER, druid_material_icons::normal::toggle::STAR_BORDER_PURPLE500, druid_material_icons::normal::toggle::STAR_HALF, druid_material_icons::normal::toggle::STAR_OUTLINE, druid_material_icons::normal::toggle::STAR_PURPLE500, druid_material_icons::normal::toggle::TOGGLE_OFF, druid_material_icons::normal::toggle::TOGGLE_ON];
//    for i in 0..v.len() {
//        flex.add_child(Flex::row().with_child(widgets::widgets::Icon::new(v[i])).with_child(druid::widget::Label::new(i.to_string())));
//    }
//    Scroll::new(flex)
//}



// UI Builder functions
pub fn navigator() -> impl Widget<AppState> {

    ViewSwitcher::new(
        |data: &AppState, _env| data.active_page,
        move |active_page, _, _ | match active_page {
            PageType::Home => home_page().lens(AppState::home_page_data).boxed(),
            PageType::Reader => read_ebook().boxed(),
        },
    )
    .controller(MainController {})
    .env_scope(|env, _: &AppState| {
        style::add_to_env(env);
    })
}

pub fn home_page() -> impl Widget<HomePageData> {
    let list = Scroll::new(List::new(|| RecentWidget::new().padding(5.0).expand_width()))
        .vertical()
        .lens(HomePageData::recents);

    let title = druid::widget::Label::new("Rust Ebook Reader")
        .with_text_size(26.0)
        .with_text_color(Color::WHITE)
        .center();

    let open_epub = RoundButton::new(druid_material_icons::normal::content::ADD_CIRCLE)
        .with_click_handler(|event, _, _env| {
            let filedialog = druid::FileDialogOptions::new();

            event.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
                filedialog.allowed_types(vec![druid::FileSpec::new("Epub (.epub)", &["epub"])]),
            ));
        })
        .with_radius(40.);

    let layout = Flex::column()
        .with_child(
            Flex::row()
                .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
                .with_child(title)
                .with_child(open_epub)
                .expand_width(),
        )
        .with_flex_child(list, 1.);
    druid::widget::Container::new(layout)
        .background(style::get_color_unchecked(style::PRIMARY_DARK))
}

pub fn read_ebook() -> impl Widget<AppState> {
    let ret = widgets::epub_page::textcontainer::TextContainer::new().lens(AppState::epub_data);

    let flex = Flex::row()
        .with_child(Sidebar::new().lens(AppState::epub_data))
        .with_flex_child(ret.expand(), 1.);
    flex.controller(EpubPageController {})
}




struct MainController;
impl Controller<AppState, ViewSwitcher<AppState, PageType>> for MainController {
    fn event(
        &mut self,
        child: &mut ViewSwitcher<AppState, PageType>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NAVIGATE_TO) => {
                let page = cmd.get_unchecked(NAVIGATE_TO);
                data.active_page = page.to_owned();

                ctx.request_layout();
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}


