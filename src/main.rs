#![desc = "Find similar files"]
#![license = "GPLv2"]
#![warn(non_camel_case_types)]

extern crate algos;

use std::io::fs::readdir;
use std::comm::{channel, Sender, Receiver};
use algos::match_norm_sim;

static NTASKS: uint = 4;

type Data = (String, String);

struct Res {
    pair: Data,
    mch: f64
}

fn worker(rx: &Receiver<Data>, num: uint, tx: Sender<Res>) {
    for _ in range(0, num) {
        let (f, s) = rx.recv();
        let m = match_norm_sim(f.as_bytes(), s.as_bytes());
        tx.send(Res { pair: (f, s), mch: m });
    }
}

fn work_balancer(data: &Vec<Path>, tx: &Sender<Res>) {
    let total_load: uint = (data.len()*(data.len()-1))/2;
    let thread_load: uint = total_load/NTASKS;

    let mut tchs: Vec<Sender<Data>> = Vec::with_capacity(NTASKS);
    for id in range(0, NTASKS) {
        let (tx_d, rx_d): (Sender<Data>, Receiver<Data>) = channel();
        tchs.push(tx_d);
        let rest_load = if total_load % NTASKS != 0
            && id < total_load % NTASKS {
            1 } else { 0
        };
        let tx_r = tx.clone();
        spawn(proc() {
            worker(&rx_d, thread_load + rest_load, tx_r);
        });
    }

    let mut m: uint = 0;
    for i in range(0, data.len()-1) {
        for j in range(i+1, data.len()) {
            let f = String::from_str(data[i].as_str().unwrap());
            let s = String::from_str(data[j].as_str().unwrap());
            tchs[m].send((f, s));
            m=(m+1)%NTASKS;
        }
    }
}

fn find_similar(data: &Vec<Path>) {
    if data.len() < 2 {
        return;
    }

    let (tx, rx): (Sender<Res>, Receiver<Res>) = channel();

    work_balancer(data, &tx);

    let load: uint = data.len()*(data.len()-1)/2;
    let mut res: Vec<Res> = Vec::with_capacity(load);
    for _ in range(0, load) {
        res.push(rx.recv());
    }
    res.sort_by(|f: &Res, s: &Res| -> Ordering
        if f.mch > s.mch { Greater } else if f.mch < s.mch { Less } else {
            Equal
        }
    );
    for it in res.iter().rev() {
        let (ref f, ref s) = it.pair;
        println!("{}, {} is {}", f, s, it.mch);
    }
}

fn main() {
    let files = readdir(&Path::new("."));
    match files {
        Err(_) => (),
        Ok(ref files) => find_similar(files),
    }
}
