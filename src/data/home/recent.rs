use druid::{Data, Lens, ImageBuf, ArcStr};
use serde::{Serialize, Deserialize};

use crate::data::{epub::settings::EpubSettings, PagePosition};


#[derive(Clone, Data, Lens, Debug)]
pub struct RecentData {
    pub image_data: Option<ImageBuf>,
    pub title: ArcStr,
    pub creator: ArcStr,
    pub publisher: ArcStr,
    pub position_in_book: usize,
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
pub struct Recent {
    pub path: String,
    pub reached_position: Option<PagePosition>,

    pub epub_settings: EpubSettings,

    // ignore this field for serialization
    #[serde(skip)]
    pub image_data: Option<ImageBuf>,

    #[serde(skip)]
    pub recent_data: Option<RecentData>,
}

impl Recent {
    pub fn new(path: String) -> Self {
        Recent {
            path,
            reached_position: None,
            epub_settings: EpubSettings::default(),
            image_data: None,
            recent_data: None,
        }
    }

    pub fn set_recent_data(&mut self, recent_data: RecentData) {
        self.recent_data = Some(recent_data);
    }
}
