use std::sync::Arc;

use druid::{Lens, Data, ArcStr};

use super::page_position::PagePosition;


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