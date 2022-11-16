

pub(crate) mod epub_settings {

    pub(crate) const DEFAULT_FONT_SIZE : f64 = 16.0;
    pub(crate) const DEFAULT_MARGIN : f64 = 50.0;
    pub(crate) const DEFAULT_PARAGRAPH_SPACING : f64 = 10.0;

    pub(crate) const MIN_FONT_SIZE : f64 = 14.0;
    pub(crate) const MAX_FONT_SIZE : f64 = 28.0;

    pub(crate) const MIN_MARGIN : f64 = 30.0;
    pub(crate) const MAX_MARGIN : f64 = 80.0;

    pub(crate) const MIN_PARAGRAPH_SPACING : f64 = 0.0;
    pub(crate) const MAX_PARAGRAPH_SPACING : f64 = 50.0;


}


pub(crate) mod commands {
    use druid::Selector;

    use crate::sidebar::PanelButton;
    
    pub const INTERNAL_COMMAND: Selector<InternalUICommand> =
    Selector::new("epub_reader.ui_command");

    #[derive(Debug)]
    pub enum InternalUICommand {
        SwitchTab(PanelButton),
        GoToMenu,
        OpenEditDialog,
        OpenOCRDialog,
        SaveModification(String),

        RemoveBook
    }
}