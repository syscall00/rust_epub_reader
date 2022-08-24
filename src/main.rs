#![windows_subsystem = "windows"]
#![cfg_attr(debug_assertions,allow(dead_code, unused_imports))]

use std::usize;
use druid::platform_menus::mac::file::print;
use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::widget::{prelude::*, Button};
use druid::widget::{Scroll};
use druid::{AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, FontStyle, FontWeight, Handled, Lens, LocalizedString, Menu, Selector, Target, Widget, WidgetExt, WindowDesc, WindowId, commands,  Key, Vec2}; // FontFamily
use crate::commands::SCROLL_TO_VIEW;

use druid::im::{vector, Vector};
use druid::widget::{Flex, Label, List};

use druid::{Rect};

mod epub_page;
mod tool;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Epub Reader");

const SPACER_SIZE: f64 = 8.0;
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);
const SCROLL_TO: Selector<u64> = Selector::new("scroll-view.goto");
const SELECTED_TOOL: Key<u64> = Key::new("org.linebender.example.important-label-color");
const OPEN_FILE: Selector<String> = Selector::new("druid-builtin.open-file-link");


#[derive(Debug, PartialEq)]
enum HtmlTag {
    Header(u8),
    Link(i32),
    Image(String),
    Paragraph,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    Title,
    Unhandled,
}
// TODO: implement links and images
impl From<&str> for HtmlTag {
    fn from(tag_string: &str) -> Self {
        match tag_string {
            "h1" => HtmlTag::Header(1),
            "h2" => HtmlTag::Header(2),
            "h3" => HtmlTag::Header(3),
            "h4" => HtmlTag::Header(4),
            "h5" => HtmlTag::Header(5),
            "h6" => HtmlTag::Header(6),
            "a" => HtmlTag::Link(-1),
            "img" => HtmlTag::Image("".to_string()),
            "p" => HtmlTag::Paragraph,
            "strong" | "b" => HtmlTag::Bold,
            "em" | "i" => HtmlTag::Italic,
            "u" => HtmlTag::Underline,
            "del" | "s" => HtmlTag::StrikeThrough,
            "title" => HtmlTag::Title,
            _ => HtmlTag::Unhandled,
        }
    }
}

fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    for tok_result in xmlparser::Tokenizer::from(text) {
        if tok_result.is_err() {
            // handle error
            continue;
        }
        let token = tok_result.unwrap();
        match token {
            xmlparser::Token::ElementStart {
                prefix: _,
                local,
                span: _,
            } => {
                token_stack.push((current_pos, HtmlTag::from(local.as_str())));
            }
            xmlparser::Token::ElementEnd { end, span: _ } => match end {
                xmlparser::ElementEnd::Open => {
                    continue;
                }
                xmlparser::ElementEnd::Close(_, closed_token) => {
                    let (pos, tk) = token_stack.pop().expect("No token on stack");
                    if tk != HtmlTag::from(closed_token.as_str()) {
                        println!(
                            "ERROR: closing tag {:?} does not match started tag {:?}",
                            closed_token.as_str(),
                            tk
                        );
                        continue;
                    }
                    //println!("Tag {:?}", &closed_token);

                    add_attribute_for_token(
                        &tk,
                        builder.add_attributes_for_range(pos..current_pos),
                    );

                    if tk != HtmlTag::Unhandled && add_newline_after_tag(&tk) {
                        builder.push("\n\n");
                        current_pos += 2;
                    }
                }
                xmlparser::ElementEnd::Empty => {
                    token_stack.pop().expect("No token on stack");
                }
            },

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) =  token_stack.last().unwrap();

                if !should_tag_be_written(inner_tag) || text.trim().is_empty() {
                    continue;
                } else {
                    let t = text.as_str().replace("\n", "");
                    builder.push(&t);
                    current_pos = current_pos + t.len();
                }
            }
            _ => continue,
            /*
            xmlparser::Token::Declaration { version, encoding, standalone, span } => {
                // for now, ignore declarations
                continue;
            },
            xmlparser::Token::EmptyDtd { name, external_id, span } => {
                // for now, ignore the DTD
                continue;
            },
            xmlparser::Token::Attribute { prefix: _, local: _, value :_ , span :_ } => {
                // for now could be ignored
                continue;
            },

            xmlparser::Token::ProcessingInstruction { target, content, span } => todo!(),
            xmlparser::Token::DtdStart { name, external_id, span } => todo!(),
            xmlparser::Token::EntityDeclaration { name, definition, span } => todo!(),
            xmlparser::Token::DtdEnd { span } => todo!(),

            */
        }
    }
    //println!("Length: {:?}", current_pos);
    builder.build()
}

fn add_newline_after_tag(tag: &HtmlTag) -> bool {
    matches!(
        tag,
        HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Link(_) | HtmlTag::Image(_)
    )
}

fn should_tag_be_written(tag: &HtmlTag) -> bool {
    !matches!(
        tag,
        HtmlTag::Title
    )
}

fn add_attribute_for_token(token: &HtmlTag, mut attrs: AttributesAdder) {
    match token {
        HtmlTag::Header(lvl) => {
            attrs.size(16. + *lvl as f64).weight(FontWeight::BOLD);
        }
        HtmlTag::Bold => {
            attrs.weight(FontWeight::BOLD);
        }
        HtmlTag::Italic => {
            attrs.style(FontStyle::Italic);
        }
        HtmlTag::Underline => {
            attrs.underline(true);
        }
        HtmlTag::StrikeThrough => {
            attrs.strikethrough(true);
        }
        HtmlTag::Link(_target) => {
            //Tag::Link(_link_ty, target, _title) => {
            attrs.underline(true)
            .text_color(LINK_COLOR)
            .link(SCROLL_TO.with(100));//.with(Rect::new(10., 10., 10., 10.)));
            //.link(SCROLL_TO_VIEW.with(Rect::new(10., 10., 10., 10.)));
            //.link(OPEN_LINK.with("Aaaa".to_string()));
        }
        HtmlTag::Image(_img) => {

        }
        _ => {
            return;
        } 
        //println!("Unhandled tag: {:?}", token)},
    }
}



#[derive(Clone, Data, Lens)]
struct AppState {
    pages: Vector<PageItem>,
    file_opened: String,
    scroll_position : u64,
    selected_tool : Tool
}

impl AppState {
    pub fn new(file_opened: String) -> Self {
        let pages = AppState::load_file(&file_opened);
        AppState {
            pages,
            file_opened,
            scroll_position: 0,
            selected_tool: Tool::default()
        }
    }

    fn load_file(file_path: &str) -> Vector<PageItem> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);
        assert!(doc.is_ok());
        let mut doc = doc.unwrap();

        while doc.go_next().is_ok() {
            let page_text = rebuild_rendered_text(&doc.get_current_str().unwrap());
            
            pages.push_back(PageItem {
                page_number: doc.get_current_page(),
                page_text,
            });
        }
        pages
    }

    pub fn open_file(& mut self, file_path: String)  {
        
        self.pages = AppState::load_file(&file_path);
    }
    
}

#[derive(Clone, Lens, Data)]
struct PageItem {
    page_number: usize,
    page_text: RichText,
}


struct Delegate;

use tool::Tool;
use tracing::{event, span, Level};
use epub::doc::EpubDoc;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        println!("Sono in command: {:?}", cmd);
        /*if let Some(url) = cmd.get(OPEN_LINK) {
            #[cfg(not(target_arch = "wasm32"))]
            println!("Opening link: {}", url);
            open::that_in_background(url);
            println!("Link!!!!");
            #[cfg(target_arch = "wasm32")]
            tracing::warn!("opening link({}) not supported on web yet.", url);
            Handled::Yes
        }*/


        //println!("Sono in command: {:?}", cmd);
        //let rect = cmd.get_unchecked(SCROLL_TO_VIEW);
        //_ctx.submit_command(SCROLL_TO_VIEW.with(*rect));

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.open_file(file_info.path().to_str().unwrap().to_string());
            //println!("{:?}", data);
            return Handled::Yes;
        }
        else {
            Handled::No
        }
    }
            
}
use epub::doc::*;
pub fn main() {

    // records an event outside of any span context:
    //event!(Level::INFO, "something happened");

    // let span = span!(Level::INFO, "my_span");
    // let _guard = span.enter();
    // 
    // // records an event within "my_span".
    // event!(Level::DEBUG, "something happened inside my_span");


    let initial_state = AppState::new("/home/drivesec/Downloads/I sette mariti.epub".to_string());


    let window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .menu(make_menu)
        .window_size((700.0, 600.0));

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");

    
        
}


fn page_ui() -> impl Widget<PageItem> {
    // Change lens from RichText to PageItem in order to access both to richtext and page number
    Flex::row().with_flex_child(epub_page::EpubPage::new(0)
    .lens(PageItem::page_text)
    .padding(15.0), 1.)    
}

fn build_root_widget() -> impl Widget<AppState> {
    //let mut sc = ;
    
    // let mut ep = EpubDoc::new("/home/drivesec/Downloads/I sette mariti.epub".to_string()).unwrap();
// 
    // druid::widget::Image::new(
    //     druid::ImageBuf::from_data(&ep.get_cover().unwrap()).unwrap(),
    // )
    // .interpolation_mode(druid::piet::InterpolationMode::Bilinear)
    
    let ll = Scroll::new(
         List::new(page_ui).lens(AppState::pages))
        .vertical().controller(ScrollController);

    
    Flex::column()
    .with_default_spacer()
    .with_child(build_toolbar())
    .with_default_spacer()
    .with_flex_child(ll, 1.0)
    .env_scope(|env: &mut druid::Env, data: &AppState| {
        env.set(SELECTED_TOOL, data.selected_tool.clone());
    })

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

    Flex::row()
    .with_child(bt1)
    .with_child(bt2)
    .with_child(bt3)
    .with_child(bt4)

    
}

struct ScrollController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for ScrollController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        //println!("{:?}", ctx.)
        match event {
            Event::Command(cmd) => {
                let rect = Rect::new(0., 1000., 0., 10000.);
                ctx.submit_command(SCROLL_TO_VIEW.with(rect));
                let a = Event::Command(Command::new(SCROLL_TO_VIEW, rect, Target::Global));
                child.event(ctx, &a, data, env);
                println!("cmd: {:?}", cmd);
                if let Some(pos) = cmd.get(SCROLL_TO) {
                    println!("{}", pos);
                    
                //child.scroll().scroll_by(afa);
                            
                //let tt = sc.offset();
                //println!("PRINT{:?}", tt);
                //let aa = sc.scroll_by(tt);

                }
            },
            
            _ => child.event(ctx, event, data, env),
        }
    }

    //fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {

    //}

}



#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window_id: Option<WindowId>, _app_state: &AppState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(
            Menu::new(LocalizedString::new("common-menu-file-menu"))
                .entry(druid::platform_menus::mac::file::open())
                .entry(druid::platform_menus::mac::file::close())
                .entry(druid::platform_menus::mac::file::exit()),
        );
    }
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "openbsd"))]
    {
        base = base.entry(
            Menu::new(LocalizedString::new("common-menu-file-menu"))
                .entry(druid::platform_menus::win::file::open())
                .entry(druid::platform_menus::win::file::close())
                .entry(druid::platform_menus::win::file::exit()),
        );
    }

    base.entry(
        Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .separator()
            .entry(druid::platform_menus::common::copy()),
    )
}



