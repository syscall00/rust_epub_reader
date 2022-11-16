use crate::core::constants::commands::{InternalUICommand, INTERNAL_COMMAND};

use crate::core::commands::NAVIGATE_TO;
use crate::core::style;

use appstate::{AppState, EpubData, Recent};
use data::home::HomePageData;
use druid::{
    widget::{Button, Controller, Flex, List, Scroll, ViewSwitcher},
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx,
    Point, RenderContext, Size, TextLayout, WidgetExt,
    WidgetPod, WindowDesc,
};

use druid::Widget;
// use druid_widget_nursery::material_icons::{Icon, normal::action::ALARM_ADD};

use epub::doc::EpubDoc;
use epub_page::EpubPage;
use sidebar::Sidebar;
mod appstate;
mod core;
mod data;
mod epub_page;
mod sidebar;
mod tool;
mod widgets;


use druid_material_icons::normal::action::*;
use widgets::widgets::RoundButton;
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
    let window = WindowDesc::new(navigator(data.clone()))
        .title("Navigation")
        .window_size((1000.0, 800.0));


    //let window = WindowDesc::new(build_ui());
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(appstate::Delegate)
        .launch(data)
        //.launch(())
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

pub fn navigator(data: AppState) -> impl Widget<AppState> {
    let _topbar = crate::widgets::home_page::topbar::Topbar::new();

    ViewSwitcher::new(
        |data: &AppState, _env| data.active_page,
        move |active_page, _data, _env| match active_page {
            PageType::Home => home_page().lens(AppState::home_page_data).boxed(),
            PageType::Reader => read_ebook(data.epub_data.clone()).boxed(),
        },
    )
    .controller(MainController {})
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

pub fn read_ebook(data: EpubData) -> Box<dyn Widget<AppState>> {
    Flex::row()
        .with_child(Sidebar::new().lens(AppState::epub_data))
        .with_flex_child(EpubPage::new(data).expand().lens(AppState::epub_data), 1.)
        .boxed()
}

struct ListItems {
    title_label: druid::TextLayout<String>,
    creator_label: TextLayout<String>,
    publisher_label: TextLayout<String>,
    position_in_book_label: TextLayout<String>,

    image: WidgetPod<Recent, Box<dyn Widget<Recent>>>,
    remove_button : WidgetPod<(), Box<dyn Widget<()>>>,
}

impl ListItems {
    pub fn new() -> Self {
        let title_label = druid::TextLayout::default();
        let creator_label = druid::TextLayout::default();
        let publisher_label = druid::TextLayout::default();
        let position_in_book_label = druid::TextLayout::default();

        let image = WidgetPod::new(
            druid::widget::ViewSwitcher::new(
                |data: &Recent, _env| data.image_data.is_some(),
                |image, _data, _env| match image {
                    true => druid::widget::Image::new(_data.image_data.clone().unwrap()).boxed(),
                    false => Flex::column()
                        .with_child(
                            druid::widget::Label::new(String::from("Loading...")).padding(5.0),
                        )
                        .with_child(druid::widget::Spinner::new())
                        .boxed(),
                },
            )
            .boxed(),
        );

        let remove_button = WidgetPod::new(
            RoundButton::new(druid_material_icons::normal::navigation::CANCEL)
            .on_click(|ctx, _data, _env| {
                ctx.submit_command(INTERNAL_COMMAND.with(InternalUICommand::RemoveBook));
            })
            .padding(5.0)
            .boxed(),
        );
        ListItems {
            title_label,
            creator_label,
            publisher_label,
            position_in_book_label,
            image,
            remove_button,
        }
    }
}

impl Widget<Recent> for ListItems {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Recent,
        _env: &druid::Env,
    ) {
        match event {
            druid::Event::MouseUp(mouse_event) => {
                // if first half of the widget is clicked, open the book
                if mouse_event.pos.x < ctx.size().width / 2.0 {
                    ctx.set_handled();

                    ctx.submit_command(druid::Command::new(
                        crate::core::commands::OPEN_RECENT,
                        data.clone(),
                        druid::Target::Auto,
                    ));
                    ctx.submit_command(NAVIGATE_TO.with(PageType::Reader));
                }
            }
            druid::Event::MouseMove(mouse_event) => {
                if mouse_event.pos.x < ctx.size().width / 2.0 {

                    ctx.set_handled();
                    ctx.set_cursor(&druid::Cursor::Pointer);
                }
                else {
                    ctx.set_cursor(&druid::Cursor::Arrow);
                }
            }
            druid::Event::Command(cmd) => {
                if cmd.is(FINISH_SLOW_FUNCTION) {
                    data.image_data = Some(cmd.get_unchecked(FINISH_SLOW_FUNCTION).clone());
                }
            }
            _ => {}
        }
        self.image.event(ctx, event, data, _env);
        self.remove_button.event(ctx, event, &mut (), _env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Recent,
        _env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::WidgetAdded => {
                let mut ep = EpubDoc::new(data.path.clone()).unwrap();
                const UNTITLED_BOOK: &str = "Untitled";
                const UNKNOWN_CREATOR_OR_PUBLISHER: &str = "Unknown";
                let binding = ep.get_cover();

                if binding.is_ok() {
                    let img_data = binding.as_ref().unwrap();
                    // retrieve widget id
                    wrapped_slow_function(
                        ctx.get_external_handle(),
                        img_data.to_owned(),
                        ctx.widget_id(),
                    );

                }

                let title = ep
                    .mdata("title")
                    .unwrap_or(UNTITLED_BOOK.to_string())
                    .to_string();
                let creator = ep
                    .mdata("creator")
                    .unwrap_or(UNKNOWN_CREATOR_OR_PUBLISHER.to_string())
                    .to_string();
                let publisher = ep
                    .mdata("publisher")
                    .unwrap_or(UNKNOWN_CREATOR_OR_PUBLISHER.to_string())
                    .to_string();

                self.title_label.set_text(title);
                self.title_label.set_text_size(18.);
                self.title_label.set_text_color(Color::WHITE);

                self.creator_label.set_text(creator);
                self.creator_label.set_text_size(14.);
                self.creator_label.set_text_color(Color::WHITE);

                self.publisher_label.set_text(publisher);
                self.publisher_label.set_text_size(14.);
                self.publisher_label.set_text_color(Color::WHITE);

                self.position_in_book_label.set_text("".to_owned()); //data.reached_position.to_string());
                self.position_in_book_label.set_text_size(14.);
                self.position_in_book_label.set_text_color(Color::WHITE);
            }
            _ => {}
        }
        self.image.lifecycle(ctx, event, data, _env);
        self.remove_button.lifecycle(ctx, event, &(), _env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Recent,
        data: &Recent,
        env: &druid::Env,
    ) {
        if !old_data.same(data) {
            self.image.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Recent,
        env: &druid::Env,
    ) -> druid::Size {
        const IMAGE_HEIGHT: f64 = 180.;
        const TITLE_TEXT_WRAP: f64 = 150.;

        self.title_label.set_wrap_width(TITLE_TEXT_WRAP);
        self.title_label.layout();
        self.title_label.rebuild_if_needed(ctx.text(), env);

        self.creator_label.set_wrap_width(TITLE_TEXT_WRAP);
        self.creator_label.layout();
        self.creator_label.rebuild_if_needed(ctx.text(), env);

        self.publisher_label.set_wrap_width(TITLE_TEXT_WRAP);
        self.publisher_label.layout();
        self.publisher_label.rebuild_if_needed(ctx.text(), env);

        self.position_in_book_label.set_wrap_width(TITLE_TEXT_WRAP);
        self.position_in_book_label.layout();
        self.position_in_book_label
            .rebuild_if_needed(ctx.text(), env);

        self.image.layout(
            ctx,
            &BoxConstraints::tight(Size::new(130., IMAGE_HEIGHT)),
            data,
            env,
        );
        self.image.set_origin(ctx, data, env, Point::new(10., 10.));

        let btn_size = self.remove_button.layout(ctx, bc, &(), env);
        // put remove button on the right side of the widget
        self.remove_button
            .set_origin(ctx, &(), env, Point::new(bc.max().width-btn_size.width, 100.-btn_size.height/2.));
        druid::Size::new(bc.max().width, 200.)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &Recent, _env: &druid::Env) {
        let size = ctx.size();
        ctx.fill(size.to_rect(), &style::get_color_unchecked(style::PRIMARY_DARK));
        const LABEL_PADDING: f64 = 5.;

        let mut y = 15.;
        self.title_label.draw(ctx, Point::new(150., y));
        y += self.title_label.size().height + LABEL_PADDING;
        self.creator_label.draw(ctx, Point::new(150., y));
        y += self.creator_label.size().height + LABEL_PADDING;

        self.publisher_label.draw(ctx, Point::new(150., y));
        y += self.publisher_label.size().height + LABEL_PADDING;

        self.position_in_book_label.draw(ctx, Point::new(150., y));
        self.image.paint(ctx, _data, _env);

        if ctx.is_hot() {
            self.remove_button.paint(ctx, &(), _env);
        }
    }
}
// main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets

const FINISH_SLOW_FUNCTION: druid::Selector<druid::ImageBuf> = druid::Selector::new("asd");

fn wrapped_slow_function(
    sink: druid::ExtEventSink,
    img_data: Vec<u8>,
    widget_target: druid::WidgetId,
) {
    std::thread::spawn(move || {
        //let number = 0;//slow_function(number);
        let img_buf = druid::ImageBuf::from_data(&img_data).unwrap();

        // Once the slow function is done we can use the event sink (the external handle).
        // This sends the `FINISH_SLOW_FUNCTION` command to the main thread and attach
        // the number as payload.

        sink.submit_command(
            FINISH_SLOW_FUNCTION,
            img_buf,
            druid::Target::Widget(widget_target),
        )
        .expect("command failed to submit");
    });
}
pub fn home_page() -> impl Widget<HomePageData> {
    //let list = Scroll::new(
    //    SquaresGrid::new(|| 
    //        ListItems::new().padding(5.0).expand_width()
    //    )
    //        .with_cell_size(Size::new(200.0, 240.0))
    //        .with_spacing(20.0)
    //        
//
    //)
    //.vertical().lens(HomePageData::recents);
    //.on_command(FINISH_SLOW_FUNCTION, |ctx, img_buf, data| {
    //    println!("Received event");
    //    data.image.set_image(img_buf);
    //    ctx.request_layout();
    //}
    let list = Scroll::new(
        List::new(|| {
                ListItems::new()
                .padding(5.0)
                .expand_width()
            }
        )
    ).vertical().lens(HomePageData::recents);

    let title = druid::widget::Label::new("Rust Ebook Reader")
        .with_text_size(24.0)
        .with_text_color(Color::WHITE)
        .center()
        .padding(10.0);

    let open_epub =
        RoundButton::new(druid_material_icons::normal::content::ADD_CIRCLE).with_click_handler(|event, _, _env| {
            let filedialog = druid::FileDialogOptions::new();

            event.submit_command(druid::commands::SHOW_OPEN_PANEL.with(
                filedialog.allowed_types(vec![druid::FileSpec::new("Epub (.epub)", &["epub"])]),
            ));
        }).with_radius(40.);

    let layout = Flex::column()
    .with_child(
        Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::Start)
        .with_child(title).with_child(open_epub)
    ).with_child(list);
    layout
}
