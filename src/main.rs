use std::{sync::Arc, path::PathBuf};

use application_state::{AppState, HomePageData, Recent, EpubData, EpubMetrics};
use druid::{
    widget::{
        Button, Container, Flex, List, Image, FillStrat},
    AppLauncher, Color, Selector, WidgetExt,
    WindowDesc, Key, EventCtx, FileInfo, LensExt, WidgetPod, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, PaintCtx, Size, Point,
};

use druid::Widget;
use druid_widget_nursery::material_icons::{Icon, normal::action::ALARM_ADD};

use druid_widget_nursery::navigator::{Navigator, ViewController};
use epub::doc::EpubDoc;
use epub_page::EpubPage;
use sidebar::Sidebar;
use tool::Tool;
use widgets::epub_page::toolbar;
mod widgets;
mod tool;
mod epub_page;
mod application_state;
mod sidebar;

use crate::widgets::navigator::uiview as nav_uiview;
const SELECTED_TOOL: Key<u64> = Key::new("org.linebender.example.important-label-color");
const NAVIGATE_TO: Selector<usize> = Selector::new("navigate_to");

fn main() {

    let data = AppState::new();
    let window = WindowDesc::new(navigator(data.clone())).title("Navigation").window_size((1000.0, 800.0));

    
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(application_state::Delegate)
        .launch(data)
        .unwrap();
}


pub fn navigator(data : AppState) -> impl Widget<AppState> {
    // druid::widget::TextBox::new().lens(AppState::text)
    let topbar = crate::widgets::home_page::topbar::Topbar::new();
    Loader::new()
    .with_page(home_page(data.clone()).lens(AppState::home_page_data))
    .with_page(read_ebook(data.epub_data.clone()))
    //Navigator::new(nav_uiview::UiView::new("home_page".to_string()), || 
    //{Box::new(home_page().lens(AppState::home_page_data)) })
    //    .with_view_builder(nav_uiview::UiView::new("read_ebook".to_string()), read_ebook)
         
}

pub fn read_ebook(data : EpubData) -> Box<dyn Widget<AppState>> {

    let epub_page = EpubPage::new(data).lens(AppState::epub_data);

       

        let toolbar = toolbar::Toolbar::new().lens(AppState::epub_data);
        let row = Flex::row()
        .with_flex_child(Sidebar::new(), 0.3)
        .with_default_spacer()
        .with_flex_child(epub_page, 1.);
           
        let ex = Flex::column()
           .with_default_spacer()
           .with_child(toolbar)//build_toolbar())
           .with_default_spacer()
           .with_flex_child(row, 1.0)
           .env_scope(|env: &mut druid::Env, data: &AppState| {
               env.set(SELECTED_TOOL, data.selected_tool.clone());
           });

    Box::new(ex)
       
               
/*
    let layout = Flex::row()
        .with_flex_child(open_epub, 1.);
    Box::new(Container::new(layout).background(Color::WHITE))*/

}



fn _build_toolbar() -> impl Widget<AppState> {


    let slider = druid::widget::Slider::new()
        .with_range(0.0, 100.0)
        .lens(AppState::epub_data.then(EpubData::epub_metrics.then(EpubMetrics::percentage_page_in_book)));
    let bt1 = Button::new("Arrow")
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.selected_tool = Tool::Arrow;
    });

    let bt2 = Button::new("Note")
    .on_click(|_ctx, data: &mut AppState, _env | {
        data.selected_tool = Tool::Note;
    });

    let bt3 = Button::new("Marker")
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.selected_tool = Tool::Marker;
    });
    let bt4 = Button::new("Eraser")
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.selected_tool = Tool::Eraser;
    });
    let bt5 = Button::new("Close")
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.pop_view();
    });

    let _icon = Icon::new(ALARM_ADD);
    Flex::row()
    .with_child(bt1)
    .with_child(bt2)
    .with_child(bt3)
    .with_child(bt4)
    .with_child(bt5)
    .with_child(slider)
    //.with_child(Wedge::new().lens(AppState::selected))
    //.with_child(icon)


    
}

// Here you define Viewcontroller for your application_state::AppState. The navigator widget will
// only accept application_state::AppStates that implement this trait. The methods here are used
// handle modifying your navigation state without manually doing that with your
// own methods. Look at the docs to see what each method is useful for.
impl ViewController<nav_uiview::UiView> for AppState {
    fn add_view(&mut self, view: nav_uiview::UiView) {
        let views: &mut Vec<nav_uiview::UiView> = Arc::make_mut(&mut self.nav_state);
        views.push(view);
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    fn pop_view(&mut self) {
        let views = Arc::make_mut(&mut self.nav_state);
        views.pop();
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    fn current_view(&self) -> &nav_uiview::UiView { 
        self.nav_state.last().unwrap()
    }

    fn len(&self) -> usize {
        self.nav_state.len()
    }

    fn is_empty(&self) -> bool {
        self.nav_state.is_empty()
    }
}

struct ListItems {

    layout: druid::TextLayout<String>,
    image : WidgetPod<Recent, Image>,


} 
impl ListItems {
    pub fn new() -> Self {
        let layout = druid::TextLayout::default();
        let mut ep = EpubDoc::new("/home/syscall/Desktop/Wolves of the Calla - Stephen King.epub").unwrap();
        
        let binding = ep.get_cover();
        let img_data = binding.as_ref().unwrap();
        let img_buf = druid::ImageBuf::from_data(&img_data).unwrap();
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
                let cmd = druid::commands::OPEN_FILE;
                let f : FileInfo = FileInfo { path: PathBuf::from(data.path.clone()), format: None };
                ctx.submit_command(druid::Command::new(cmd, f, druid::Target::Auto));


                ctx.submit_command(NAVIGATE_TO.with(1));
            }
            //druid::Event::MouseUp(_) => todo!(),
            druid::Event::MouseMove(_) => {
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
                    self.layout.set_text(data.name.clone());
                    self.layout.set_text_color(Color::BLACK);

                }
                _ => {} 
            }
            self.image.lifecycle(_ctx, event, data, _env);
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &Recent, _data: &Recent, _env: &druid::Env) {

        self.image.update(_ctx, _data, _env)
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &Recent, env: &druid::Env) -> druid::Size {
        //if self.layout.needs_rebuild() {
            
            self.layout.set_wrap_width(bc.max().width);

            self.layout.layout();

        self.layout.rebuild_if_needed(ctx.text(), env);

        self.image.layout(ctx, &BoxConstraints::tight(Size::new(130., 180.)), _data, env);
        self.layout.size()



        //druid::Size::new(self.layout.size().width, self.layout.size().height+ 180.)    
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &Recent, _env: &druid::Env) {
        let origin = druid::Point::new(0., 0.0);

        // This is the builder-style way of drawing text.
        self.layout.draw(ctx, origin);
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
        
    
         ListItems::new()
         //.align_vertical(druid::UnitPoint::LEFT)
         .padding(10.0)
         //.expand()
         
         .background(Color::rgb(0.5, 0.5, 0.5))
         //Button::new(|data: &String, _: &_| format!("{}", data)).on_click(|_event, _, _env| {
         //.height(50.0)

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
    
               
           // druid::widget::Label::new();
    let layout = Flex::column()
        .with_flex_child(open_epub, 1.)
        .with_flex_child(list, 1.);//.lens(AppState::home_page_data);
    Container::new(layout).background(Color::WHITE)
}



// Create a main widget able to switch between the different pages
//pub struct MainWidget {
//    navigator: Navigator<AppState>,
//
//}



pub struct Loader {
    pages : Vec<WidgetPod<AppState, Box<dyn Widget<AppState>>>>,
    current_page : usize,
}

impl Loader {
    pub fn new() -> Self {
        let pages = Vec::new();
        Self { pages, current_page: 0 }
    }

    pub fn with_page(mut self, page: impl Widget<AppState> + 'static) -> Self {
        self.pages.push(WidgetPod::new(Box::new(page)));
        self
    }
}

// Create a widtget that can switch between the different pages
impl Widget<AppState> for Loader {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {

        // Switch page through a command 
        match event {
            Event::Command(cmd) if cmd.is(NAVIGATE_TO) => {
                let page = cmd.get_unchecked(NAVIGATE_TO).clone();
                self.current_page = page;
                ctx.children_changed();
            }
            _ => {}
        }

        self.pages[self.current_page].event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        //self.pages[self.current_page].lifecycle(ctx, event, data, env);
        for child in self.pages.iter_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, env: &Env) {
        self.pages[self.current_page].update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        let size = self.pages[self.current_page].layout(ctx, bc, data, env);
        self.pages[self.current_page].set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.pages[self.current_page].paint(ctx, data, env);
    }
}



// this holds state that will be used when on the edit page
/*
COULD BE USEFUL FOR Correttore di Bozza!!!
#[derive(Clone, Data, Lens, Debug)]
pub struct EditState {
    contact: Contact,
    index: usize,
    was_saved: bool,
}

impl EditState {
    pub fn new(data: application_state::AppState) -> Self {
        let (contact, index) = if let Some(idx) = data.selected {
            (data.contacts[idx].clone(), idx)
        } else {
            (
                Contact::new("".to_owned(), "".to_owned(), 31, "".to_owned()),
                0,
            )
        };
        Self {
            contact,
            index,
            was_saved: false,
        }
    }
}*/
