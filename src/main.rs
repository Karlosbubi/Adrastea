//! Quality of life to be removed before beta
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

mod data;
mod service;
mod settings;

use data::*;
use service::*;
use settings::*;

use diesel::Queryable;
use std::{thread, time::Duration};
//use Settings::*;

const CONFIG_PATH: &str = "$HOME/.config/adrastea.conf";

fn main() {
    println!("Hello, world!");
    let mut test = Settings::new();
    test.add_feed("https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string());
    test.add_feed("https://rss.dw.com/xml/rss-de-all".to_string());

    let serv = Service::new(test);

    let rt = tokio::runtime::Runtime::new().unwrap();
    _ = rt.block_on(serv.daemon());
}
