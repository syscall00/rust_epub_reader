use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};

use druid::{im::Vector, piet::TextStorage, ArcStr, Data, Lens};
use epub::doc::{EpubDoc, NavPoint};

use crate::{
    data::{IndexedText, PagePosition},
    dom::{generate_renderable_tree, Renderable},
};

use super::{edit_data::EditData, ocr_data::OcrData, settings::EpubSettings, sidebar::SidebarData};

/**
 * EpubData is the main struct that contains all the data of the book.
 * Based on the user's actions, a subset of this data is passed to the widgets.
 */
#[derive(Clone, Lens, Data, Default)]
pub struct EpubData {
    pub page_position: PagePosition,

    sidebar_data: SidebarData,
    pub epub_settings: EpubSettings,

    ocr_data: OcrData,
    pub edit_data: EditData,

    #[data(ignore)]
    doc: Option<Arc<Mutex<EpubDoc<BufReader<File>>>>>,

    #[data(ignore)]
    cached_chapters: Option<Vec<Vec<String>>>,
}

impl EpubData {
    fn toc_recursive_parser(
        toc: &Vec<NavPoint>,
        chapters: &mut Vector<IndexedText>,
        epub_doc: &mut EpubDoc<BufReader<File>>,
    ) {
        toc.iter().for_each(|toc_elem| {
            let toc_content = toc_elem
                .content
                .clone()
                .into_os_string()
                .into_string()
                .unwrap();

            let pos = toc_content.find('#').unwrap_or(toc_content.len());
            let (toc_content, _) = toc_content.split_at(pos);
            epub_doc.set_current_page(0).unwrap();

            while epub_doc.get_current_page() < epub_doc.get_num_pages()
                && epub_doc
                    .get_current_path()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    != toc_content
            {
                epub_doc.go_next().unwrap();
            }

            chapters.push_back(IndexedText::new(
                ArcStr::from(toc_elem.label.clone()),
                Arc::new(PagePosition::new(epub_doc.get_current_page(), 0)),
            ));
            Self::toc_recursive_parser(&toc_elem.children, chapters, epub_doc);
        });
    }

    pub fn new(chapters: Vector<ArcStr>, doc: EpubDoc<std::io::BufReader<File>>) -> Self {
        let epub = doc.get_epub_path();
        let mut other_doc = EpubDoc::new(epub).unwrap();

        let mut toc = Vector::new();

        Self::toc_recursive_parser(&doc.toc, &mut toc, &mut other_doc);

        let mut edit_data = EditData::default();
        edit_data.set_edited_chapter(chapters[0].clone().to_string());

        EpubData {
            sidebar_data: SidebarData::new(toc),
            page_position: PagePosition::default(),
            epub_settings: EpubSettings::default(),
            ocr_data: OcrData::default(),
            edit_data,

            doc: Some(Arc::new(Mutex::new(doc))),
            cached_chapters: None,
        }
    }

    pub fn save_new_epub(&mut self, file_path: &str) {
        if self.doc.is_none() {
            return;
        }
        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();
        let page_to_modify = doc.get_current_path().unwrap();

        let file = File::create(file_path).unwrap();

        let res = doc.modify_file(&page_to_modify, &file, &self.edit_data.edited_chapter());

        self.cached_chapters = None;

        match res {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }

    /**
     * Get the current chapter as a vector of Renderable
     *
     * @return the current epub path
     */
    pub fn get_epub_path(&self) -> String {
        if self.doc.is_none() {
            return String::new();
        }
        let doc = self.doc.as_ref().unwrap().lock().unwrap();
        doc.get_epub_path()
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    }

    /**
     * Get the current chapter as a vector of Renderable
     *
     * @return the current chapter as a vector of Renderable
     */
    pub fn get_current_chap(&self) -> Vector<Renderable> {
        if self.doc.is_none() {
            return Vector::new();
        }

        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();

        generate_renderable_tree(
            doc.get_current_str().as_ref().unwrap(),
            self.epub_settings.font_size,
        )
    }

    /**
     * Get rendered text of the entire book
     * Useful for searching
     * It uses cached_chapters to avoid re-rendering the entire book.
     *
     * @return rendered text of the entire book
     */
    pub fn get_only_strings(&mut self) -> Vec<Vec<String>> {
        if self.doc.is_none() {
            return Vec::new();
        }
        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();
        if self.cached_chapters.is_none() {
            let mut cached_chapters = Vec::new();
            let spine = doc.spine.clone();
            // calculate self.cached_chapters
            spine.iter().for_each(|spine| {
                let res = String::from_utf8(doc.get_resource(spine).unwrap()).unwrap();
                let renderable = generate_renderable_tree(&res, self.epub_settings.font_size)
                    .iter()
                    .filter_map(|r| match r {
                        Renderable::Text(r) => Some(String::from(r.as_str().clone())),
                        _ => None,
                    })
                    .collect::<Vec<String>>();

                cached_chapters.push(renderable);
            });
            self.cached_chapters = Some(cached_chapters);
        }

        return self.cached_chapters.clone().unwrap().clone();
    }

    /**
     * Search the current chapter for the search input
     * and set the results in the sidebar_data
     * // TODO: Comment more
     */
    pub fn search_string_in_book(&mut self) {
        const MAX_SEARCH_RESULTS: usize = 100;
        const BEFORE_MATCH: usize = 13;
        let mut results = Vector::new();

        if !self.sidebar_data.search_input.is_empty() {
            let search_lenght = self.sidebar_data.search_input.len();

            'outer: for (i, chapter) in self.get_only_strings().iter().enumerate() {
                for (j, richtext) in chapter.iter().enumerate() {
                    let matches: Vec<usize> = richtext
                        .match_indices(&self.sidebar_data.search_input)
                        .map(|(i, _)| i)
                        .collect();
                    for occ_match in matches {
                        let range_position =
                            PagePosition::with_range(i, j, occ_match..occ_match + search_lenght);

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
        self.sidebar_data.search_results = results
    }

    pub fn change_position(&mut self, page_position: PagePosition) {
        if self.doc.is_none() {
            return;
        }
        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();

        if doc.set_current_page(page_position.chapter()).is_err() {
            return;
        }

        self.page_position = page_position;
        self.edit_data
            .set_edited_chapter(doc.get_current_str().unwrap());
    }

    pub fn set_position_in_page(&mut self, position_in_page: usize) {
        self.page_position.set_richtext_number(position_in_page);
    }

    pub fn next_chapter(&mut self) -> bool {
        if self.doc.is_none() {
            return false;
        }
        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();

        let next = doc.go_next();
        if next.is_err() {
            return false;
        }
        self.edit_data
            .set_edited_chapter(doc.get_current_str().unwrap());

        self.page_position.set_chapter(doc.get_current_page());
        return true;
    }

    pub fn prev_chapter(&mut self) -> bool {
        if self.doc.is_none() {
            return false;
        }
        let mut doc = self.doc.as_ref().unwrap().lock().unwrap();

        let prev = doc.go_prev();
        if prev.is_err() {
            return false;
        }
        self.edit_data
            .set_edited_chapter(doc.get_current_str().unwrap());

        self.page_position.set_chapter(doc.get_current_page());

        return true;
    }
}
