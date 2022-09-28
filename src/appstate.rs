use druid::im::{Vector};
use druid::text::RichText;
use druid::{
    commands, AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, Lens, Target,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::PageType;
use crate::tool::Tool;
use epub::doc::EpubDoc;


#[derive(Clone, Data, Lens)]
pub struct AppState {

    pub selected: bool,
    pub selected_tool: Tool,
    pub epub_data : EpubData,
    pub home_page_data: HomePageData,
    pub active_page : PageType,
    pub search_input : String,
}



const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);

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

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.open_file(file_info.path().to_str().unwrap().to_string());


            return Handled::Yes;
        } else {
            Handled::No
        }
    }
}

impl AppState {
    pub fn new() -> Self {

        AppState {
            selected: false,

            selected_tool: Tool::default(),
            home_page_data: HomePageData::new(),
            epub_data: EpubData::new(Vector::new()),
            active_page : PageType::Home,
            search_input : String::new(),
        }
    }

    fn load_file(file_path: &str) -> Vector<ArcStr> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);
        assert!(doc.is_ok());
        let mut doc = doc.unwrap();
        let _m = 0;
            while doc.go_next().is_ok() {
                pages.push_back(ArcStr::from(doc.get_current_str().unwrap().clone()));
                
            }
         pages
    }

    pub fn open_file(&mut self, file_path: String) {
        let pages = AppState::load_file(&file_path);
        self.epub_data = EpubData::new(pages);

    }

}




#[derive(Clone, Lens, Default, Debug)]
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

impl Data for EpubMetrics {
    fn same(&self, other: &Self) -> bool {
        self.book_length == other.book_length
        && self.current_chapter == other.current_chapter
        && self.position_in_chapter == other.position_in_chapter
        
    }
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

        //println!("EpubMetrics: {:?}", self);
    }

}



#[derive(Clone, Lens, Data)]
pub struct EpubData {
    pub current_chapter : usize,

    pub epub_metrics : EpubMetrics,


    // Plain text of all book 
    pub chapters: Vector<ArcStr>,

    pub visualized_page : RichText,
    pub visualized_page_position : usize,

    pub table_of_contents : Vector<TocItems>,
    pub search_results : Vector<SearchResult>,
    
    // maintain font size
    pub font_size : f64,
}

#[derive(Clone, Lens, Data)]
pub struct SearchResult {
    pub key : String,
    pub value: Arc<PagePosition>
}

impl SearchResult {
    pub fn new(key : String, value : Arc<PagePosition>) -> SearchResult {
        Self {
            key,
            value
        }
    }

}

#[derive(Clone, Lens, Data)]
pub struct TocItems {
    pub key : String,
    pub value: Arc<PagePosition>
}

impl TocItems {
    pub fn new(key : String, value : Arc<PagePosition>) -> TocItems {
        Self {
            key,
            value
        }
    }

}


impl EpubData {
    pub fn new(chapters: Vector<ArcStr>) -> Self {
        let mut map : Vector<TocItems> = Vector::new();
        for i in 0..chapters.len() {
        let (_, mp) = rebuild_rendered_text(&chapters[i], i as i32);
            if mp.len() != 0 {
                map.push_back(mp[0].clone());
            }
            else {
                println!("{:?}", mp.len());
            }
        }
        let visualized_page = if chapters.len() > 0 {
         rebuild_rendered_text(&chapters[0], -1).0

        }
        else {
            RichText::new(ArcStr::from(""))
        };
        let epub_metrics = EpubMetrics::new(&chapters, visualized_page.len());


        
        EpubData { 
            current_chapter: 0, 
            chapters, 
            visualized_page,
            visualized_page_position : 0,
            epub_metrics,
            table_of_contents : map,
            font_size: 14.,
            search_results: Vector::new()
        }
        
    }

    // Search the match in all text and 
    // return a tuple with a string containing 5 words near match result and a PagePosition referring to the match
    pub fn search_string_in_book(&mut self, search_string : &str) {
        let mut results = Vector::new();
        
        for (i, chapter) in self.chapters.iter().enumerate() {
            if let Some(start) = chapter.find(search_string) {
                let mut start = start as i32 - 15;
                if start < 0 {
                    start = 0;
                }
                let mut end = start as i32 + 15;
                if end > chapter.len() as i32 {
                    end = chapter.len() as i32;
                }
                let page_position = PagePosition::new(i as i32, start, end);

                let key = chapter[start as usize..end as usize].to_owned();
                let value = Arc::new(page_position);
                let search_result = SearchResult::new(key.to_string(), value);
                results.push_back(search_result);
            }
        }


        self.search_results = results
    }
    

    fn get_current_chapter(&self) -> &ArcStr {
        &self.chapters[self.current_chapter]
    }
    
    pub fn next_chapter(&mut self) {
        self.current_chapter+=1;
        //self.pages = self.chapters[self.current_chapter].clone();
        //self.visualized_page.as_str()[0..10];
        let (rich, _) = rebuild_rendered_text(self.get_current_chapter(), -1);
        self.epub_metrics.change_chapter(self.current_chapter, rich.len());

        self.visualized_page = rich;
    }

    pub fn move_to_pos(&mut self, position : &PagePosition) {
        self.current_chapter = position.chapter;
        let (rich, _) = rebuild_rendered_text(self.get_current_chapter(), -1);

        self.epub_metrics.change_chapter(self.current_chapter, rich.len());
        self.visualized_page = rich;

    }

    pub fn previous_chapter(&mut self) {
        self.current_chapter-=1;

        let (rich, _) = rebuild_rendered_text(self.get_current_chapter(), -1);
        self.epub_metrics.change_chapter(self.current_chapter, rich.len());

        self.visualized_page = rich;
        // go to last position in chapter
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
            HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Image(_) // | HtmlTag::Link(_) 
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
                    .text_color(LINK_COLOR);
                    //.link(SCROLL_TO.with(100)); //.with(Rect::new(10., 10., 10., 10.)));
            }
            HtmlTag::Image(_img) => {}
            _ => {
                return;
            } //println!("Unhandled tag: {:?}", token)},
        }
    }
    
}

#[derive (Clone)]
pub struct PagePosition {
    pub chapter: usize, // chap_num
    pub page: usize, // page_pos 
}

impl PagePosition {
    pub fn new(chapter: i32, start: i32, end: i32) -> Self {
        PagePosition {
            chapter: chapter as usize,
            page: start as usize,
        }
    }
}

pub fn rebuild_rendered_text(text: &str, char_num : i32 ) -> (RichText, Vector<TocItems>) {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    let mut chaps : Vector<TocItems> = Vector::new();

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

                        builder.push("\n\n");
                        
                    }
                }
                xmlparser::ElementEnd::Empty => {
                    token_stack.pop().expect("No token on stack");
                }
            },

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();
                if *inner_tag == HtmlTag::Header(1) {
                    if char_num != -1 {
                        chaps.push_back(
                            TocItems::new(text.to_string(), 
                            Arc::new(PagePosition{ chapter: char_num as usize, page: current_pos}
                            )));
                        }
                }
                if inner_tag.should_tag_be_written() || text.trim().is_empty() {
                    continue;
                } else {


                    let t = text.as_str().replace("\n", "");
                    current_pos = current_pos + t.len();

                    builder.push(&t);
                    
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
    (builder.build(), chaps)
}


//#[derive(Clone, Lens, Data)]
//pub struct PageItem {
//    pub page_number: u32,
//    pub plain_text: ArcStr,
//    pub html_text: ArcStr,
//    pub page_text: RichText,
//}