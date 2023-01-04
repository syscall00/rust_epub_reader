use druid::im::Vector;
use druid::piet::TextStorage;
use druid::text::RichText;
use druid::{
    AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, ImageBuf, Lens, Target,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::core::constants::commands::INTERNAL_COMMAND;
use crate::data::epub::settings::EpubSettings;
use crate::data::home::HomePageData;
use crate::PageType;
use epub::doc::EpubDoc;

use crate::dom::{generate_renderable_tree, Renderable};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub epub_data: EpubData,
    pub home_page_data: HomePageData,
    pub active_page: PageType,
}

#[derive(Clone, Data, Lens, Debug)]
pub struct RecentData {
    pub image_data: Option<ImageBuf>,
    pub title: ArcStr,
    pub creator: ArcStr,
    pub publisher: ArcStr,
    pub position_in_book: usize,
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
pub struct Recent {
    pub path: String,
    pub reached_position: Option<PagePosition>,

    pub epub_settings: EpubSettings,

    // ignore this field for serialization
    #[serde(skip)]
    pub image_data: Option<ImageBuf>,

    #[serde(skip)]
    pub recent_data: Option<RecentData>,
}

impl Recent {
    pub fn new(path: String) -> Self {
        Recent {
            path,
            reached_position: None,
            epub_settings: EpubSettings::default(),
            image_data: None,
            recent_data: None,
        }
    }

    pub fn set_recent_data(&mut self, recent_data: RecentData) {
        self.recent_data = Some(recent_data);
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
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {

            let recent = Recent::new(file_info.path().to_str().unwrap().to_string());
            // if recent already exists, do not add it again
            if !(data
                .home_page_data
                .recents
                .iter()
                .any(|r| r.path == recent.path))
            {
                data.open_file(&recent);
                data.home_page_data.add_to_recents(recent);
                data.active_page = PageType::Reader;
            }
            return Handled::Yes;
        }
        else if let Some(file_info) = cmd.get(crate::core::commands::OPEN_RECENT) {
            data.open_file(file_info);
            return Handled::Yes;
        } 
        else if let Some(command) = cmd.get(INTERNAL_COMMAND) {
            let ret = match command {
                crate::core::constants::commands::InternalUICommand::RemoveBook(book_path) => {
                    // remove book from recent
                    data.home_page_data.remove_from_recents(book_path);
                    return Handled::Yes;
                }
                crate::core::constants::commands::InternalUICommand::UpdateBookInfo(book_path) => {
                    let recent = data.home_page_data.get_recent(book_path);
                    let mut recent = if let Some(recent) = recent {
                        recent
                    } else {
                        Recent::new(book_path.to_string())
                    };

                    recent.reached_position = Some(data.epub_data.page_position.clone());
                    println!("Reached position: {:?}", recent.reached_position);
                    recent.epub_settings = data.epub_data.epub_settings.clone();
                    data.home_page_data.update_recent(recent);

                    return Handled::Yes;
                }
                _ => Handled::No,
            };
            return ret;
        } else {
            Handled::No
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            home_page_data: HomePageData::new(),
            epub_data: EpubData::empty_epub_data(),
            active_page: PageType::Home,
        }
    }

    fn load_file(file_path: &str) -> Vector<ArcStr> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);

        assert!(doc.is_ok());
        let mut doc = doc.unwrap();
        let _m = 0;
        loop {
            pages.push_back(ArcStr::from(doc.get_current_str().unwrap().clone()));
            if !doc.go_next().is_ok() {
                break;
            };
        }
        pages
    }

    pub fn open_file(&mut self, file_info: &Recent) {
        let pages = AppState::load_file(&file_info.path);
        let doc = EpubDoc::new(&file_info.path);

        assert!(doc.is_ok());
        let doc = doc.unwrap();

        self.epub_data = EpubData::new(pages, file_info.path.to_owned(), doc);
        self.epub_data.epub_settings = file_info.epub_settings.to_owned();
        if let Some(page_index) = &file_info.reached_position {
            self.epub_data.page_position = page_index.to_owned();
            self.epub_data.change_position(page_index.clone());
        }
    }
}

#[derive(Clone, Lens, Default, Debug, Data)]
pub struct EpubMetrics {
    pub num_chapters: usize,
    pub current_chapter: usize,
}

impl EpubMetrics {
    pub fn new(pages: &Vector<ArcStr>) -> Self {
        let num_chapters = pages.len();

        EpubMetrics {
            num_chapters,
            current_chapter: 0,
        }
    }

    pub fn change_chapter(&mut self, chapter_num: usize) {
        self.current_chapter = chapter_num;
    }
}

#[derive(Clone, Lens, Data)]
pub struct SidebarData {
    pub table_of_contents: Vector<IndexedText>,
    pub search_results: Vector<IndexedText>,

    pub search_input: String,
}

impl SidebarData {
    pub fn new(table_of_contents: Vector<IndexedText>) -> Self {
        SidebarData {
            table_of_contents,
            search_results: Vector::new(),
            search_input: String::default(),
        }
    }
}


#[derive(Clone, Lens, Data)]
pub struct EpubData {
    pub epub_metrics: EpubMetrics,

    // Plain text of all book
    pub chapters: Vector<ArcStr>,
    pub renderable_chapters: Vector<Vector<Renderable>>,

    pub page_position: PagePosition,
    pub sidebar_data: SidebarData,
    pub epub_settings: EpubSettings,

    pub ocr_data: OcrData,

    pub book_path: String,
    
    
    
    
    pub visualized_chapter: String,
    pub edit_mode: bool,

    pub doc: Arc<std::sync::Mutex<EpubDoc<std::io::BufReader<File>>>>,
}

#[derive(Clone, Lens, Data)]
pub struct OcrData {
    pub image_to_pos: String,
    pub image_for_pos_1: String,
    pub image_for_pos_2 : String,

    pub ocr_result: PagePosition,
    pub reverse_ocr_result: PagePosition,

    pub mode: OcrMode,

    pub processing: bool,

}

#[derive(Clone, Data, PartialEq)]
pub enum OcrMode {
    FindByPhoto,
    FindByVirtual,
}
pub const EMPTY_STRING: &str = "Please, choose an image";

impl Default for OcrData {
    fn default() -> Self {
        OcrData {
            image_to_pos: EMPTY_STRING.to_owned(),
            image_for_pos_1: EMPTY_STRING.to_owned(),
            image_for_pos_2: EMPTY_STRING.to_owned(),
            ocr_result: PagePosition::new(0,0),
            reverse_ocr_result: PagePosition::new(0,0),
            mode: OcrMode::FindByPhoto,
            processing: false,
        }
    }
}

#[derive(Clone, Lens, Data)]
pub struct IndexedText {
    pub key: ArcStr,
    pub value: Arc<PagePosition>,
}

impl IndexedText {
    pub fn new(key: ArcStr, value: Arc<PagePosition>) -> Self {
        IndexedText { key, value }
    }
}
impl Default for IndexedText {
    fn default() -> Self {
        IndexedText {
            key: ArcStr::from(""),
            value: Arc::new(PagePosition::new(0, 0)),
        }
    }
}
impl EpubData {
    pub fn empty_epub_data() -> Self {
        EpubData {
            epub_metrics: EpubMetrics::new(&Vector::new()),
            chapters: Vector::new(),
            renderable_chapters: Vector::new(),
            visualized_chapter: String::new(),
            sidebar_data: SidebarData::new(Vector::new()),
            edit_mode: false,
            epub_settings: EpubSettings::default(),

            book_path: String::new(),
            page_position: PagePosition::new(0, 0),
            ocr_data: OcrData::default(),
            // TODO: Are you fucking serious?
            doc: Arc::new(Mutex::new(
                EpubDoc::new("examples/1.epub").unwrap(),
            )),
        }
    }

    pub fn new(chapters: Vector<ArcStr>, path: String,  mut doc: EpubDoc<std::io::BufReader<File>>) -> Self {
        let epub_settings = EpubSettings::default();
        let mut renderable_chapters: Vector<Vector<Renderable>> = Vector::new();
        let toc = doc
            .toc
            .iter()
            .map(|toc| {
                println!("toc: {:?}, {:?}, {:?}", toc.label, toc.content, toc.play_order);
                let key = toc.label.clone();
                let value = PagePosition::new(toc.play_order-1, 0);
                IndexedText::new(ArcStr::from(key), Arc::new(value))
            })
            .collect();

 
        renderable_chapters.push_back(generate_renderable_tree(&doc.get_current_str().unwrap(), epub_settings.font_size));

        
        while doc.go_next().is_ok() {
            let renderable = generate_renderable_tree(&doc.get_current_str().unwrap(), epub_settings.font_size);
            renderable_chapters.push_back(renderable);
        }




        let epub_metrics = EpubMetrics::new(&chapters);

        EpubData {
            visualized_chapter: chapters[0].clone().to_string(),
            chapters,

            epub_metrics,
            edit_mode: false,
            sidebar_data: SidebarData::new(toc),
            book_path : path,
            page_position: PagePosition::new(0, 0),
            renderable_chapters,
            epub_settings,
            ocr_data: OcrData::default(),
            doc: Arc::new(Mutex::new(doc)),
        }
    }

    pub fn save_new_epub(&mut self, file_path: &str) {
        // TODO: Are you fucking serious?
        let page_to_modify = self.doc.lock().unwrap().get_current_path().unwrap();

        let file = File::create(file_path).unwrap();

        let res =
            self.doc
                .lock()
                .unwrap()
                .modify_file(&page_to_modify, &file, &self.visualized_chapter);

        match res {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }

    }

    pub fn get_current_chap(&self) -> &Vector<Renderable> {
        
        &self.renderable_chapters[self.page_position.chapter()]
    }

    pub fn get_chap_len(&self, chap: usize) -> usize {
        self.renderable_chapters[chap].len()
    }

    pub fn has_next_chapter(&self) -> bool {
        return self.page_position.chapter() < self.chapters.len() - 1;
    }

    pub fn has_prev_chapter(&self) -> bool {
        return self.page_position.chapter() > 0;
    }

    pub fn get_only_strings(&self) -> Vector<Vector<String>> {
        self.renderable_chapters
            .iter()
            .map(|r| {
                r.iter()
                    .filter_map(|r| match r {
                        Renderable::Text(r) => Some(String::from(r.as_str().clone())),
                        _ => None,
                    })
                    .collect::<Vector<String>>()
            })
            .collect::<Vector<Vector<String>>>()
    }
    // Search the match in all text and
    // return a tuple with a string containing 5 words near match result referring to the match

    pub fn search_string_in_book(&mut self) {
        const MAX_SEARCH_RESULTS: usize = 100;
        const BEFORE_MATCH: usize = 13;
        let mut results = Vector::new();

        if !self.sidebar_data.search_input.is_empty() {
            let search_lenght = self.sidebar_data.search_input.len();

            // extract only text from self.renderable_chapters
            // and search for the match
            
            'outer: for (i, chapter) in self.get_only_strings().iter().enumerate() {
                for (j, richtext) in chapter.iter().enumerate() {
                    let matches: Vec<usize> = richtext
                        //.as_str()
                        .match_indices(&self.sidebar_data.search_input)
                        .map(|(i, _)| i)
                        .collect();
                    for occ_match in matches {
                        let range_position =
                            PagePosition::with_range(i, j, occ_match..occ_match + search_lenght);

                        //let page_position = PagePosition::new(i, start, end);
                        let start = if occ_match > BEFORE_MATCH {
                            occ_match - BEFORE_MATCH
                        } else {
                            0
                        };
                        let end =
                            if occ_match + search_lenght + BEFORE_MATCH < richtext.as_str().len() {
                                occ_match + search_lenght + BEFORE_MATCH
                            } else {
                                richtext.as_str().len()
                            };
                        //

                        // find end of word
                        let text = ArcStr::from(utf8_slice::slice(&richtext.as_str(), start, end));

                        //let text = ArcStr::from(richtext.as_str()[start..end].to_string());
                        //let text = ArcStr::from(richtext.as_str().chars().skip(occ_match as usize).take((occ_match) as usize).collect::<String>());
                        let value = Arc::new(range_position);
                        let search_result = IndexedText::new(ArcStr::from(text.to_string()), value);
                        results.push_back(search_result);
                        if results.len() > MAX_SEARCH_RESULTS {
                            break 'outer;
                        }
                    }
                }
            }
        }

        println!("Search results: {:?}", results.len());
        self.sidebar_data.search_results = results
    }

    pub fn change_position(&mut self, page_position: PagePosition) {

        // TODO: Remove this
       // self.epub_metrics.current_chapter = page_position.chapter;
        self.visualized_chapter = self.chapters[page_position.chapter].clone().to_string();
        self.page_position = page_position;
    }
}


#[derive(Clone, Debug, Data, Serialize, Deserialize, PartialEq)]
pub struct PagePosition {
    chapter: usize,
    richtext_number: usize,
    #[serde(skip)]
    range: Option<std::ops::Range<usize>>,
    #[serde(skip)]
    dirty: bool,
}

impl ToString for PagePosition {
    fn to_string(&self) -> String {
        format!(
            "Chapter: {} - Pos: {}",
            self.chapter, self.richtext_number
        )
    }
}
impl PagePosition {
    pub fn new(chapter: usize, richtext_number: usize) -> Self {
        PagePosition {
            chapter,
            richtext_number,
            range: None,
            dirty: false,
        }
    }

    pub fn with_range(
        chapter: usize,
        richtext_number: usize,
        range: std::ops::Range<usize>,
    ) -> Self {
        PagePosition {
            chapter,
            richtext_number,
            range: Some(range),
            dirty: false,
        }
    }

    pub fn chapter(&self) -> usize {
        self.chapter
    }

    pub fn richtext_number(&self) -> usize {
        self.richtext_number
    }

    pub fn range(&self) -> &Option<std::ops::Range<usize>> {
        &self.range
    }

    pub fn set_chapter(&mut self, chapter: usize) {
        self.chapter = chapter;
    }
    pub fn set_richtext_number(&mut self, richtext_number: usize) {
        self.richtext_number = richtext_number;
        self.invert_dirty()
    }
    pub fn invert_dirty(&mut self) {
        self.dirty = !self.dirty;
    }
}
