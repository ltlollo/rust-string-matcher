#![feature(collections)]
#![feature(scoped)]

extern crate algos;

use std::fs::{self, read_dir};
use std::cmp::Ordering::{self, Equal, Less, Greater};
use algos::match_norm_sim;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

type Data<'a> = (&'a String, &'a String);
struct Res<'a> {
    data: Data<'a>,
    mch: f64
}

static NTHREADS: usize = 4;

fn find_similar(data: &Vec<String>, window: usize) {
    if data.len() < 2 {
        return;
    }
    let chunksize = data.len() / NTHREADS;
    let (tx, rx): (Sender<Vec<Res>>, Receiver<Vec<Res>>) = mpsc::channel();
    {
        let mut guards = Vec::with_capacity(NTHREADS);
        for (i, chunk) in data[0..data.len()-1].chunks(chunksize).enumerate() {
            let ch = tx.clone();
            guards.push(thread::scoped(move || {
                let mut res = Vec::new();
                for (k, ele) in chunk.iter().enumerate() {
                    for j in (i*chunksize+k+1..data.len()) {
                            let ref f = ele;
                            let ref s = data[j];
                            let m = match_norm_sim(f.as_bytes(), s.as_bytes());
                            let r = Res{ data: (f, s), mch: m };
                            res.push(r);
                    }
                }
                ch.send(res).unwrap();
            }));
        }
    }
    let mut res = Vec::new();
    for _ in 0..NTHREADS {
        let mut v = rx.recv().unwrap();
        res.append(&mut v);
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
    files.sort_by(|f: &String, s: &String| -> Ordering {
        f.len().cmp(&s.len())
    });
    find_similar(&files, 100); 
}
