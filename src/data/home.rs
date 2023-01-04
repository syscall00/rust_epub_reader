use druid::{Data, Lens, im::Vector};

use crate::appstate::*;




#[derive(Clone, Data, Lens)]
pub struct HomePageData {
    // Use a string for save paths in order to make
    // data more easy
    pub recents: Vector<Recent>,
}

impl HomePageData {
    const RECENTS_PATH: &'static str = ".recents";
    pub fn new() -> Self {
        let recents = HomePageData::load_from_state_file();
        HomePageData { recents }
    }

    fn load_from_state_file() -> Vector<Recent> {
        let md = std::fs::metadata(HomePageData::RECENTS_PATH);
        // file does not exists!!!
        let recents_string = if md.is_err() {
            std::fs::File::create(HomePageData::RECENTS_PATH).unwrap();
            return Vector::default();

        } else {
          std::fs::read_to_string(HomePageData::RECENTS_PATH).unwrap()

        };
        let recents : Vec<Recent> = serde_json::from_str(&recents_string).unwrap();
        recents.into()
    }

    fn write_to_state_file(&self) {
        let t : Vec<Recent> = self.recents.clone().into_iter().collect();
        let recents_string = serde_json::to_string(&t).unwrap();
        std::fs::write(HomePageData::RECENTS_PATH, recents_string).unwrap();
    }

    pub fn with_recents(mut self, recents: Vector<Recent>) -> Self {
        self.recents = recents;
        self
    }

    pub fn add_to_recents(&mut self, r: Recent) {
        self.recents.push_back(r.to_owned());
        self.write_to_state_file();
    }
    
    pub fn remove_from_recents(&mut self, book_path: &String) {
        self.recents.retain(|x| &x.path != book_path);
        self.write_to_state_file();
    }

    pub fn update_recent(&mut self, r: Recent) {
        // substitute the old recent with the new one
        let position = self.recents.iter().position(|x| &x.path == &r.path).unwrap();
        self.recents[position] = r;

        self.write_to_state_file();
    }

    pub fn get_recent(&self, book_path: &String) -> Option<Recent> {
        self.recents.iter().find(|x| &x.path == book_path).map(|x| x.to_owned())
    }
    
}
