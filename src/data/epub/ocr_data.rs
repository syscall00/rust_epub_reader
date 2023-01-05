use druid::{Lens, Data};

use crate::data::PagePosition;



pub const EMPTY_STRING: &str = "Please, choose an image";

#[derive(Clone, Lens, Data)]
pub struct OcrData {
    pub image_to_pos: String,
    pub image_for_pos_1: String,
    pub image_for_pos_2 : String,

    pub ocr_result: PagePosition,
    pub reverse_ocr_result: PagePosition,

    pub mode: OcrMode,

    pub processing: bool,

}

#[derive(Clone, Data, PartialEq)]
pub enum OcrMode {
    FindByPhoto,
    FindByVirtual,
}

impl Default for OcrData {
    fn default() -> Self {
        OcrData {
            image_to_pos: EMPTY_STRING.to_owned(),
            image_for_pos_1: EMPTY_STRING.to_owned(),
            image_for_pos_2: EMPTY_STRING.to_owned(),
            ocr_result: PagePosition::new(0,0),
            reverse_ocr_result: PagePosition::new(0,0),
            mode: OcrMode::FindByPhoto,
            processing: false,
        }
    }
}
