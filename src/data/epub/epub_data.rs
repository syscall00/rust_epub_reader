use std::{sync::{Arc, Mutex}, fs::File};

use druid::{Lens, Data, im::Vector, ArcStr, piet::TextStorage};
use epub::doc::EpubDoc;

use crate::{dom::{Renderable, generate_renderable_tree}, data::{PagePosition, IndexedText}};

use super::{ocr_data::OcrData, edit_data::EditData, settings::EpubSettings, sidebar::SidebarData};


/**
 * EpubData is the main struct that contains all the data of the book.
 * Based on the user's actions, a subset of this data is passed to the widgets.
 */
#[derive(Clone, Lens, Data)]
pub struct EpubData {

    pub renderable_chapters: Vector<Vector<Renderable>>,
    pub book_path: String,
    
    pub page_position: PagePosition,
    pub sidebar_data: SidebarData,

    pub ocr_data: OcrData,
    pub edit_data: EditData,

    pub epub_settings: EpubSettings,

    pub doc: Arc<std::sync::Mutex<EpubDoc<std::io::BufReader<File>>>>,
}



impl EpubData {
    pub fn empty_epub_data() -> Self {
        EpubData {
            renderable_chapters: Vector::new(),
            sidebar_data: SidebarData::new(Vector::new()),
            epub_settings: EpubSettings::default(),

            book_path: String::new(),
            page_position: PagePosition::default(),
            ocr_data: OcrData::default(),
            edit_data: EditData::default(),
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

        let mut edit_data = EditData::default();
        edit_data.set_edited_chapter(chapters[0].clone().to_string());

        EpubData {

            sidebar_data: SidebarData::new(toc),
            book_path : path,
            page_position: PagePosition::default(),
            renderable_chapters,
            epub_settings,
            ocr_data: OcrData::default(),
            edit_data,


            doc: Arc::new(Mutex::new(doc)),
        }
    }

    pub fn save_new_epub(&mut self, file_path: &str) {
        let page_to_modify = self.doc.lock().unwrap().get_current_path().unwrap();

        let file = File::create(file_path).unwrap();

        let res =
            self.doc
                .lock()
                .unwrap()
                .modify_file(&page_to_modify, &file, &self.edit_data.edited_chapter());

        self.renderable_chapters[self.page_position.chapter()] = generate_renderable_tree(&self.edit_data.edited_chapter(), self.epub_settings.font_size);
        match res {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }

    }

    pub fn get_current_chap(&self) -> Vector<Renderable> {
        
        //&self.renderable_chapters[self.page_position.chapter()]
        generate_renderable_tree(self.edit_data.edited_chapter(), self.epub_settings.font_size)
    }

    pub fn get_chap_len(&self, chap: usize) -> usize {
        self.renderable_chapters[chap].len()
    }

    pub fn has_next_chapter(&self) -> bool {
        return self.page_position.chapter() < self.doc.lock().unwrap().get_num_pages() - 1;
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
        self.page_position = page_position;
        let mut doc = self.doc.lock().unwrap();
        doc.set_current_page(self.page_position.chapter()).unwrap();
        self.edit_data.set_edited_chapter(doc.get_current_str().unwrap());


    }
}

