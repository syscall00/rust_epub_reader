#![windows_subsystem = "windows"]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::usize;
use druid::text::{AttributesAdder, RichText, RichTextBuilder, Selection};
use druid::widget::prelude::*;
use druid::widget::{LineBreaking, RawLabel, Scroll};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, FontStyle, FontWeight, Handled,
    Lens, LocalizedString, Menu, Selector, Target, Widget, WidgetExt, WindowDesc,
    WindowId,
}; // FontFamily


use druid::im::{vector, Vector};
use druid::widget::{Flex, Label, List};

mod epub_page;

#[derive(Debug, PartialEq)]
enum HtmlTag {
    Header(u8),
    Link(String),
    Image(String),
    Paragraph,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
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
            "a" => HtmlTag::Link("".to_string()),
            "img" => HtmlTag::Image("".to_string()),
            "p" => HtmlTag::Paragraph,
            "strong" | "b" => HtmlTag::Bold,
            "em" | "i" => HtmlTag::Italic,
            "u" => HtmlTag::Underline,
            "del" | "s" => HtmlTag::StrikeThrough,
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
                if text.trim().is_empty() {
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
    !matches!(
        tag,
        HtmlTag::Italic | HtmlTag::Bold | HtmlTag::StrikeThrough | HtmlTag::Link(_)
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
        HtmlTag::Link(_) => {
            //Tag::Link(_link_ty, target, _title) => {
            attrs.underline(true).text_color(LINK_COLOR); //.link(OPEN_LINK.with(target.to_string()));
        }
        _ => {
            return;
        } //println!("Unhandled tag: {:?}", token)},
    }
}



const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Epub Reader");

const TEXT: &str = "";

const SPACER_SIZE: f64 = 8.0;
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);
const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");

#[derive(Clone, Data, Lens)]
struct AppState {
    raw: String,
    rendered: RichText,
    pages: Vector<PageItem>,
}

#[derive(Clone, Lens, Data)]
struct PageItem {
    page_number: usize,
    page_text: RichText,
}


struct Delegate;

use tracing::{event, span, Level};
use epub::doc::EpubDoc;

impl<T: Data> AppDelegate<T> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut T,
        _env: &Env,
    ) -> Handled {
        if let Some(url) = cmd.get(OPEN_LINK) {
            #[cfg(not(target_arch = "wasm32"))]
            open::that_in_background(url);
            #[cfg(target_arch = "wasm32")]
            tracing::warn!("opening link({}) not supported on web yet.", url);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
use epub::doc::*;
pub fn main() {

    // records an event outside of any span context:
    event!(Level::INFO, "something happened");

    // let span = span!(Level::INFO, "my_span");
    // let _guard = span.enter();
    // 
    // // records an event within "my_span".
    // event!(Level::DEBUG, "something happened inside my_span");

    let doc = EpubDoc::new("/home/drivesec/Downloads/I sette mariti.epub");
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();


    //let v = doc.get_resource("ncx").unwrap();
    //let s = std::str::from_utf8(&v);
    //println!("{:?}", doc.resources );
    //println!("{:?}", s);


    let mut pages : Vector<PageItem> = Vector::new();


    while doc.go_next().is_ok() {
        let page_text = rebuild_rendered_text(&doc.get_current_str().unwrap());
        pages.push_back(PageItem {
            page_number: doc.get_current_page(),
            page_text,
        });
    }


    let initial_state = AppState {
        raw: TEXT.to_owned(),
        rendered: rebuild_rendered_text(TEXT),
        pages: pages
    };



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



fn item_ui() -> impl Widget<PageItem> {
    let label = RawLabel::new()
    .with_text_color(Color::BLACK)
    .with_line_break_mode(LineBreaking::WordWrap)
    .lens(PageItem::page_text)
    .expand_width()
    .padding((SPACER_SIZE * 4.0, SPACER_SIZE));
    
    label
}

fn page_ui() -> impl Widget<PageItem> {


    // Change lens from RichText to PageItem in order to access both to richtext and page number
    let p = epub_page::EpubPage::new(0)
    .lens(PageItem::page_text)
    .padding(15.0);
    Flex::row().with_flex_child(p, 1.)    
}

fn build_root_widget() -> impl Widget<AppState> {
    //let list = Scroll::new(
    //    List::new(item_ui).lens(AppState::pages)).vertical();

    let ll = Scroll::new(
        List::new(page_ui).lens(AppState::pages)).vertical();

        //Split::columns(list, ll)
    ll
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



