use druid::{Lens, Data, im::Vector};

use crate::data::IndexedText;



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
