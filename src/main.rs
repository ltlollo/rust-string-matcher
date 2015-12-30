extern crate lib;

use std::fs;
use std::cmp::Ordering;

use lib::show_similar;

fn main() {
    let mut files = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let fstr = String::from(entry.unwrap()
                                     .path()
                                     .file_name()
                                     .unwrap()
                                     .to_str()
                                     .unwrap());
        files.push(fstr.chars().collect());
    }
    files.sort_by(|f: &Vec<char>, s: &Vec<char>| -> Ordering { f.len().cmp(&s.len()) });
    show_similar(&files, 100);
}
