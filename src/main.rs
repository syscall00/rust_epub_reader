use crate::core::commands::NAVIGATE_TO;
use std::{path::PathBuf};

use appstate::{AppState, HomePageData, Recent, EpubData};
use druid::{
    widget::{
        Button, Flex, List, Image, FillStrat, ViewSwitcher, Split},
    AppLauncher, Color, Selector, WidgetExt,
    WindowDesc, EventCtx, FileInfo, WidgetPod, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, PaintCtx, Size, Point, Data,
};

use druid::Widget;
// use druid_widget_nursery::material_icons::{Icon, normal::action::ALARM_ADD};

use epub::doc::EpubDoc;
use epub_page::{EpubPage, Container};
use sidebar::Sidebar;
use widgets::epub_page::toolbar;
mod widgets;
mod tool;
mod epub_page;
mod appstate;
mod sidebar;
mod core;

#[derive(Data, PartialEq, Clone, Copy)]
pub enum PageType {
    Home,
    Reader,
}


fn main() {

    let data = AppState::new();
    let window = WindowDesc::new(navigator(data.clone())).title("Navigation").window_size((1000.0, 800.0));

    
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(appstate::Delegate)
        .launch(data)
        .unwrap();
}


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

    //let epub_page = EpubPage::new(data).lens(AppState::epub_data);


    ////Flex::row()
    ////    .with_flex_child(Sidebar::new().lens(AppState::epub_data), 0.2)
    ////    .with_child(EpubPage::new(data).lens(AppState::epub_data))
    ////    .boxed()
    ////
    Container::column()
    .with_child_and_size(Sidebar::new(),0.25)
    .with_child(EpubPage::new(data))
    .lens(AppState::epub_data)
    .boxed()
    
}


struct ListItems {
    layout: druid::TextLayout<String>,
    image : WidgetPod<Recent, Image>,
} 

impl ListItems {
    pub fn new() -> Self {
        let layout = druid::TextLayout::default();
        
        let img_buf = druid::ImageBuf::empty();
        let image = WidgetPod::new(Image::new(img_buf)
            .fill_mode(FillStrat::Fill));
        ListItems{ layout, image }
    }

}

impl Widget<Recent> for ListItems {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut Recent, _env: &druid::Env) {
        match event {
            //druid::Event::WindowSize(_) => todo!(),
            druid::Event::MouseDown(_) => {
                ctx.set_handled();
                let f = FileInfo { path: PathBuf::from(data.path.clone()), format: None };
                ctx.submit_command(druid::Command::new(druid::commands::OPEN_FILE, f, druid::Target::Auto));

                ctx.submit_command(NAVIGATE_TO.with(PageType::Reader));
            }
            //druid::Event::MouseUp(_) => todo!(),
            druid::Event::MouseMove(_) => {
                ctx.set_handled();
                ctx.set_cursor(&druid::Cursor::Pointer);
            },
            //druid::Event::Wheel(_) => todo!(),
            _ => {}
        
        }
        self.image.event(ctx, event, data, _env);
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Recent, _env: &druid::Env) {
            match event {
                druid::LifeCycle::WidgetAdded => {
                    let mut ep = EpubDoc::new(data.path.clone()).unwrap();
                    const UNTITLED_BOOK : &str = "Untitled";

                    let title = ep.mdata("title").unwrap_or(UNTITLED_BOOK.to_string()).to_string();
                    self.layout.set_text(title);
                    self.layout.set_text_color(Color::WHITE);
                    ep = EpubDoc::new(data.path.clone()).unwrap();

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
        //if self.layout.needs_rebuild() {
            
            self.layout.set_wrap_width(130.);

            self.layout.layout();

        self.layout.rebuild_if_needed(ctx.text(), env);

        self.image.layout(ctx, &BoxConstraints::tight(Size::new(130., 180.)), data, env);
        self.image.set_origin(ctx, data, env, Point::ORIGIN);
        druid::Size::new(130., 200.)



        //druid::Size::new(self.layout.size().width, self.layout.size().height+ 180.)    
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &Recent, _env: &druid::Env) {

        // This is the builder-style way of drawing text.
        self.layout.draw(ctx, Point::new(0., 180.));
        //let ret  = druid::Rect::new(20., 20., 150. , 200.);
        //let img_data = epub::doc::EpubDoc::new(_data.path.to_string()).unwrap().get_cover().unwrap();

        //let a = druid::ImageBuf::from_data(&img_data).unwrap();
        self.image.paint(ctx, _data, _env);


        //let image = a.to_image(ctx.render_ctx);
        //ctx.draw_image(&image, ret, druid::piet::InterpolationMode::Bilinear);
    }
}
// main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets

pub fn home_page(data : AppState) -> impl Widget<HomePageData> {
    
    let list = List::new(|| {
        //|item: &Recent, _env: &_| -> Box<dyn Widget<Recent>>{
        //    let list_items = ListItems::new();
        //    list_items.boxed()
        //}
        //druid::widget::Label::new(|item: &Recent, _env: &_| item.name.clone())
        //    .padding(5.0)
        //    .background(Color::rgb8(0x80, 0x80, 0x80))
        //    .expand_width()
        //    .center()
         ListItems::new()
         .align_vertical(druid::UnitPoint::LEFT)
         .padding(10.0)
         
         //.background(Color::rgb(0.5, 0.5, 0.5))
         
    }).lens(HomePageData::recents);


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
        .with_flex_child(open_epub, 1.)
        .with_flex_child(list, 1.);//.lens(AppState::home_page_data);
        layout
    //Container::new(layout).background(Color::WHITE)
}




