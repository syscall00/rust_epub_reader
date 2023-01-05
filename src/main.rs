use crate::core::constants::APP_NAME;

use crate::core::constants::commands::{InternalUICommand, INTERNAL_COMMAND};

use crate::core::style;

use data::appstate::Delegate;
use data::home::HomePageData;
use data::AppState;
use druid::{
    widget::{Controller, Flex, List, Scroll, ViewSwitcher},
    AppLauncher, Color, Data, Env, Event, EventCtx, WidgetExt, WindowDesc,
};

use druid::Widget;

mod core;
mod data;
mod widgets;

mod dom;
mod ocr;

use widgets::{
    epub_page::{epub_controller::EpubPageController, sidebar::Sidebar},
    recent_item::RecentWidget,
    RoundButton,
};

#[derive(Data, PartialEq, Clone, Copy, Debug)]
pub enum PageType {
    Home,
    Reader,
}

fn main() {
    let data = AppState::new();
    let window = WindowDesc::new(navigator())
        .title(APP_NAME)
        .window_size((1000.0, 800.0));

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate)
        .launch(data)
        .unwrap();
}

// UI Builder functions
pub fn navigator() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.active_page,
        move |active_page, _, _| match active_page {
            PageType::Home => home_page().lens(AppState::home_page_data).boxed(),
            PageType::Reader => read_ebook().boxed(),
        },
    )
    .controller(MainController {})
}

pub fn home_page() -> impl Widget<HomePageData> {
    let list = Scroll::new(List::new(|| {
        RecentWidget::new().padding(5.0).expand_width()
    }))
    .vertical()
    .lens(HomePageData::recents);

    let title = druid::widget::Label::new(APP_NAME)
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

/**
 * Main Controller handles the navigation between UI pages.
 */
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
            Event::Command(cmd) => {
                if let Some(internal) = cmd.get(INTERNAL_COMMAND) {
                    match internal {
                        InternalUICommand::UINavigate(page) => {
                            data.active_page = *page;
                            ctx.request_layout();
                        }
                        _ => {}
                    }

                    ctx.request_layout();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}
