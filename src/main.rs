use std::{sync::Arc, path::PathBuf};

use application_state::{AppState, PageItem, HomePageData, Recent};
use druid::{
    widget::{
        Button, Container, Flex, List, Scroll},
    AppLauncher, Color, Selector, WidgetExt,
    WindowDesc, Key, EventCtx, FileInfo, RenderContext,
};
use druid::{
    Data
};
use druid::{Lens, Widget};

use druid_widget_nursery::{navigator::{Navigator, ViewController}, Wedge};
use epub_page::EpubPage;
use tool::Tool;
mod widgets;
mod tool;
mod epub_page;
mod application_state;

use crate::widgets::navigator::uiview as nav_uiview;
use crate::widgets::navigator::controller as nav_controller;
const _CONTACT_DETAIL: Selector<usize> = Selector::new("contact detail");
const SELECTED_TOOL: Key<u64> = Key::new("org.linebender.example.important-label-color");

fn main() {
    let window = WindowDesc::new(navigator()).title("Navigation");

    let _contacts = get_data();

    
    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(application_state::Delegate)
        .launch(AppState::new())
        .unwrap();
}

// creates the navigator widget responsible for changing views
pub fn navigator() -> impl Widget<application_state::AppState> {
    //druid::widget::ViewSwitcher::new()
    Navigator::new(nav_uiview::UiView::new("home_page".to_string()), || 
    {Box::new(home_page().lens(AppState::home_page_data)) })
        .with_view_builder(nav_uiview::UiView::new("read_ebook".to_string()), read_ebook)
         
        //.controller(nav_controller::NavigatorController)
}

pub fn read_ebook() -> Box<dyn Widget<application_state::AppState>> {

    let epub_page = EpubPage::new().lens(AppState::epub_data);
    let ll = epub_page;
    
    //let ll = Split::columns(epub_page, 
        //EpubPage::new().lens(AppState::current_page));
        //List::new(|| Flex::row()
        //    .with_flex_child(
        //        crate::epub_page::EpubPage::new(), 1.)
        //    )
        //.lens(AppState::pages);//)
        //.vertical();//.controller(ScrollController);
       
           
        let ex = Flex::column()
           .with_default_spacer()
           .with_child(build_toolbar())
           .with_default_spacer()
           .with_flex_child(ll, 1.0)
           .env_scope(|env: &mut druid::Env, data: &AppState| {
               env.set(SELECTED_TOOL, data.selected_tool.clone());
           });

    Box::new(ex)
       
               
/*
    let layout = Flex::row()
        .with_flex_child(open_epub, 1.);
    Box::new(Container::new(layout).background(Color::WHITE))*/

}



fn build_toolbar() -> impl Widget<AppState> {


    let slider = druid::widget::Slider::new()
        .with_range(0.0, 100.0)
        .lens(AppState::slider_value);
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

    let icon = Icon::new(ALARM_ADD);
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
use druid_widget_nursery::material_icons::{Icon, normal::action::ALARM_ADD};

// Here you define Viewcontroller for your application_state::AppState. The navigator widget will
// only accept application_state::AppStates that implement this trait. The methods here are used
// handle modifying your navigation state without manually doing that with your
// own methods. Look at the docs to see what each method is useful for.
impl ViewController<nav_uiview::UiView> for application_state::AppState {
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


} 
impl ListItems {
    pub fn new() -> Self {
        let layout = druid::TextLayout::default();
        ListItems{ layout }
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
            }
            //druid::Event::MouseUp(_) => todo!(),
            druid::Event::MouseMove(_) => {
                //if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                //} else {
                //    ctx.clear_cursor();
                //}

            },
            //druid::Event::Wheel(_) => todo!(),
            _ => {}
        
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Recent, env: &druid::Env) {
            match event {
                druid::LifeCycle::WidgetAdded => {
                    self.layout.set_text(data.name.clone());
                    self.layout.set_text_color(Color::BLACK);

                }
                _ => {} 
            }
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &Recent, _data: &Recent, _env: &druid::Env) {

        
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &Recent, env: &druid::Env) -> druid::Size {
        //if self.layout.needs_rebuild() {
            
            self.layout.set_wrap_width(bc.max().width);

            self.layout.layout();

        self.layout.rebuild_if_needed(ctx.text(), env);

        //}
        //self.image.layout(ctx, bc, &data.image_data, env);

                // If either the width or height is constrained calculate a value so that the image fits
        // in the size exactly. If it is unconstrained by both width and height take the size of
        // the image.
        //let max = bc.max();
        //let image_size = druid::Size::new(30., 55.);
        //let size = if bc.is_width_bounded() && !bc.is_height_bounded() {
        //    let ratio = max.width / image_size.width;
        //    druid::Size::new(image_size.width, ratio * image_size.height)
        //} else if bc.is_height_bounded() && !bc.is_width_bounded() {
        //    let ratio = max.height / image_size.height;
        //    druid::Size::new(ratio * image_size.width, max.height)
        //} else {
        //    bc.constrain(image_size)
        //};
        //size
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
        


        //let image = a.to_image(ctx.render_ctx);
        //ctx.draw_image(&image, ret, druid::piet::InterpolationMode::Bilinear);
    }
}
    // main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets
pub fn home_page() -> impl Widget<HomePageData> {
    
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

// details views - this is the second view after clicking on a contact
/*pub fn contact_details() -> Box<dyn Widget<application_state::AppState>> {
    let name = Label::dynamic(|data: &application_state::AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("Name: {}", data.contacts[idx].name)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.);

    let email = Label::new(|data: &application_state::AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("Email: {}", data.contacts[idx].email)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.);

    let age = Label::new(|data: &application_state::AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("Age: {}", data.contacts[idx].age)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.);

    let favorite_food = Label::new(|data: &application_state::AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("Favorite food: {}", data.contacts[idx].favorite_food)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.);

    // you might want to define a command that pops a view so that you may scope down your application_state::AppState
    let back_button = Button::new("Back").on_click(|_event, data: &mut application_state::AppState, _env| {
        data.pop_view();
    });

    let layout = Flex::column()
        .with_child(name)
        .with_child(email)
        .with_child(age)
        .with_child(favorite_food)
        .cross_axis_alignment(CrossAxisAlignment::Start);
    let layout = Flex::column()
        .with_child(back_button)
        .with_child(layout)
        //.with_child(edit_button)
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::SpaceAround);

    let container = Container::new(layout.center()).background(Color::GRAY);

    Box::new(container)
}
*/




// a little special implementation to give the list view all that it needs
/*impl ListIter<(Arc<Vec<nav_uiview::UiView>>, Contact, Option<usize>, usize)> for application_state::AppState {
    fn for_each(
        &self,
        mut cb: impl FnMut(&(Arc<Vec<nav_uiview::UiView>>, Contact, Option<usize>, usize), usize),
    ) {
        for (idx, contact) in self.contacts.iter().enumerate() {
            let nav_state = self.nav_state.clone();
            cb(&(nav_state, contact.clone(), self.selected, idx), idx);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Arc<Vec<nav_uiview::UiView>>, Contact, Option<usize>, usize), usize),
    ) {
        let mut any_shared_changed = false;
        for (idx, contact) in self.contacts.iter().enumerate() {
            let mut d = (self.nav_state.clone(), contact.clone(), self.selected, idx);

            cb(&mut d, idx);
            if !any_shared_changed && !self.nav_state.same(&d.0) {
                any_shared_changed = true;
            }
            if any_shared_changed {
                self.nav_state = d.0;
                self.selected = d.2;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.contacts.len()
    }
}
*/



#[derive(Clone, Data, Lens, Debug)]
pub struct Contact {
    name: String,
    email: String,
    favorite_food: String,
    age: u32,
}

impl Contact {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        age: u32,
        favorite_food: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let email = email.into();
        let favorite_food = favorite_food.into();
        Self {
            name,
            email,
            favorite_food,
            age,
        }
    }
}
pub fn get_data() -> Vec<Contact> {
    vec![
        Contact {
            name: "Billy Bob".to_string(),
            email: "Billybob@gmail.com".to_string(),
            favorite_food: "Curry".to_string(),
            age: 39,
        },
    ]
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
