#![feature(collections)]
#![feature(core)]

extern crate core;
extern crate lib;

use std::fs::{self, read_dir};
use std::cmp::Ordering;

use lib::show_similar;

fn main() {
    let mut files = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let fstr = String::from_str(entry
                                    .unwrap()
                                    .path()
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap());
        files.push(fstr);
    }
    files.sort_by(|f: &String, s: &String| -> Ordering {
        f.len().cmp(&s.len())
    });
    show_similar(&files, 100);
}
