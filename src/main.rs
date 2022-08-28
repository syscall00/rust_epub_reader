use std::sync::Arc;

use application_state::{AppState, PageItem};
use druid::{
    widget::{
        Button, Container, Flex, List, Scroll,
    },
    AppLauncher, Color, Selector, WidgetExt,
    WindowDesc, Key,
};
use druid::{
    Data
};
use druid::{Lens, Widget};

use druid_widget_nursery::{navigator::{Navigator, ViewController}, Wedge};
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
        .launch(application_state::AppState::new())
        .unwrap();
}

// creates the navigator widget responsible for changing views
pub fn navigator() -> impl Widget<application_state::AppState> {
    Navigator::new(nav_uiview::UiView::new("home_page".to_string()), home_page)
        .with_view_builder(nav_uiview::UiView::new("read_ebook".to_string()), read_ebook)
        
        .controller(nav_controller::NavigatorController)
}

pub fn read_ebook() -> Box<dyn Widget<application_state::AppState>> {
 

    let ll = Scroll::new(
                List::new(page_ui).lens(AppState::pages))
               .vertical();//.controller(ScrollController);
       
           
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



fn page_ui() -> impl Widget<PageItem> {
    // Change lens from RichText to PageItem in order to access both to richtext and page number
    Flex::row().with_flex_child(crate::epub_page::EpubPage::new(), 1.)    
}
fn build_toolbar() -> impl Widget<AppState> {

    let bt1 = Button::new("Arrow")
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.selected_tool = Tool::Arrow;
    });

    let bt2 = Button::new("Pen")
    .on_click(|_ctx, data: &mut AppState, _env | {
        data.selected_tool = Tool::Pen;
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
    .with_child(Wedge::new().lens(AppState::selected))
    .with_child(icon)


    
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

// main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets
pub fn home_page() -> Box<dyn Widget<application_state::AppState>> {
    /*
    let list = List::new(|| {
        let name_text = Label::new(
            |(_views, contact, _selection, _idx): &(
                Arc<Vec<nav_uiview::UiView>>,
                Contact,
                Option<usize>,
                usize,
            ),
             _env: &_| { contact.name.clone() },
        )
        .with_text_color(Color::BLACK)
        .with_text_size(20.);
        let email_text = Label::new(
            |(_views, contact, _selected, _idx): &(
                Arc<Vec<nav_uiview::UiView>>,
                Contact,
                Option<usize>,
                usize,
            ),
             _env: &_| contact.email.clone(),
        )
        .with_text_color(Color::BLACK)
        .with_text_size(20.);

        let details = Flex::column().with_child(name_text).with_child(email_text);
        let layout = Flex::row().with_child(details);
        let layout = layout.on_click(|event, data, _env| {
            let new_views = Arc::make_mut(&mut data.0);
            new_views.push(nav_uiview::UiView::new("contact details".to_string()));
            data.0 = Arc::new(new_views.to_owned());
            data.2 = Some(data.3);
            event.submit_command(Command::new(CONTACT_DETAIL, data.3, Target::Auto));
        });

        layout.background(Painter::new(|ctx, _data, _env| {
            let is_hot = ctx.is_hot();
            let is_active = ctx.is_active();
            let rect = ctx.size().to_rect();
            let background_color = if is_active {
                Color::rgb8(0x88, 0x88, 0x88)
            } else if is_hot {
                Color::rgb8(0xdd, 0xdd, 0xdd)
            } else {
                Color::WHITE
            };
            ctx.stroke(rect, &background_color, 0.);
            ctx.fill(rect, &background_color);
        }))
    });*/

    //let layout = Flex::row()
    //    .with_flex_child(Scroll::new(list.with_spacing(20.)).center(), 1.)
    //    .must_fill_main_axis(true)
    //    .expand_width();


    
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
    
               

    let layout = Flex::row()
        .with_flex_child(open_epub, 1.);
    Box::new(Container::new(layout).background(Color::WHITE))
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
