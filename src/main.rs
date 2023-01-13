//! Quality of life to be removed before beta 
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use clap::Parser;

const CONFIG_PATH : &str = "$HOME/.config/adrastea.conf";
struct Feed {
    id: i32,
    name: String,
    url: String,
}
struct Article {
    title: String,
    version: i32,
    path: String,
    feed: Feed,
    url: String,
}

struct Settings {
    feeds: Vec<Feed>,
    check_every: i32,
    save_root: String,
}

fn main() {
    println!("Hello, world!");
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            feeds: Vec::new(),
            check_every: 5,
            save_root: "$HOME/.adrastea".to_string(),
        }
    }
    pub fn add_feed(&mut self, url: String) {
        let name: String = "".to_string(); //TODO : Parse feed title
        let id: i32 = 0; //TODO : Opbtain Id from DB

        self.feeds.push(Feed { id, name, url })
    }
}
