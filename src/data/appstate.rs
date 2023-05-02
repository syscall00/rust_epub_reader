use druid::{AppDelegate, Command, Data, DelegateCtx, Env, Handled, Lens, Target};

use crate::{
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND},
    data::{epub::EpubData, home::Recent, HomePageData},
    PageType,
};
use epub::doc::EpubDoc;

/**
 * Struct used for maintaining all the data that is displayed in the app.
 * Contains the data for the home page and the reader page.
 * Also contains the current active page.
 */
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

            if let Some(path) = file_info.path().to_str() {
                let recent = Recent::new(path.to_owned());
                // if recent already exists, do not add it again
                if !(data
                    .home_page_data
                    .recents
                    .iter()
                    .any(|r| r.path == recent.path))
                {
                    match data.open_file(&recent) {
                        Ok(_) => {
                            data.home_page_data.add_to_recents(recent);
                            data.active_page = PageType::Reader;
                        }
                        Err(e) => {
                            println!("Error opening file: {:?}", e);
                        }
                    }
                }
            }

            return Handled::Yes;
        } else if let Some(command) = cmd.get(INTERNAL_COMMAND) {
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
                    recent.epub_settings = data.epub_data.epub_settings.clone();
                    data.home_page_data.update_recent(recent);

                    return Handled::Yes;
                }
                InternalUICommand::OpenRecent(recent) => {
                    match data.open_file(recent) {
                        Err(e) => println!("Error: {:?}", e),
                        _ => {}
                    }
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
            epub_data: EpubData::default(),
            active_page: PageType::Home,
        }
    }

    /**
     * Opens a file and sets the epub_data to the new file.
     *
     * @param file_info - The file to open
     *
     */
    pub fn open_file(&mut self, file_info: &Recent) -> Result<(), Error> {
        let doc = EpubDoc::new(&file_info.path);

        assert!(doc.is_ok());
        let doc = doc.map_err(|e| Error::EpubError(e.to_string()))?;

        self.epub_data = EpubData::new(doc);
        self.epub_data.epub_settings = file_info.epub_settings.to_owned();
        if let Some(page_index) = &file_info.reached_position {
            self.epub_data.change_position(page_index.clone());
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub enum Error {
    EpubError(String),
}