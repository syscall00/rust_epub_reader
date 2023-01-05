use druid::{Lens, Data};
use serde::{Serialize, Deserialize};

use crate::core::constants;


/**
 * EpubSettings contains all the settings that can be changed by the user.
 * The settings are saved in a file and loaded at startup.
 */

#[derive(druid::Data, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum VisualizationMode {
    SinglePage = 0,
    TwoPage = 1,
}


#[derive(Lens, Clone, Data, Serialize, Deserialize, Debug)]
pub struct EpubSettings {
    
    pub font_size: f64,
    pub margin: f64,
    pub paragraph_spacing: f64,

    pub visualization_mode: VisualizationMode,
}
impl EpubSettings {
    pub fn new() -> Self {
        EpubSettings::default()
    }

}

impl Default for EpubSettings {
    fn default() -> Self {
        EpubSettings {
            font_size: constants::epub_settings::DEFAULT_FONT_SIZE,
            margin: constants::epub_settings::DEFAULT_MARGIN,
            paragraph_spacing: constants::epub_settings::DEFAULT_PARAGRAPH_SPACING,

            visualization_mode: VisualizationMode::SinglePage,
        }
    }
}