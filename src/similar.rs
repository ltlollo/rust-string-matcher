extern crate core;
extern crate thread_scoped;

use self::core::fmt;
use algos::match_norm_sim;
use self::thread_scoped::scoped;
use std::sync::mpsc;
use std::cmp::Ordering;

type Data<'a> = (&'a Vec<char>, &'a Vec<char>);

pub struct StrMatch<'a> {
    data: Data<'a>,
    mch: f64,
}

impl<'a> fmt::Display for StrMatch<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (ref f, ref s) = self.data;
        let f: String = f.iter().map(|x| *x).collect();
        let s: String = s.iter().map(|x| *x).collect();
        write!(fmt, "{}, {}, {}", f, s, self.mch)
    }
}

static NTHREADS: usize = 8;

pub fn find_similar(data: &Vec<Vec<char>>) -> Vec<StrMatch> {
    let mut res = Vec::new();
    if data.len() < 2 {
        return res;
    }
    let chunksize = data.len() / NTHREADS;
    let (tx, rx) = mpsc::channel();
    {
        let mut guards = Vec::with_capacity(NTHREADS);
        for (i, chunk) in data[0..data.len() - 1].chunks(chunksize).enumerate() {
            let ch = tx.clone();
            unsafe {
                guards.push(scoped(move || {
                    let mut res = Vec::new();
                    for (j, f) in chunk.iter().enumerate() {
                        for s in data[i * chunksize + j + 1..data.len()].iter() {
                            let m = match_norm_sim(&f[..], &s[..]);
                            let r = StrMatch {
                                data: (f, s),
                                mch: m,
                            };
                            res.push(r);
                        }
                    }
                    ch.send(res).unwrap();
                }));
            }
        }
    }
    for _ in 0..NTHREADS {
        let mut v = rx.recv().unwrap();
        res.append(&mut v);
    }
    res.sort_by(|f: &StrMatch, s: &StrMatch| -> Ordering { f.mch.partial_cmp(&s.mch).unwrap() });
    res
}

pub fn show_similar(data: &Vec<Vec<char>>, window: usize) {
    let res = find_similar(data);
    for str_match in res.iter().rev().take(window) {
        println!("{}", str_match);
    }
}
