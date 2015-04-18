#![feature(custom_attribute)]
#![feature(collections)]
#![desc = "Find similar files"]
#![license = "GPLv2"]

extern crate algos;

use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};
use std::cmp::Ordering::{self, Equal, Less, Greater};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use algos::match_norm_sim;

static NTASKS: usize = 4;

type Data = (String, String);

struct Res {
    pair: Data,
    mch: f64
}

fn worker(rx: &Receiver<Data>, num: usize, tx: Sender<Res>) {
    for _ in (0..num) {
        let (f, s) = rx.recv().unwrap();
        let m = match_norm_sim(f.as_bytes(), s.as_bytes());
        tx.send(Res { pair: (f, s), mch: m }).unwrap();
    }
}

fn work_balancer(data: &Vec<String>, tx: &Sender<Res>) {
    let total_load: usize = (data.len()*(data.len()-1))/2;
    let thread_load: usize = total_load/NTASKS;

    let mut tchs: Vec<Sender<Data>> = Vec::with_capacity(NTASKS);
    for id in (0..NTASKS) {
        let (tx_d, rx_d): (Sender<Data>, Receiver<Data>) = channel();
        tchs.push(tx_d);
        let rest_load = if total_load % NTASKS != 0
            && id < total_load % NTASKS {
            1 } else { 0
        };
        let tx_r = tx.clone();
        thread::spawn(move || {
            worker(&rx_d, thread_load + rest_load, tx_r);
        });
    }

    let mut m: usize = 0;
    for i in (0..data.len()-1) {
        for j in (i+1..data.len()) {
            let f = data[i].clone();
            let s = data[j].clone();
            tchs[m].send((f, s)).unwrap();
            m=(m+1)%NTASKS;
        }
    }
}

fn find_similar(data: &Vec<String>) {
    if data.len() < 2 {
        return;
    }
    let (tx, rx): (Sender<Res>, Receiver<Res>) = channel();

    work_balancer(data, &tx);

    let load: usize = data.len()*(data.len()-1)/2;
    let mut res: Vec<Res> = Vec::with_capacity(load);
    for _ in (0..load) {
        res.push(rx.recv().unwrap());
    }
    res.sort_by(|f: &Res, s: &Res| -> Ordering {
        if f.mch > s.mch { Greater } else if f.mch < s.mch { Less } else {
            Equal
        }
    });
    for it in res.iter().rev().take(100) {
        let (ref f, ref s) = it.pair;
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
    find_similar(&files);
}
