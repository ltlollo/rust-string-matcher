#![feature(custom_attribute)]
#![feature(collections)]
#![desc = "Find similar files"]
#![license = "GPLv2"]

extern crate algos;

use std::fs::{self, read_dir};
use std::cmp::Ordering::{self, Equal, Less, Greater};
use algos::match_norm_sim;

type Data<'a> = (&'a String, &'a String);
struct Res<'a> {
    data: Data<'a>,
    mch: f64
}

fn find_similar(data: &Vec<String>, window: usize) {
    if data.len() < 2 {
        return;
    }
    let mut res = Vec::new();
    for i in (0..data.len()-1) {
        for j in (i+1..data.len()) {
            let ref f = data[i];
            let ref s = data[j];
            let m = match_norm_sim(f.as_bytes(), s.as_bytes());
            let r = Res{ data: (f, s), mch: m };
            res.push(r);
        }
    }
    res.sort_by(|f: &Res, s: &Res| -> Ordering {
        if f.mch > s.mch {
            Greater
        } else if f.mch < s.mch {
            Less
        } else {
            Equal
        }
    });
    for it in res.iter().rev().take(window) {
        let (ref f, ref s) = it.data;
        println!("{}, {} is {}", f, s, it.mch);
    }
}

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
    find_similar(&files, 100); 
}
