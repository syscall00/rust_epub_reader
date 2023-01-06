use druid::{Data, Lens};

/**
 * Struct used during the editing of a book.
 * Contains a T/F flag to indicate if the book is being edited and
 * the text of the chapter that is being edited.
 */
#[derive(Clone, Lens, Data)]
pub struct EditData {
    edit_mode: bool,
    visualized_chapter: String,
}

impl EditData {
    pub fn is_editing(&self) -> bool {
        self.edit_mode
    }

    pub fn set_editing(&mut self, edit_mode: bool) {
        self.edit_mode = edit_mode;
    }

    pub fn edited_chapter(&self) -> &String {
        &self.visualized_chapter
    }

    pub fn set_edited_chapter(&mut self, edited_chapter: String) {
        self.visualized_chapter = edited_chapter;
    }
}

impl Default for EditData {
    fn default() -> Self {
        EditData {
            edit_mode: false,
            visualized_chapter: String::default(),
        }
    }
}
