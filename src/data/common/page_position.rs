use druid::Data;
use serde::{Serialize, Deserialize};

/**
 * This struct is used to store the a position in the book.
 * It can also represent a range of text 
 * 
 */
#[derive(Clone, Debug, Data, Serialize, Deserialize)]
pub struct PagePosition {
    chapter: usize,
    richtext_number: usize,
    #[serde(skip)]
    range: Option<std::ops::Range<usize>>,
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
    pub const ZERO: PagePosition = PagePosition::new(0, 0);

    pub const fn new(chapter: usize, richtext_number: usize) -> Self {
        PagePosition {
            chapter,
            richtext_number,
            range: None,
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
    }
}



impl Default for PagePosition {
    fn default() -> Self {
        PagePosition::new(usize::MAX, usize::MAX)
    }
}


impl PartialEq for PagePosition {
    fn eq(&self, other: &Self) -> bool {
        self.chapter == other.chapter && self.richtext_number == other.richtext_number
    }
}

impl PartialOrd for PagePosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.chapter == other.chapter {
            self.richtext_number.partial_cmp(&other.richtext_number)
        } else {
            self.chapter.partial_cmp(&other.chapter)
        }
    }
}