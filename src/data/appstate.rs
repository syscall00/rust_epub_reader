use druid::im::Vector;
use druid::{
    AppDelegate, ArcStr, Command, Data, DelegateCtx, Env, Handled, Lens, Target,
};

use crate::core::constants::commands::{INTERNAL_COMMAND, InternalUICommand};
use crate::data::HomePageData;
use crate::PageType;
use crate::data::epub::epub_data::EpubData;
use crate::data::home::Recent;
use epub::doc::EpubDoc;


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub epub_data: EpubData,
    pub home_page_data: HomePageData,
    pub active_page: PageType,
}

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {

            let recent = Recent::new(file_info.path().to_str().unwrap().to_string());
            // if recent already exists, do not add it again
            if !(data
                .home_page_data
                .recents
                .iter()
                .any(|r| r.path == recent.path))
            {
                data.open_file(&recent);
                data.home_page_data.add_to_recents(recent);
                data.active_page = PageType::Reader;
            }
            return Handled::Yes;
        }
        else if let Some(command) = cmd.get(INTERNAL_COMMAND) {
            let ret = match command {
                InternalUICommand::RemoveBook(book_path) => {
                    // remove book from recent
                    data.home_page_data.remove_from_recents(book_path);
                    return Handled::Yes;
                }
                InternalUICommand::UpdateBookInfo(book_path) => {
                    let recent = data.home_page_data.get_recent(book_path);
                    let mut recent = if let Some(recent) = recent {
                        recent
                    } else {
                        Recent::new(book_path.to_string())
                    };

                    recent.reached_position = Some(data.epub_data.page_position.clone());
                    println!("Reached position: {:?}", recent.reached_position);
                    recent.epub_settings = data.epub_data.epub_settings.clone();
                    data.home_page_data.update_recent(recent);

                    return Handled::Yes;
                }
                InternalUICommand::OpenRecent(recent) => {
                    data.open_file(recent);
                    return Handled::Yes;
                    }
                _ => Handled::No,
            };
            return ret;
        } else {
            Handled::No
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            home_page_data: HomePageData::new(),
            epub_data: EpubData::empty_epub_data(),
            active_page: PageType::Home,
        }
    }

    fn load_file(file_path: &str) -> Vector<ArcStr> {
        let mut pages = Vector::new();
        let doc = EpubDoc::new(&file_path);

        assert!(doc.is_ok());
        let mut doc = doc.unwrap();
        let _m = 0;
        loop {
            pages.push_back(ArcStr::from(doc.get_current_str().unwrap().clone()));
            if !doc.go_next().is_ok() {
                break;
            };
        }
        pages
    }

    pub fn open_file(&mut self, file_info: &Recent) {
        let pages = AppState::load_file(&file_info.path);
        let doc = EpubDoc::new(&file_info.path);

        assert!(doc.is_ok());
        let doc = doc.unwrap();

        self.epub_data = EpubData::new(pages, file_info.path.to_owned(), doc);
        self.epub_data.epub_settings = file_info.epub_settings.to_owned();
        if let Some(page_index) = &file_info.reached_position {
            self.epub_data.page_position = page_index.to_owned();
            self.epub_data.change_position(page_index.clone());
        }
    }
}


