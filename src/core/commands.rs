use druid::Selector;

use crate::{sidebar::InternalUICommand, appstate::{PageIndex}, PageType};


pub const INTERNAL_COMMAND: Selector<InternalUICommand> =
    Selector::new("epub_reader.ui_command");



pub const GO_TO_POS: Selector<PageIndex> = Selector::new("go_to_pos");

pub const NAVIGATE_TO: Selector<PageType> = Selector::new("navigate_to");

pub const CHANGE_PAGE: Selector<bool> = Selector::new("change_page");


// Commands for EpubPage view
pub const REQUEST_EDIT: Selector<()> = Selector::new("request_edit");


pub const SAVE_EPUB: Selector<()> = Selector::new("save_epub");
#[derive(druid::Data, Clone, PartialEq, Debug)]
pub enum VisualizationMode {
    SinglePage = 0,
    TwoPage = 1,
}