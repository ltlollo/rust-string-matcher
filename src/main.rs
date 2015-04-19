#![feature(collections)]
#![feature(scoped)]
#![feature(custom_derive)]
#![feature(core)]

extern crate algos;
extern crate core;

use std::fs::{self, read_dir};
use core::fmt;
use algos::match_norm_sim;
use std::thread;
use std::sync::mpsc;
use std::cmp::Ordering;

type Data<'a> = (&'a String, &'a String);

struct StrMatch<'a> {
    data: Data<'a>,
    mch: f64
}

impl<'a> fmt::Display for StrMatch<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (ref f, ref s) = self.data;
        write!(fmt, "{}, {}, {}", f, s, self.mch)
    }
}

static NTHREADS: usize = 4;

fn find_similar(data: &Vec<String>) -> Vec<StrMatch> {
    let mut res = Vec::new();
    if data.len() < 2 {
        return res;
    }
    let chunksize = data.len() / NTHREADS;
    let (tx, rx) = mpsc::channel();
    {
        let mut guards = Vec::with_capacity(NTHREADS);
        for (i, chunk) in data[0..data.len()-1].chunks(chunksize).enumerate() {
            let ch = tx.clone();
            guards.push(thread::scoped(move || {
                let mut res = Vec::new();
                for (j, f) in chunk.iter().enumerate() {
                    for s in data[i*chunksize+j+1..data.len()].iter() {
                            let m = match_norm_sim(f.as_bytes(), s.as_bytes());
                            let r = StrMatch{ data: (f, s), mch: m };
                            res.push(r);
                    }
                }
                ch.send(res).unwrap();
            }));
        }
    }
    for _ in 0..NTHREADS {
        let mut v = rx.recv().unwrap();
        res.append(&mut v);
    }
    res.sort_by(|f: &StrMatch, s: &StrMatch| -> Ordering {
        f.mch.partial_cmp(&s.mch).unwrap()
    });
    res
}

fn show_similar(data: &Vec<String>, window: usize) {
    let res = find_similar(data);
    for str_match in res.iter().rev().take(window) {
        println!("{}", str_match);
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
    files.sort_by(|f: &String, s: &String| -> Ordering {
        f.len().cmp(&s.len())
    });
    show_similar(&files, 100);
}
