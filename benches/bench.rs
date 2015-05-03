#![feature(test)]

extern crate lib;
extern crate test;

use lib::find_similar;
use test::Bencher;

#[bench]
fn measure_find_similar_usr_bin(b: &mut Bencher) {
    let files = (0..128).map(|_| {
            (0..128).map(|_| 'a' as char).collect()
        }).collect();
    b.iter(|| find_similar(&files));
}
