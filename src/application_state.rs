use druid::im::Vector;
use druid::text::RichText;
use druid::{
    commands, AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, Lens, Target,
};
use std::sync::Arc;

use crate::tool::Tool;
use crate::widgets::navigator::uiview::{self as nav_uiview, UiView};
use epub::doc::EpubDoc;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub nav_state: Arc<Vec<nav_uiview::UiView>>,
    //contacts: Arc<Vec<Contact>>,
    pub selected: bool,
    pages: Vector<PageItem>,
    file_opened: String,
    scroll_position: u64,
    pub selected_tool: Tool,
    pub home_page_data: HomePageData,
}

#[derive(Clone, Data, Lens)]
pub struct HomePageData {
    // Use a string for save paths in order to make
    // data more easy
    pub recents: Vector<Recent>,
}

#[derive(Clone, Data, Lens)]
pub struct Recent {
    pub name: String,
    pub path: String,
    pub image_data: druid::ImageBuf,
}
impl Recent {
    pub fn new(name: String, path: String, image_data: druid::ImageBuf) -> Self {
        Recent {
            name,
            path,
            image_data,
        }
    }
}

impl HomePageData {
    pub fn new() -> Self {
        let recents = HomePageData::load_from_state_file();
        HomePageData { recents }
    }

    fn load_from_state_file() -> Vector<Recent> {
        let mut recents: Vector<Recent> = Vector::new();
        let stub_path = "/home/syscall/Downloads/pavese_dialoghi_con_leuco.epub";
        for _i in 0..1 {
            let mut ep = epub::doc::EpubDoc::new(stub_path.to_string()).unwrap();
            let image_data = druid::ImageBuf::from_data(&ep.get_cover().unwrap()).unwrap();

            recents.push_back(Recent::new(
                "Dialoghi con Leuco".to_owned(),
                "/home/syscall/Downloads/pavese_dialoghi_con_leuco.epub".to_owned(),
                image_data,
            ));
        }

        recents
    }

    pub fn with_recents(mut self, recents: Vector<Recent>) -> Self {
        self.recents = recents;
        self
    }

    pub fn add_to_recents(&mut self, r: Recent) {
        self.recents.push_back(r.to_owned());
    }
}

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        //println!("Sono in command: {:?}", cmd);
        //let rect = cmd.get_unchecked(SCROLL_TO_VIEW);
        //_ctx.submit_command(SCROLL_TO_VIEW.with(*rect));

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.open_file(file_info.path().to_str().unwrap().to_string());
            //println!("{:?}", data);

            let new_views = Arc::make_mut(&mut data.nav_state);
            new_views.push(UiView::new("read_ebook".to_string()));
            _ctx.submit_command(Command::new(CONTACT_DETAIL, 1, Target::Auto));

            return Handled::Yes;
        } else {
            Handled::No
        }
    }
}
const CONTACT_DETAIL: druid::Selector<usize> = druid::Selector::new("contact detail");

impl AppState {
    pub fn new() -> Self {
        let pages = Vector::new(); // AppState::load_file(&file_opened);
        AppState {
            nav_state: Arc::new(vec![nav_uiview::UiView::new("home_page".to_string())]),
            selected: false,
            pages,
            file_opened: "".to_string(),
            scroll_position: 0,
            selected_tool: Tool::default(),
            home_page_data: HomePageData::new(),
        }
    }

    fn load_file(file_path: &str) -> Vector<PageItem> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);
        assert!(doc.is_ok());
        let mut doc = doc.unwrap();
        let mut m = 0;
        //let asd = generate_pages(doc);
        //asd
        let asd = 2500;
            while doc.go_next().is_ok() {
        
                let t = &doc.get_current_str().unwrap();
                let mut i = 0;
                loop  {
                    let ot = if i+1*asd < t.len() { t.len() } else { i+1 };
                    let (plain_text, page_text) = rebuild_rendered_text(&t[i..ot]);
                    m += 1;
                    pages.push_back(PageItem {
                        page_number: doc.get_current_page() as u32,
                        plain_text,
                        page_text
                    });
                    i+=1;
                    if i < t.len()/asd { break; }
                    
                }
        
        
            }
            println!("Numero di pagine totali: {}", m);
         pages
    }

    pub fn open_file(&mut self, file_path: String) {
        self.pages = AppState::load_file(&file_path);
    }
}

#[derive(Clone, Lens)]
pub struct PageItem {
    pub page_number: u32,
    pub plain_text: ArcStr,
    pub page_text: RichText,
}

impl Data for PageItem {
    fn same(&self, other: &Self) -> bool {
        self.page_number.same(&other.page_number)
    }
}

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

fn generate_pages(mut doc: epub::doc::EpubDoc<std::fs::File>) -> Vector<PageItem> {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut str = String::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    let mut pages: Vector<PageItem> = Vector::new();
    let mut cpos = 0;

    let mut pg_num: u32 = 1;

    let mut page_fin : bool = false;
    let mut page_split : bool = false;

    doc.go_next().unwrap();

    loop {

        let mut exited: bool = false;
        let mut panik: bool = false;

        let curr_page = doc.get_current_str().unwrap();
        let mut tokenizer = xmlparser::Tokenizer::from(&curr_page[cpos..]);

        while !exited {
            let tok_option = tokenizer.next();
            if tok_option.is_none() {
                page_fin = true;
                exited = true;
                continue;
            }
            let tok_result = tok_option.unwrap();

            if tok_result.is_err() {
                // handle error
                println!("Errore!!!");
                exited = true;
                panik = true;
                continue;
            }
            let token = tok_result.unwrap();
            let t_clone = token.clone();
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
                            str.push_str("\n\n");
                            current_pos += 2;
                        }
                    }
                    xmlparser::ElementEnd::Empty => {
                        token_stack.pop().expect("No token on stack");
                    }
                },

                xmlparser::Token::Text { text } => {
                    // TODO: Handle case of no tags, text only
                    let (_, inner_tag) = token_stack.last().unwrap();

                    if !should_tag_be_written(inner_tag) || text.trim().is_empty() {
                        continue;
                    } else {
                        let t = text.as_str().replace("\n", "");
                        builder.push(&t);
                        str.push_str(&t);
                        current_pos = current_pos + t.len();
                    }
                }
                _ => continue,
            }
            
            match t_clone {
    //xmlparser::Token::Declaration { version : _, encoding : _, standalone: _, span } => cpos = span.end(),
    //xmlparser::Token::ElementStart { prefix: _, local: _, span } => cpos = span.end(),
    //xmlparser::Token::EmptyDtd { name:_, external_id:_, span } => cpos = span.end(),
    //xmlparser::Token::Comment { text:_, span } => cpos = span.end(),
    //xmlparser::Token::ProcessingInstruction { target:_, content:_, span } => cpos = span.end(),
    //xmlparser::Token::DtdStart { name:_, external_id:_, span } => cpos = span.end(),
    //xmlparser::Token::EntityDeclaration { name:_, definition:_, span } => cpos = span.end(),
    //xmlparser::Token::DtdEnd { span } => cpos = span.end(),
    //xmlparser::Token::Attribute { prefix:_, local:_, value:_, span } => cpos = span.end(),
    xmlparser::Token::ElementEnd { end:_, span } => cpos = span.end(),
    //xmlparser::Token::Text { text } => cpos = text.end(),
    //xmlparser::Token::Cdata { text:_, span } => cpos = span.end(),
    _ => {}
            }
            if  str.len() >= 1800 {
                    page_split = true;
                    exited = true;
                    println!("Cpos:{} ", cpos);

            }

        }

        if page_split || page_fin {
            pages.push_back(PageItem {
                page_number: pg_num,
                plain_text: druid::ArcStr::from(str),
                page_text: builder.build()
            });

            str = String::new();
            builder = druid::text::RichTextBuilder::new();

            pg_num += 1;
        }
        if page_fin {
            let nexx = doc.go_next();
            if nexx.is_err() {
                break;
            }
            page_fin = false;
            cpos = 0;
        }
        if panik {
            break;
        }
    }
    println!("Length: {:?}", current_pos);
    pages
    //(druid::ArcStr::from(str), builder.build())
}

fn rebuild_rendered_text(text: &str) -> (druid::ArcStr, RichText) {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut str = String::new();
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
                        str.push_str("\n\n");
                        current_pos += 2;
                    }
                }
                xmlparser::ElementEnd::Empty => {
                    token_stack.pop().expect("No token on stack");
                }
            },

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();

                if !should_tag_be_written(inner_tag) || text.trim().is_empty() {
                    continue;
                } else {
                    let t = text.as_str().replace("\n", "");
                    builder.push(&t);
                    str.push_str(&t);
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
    println!("Length: {:?}", current_pos);
    (druid::ArcStr::from(str), builder.build())
}

fn add_newline_after_tag(tag: &HtmlTag) -> bool {
    matches!(
        tag,
        HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Link(_) | HtmlTag::Image(_)
    )
}

fn should_tag_be_written(tag: &HtmlTag) -> bool {
    !matches!(tag, HtmlTag::Title)
}
const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);
const SCROLL_TO: druid::Selector<u64> = druid::Selector::new("scroll-view.goto");

fn add_attribute_for_token(token: &HtmlTag, mut attrs: druid::text::AttributesAdder) {
    match token {
        HtmlTag::Header(lvl) => {
            attrs
                .size(16. + *lvl as f64)
                .weight(druid::FontWeight::BOLD);
        }
        HtmlTag::Bold => {
            attrs.weight(druid::FontWeight::BOLD);
        }
        HtmlTag::Italic => {
            attrs.style(druid::FontStyle::Italic);
        }
        HtmlTag::Underline => {
            attrs.underline(true);
        }
        HtmlTag::StrikeThrough => {
            attrs.strikethrough(true);
        }
        HtmlTag::Link(_target) => {
            //Tag::Link(_link_ty, target, _title) => {
            attrs
                .underline(true)
                .text_color(LINK_COLOR)
                .link(SCROLL_TO.with(100)); //.with(Rect::new(10., 10., 10., 10.)));
                                            //.link(SCROLL_TO_VIEW.with(Rect::new(10., 10., 10., 10.)));
                                            //.link(OPEN_LINK.with("Aaaa".to_string()));
        }
        HtmlTag::Image(_img) => {}
        _ => {
            return;
        } //println!("Unhandled tag: {:?}", token)},
    }
}
