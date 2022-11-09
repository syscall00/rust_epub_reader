use crate::core::commands::NAVIGATE_TO;
use std::{path::PathBuf};

use appstate::{AppState, HomePageData, Recent, EpubData};
use druid::{
    widget::{
        Button, Flex, List, Image, FillStrat, ViewSwitcher, Scroll,},
    AppLauncher, Color, WidgetExt,
    WindowDesc, EventCtx, FileInfo, WidgetPod, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, PaintCtx, Size, Point, Data, TextLayout, RenderContext,
};

use druid::Widget;
// use druid_widget_nursery::material_icons::{Icon, normal::action::ALARM_ADD};

use epub::doc::EpubDoc;
use epub_page::{EpubPage};
use sidebar::Sidebar;
use widgets::epub_page::textcontainer::Icon;
mod widgets;
mod tool;
mod epub_page;
mod appstate;
mod sidebar;
mod core;

use druid_material_icons::normal::action::*;
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

    let data = AppState::new();
    let window = WindowDesc::new(navigator(data.clone())).title("Navigation").window_size((1000.0, 800.0));

    //let window = WindowDesc::new(build_ui());
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(appstate::Delegate)
        .launch(data)
        //.launch(())
        .unwrap();
}

/*pub fn build_ui() -> impl Widget<()> {
    let mut flex = Flex::column();
     
    //let mut icons_from_file = 
    let v = vec![0];
    //let v = vec![druid_material_icons::normal::notification::ACCOUNT_TREE,druid_material_icons::normal::notification::ADB,druid_material_icons::normal::notification::ADD_CALL,druid_material_icons::normal::notification::AIRLINE_SEAT_FLAT,druid_material_icons::normal::notification::AIRLINE_SEAT_FLAT_ANGLED,druid_material_icons::normal::notification::AIRLINE_SEAT_INDIVIDUAL_SUITE,druid_material_icons::normal::notification::AIRLINE_SEAT_LEGROOM_EXTRA,druid_material_icons::normal::notification::AIRLINE_SEAT_LEGROOM_NORMAL,druid_material_icons::normal::notification::AIRLINE_SEAT_LEGROOM_REDUCED,druid_material_icons::normal::notification::AIRLINE_SEAT_RECLINE_EXTRA,druid_material_icons::normal::notification::AIRLINE_SEAT_RECLINE_NORMAL,druid_material_icons::normal::notification::BLUETOOTH_AUDIO,druid_material_icons::normal::notification::CONFIRMATION_NUMBER,druid_material_icons::normal::notification::DIRECTIONS_OFF,druid_material_icons::normal::notification::DISC_FULL,druid_material_icons::normal::notification::DO_DISTURB,druid_material_icons::normal::notification::DO_DISTURB_ALT,druid_material_icons::normal::notification::DO_DISTURB_OFF,druid_material_icons::normal::notification::DO_DISTURB_ON,druid_material_icons::normal::notification::DO_NOT_DISTURB,druid_material_icons::normal::notification::DO_NOT_DISTURB_ALT,druid_material_icons::normal::notification::DO_NOT_DISTURB_OFF,druid_material_icons::normal::notification::DO_NOT_DISTURB_ON,druid_material_icons::normal::notification::DRIVE_ETA,druid_material_icons::normal::notification::ENHANCED_ENCRYPTION,druid_material_icons::normal::notification::EVENT_AVAILABLE,druid_material_icons::normal::notification::EVENT_BUSY,druid_material_icons::normal::notification::EVENT_NOTE,druid_material_icons::normal::notification::FOLDER_SPECIAL,druid_material_icons::normal::notification::IMAGESEARCH_ROLLER,druid_material_icons::normal::notification::LIVE_TV,druid_material_icons::normal::notification::MMS,druid_material_icons::normal::notification::MORE,druid_material_icons::normal::notification::NETWORK_CHECK,druid_material_icons::normal::notification::NETWORK_LOCKED,druid_material_icons::normal::notification::NO_ENCRYPTION,druid_material_icons::normal::notification::NO_ENCRYPTION_GMAILERRORRED,druid_material_icons::normal::notification::ONDEMAND_VIDEO,druid_material_icons::normal::notification::PERSONAL_VIDEO,druid_material_icons::normal::notification::PHONE_BLUETOOTH_SPEAKER,druid_material_icons::normal::notification::PHONE_CALLBACK,druid_material_icons::normal::notification::PHONE_FORWARDED,druid_material_icons::normal::notification::PHONE_IN_TALK,druid_material_icons::normal::notification::PHONE_LOCKED,druid_material_icons::normal::notification::PHONE_MISSED,druid_material_icons::normal::notification::PHONE_PAUSED,druid_material_icons::normal::notification::POWER,druid_material_icons::normal::notification::POWER_OFF,druid_material_icons::normal::notification::PRIORITY_HIGH,druid_material_icons::normal::notification::RUNNING_WITH_ERRORS,druid_material_icons::normal::notification::SD_CARD,druid_material_icons::normal::notification::SD_CARD_ALERT,druid_material_icons::normal::notification::SIM_CARD_ALERT,druid_material_icons::normal::notification::SMS,druid_material_icons::normal::notification::SMS_FAILED,druid_material_icons::normal::notification::SUPPORT_AGENT,druid_material_icons::normal::notification::SYNC,druid_material_icons::normal::notification::SYNC_DISABLED,druid_material_icons::normal::notification::SYNC_LOCK,druid_material_icons::normal::notification::SYNC_PROBLEM,druid_material_icons::normal::notification::SYSTEM_UPDATE,druid_material_icons::normal::notification::TAP_AND_PLAY,druid_material_icons::normal::notification::TIME_TO_LEAVE,druid_material_icons::normal::notification::TV_OFF,druid_material_icons::normal::notification::VIBRATION,druid_material_icons::normal::notification::VOICE_CHAT,druid_material_icons::normal::notification::VPN_LOCK,druid_material_icons::normal::notification::WC,druid_material_icons::normal::notification::WIFI,druid_material_icons::normal::notification::WIFI_OFF,];
    for i in 0..v.len() {
        flex.add_child(Flex::row().with_child(Icon::new(v[i])).with_child(druid::widget::Label::new(i.to_string())));
    }
    Scroll::new(flex)
}*/
pub fn navigator(data : AppState) -> Box<dyn Widget<AppState>> {
    let _topbar = crate::widgets::home_page::topbar::Topbar::new();


    let switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.active_page,
        move |active_page, _data, _env| {
            match active_page {
                PageType::Home => home_page(data.clone()).lens(AppState::home_page_data).boxed(),
                PageType::Reader => read_ebook(data.epub_data.clone()).boxed(),
            }
        },
    );
    MainContainer::new(switcher.boxed()).boxed()
         
}


pub struct MainContainer {
    page_switcher: WidgetPod<AppState, Box<dyn Widget<AppState>>>
}


impl MainContainer {
    pub fn new(page_switcher : Box<dyn Widget<AppState>>) -> Self {
        Self {
            page_switcher: WidgetPod::new(page_switcher)
        }
    }
}


impl Widget<AppState> for MainContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(NAVIGATE_TO) => {
                if let Some(index) = cmd.get(NAVIGATE_TO) {
                    data.active_page = *index;
                    ctx.request_layout();
                }
            }
            _ => {}
        }
        self.page_switcher.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.page_switcher.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old: &AppState, data: &AppState, env: &Env) {
        self.page_switcher.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        let size = self.page_switcher.layout(ctx, bc, data, env);
        self.page_switcher.set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.page_switcher.paint(ctx, data, env);
    }
}


pub fn read_ebook(data : EpubData) -> Box<dyn Widget<AppState>> {

    Flex::row()
        .with_child(Sidebar::new().lens(AppState::epub_data))
        .with_flex_child(EpubPage::new(data).expand()
        .lens(AppState::epub_data), 1.)
        .boxed()
}


struct ListItems {
    title_label: druid::TextLayout<String>,
    creator_label: TextLayout<String>,
    publisher_label: TextLayout<String>,
    position_in_book_label: TextLayout<String>,
    
    image : WidgetPod<Recent, Image>,
} 

impl ListItems {
    pub fn new() -> Self {
        let title_label = druid::TextLayout::default();
        let creator_label = druid::TextLayout::default();
        let publisher_label = druid::TextLayout::default();
        let position_in_book_label = druid::TextLayout::default();
        
        let img_buf = druid::ImageBuf::empty();
        let image = WidgetPod::new(Image::new(img_buf)
            .fill_mode(FillStrat::Fill));
        ListItems{ 
            title_label,
            creator_label,
            publisher_label,
            position_in_book_label,
            image 
        }
    }

}

impl Widget<Recent> for ListItems {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut Recent, _env: &druid::Env) {
        match event {
            druid::Event::MouseUp(_) => {
                ctx.set_handled();
                let f = FileInfo { path: PathBuf::from(data.path.clone()), format: None };
                ctx.submit_command(druid::Command::new(druid::commands::OPEN_FILE, f, druid::Target::Auto));

                ctx.submit_command(NAVIGATE_TO.with(PageType::Reader));
            }
            druid::Event::MouseMove(_) => {
                ctx.set_handled();
                ctx.set_cursor(&druid::Cursor::Pointer);
            },
            _ => {}
        
        }
        self.image.event(ctx, event, data, _env);
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Recent, _env: &druid::Env) {
            match event {
                druid::LifeCycle::WidgetAdded => {
                    let mut ep = EpubDoc::new(data.path.clone()).unwrap();
                    const UNTITLED_BOOK : &str = "Untitled";
                    const UNKNOWN_CREATOR_OR_PUBLISHER : &str = "Unknown";

                    
                    let title = ep.mdata("title").unwrap_or(UNTITLED_BOOK.to_string()).to_string();
                    let creator = ep.mdata("creator").unwrap_or(UNKNOWN_CREATOR_OR_PUBLISHER.to_string()).to_string();
                    let publisher = ep.mdata("publisher").unwrap_or(UNKNOWN_CREATOR_OR_PUBLISHER.to_string()).to_string();
                  

                    self.title_label.set_text(title);
                    self.title_label.set_text_size(18.);
                    self.title_label.set_text_color(Color::WHITE);

                    self.creator_label.set_text(creator);
                    self.creator_label.set_text_size(14.);
                    self.creator_label.set_text_color(Color::WHITE);

                    self.publisher_label.set_text(publisher);
                    self.publisher_label.set_text_size(14.);
                    self.publisher_label.set_text_color(Color::WHITE);

                    self.position_in_book_label.set_text(data.reached_position.to_string());
                    self.position_in_book_label.set_text_size(14.);
                    self.position_in_book_label.set_text_color(Color::WHITE);

                    let binding = ep.get_cover();
                    let img_data = binding.as_ref().unwrap();
                    let img_buf = druid::ImageBuf::from_data(&img_data).unwrap();

                    self.image.widget_mut().set_image_data(img_buf);

                }
                _ => {} 
            }
            self.image.lifecycle(_ctx, event, data, _env);
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &Recent, _data: &Recent, _env: &druid::Env) {

        self.image.update(_ctx, _data, _env)
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Recent, env: &druid::Env) -> druid::Size {
        const IMAGE_HEIGHT : f64 = 180.;
        const TITLE_TEXT_WRAP : f64 = 150.;
            
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
        self.position_in_book_label.rebuild_if_needed(ctx.text(), env);


        self.image.layout(ctx, &BoxConstraints::tight(Size::new(130., IMAGE_HEIGHT)), data, env);
        self.image.set_origin(ctx, data, env, Point::new(10., 10., ));
        druid::Size::new(bc.max().width, 200.)

    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &Recent, _env: &druid::Env) {
        let size = ctx.size();
        ctx.fill(size.to_rect(), &Color::RED);
        const LABEL_PADDING: f64 = 5.;

        let mut y = 15.;
        self.title_label.draw(ctx, Point::new(150., y));
        y+= self.title_label.size().height + LABEL_PADDING;
        self.creator_label.draw(ctx, Point::new(150., y));
        y+= self.creator_label.size().height + LABEL_PADDING;

        self.publisher_label.draw(ctx, Point::new(150., y));
        y+= self.publisher_label.size().height + LABEL_PADDING;

        self.position_in_book_label.draw(ctx, Point::new(150., y));
        self.image.paint(ctx, _data, _env);


    }
}
// main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets

pub fn home_page(_ : AppState) -> impl Widget<HomePageData> {
    
    let list = Scroll::new(List::new(|| {
         ListItems::new()
         .padding(5.0)
         .expand_width()
                  
    })).vertical().lens(HomePageData::recents);


    let open_epub = Button::new("Open new epub".to_string())
            .on_click(|event, _, _env| {
                let filedialog = druid::FileDialogOptions::new();
                let mut allowed = Vec::new();
                allowed.push(druid::FileSpec::new("Epub (*.epub)", &["epub"]));
            
                event.submit_command(
                    druid::commands::SHOW_OPEN_PANEL.with(
                        filedialog.allowed_types(allowed)
                    )
                );
            });
    
               
    let layout = Flex::column()
        .with_child(open_epub)
        .with_child(list);
        layout

        
}




