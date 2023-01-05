pub(crate) mod epub_settings {

    pub(crate) const DEFAULT_FONT_SIZE: f64 = 16.0;
    pub(crate) const DEFAULT_MARGIN: f64 = 50.0;
    pub(crate) const DEFAULT_PARAGRAPH_SPACING: f64 = 10.0;

    pub(crate) const MIN_FONT_SIZE: f64 = 14.0;
    pub(crate) const MAX_FONT_SIZE: f64 = 28.0;

    pub(crate) const MIN_MARGIN: f64 = 30.0;
    pub(crate) const MAX_MARGIN: f64 = 80.0;

    pub(crate) const MIN_PARAGRAPH_SPACING: f64 = 0.0;
    pub(crate) const MAX_PARAGRAPH_SPACING: f64 = 50.0;
}

pub(crate) mod commands {
    use druid::{FileInfo, Selector};

    use crate::{widgets::{epub_page::sidebar::PanelButton, PromptOption}, data::{Recent, PagePosition}, PageType};

    pub const MODIFY_EPUB_PATH: Selector<FileInfo> = Selector::new("epub_reader.modify-epub");

    pub const OPEN_OCR_FILE: druid::Selector<druid::FileInfo> = druid::Selector::new("epub_reader.open-ocr-file");
    pub const OPEN_REVERSE_OCR_1: druid::Selector<druid::FileInfo> = druid::Selector::new("epub_reader.open-reverse-ocr-1");
    pub const OPEN_REVERSE_OCR_2: druid::Selector<druid::FileInfo> = druid::Selector::new("epub_reader.open-reverse-ocr-2");



    pub const INTERNAL_COMMAND: Selector<InternalUICommand> =
        Selector::new("epub_reader.ui_command");

    #[derive(Debug)]
    pub enum InternalUICommand {
        SwitchTab(PanelButton),
        GoToMenu,
        OpenOCRDialog,

        OpenRecent(Recent),
        RemoveBook(String),
        UpdateBookInfo(String), 

        OpenEditDialog,
        RequestSaveEdit,
        SaveEditAs,
        CloseEdit,
        SaveModification(String),
        PromptEditSave(PromptOption),
        

        RequestOCRSearch(String),
        RequestReverseOCR((String, String)),
        OCRSearchCompleted(PagePosition),
        ReverseOCRCompleted(PagePosition),

        EpubGoToPos(PagePosition),
        EpubNavigate(bool),

        UINavigate(PageType),
    }
}
