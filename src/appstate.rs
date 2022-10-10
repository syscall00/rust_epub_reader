use druid::im::{Vector};
use druid::piet::TextStorage;
use druid::text::RichText;
use druid::{
    commands, AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, Lens, Target,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::PageType;
use crate::core::commands::VisualizationMode;
use crate::core::style::LINK_COLOR;
use crate::tool::Tool;
use epub::doc::EpubDoc;

use self::epub_data_derived_lenses::visualized_chapter;


#[derive(Clone, Data, Lens)]
pub struct AppState {

    pub epub_data : EpubData,
    pub home_page_data: HomePageData,
    pub active_page : PageType,
}




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
            home_page_data: HomePageData::new(),
            epub_data: EpubData::new(Vector::new()),
            active_page : PageType::Home,
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


#[derive(Clone, Lens, Default, Debug, Data)]
pub struct EpubMetrics {
    // Static metrics:
    pub num_chapters: usize,
    pub book_length : usize,

    // calculate at change of new chapter:
    pub current_chapter: usize,
    pub current_page_in_chapter: usize,
    pub complessive_page_number: usize,
    pub chapter_length: usize,

}

impl EpubMetrics {
    pub fn new(pages : &Vector<ArcStr>, initial_len : usize) -> Self {
        let num_chapters = pages.len();
        let book_length = pages.iter().map(|p| p.len()).sum();
        let chapter_length = initial_len;

        EpubMetrics {
            num_chapters,
            book_length,
            current_chapter: 0,
            current_page_in_chapter : 0,
            chapter_length,
            complessive_page_number: 0,
        }
    }
    pub fn get_next_page_in_chap(&self) -> usize {
        self.current_page_in_chapter + 1
    }

    pub fn get_previous_page_in_chap(&self) -> usize {
        if self.current_page_in_chapter == 0 {
            0
        } else {
            self.current_page_in_chapter - 1
        }
    }

    pub fn change_chapter(&mut self, chapter_length : usize) {
        self.chapter_length = chapter_length;
        self.change_page(0);

    }
    
    pub fn change_page (&mut self, current_position : usize) {
        //self.position_in_chapter = current_position;
        //self.percentage_page_in_chapter = self.position_in_chapter as f64 / self.chapter_length as f64 * 100.;
        //self.percentage_page_in_book = self.position_in_chapter as f64 / self.book_length as f64 * 100.;

    }

}



#[derive(Clone, Lens, Data)]
pub struct EpubData {

    pub epub_metrics : EpubMetrics,
    pub edit_mode : bool,

    // Plain text of all book 
    pub chapters: Vector<ArcStr>,
    pub rich_chapters: Vector<RichText>,

    pub visualized_page : RichText,
    pub visualized_chapter : String,
    pub visualized_page_position : usize,

    pub table_of_contents : Vector<IndexedText>,
    pub search_results : Vector<IndexedText>,
    pub book_highlights : Vector<IndexedText>,
    
    // maintain font size
    pub font_size : f64,

    pub search_input : String,
    pub visualization_mode : VisualizationMode,


    pub selected_tool : Tool
    
}


#[derive(Clone, Lens, Data)]
pub struct IndexedText {
    pub key : ArcStr,
    pub value : Arc<PagePosition>,
}

impl IndexedText {
    pub fn new(key : ArcStr, value : Arc<PagePosition>) -> Self {
        IndexedText {
            key,
            value,
        }
    }
}


impl EpubData {
    pub fn new(chapters: Vector<ArcStr>) -> Self {
        let mut map : Vector<IndexedText> = Vector::new();
        let mut rich_chapters : Vector<RichText> = Vector::new();
        for i in 0..chapters.len() {
        let (rich, mp) = rebuild_rendered_text(&chapters[i], i as i32);
            if mp.len() != 0 {
                map.push_back(mp[0].clone());
            }
            rich_chapters.push_back(rich);

        }
        if chapters.len() == 0 {
            map.push_back(IndexedText::new(ArcStr::from("No chapters found"), Arc::new(PagePosition::new(0, 0, 0))));
            rich_chapters.push_back(RichText::new(ArcStr::from("No chapters found")));
        }
        let visualized_chapterr = if chapters.len() > 0  {
            chapters[0].clone().to_string()
        } 
        else {
            String::new()
        };
        
        
        let epub_metrics = EpubMetrics::new(&chapters, rich_chapters[0].len());

        EpubData { 
            visualized_chapter : visualized_chapterr,
            chapters, 
            visualized_page : rich_chapters[0].clone(),
            visualized_page_position : 0,
            epub_metrics,
            table_of_contents : map,
            edit_mode : false,
            font_size: 14.,
            search_results: Vector::new(),
            search_input : String::new(),
            visualization_mode : VisualizationMode::Single,
            book_highlights : Vector::new(),
            rich_chapters,
            selected_tool : Tool::default(),
            
        }
        
    }

    // Search the match in all text and 
    // return a tuple with a string containing 5 words near match result and a PagePosition referring to the match
    pub fn search_string_in_book(&mut self) {
        const MAX_SEARCH_RESULTS : usize = 100;
        const CHARACTERS_BEFORE : usize = 10;
        const CHARACTERS_AFTER : usize = 10;
        let mut results = Vector::new();
        if !self.search_input.is_empty() {
         
        
            for (i, chapter) in self.rich_chapters.iter().enumerate() {
                let start_matches : Vec<usize> = chapter.as_str().match_indices(&self.search_input).map(|(i, _)|i).collect();
                for st in start_matches {

                    let start = if st < CHARACTERS_BEFORE {
                        0
                    }
                    else {
                        st - CHARACTERS_BEFORE
                    };

                    
                    let end = if start + CHARACTERS_AFTER > chapter.len() {
                        chapter.len() 
                    } 
                    else {
                        start + CHARACTERS_AFTER
                    };

                    let page_position = PagePosition::new(i, start, end);
                    let text = utf8_slice::slice(&chapter.as_str(), start as usize, end as usize);
                    //let text = ArcStr::from(chapter.as_str().chars().skip(start as usize).take((end-start) as usize).collect::<String>());
                    let value = Arc::new(page_position);
                    let search_result = IndexedText::new(ArcStr::from(text.to_string()), value);
                    results.push_back(search_result);
                    if results.len() > MAX_SEARCH_RESULTS {
                        break;
                    }
                }
            }
        }

        println!("Search results: {:?}", results.len());
        self.search_results = results
    }
    

    pub fn add_book_highlight(&mut self, start : usize, end: usize) {
        let text = utf8_slice::slice(&self.visualized_page.as_str(), start as usize, end as usize);
        let page_position = PagePosition::new(self.epub_metrics.current_chapter, start, end);
        let value = Arc::new(page_position);
        let hightlight = IndexedText::new(ArcStr::from(text.to_string()), value);
        self.book_highlights.push_back(hightlight);
    }

    fn get_current_chapter(&self) -> &ArcStr {
        &self.chapters[self.epub_metrics.current_chapter]
    }
    
    pub fn next(&mut self, can_move : bool) {
        self.epub_metrics.current_page_in_chapter += 1;
        self.epub_metrics.complessive_page_number += 1;
        if !can_move {
            self.next_chapter();
        }

    }
    pub fn next_chapter(&mut self) {
        self.epub_metrics.current_chapter+=1;
        //self.pages = self.chapters[self.current_chapter].clone();
        //self.visualized_page.as_str()[0..10];
        self.visualized_page = self.rich_chapters[self.epub_metrics.current_chapter].clone();
        self.epub_metrics.change_chapter(self.visualized_page.len());

    }

    pub fn move_to_pos(&mut self, position : &PagePosition) {
        let t = self.rich_chapters[position.chapter].clone();
        self.epub_metrics.change_chapter(t.len());
        self.epub_metrics.current_page_in_chapter = position.page;
        println!("Page: {:?}", position);
        self.visualized_page = t;

    }

    pub fn previous_chapter(&mut self) {
        let (rich, _) = rebuild_rendered_text(self.get_current_chapter(), -1);
        self.epub_metrics.change_chapter(rich.len());

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
            } 
        }
    }
    
}

#[derive (Clone, Debug)]
pub struct PagePosition {
    pub chapter: usize, // chap_num
    pub page: usize, // page_pos
    pub slice : (usize, usize),
}

impl PagePosition {
    pub fn new(chapter: usize, start: usize, end: usize) -> Self {
        PagePosition {
            chapter,
            page: start,
            slice: (start, end)
        }
    }
}

pub fn rebuild_rendered_text(text: &str, char_num : i32 ) -> (RichText, Vector<IndexedText>) {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    let mut chaps  = Vector::new();

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
            xmlparser::Token::ElementEnd { end, span: local } => 
            {
            match end {
                
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

                    tk.add_attribute_for_token(builder.add_attributes_for_range(pos..current_pos));

                    if tk != HtmlTag::Unhandled && tk.add_newline_after_tag() {
                        current_pos += 2;

                        builder.push("\n\n");
                        
                    }
                }
                xmlparser::ElementEnd::Empty => {
                    token_stack.pop().expect("No token on stack");
                }
            }


            },

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();
                if *inner_tag == HtmlTag::Header(1) {
                    if char_num != -1 {
                        chaps.push_back(
                            IndexedText::new(ArcStr::from(text.to_string()), 
                            Arc::new(PagePosition::new(char_num as usize, current_pos, 0))));
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
            xmlparser::Token::Attribute { prefix: _, local: loc, value : val , span : span } => {
                //println!("attr: {:?} = {:?}", loc, val);
                continue;
            },

            _ => continue,
            /*
            xmlparser::Token::Declaration { version, encoding, standalone, span } => {
                // for now, ignore declarations
                continue;
            },
            xmlparser::Token::EmptyDtd { nfame, external_id, span } => {
                // for now, ignore the DTD
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