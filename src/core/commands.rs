use druid::Selector;

use crate::{sidebar::InternalUICommand, appstate::PagePosition};


pub const INTERNAL_COMMAND: Selector<InternalUICommand> =
    Selector::new("epub_reader.ui_command");



pub const GO_TO_POS: Selector<PagePosition> = Selector::new("go_to_pos");


pub const CHANGE_PAGE: Selector<bool> = Selector::new("change_page");


pub const SEARCH: Selector<String> = Selector::new("change_pageeee");