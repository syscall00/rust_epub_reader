use std::sync::Arc;

use druid::{Lens, Data, ArcStr};
use super::page_position::PagePosition;

/**
 * This struct is used to index chunks of text of a book.
 * Examples of this are the table of contents, search results and possibly links.
 * 
 * This struct enhances the PagePosition struct by adding the key indexed.
 */
#[derive(Clone, Lens, Data)]
pub struct IndexedText {
    #[lens(name = "key_lens")]
    key: ArcStr,
    
    #[lens(name = "value_lens")]
    value: Arc<PagePosition>,
}

impl IndexedText {  
    pub fn new(key: ArcStr, value: Arc<PagePosition>) -> Self {
        IndexedText { key, value }
    }
    
    pub fn key(&self) -> &ArcStr {
        &self.key
    }

    pub fn value(&self) -> &Arc<PagePosition> {
        &self.value
    }

}
impl Default for IndexedText {
    fn default() -> Self {
        IndexedText {
            key: ArcStr::from(""),
            value: Arc::new(PagePosition::default()),
        }
    }
}