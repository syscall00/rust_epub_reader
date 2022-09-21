use druid::im::Vector;
use druid::text::RichText;
use druid::{
    commands, AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, Lens, Target,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::tool::Tool;
use crate::widgets::navigator::uiview::{self as nav_uiview, UiView};
use epub::doc::EpubDoc;
use rand::Rng;

use self::epub_data_derived_lenses::current_chapter;
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub nav_state: Arc<Vec<nav_uiview::UiView>>,
    //contacts: Arc<Vec<Contact>>,
    pub selected: bool,
    pages: Vector<PageItem>,
    file_opened: String,
    pub selected_tool: Tool,
    //pub current_page : PageItem,
    pub epub_data : EpubData,
    pub home_page_data: HomePageData,
    pub slider_value : f64,

    my_tree : Vector<HtmlTag>
}
const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);
const SCROLL_TO: druid::Selector<u64> = druid::Selector::new("scroll-view.goto");

#[derive(Clone, Data, Lens)]
pub struct HomePageData {
    // Use a string for save paths in order to make
    // data more easy
    pub recents: Vector<Recent>,
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
pub struct Recent {
    pub name: String,
    pub path: String,
    //pub image_data: Vector<u8>,
}
impl Recent {
    pub fn new(name: String, path: String) -> Self {
        Recent {
            name,
            path,
            //image_data,
        }
    }
}

impl HomePageData {
    pub fn new() -> Self {
        let recents = HomePageData::load_from_state_file();
        HomePageData { recents }
    }

    fn load_from_state_file() -> Vector<Recent> {
        let recents_fname = ".recents";
        let md = std::fs::metadata(recents_fname);
        // file does not exists!!!
        let recents_string = if md.is_err() {
            std::fs::File::create(recents_fname).unwrap();
            return Vector::default();

        } else {
          std::fs::read_to_string(recents_fname).unwrap()

        };
        

        let recents : Vec<Recent> = serde_json::from_str(&recents_string).unwrap();
        recents.into()
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
        //let rect = cmd.get_unchecked(SCROLL_TO_VIEW);
        //_ctx.submit_command(SCROLL_TO_VIEW.with(*rect));

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.open_file(file_info.path().to_str().unwrap().to_string());

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
            selected_tool: Tool::default(),
            //current_page : clone,
            home_page_data: HomePageData::new(),
            my_tree : Vector::new(),
            slider_value : 0.,
            epub_data: EpubData::default(),
        }
    }

    fn load_file(file_path: &str) -> Vector<PageItem> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);
        assert!(doc.is_ok());
        let mut doc = doc.unwrap();
        let mut m = 0;
            while doc.go_next().is_ok() {
        
                let page_text = rebuild_rendered_text(&doc.get_current_str().unwrap(), 0);
                let pi = PageItem {
                    page_number: m,
                    html_text: ArcStr::from(doc.get_current_str().unwrap().clone()),
                    plain_text: page_text.0,
                    page_text: page_text.1,
                };
                pages.push_back(pi);
                m += 1;
            }
            println!("Numero di pagine totali: {}", m);
         pages
    }

    pub fn open_file(&mut self, file_path: String) {
        self.pages = AppState::load_file(&file_path);
        //let cl = self.pages.get(0).clone().unwrap().to_owned();
        //self.current_page = cl;
        self.epub_data = EpubData::new(self.pages.iter().map(|p| p.html_text.clone()).collect());

        
        //self.pages.clear();
        //self.pages = AppState::load_file(&file_path);
    }

}

#[derive(Clone, Lens, Data)]
pub struct PageItem {
    pub page_number: u32,
    pub plain_text: ArcStr,
    pub html_text: ArcStr,
    pub page_text: RichText,
}


#[derive(Clone, Lens, Data, Default, Debug)]
pub struct EpubMetrics {
    /*
    Metric for book position:
    - Num of chapter
    - Num of page in chapter (obtained dividing max length of chapter by page length)
    - Percentage of page in chapter (obtained dividing current position by max position of chapter)
    - Percentage of book (obtained dividing current position by max position of book)
    - Position in page
    - Position in book    
    */

    // Static metrics:
    pub num_chapters: usize,
    pub book_length : usize,

    // calculate at change of new chapter:
    pub current_chapter: usize,
    pub chapter_length: usize,


    // calculate at change of new page:
    pub position_in_chapter: usize,
    pub percentage_page_in_chapter: f64,
    pub percentage_page_in_book: f64,
    // pub position_in_book: u32,
}

impl EpubMetrics {
    pub fn new(pages : &Vector<ArcStr>, initial_len : usize) -> Self {
        let num_chapters = pages.len();
        let book_length = pages.iter().map(|p| p.len()).sum();
        let chapter_length = initial_len;
        let position_in_chapter = 0;
        let percentage_page_in_chapter = 0.;
        let percentage_page_in_book = 0.;
        EpubMetrics {
            num_chapters,
            book_length,
            current_chapter: 0,
            chapter_length,
            position_in_chapter,
            percentage_page_in_chapter,
            percentage_page_in_book,
        }
    }
    

    pub fn change_chapter(&mut self, new_chapter : usize, chapter_length : usize) {
        self.current_chapter = new_chapter;
        self.chapter_length = chapter_length;
        self.change_page(0);

    }
    
    pub fn change_page (&mut self, current_position : usize) {
        self.position_in_chapter = current_position;
        self.percentage_page_in_chapter = self.position_in_chapter as f64 / self.chapter_length as f64 * 100.;
        self.percentage_page_in_book = self.position_in_chapter as f64 / self.book_length as f64 * 100.;

        println!("EpubMetrics: {:?}", self);
    }

}

#[derive(Clone, Lens, Data)]
pub struct EpubData {
    pub current_chapter : usize,

    pub epub_metrics : EpubMetrics,


    // Plain text of all book 
    pub chapters: Vector<ArcStr>,

    pub visualized_page : RichText,
}

impl EpubData {
    pub fn new(chapters: Vector<ArcStr>) -> Self {
        let (_, visualized_page) = rebuild_rendered_text(&chapters[0], 0);

        let epub_metrics = EpubMetrics::new(&chapters, visualized_page.len());

        EpubData { 
            current_chapter: 0, 
            chapters, 
            visualized_page,
            epub_metrics
        }
        
    }

    /*
    Should return a list with
    - chapter number
    - page position in chapter
    */
    pub fn search_in_book(&mut self, search_string : &str, index: usize) {
        let mut search_results = Vector::new();
        for (i, chapter) in self.chapters.iter().enumerate() {
            let mut chapter_results = Vector::new();
            let mut current_position = 0;
            while let Some(pos) = chapter[  current_position..].find(search_string) {
                let (_, rich) = rebuild_rendered_text(&chapter, current_position + pos);
                chapter_results.push_back(rich);
                current_position += pos + search_string.len();
            }
            if chapter_results.len() > 0 {
                search_results.push_back((i, chapter_results));
            }
        }
        search_results.iter().for_each(|(i, v)| println!("Chapter {} has {} results", i, v.len()));
        self.move_to_pos(search_results[0].0, search_results[0].1[index-1].len());
    }

    pub fn move_to_pos(&mut self, chapter : usize, pos : usize) {
        self.current_chapter = chapter;
        self.epub_metrics.change_chapter(chapter, self.get_current_chapter().len());


        let (_, rich) = rebuild_rendered_text(self.get_current_chapter(), pos);
        self.visualized_page = rich;
        self.epub_metrics.change_page(pos);

    }

    fn get_current_chapter(&self) -> &ArcStr {
        &self.chapters[self.current_chapter]
    }
    

    pub fn next_page(&mut self, current_page : usize) {
        let (_, rich) = rebuild_rendered_text(self.get_current_chapter(), current_page);
        self.visualized_page = rich;
        self.epub_metrics.change_page(current_page);
    
    }


    pub fn next_chapter(&mut self) {
        self.current_chapter+=1;
        //self.pages = self.chapters[self.current_chapter].clone();
        let (_, rich) = rebuild_rendered_text(self.get_current_chapter(), 0);
        self.epub_metrics.change_chapter(self.current_chapter, rich.len());

        self.visualized_page = rich;


    }
    pub fn previous_chapter() {

    }
}

impl Default for EpubData {
    fn default() -> Self {
        Self { current_chapter: Default::default(), chapters: Default::default(), 
            visualized_page: RichText::new(ArcStr::from("".clone())), epub_metrics: Default::default() }
    }
}

#[derive(Debug, PartialEq, Data, Clone)]
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
impl HtmlTag {
    pub fn add_newline_after_tag(&self) -> bool {
        matches!(
            self,
            HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Link(_) | HtmlTag::Image(_)
        )
    }
    
    pub fn should_tag_be_written(&self) -> bool {
        matches!(self, HtmlTag::Title)
    }

    pub fn add_attribute_for_token(&self, mut attrs: druid::text::AttributesAdder) {
        match self {
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
    
}


pub fn rebuild_rendered_text(text: &str, len : usize) -> (druid::ArcStr, RichText) {
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

                    tk.add_attribute_for_token(builder.add_attributes_for_range(pos..current_pos));

                    if tk != HtmlTag::Unhandled && tk.add_newline_after_tag() {
                        current_pos += 2;

                        if current_pos > len {
                        builder.push("\n\n");
                        str.push_str("\n\n");
                        }
                    }
                }
                xmlparser::ElementEnd::Empty => {
                    token_stack.pop().expect("No token on stack");
                }
            },

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();

                if inner_tag.should_tag_be_written() || text.trim().is_empty() {
                    continue;
                } else {
                    let t = text.as_str().replace("\n", "");
                    current_pos = current_pos + t.len();

                    if current_pos > len {
                    builder.push(&t);
                    str.push_str(&t);
                    }
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
    (druid::ArcStr::from(str), builder.build())
}
