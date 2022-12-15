use druid::{Data, Lens, im::Vector};

use crate::appstate::*;




#[derive(Clone, Data, Lens)]
pub struct HomePageData {
    // Use a string for save paths in order to make
    // data more easy
    pub recents: Vector<Recent>,
}

impl HomePageData {
    pub fn new() -> Self {
        let recents = HomePageData::load_from_state_file();
        HomePageData { recents }
    }

    fn load_from_state_file() -> Vector<Recent> {
        let recents_fname = ".recents";
        let md = std::fs::metadata(recents_fname);
        // file does not exists!!!
        let recents_string = if md.is_err() {
            std::fs::File::create(recents_fname).unwrap();
            return Vector::default();

        } else {
          std::fs::read_to_string(recents_fname).unwrap()

        };
        let mut recent  = Recent::new("".to_string());
        recent.reached_position = Some(PagePosition::new(0, 10 ));
        let recents : Vec<Recent> = serde_json::from_str(&recents_string).unwrap();
        recents.into()
    }

    pub fn with_recents(mut self, recents: Vector<Recent>) -> Self {
        self.recents = recents;
        self
    }

    pub fn add_to_recents(&mut self, r: Recent) {
        self.recents.push_back(r.to_owned());
    }
}
