//! Quality of life to be removed before beta 
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

mod settings;
mod data;
mod service;

use settings::*;
use data::*;
use service::*;

use std::{thread, time::Duration};
use diesel::Queryable;
//use Settings::*;

const CONFIG_PATH : &str = "$HOME/.config/adrastea.conf";

fn main() {
    println!("Hello, world!");
    let _test = Settings::new();
}