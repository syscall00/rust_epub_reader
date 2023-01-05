use druid::{Lens, Data};


#[derive(Clone, Lens, Data)]
pub struct EditData {
    pub edit_mode: bool,
    pub visualized_chapter: String,
}

impl EditData {
    pub fn is_editing(&self) -> bool {
        self.edit_mode
    }

    pub fn set_editing(&mut self, edit_mode: bool) {
        self.edit_mode = edit_mode;
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