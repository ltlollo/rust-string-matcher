extern crate core;

use self::core::fmt;
use algos::match_norm_sim;
use std::thread;
use std::sync::mpsc;
use std::cmp::Ordering;

type Data<'a> = (&'a String, &'a String);

pub struct StrMatch<'a> {
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

pub fn find_similar(data: &Vec<String>) -> Vec<StrMatch> {
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

pub fn show_similar(data: &Vec<String>, window: usize) {
    let res = find_similar(data);
    for str_match in res.iter().rev().take(window) {
        println!("{}", str_match);
    }
}