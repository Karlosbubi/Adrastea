use crate::data::*;

pub struct Settings<'a> {
    pub feeds: Vec<Feed<'a>>,
    pub check_every: u64,
    pub check_deep: usize,
    pub save_root: String,
}

impl Settings<'_> {
    pub fn new() -> Self {
        Settings {
            feeds: Vec::new(),
            check_every: 5,
            check_deep: 20,
            save_root: "$HOME/.adrastea".to_string(),
        }
    }
    pub fn add_feed(&mut self, url: &str) {
        let name: &str = ""; //TODO : get feed title
        let id: i32 = 0; //TODO : Opbtain Id from DB

        let link = url.clone();

        self.feeds.push(Feed { id, name, url : link })
    }
}