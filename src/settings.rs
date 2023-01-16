use std::{error::Error, fs, path};
use std::env::var;
use rss::Channel;

use crate::data::*;

#[derive(Debug, Copy, Clone)]
pub enum DB {
    SQLiteBundled,
    SQLiteSystem,
    Postgress,
    Maria,
    MongoDB,
}
pub struct Settings {
    pub feeds: Vec<Feed>,
    pub check_every: u64,
    pub check_deep: usize,
    pub save_root: String,
    pub raw_to_db: bool,
    pub db: DB,
}

static mut INDEX: i32 = 0;

impl Settings {
    pub fn new() -> Self {
        Settings {
            feeds: Vec::new(),
            check_every: 5,
            check_deep: 20,
            save_root: format!("{}/.adrastea", var("HOME").unwrap()),
            raw_to_db: false,
            db: DB::SQLiteBundled,
        }
    }
    pub fn add_feed(&mut self, url: String) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let name= rt.block_on(Settings::get_title(url.clone())).unwrap();

        let mut id = 0;
        unsafe {
            id = INDEX; //TODO : Opbtain Id from DB (and get rid of unsafe)
            INDEX += 1;
        }

        let path = format!("{}/{}", self.save_root, id);
        if !path::Path::new(&path).exists() {
            fs::create_dir_all(path).expect("FS error");
        }

        self.feeds.push(Feed { id, name, url })
    }

    async fn get_title(url: String) -> Result<String, Box<dyn Error>> {
        let tmp = reqwest::get(url).await?.bytes().await?;
        let title = Channel::read_from(&tmp[..]).unwrap().title;
        Ok(title)
    }
}
