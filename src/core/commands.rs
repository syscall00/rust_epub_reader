use druid::Selector;

use crate::{appstate::{PageIndex, Recent}, PageType};



pub const OPEN_RECENT: Selector<Recent> = Selector::new("open-recent");
pub const GO_TO_POS: Selector<PageIndex> = Selector::new("go_to_pos");
pub const NAVIGATE_TO: Selector<PageType> = Selector::new("navigate_to");
pub const CHANGE_PAGE: Selector<bool> = Selector::new("change_page");




