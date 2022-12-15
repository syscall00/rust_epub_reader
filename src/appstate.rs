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
use crate::core::style::LINK_COLOR;
use crate::data::epub::settings::EpubSettings;
use crate::data::home::HomePageData;
use crate::PageType;
use epub::doc::EpubDoc;

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
            // generate new recent from file_info

            let recent = Recent::new(file_info.path().to_str().unwrap().to_string());
            // if recent already exists, do not add it agai
            if data
                .home_page_data
                .recents
                .iter()
                .any(|r| r.path == recent.path)
            {
                return Handled::Yes;
            }
            data.open_file(&recent);
            data.home_page_data.recents.push_back(recent);
            data.active_page = PageType::Reader;
        }
        if let Some(file_info) = cmd.get(crate::core::commands::OPEN_RECENT) {
            data.open_file(file_info);
            return Handled::Yes;
        } else if let Some(command) = cmd.get(INTERNAL_COMMAND) {
            let ret = match command {
                crate::core::constants::commands::InternalUICommand::RemoveBook(book_path) => {
                    // remove book from recent
                    data.home_page_data.recents.retain(|r| r.path != *book_path);

                    // write on disk
                    //let file = File::create("recent.json").unwrap();
                    //serde_json::to_writer(file, &data.home_page_data.recents).unwrap();

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

        self.epub_data = EpubData::new(pages, doc);
        self.epub_data.epub_settings = file_info.epub_settings.to_owned();
        if let Some(page_index) = &file_info.reached_position {
            self.epub_data.page_position = page_index.to_owned();
        }
        //self.epub_data.reached_position = file_info.reached_position;
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
    pub book_highlights: Vector<IndexedText>,

    pub search_input: String,
}

impl SidebarData {
    pub fn new(table_of_contents: Vector<IndexedText>) -> Self {
        SidebarData {
            table_of_contents,
            search_results: Vector::new(),
            book_highlights: Vector::new(),

            search_input: String::default(),
        }
    }
}

#[derive(Clone, Lens, Data)]
pub struct EpubData {
    pub epub_metrics: EpubMetrics,

    // Plain text of all book
    pub chapters: Vector<ArcStr>,
    pub rich_chapters: Vector<Vector<RichText>>,

    pub page_position: crate::appstate::PagePosition,
    pub visualized_chapter: String,
    pub sidebar_data: SidebarData,
    pub edit_mode: bool,

    pub chapter_title: String,

    pub epub_settings: EpubSettings,

    pub ocr_data: OcrData,

    pub doc: Arc<std::sync::Mutex<EpubDoc<std::io::BufReader<File>>>>,
}

#[derive(Clone, Lens, Data)]
pub struct OcrData {
    pub file_path: String,
    pub image_to_position: Option<IndexedText>,
    pub position_to_image: usize,
}

#[derive(Clone, Data, PartialEq)]
pub enum OCRMode {
    FindByPhoto,
    FindByVirual,
}

impl Default for OcrData {
    fn default() -> Self {
        OcrData {
            file_path: String::new(),
            image_to_position: None,
            position_to_image: 0,
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

impl EpubData {
    pub fn empty_epub_data() -> Self {
        EpubData {
            epub_metrics: EpubMetrics::new(&Vector::new()),
            chapters: Vector::new(),
            rich_chapters: Vector::new(),
            visualized_chapter: String::new(),
            sidebar_data: SidebarData::new(Vector::new()),
            edit_mode: false,
            epub_settings: EpubSettings::default(),
            chapter_title: String::new(),

            page_position: PagePosition::new(0, 0),
            ocr_data: OcrData::default(),
            doc: Arc::new(Mutex::new(
                EpubDoc::new("/home/syscall/Downloads/1.epub").unwrap(),
            )),
        }
    }

    pub fn new(chapters: Vector<ArcStr>, doc: EpubDoc<std::io::BufReader<File>>) -> Self {
        let epub_settings = EpubSettings::default();
        let mut rich_chapters: Vector<Vector<RichText>> = Vector::new();

        let toc = doc
            .toc
            .iter()
            .map(|toc| {
                let key = toc.label.clone();
                let value = PagePosition::new(toc.play_order, 0);
                IndexedText::new(ArcStr::from(key), Arc::new(value))
            })
            .collect();

        for i in 0..chapters.len() {
            let rich = rebuild_rendered_text(&chapters[i], epub_settings.font_size);
            rich_chapters.push_back(rich);
        }

        let epub_metrics = EpubMetrics::new(&chapters);

        EpubData {
            visualized_chapter: chapters[0].clone().to_string(),
            chapters,

            epub_metrics,
            edit_mode: false,
            sidebar_data: SidebarData::new(toc),

            chapter_title: String::from("Chapter 1"),
            page_position: PagePosition::new(0, 0),
            rich_chapters,
            epub_settings,
            ocr_data: OcrData::default(),
            doc: Arc::new(Mutex::new(doc)),
        }
    }

    pub fn update_position(&mut self) {

        // use serde to update the position on disk
        //serde::Serialize::serialize(&self.page_position, &mut serde_json::Serializer::new(std::io::stdout())).unwrap();
    }

    pub fn save_new_epub(&mut self, file_path: &str) {
        let page_to_modify = self.doc.lock().unwrap().get_current_path().unwrap();

        let file = File::create(file_path).unwrap(); //"/home/syscall/Desktop/rust_epub_reader/examples/1_MOD.epub"

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

    pub fn get_current_chap(&self) -> &Vector<RichText> {
        &self.rich_chapters[self.epub_metrics.current_chapter]
    }

    pub fn has_next_chapter(&self) -> bool {
        return self.epub_metrics.current_chapter < self.chapters.len() - 1;
    }

    pub fn has_prev_chapter(&self) -> bool {
        return self.epub_metrics.current_chapter > 0;
    }

    pub fn search_with_ocr_input(&mut self, image_path: &str) -> PagePosition {
        let mut lt = leptess::LepTess::new(None, "eng").unwrap();
        lt.set_image(image_path).unwrap();

        let recognized_text = lt
            .get_utf8_text()
            .unwrap()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();

        /*
        Search for recognized text in this way:
            1. Clean the recognized text from special and unknown characters
            2. Divide the chapters in chunks big as the recognized text
            3. Generate a vector of (word, count) for the recognized text
            4. Spawn a thread from a thread pool for each chunk
            5. For each chunk, generate a vector of (word, count) and compare it with the recognized text
            6. If the two vectors are similar, return the starting index of the chunk with a percentage of similarity
        */
        // test tantity search on a lorem ipsum text
        let mut schema_builder = tantivy::schema::Schema::builder();
        let title =
            schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
        let chapter = schema_builder
            .add_u64_field("chapter", tantivy::schema::FAST | tantivy::schema::STORED);
        let position = schema_builder
            .add_u64_field("start_pos", tantivy::schema::FAST | tantivy::schema::STORED);
        let schema = schema_builder.build();

        let index = tantivy::Index::create_in_ram(schema);
        let mut index_writer = index.writer(50_000_000).unwrap();

        for i in 0..self.rich_chapters.len() {
            let mut texta = String::new();
            // For each richtext in the chapter, when size is greater than 1000, create a new document
            for (j, richtext) in self.rich_chapters[i].iter().enumerate() {
                // texta should contain recognized_text.len() character, starting from j richtext and ending at most at j + 1 richtext
                let rich_text = richtext.as_str();
                if rich_text.len() >= recognized_text.len() {
                    texta = utf8_slice::slice(rich_text, 0, recognized_text.len()).to_string();
                    let mut doc = tantivy::Document::default();
                    doc.add_text(title, &texta);
                    doc.add_u64(chapter, i as u64);
                    doc.add_u64(position, j as u64);
                    index_writer.add_document(doc).unwrap();
                    texta.clear();
                } else if rich_text.len() + texta.len() >= recognized_text.len() {
                    texta = utf8_slice::slice(rich_text, 0, recognized_text.len()).to_string();
                    let mut doc = tantivy::Document::default();
                    doc.add_text(title, &texta);
                    doc.add_u64(chapter, i as u64);
                    doc.add_u64(position, j as u64);
                    index_writer.add_document(doc).unwrap();
                    texta.clear();
                } else {
                    texta.push_str(rich_text);
                }

                //if texta.len() >= recognized_text.len() {
                //    let mut doc = tantivy::Document::default();
                //    doc.add_text(title, &texta);
                //    doc.add_u64(chapter, i as u64);
                //    doc.add_u64(position, j as u64);
                //    index_writer.add_document(doc);
                //    texta.clear();
                //}
                //else {
                //    texta.push_str(&richtext.as_str());
                //}
            }
            if texta.len() > 0 {
                let mut doc = tantivy::Document::default();
                doc.add_text(title, &texta);
                doc.add_u64(chapter, i as u64);
                doc.add_u64(position, 0 as u64);
                index_writer.add_document(doc).unwrap();
                texta.clear();
            }
        }

        //let splitted = chapt2.as_bytes().chunks(recognized_text.len()).map(|c| std::str::from_utf8(c).unwrap());
        //println!("Splitted: {:?}", chapt2);
        //for split in splitted {
        //    index_writer.add_document(tantivy::doc!(
        //        title => split,
        //        chapter => 2 as u64,
        //        position => 0 as u64
        //    )).unwrap();
        //}
        //return;
        index_writer.commit().unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        let query_parser = tantivy::query::QueryParser::for_index(
            &index,
            vec![tantivy::schema::Field::from_field_id(0)],
        );

        let query = query_parser.parse_query(&recognized_text).unwrap();

        let top_docs = searcher
            .search(&query, &tantivy::collector::TopDocs::with_limit(3))
            .unwrap();
        println!("Found {} documents.", top_docs.len());
        //for (score, doc_address) in top_docs {
        //    let retrieved_doc = searcher.doc(doc_address).unwrap();
        //    let chapter = retrieved_doc.get_first(chapter).unwrap();
        //    let position = retrieved_doc.get_first(position).unwrap();
        //    println!("Document: {:?} in chapter {:?} position {:?} with score {}", retrieved_doc, chapter, position, score);
        //    println!("\n\n\n");
        //
        //
        //}
        //// get the first document
        // get the chapter and position

        if top_docs.len() > 0 {
            let (score, doc_address) = top_docs[0];
            let retrieved_doc = searcher.doc(doc_address).unwrap();
            let chapter = retrieved_doc.get_first(chapter).unwrap();
            let position = retrieved_doc.get_first(position).unwrap();
            println!(
                "Document: {:?} in chapter {:?} position {:?} with score {}",
                retrieved_doc, chapter, position, score
            );
            println!("\n\n\n");
            return PagePosition::new(
                chapter.as_u64().unwrap() as usize,
                position.as_u64().unwrap() as usize,
            );
        }

        PagePosition::new(0, 0)
    }

    // Search the match in all text and
    // return a tuple with a string containing 5 words near match result referring to the match

    pub fn search_string_in_book(&mut self) {
        const MAX_SEARCH_RESULTS: usize = 100;
        const BEFORE_MATCH: usize = 13;
        let mut results = Vector::new();

        if !self.sidebar_data.search_input.is_empty() {
            let search_lenght = self.sidebar_data.search_input.len();

            'outer: for (i, chapter) in self.rich_chapters.iter().enumerate() {
                for (j, richtext) in chapter.iter().enumerate() {
                    let matches: Vec<usize> = richtext
                        .as_str()
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

    pub fn next_chapter(&mut self) {
        if self.epub_metrics.current_chapter < self.epub_metrics.num_chapters - 1 {
            // TODO: Remove this
            self.epub_metrics.current_chapter += 1;
            self.visualized_chapter = self.chapters[self.epub_metrics.current_chapter]
                .clone()
                .to_string();
        }
    }

    pub fn previous_chapter(&mut self) {
        if self.epub_metrics.current_chapter > 0 {
            // TODO: Remove this
            self.epub_metrics.current_chapter -= 1;
            self.visualized_chapter = self.chapters[self.epub_metrics.current_chapter]
                .clone()
                .to_string();
        }
    }

    pub fn change_position(&mut self, page_position: PagePosition) {
        self.chapter_title = utf8_slice::slice(
            &self.rich_chapters[page_position.chapter][0].as_str(),
            0,
            20,
        )
        .to_string();

        // TODO: Remove this
        self.epub_metrics.current_chapter = page_position.chapter;
        self.visualized_chapter = self.chapters[page_position.chapter].clone().to_string();
        self.page_position = page_position;
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
            HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Image(_) | HtmlTag::Link(_)
        )
    }

    pub fn should_tag_be_written(&self) -> bool {
        matches!(self, HtmlTag::Title)
    }

    pub fn add_attribute_for_token(&self, mut attrs: druid::text::AttributesAdder, font_size: f64) {
        match self {
            HtmlTag::Header(lvl) => {
                let font_size = font_size
                    * match lvl {
                        1 => 2.,
                        2 => 1.5,
                        3 => 1.17,
                        4 => 1.,
                        5 => 0.8375,
                        6 => 0.67,
                        _ => 1.,
                    };
                attrs.size(font_size).weight(druid::FontWeight::BOLD);
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
                attrs.underline(true).text_color(LINK_COLOR);
            }
            HtmlTag::Image(_img) => {}
            _ => {
                return;
            }
        }
    }
}

#[derive(Clone, Debug, Data, Serialize, Deserialize)]
pub struct PagePosition {
    chapter: usize,
    richtext_number: usize,
    range: Option<std::ops::Range<usize>>,
    dirty: bool,
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

// instead of a simple richtexts, we have to use a vector of

pub fn rebuild_rendered_text(text: &str, font_size: f64) -> Vector<RichText> {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    let mut richtexts: Vector<RichText> = Vector::new();

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
            xmlparser::Token::ElementEnd { end, span: _ } => {
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

                        tk.add_attribute_for_token(
                            builder.add_attributes_for_range(pos..current_pos),
                            font_size,
                        );

                        if tk != HtmlTag::Unhandled && tk.add_newline_after_tag() {
                            //current_pos += 1;

                            //builder.push("\n");
                        }

                        if matches!(
                            tk,
                            HtmlTag::Paragraph
                                | HtmlTag::Header(_)
                                | HtmlTag::Image(_)
                                | HtmlTag::Link(_)
                        ) {
                            if current_pos == 0 {
                                continue;
                            }
                            let text = builder.build();
                            richtexts.push_back(text);

                            builder = druid::text::RichTextBuilder::new();
                            current_pos = 0;
                        }
                    }
                    xmlparser::ElementEnd::Empty => {
                        token_stack.pop().expect("No token on stack");
                    }
                }
            }

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();

                if inner_tag.should_tag_be_written() || text.trim().is_empty() {
                    continue;
                } else {
                    let t = text.as_str().replace("\n", "");
                    current_pos = current_pos + t.len();

                    builder.push(&t);
                }
            }
            xmlparser::Token::Attribute {
                prefix: _,
                local: _,
                value: _,
                span: _,
            } => {
                //println!("attr: {:?} = {:?}", loc, val);
                continue;
            }

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
    richtexts
}
