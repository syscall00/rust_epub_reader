use druid::Data;
use serde::{Serialize, Deserialize};


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

