#![crate_name = "algos"]
#![crate_type = "lib"]
#![desc = "Some algoritms"]
#![license = "GPLv2"]
#![warn(non_camel_case_types)]

pub fn match_cont<T: PartialEq>(fst: &[T], snd: &[T]) -> uint {
    let mut mch_max :uint = 0;
    let mut mch: uint;
    let mut it = fst.iter();

    loop {
        mch = 0;
        for (f, s) in it.zip(snd.iter()) {
            if *f == *s {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None {
            break;
        }
    }
    it = snd.iter();
    loop {
        mch = 0;
        for (f, s) in it.zip(fst.iter()) {
            if *f == *s {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None {
            break;
        }
    }
    mch_max
}

pub fn match_cont_sim<T: PartialEq>(fst: &[T], snd: &[T]) -> uint {
    let mut mch_max :uint = 0;
    let mut mch: uint;
    let mut it = fst.iter();
    let mut len = fst.len();

    loop {
        mch = 0;
        for (f, s) in it.zip(snd.iter()) {
            if *f == *s {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None || mch_max >= len {
            break;
        }
        len -= 1;
    }
    it = snd.iter();
    len = snd.len();
    loop {
        mch = 0;
        for (f, s) in it.zip(fst.iter()) {
            if *f == *s {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None || mch_max >= len {
            break;
        }
        len -= 1;
    }

    mch_max
}

/*
pub fn match_cont_sim ... {
    ...
    loop {
        let mch: uint = it.zip(snd.iter()).fold(0, |accumulator, (f, s)| {
            if *f == *s {
                accumulator+1 
            } else {
                accumulator
            }
        });
        ...
    }
    it = snd.iter();
    len = snd.len();
    loop {
        let mch: uint = it.zip(fst.iter()).fold(0, |accumulator, (f, s)| {
            if *f == *s {
                accumulator+1 
            } else {
                accumulator
            }
        });
        ...
    }
    mch_max
}
*/

pub fn match_cont_sim_trim<T: PartialEq>(fst: &[T], snd: &[T],
                                         fun: |f: &T, s: &T| -> bool) -> uint {
    let mut mch_max: uint = 0;
    let mut mch: uint;
    let mut it = fst.iter();
    let mut len = fst.len();

    loop {
        mch = 0;
        for (f, s) in it.zip(snd.iter()) {
            if fun(f, s) {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None || mch_max >= len {
            break;
        }
        len -= 1;
    }
    it = snd.iter();
    len = snd.len();
    loop {
        mch = 0;
        for (f, s) in it.zip(fst.iter()) {
            if fun(f, s) {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None || mch_max >= len {
            break;
        }
        len -= 1;
    }

    mch_max
}

pub fn match_cont_trim<T: PartialEq>(fst: &[T], snd: &[T],
                                     fun: |f: &T, s: &T| -> bool) -> uint {
    let mut mch_max :uint = 0;
    let mut mch: uint;
    let mut it = fst.iter();

    loop {
        mch = 0;
        for (f, s) in it.zip(snd.iter()) {
            if fun(f, s) {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None {
            break;
        }
    }
    it = snd.iter();
    loop {
        mch = 0;
        for (f, s) in it.zip(fst.iter()) {
            if fun(f, s) {
                mch += 1;
            }
        }
        if mch > mch_max {
            mch_max = mch;
        }
        if it.next() == None {
            break;
        }
    }
    mch_max
}

pub fn match_norm<T: PartialEq>(fst: &[T], snd: &[T]) -> f64 {
    let len = if fst.len() > snd.len() { fst.len() } else { snd.len() };
    if len != 0 {
        (match_norm(fst, snd) as f64)/(len as f64)
    } else {
        0.0
    }
}

pub fn match_norm_sim<T: PartialEq>(fst: &[T], snd: &[T]) -> f64 {
    let len = if fst.len() > snd.len() { fst.len() } else { snd.len() };
    if len != 0 {
        (match_cont_sim(fst, snd) as f64)/(len as f64)
    } else {
        0.0
    }
}

pub fn match_norm_trim<T: PartialEq>(fst: &[T], snd: &[T],
                                     fun: |f: &T, s: &T| -> bool) -> f64 {
    let len = if fst.len() > snd.len() { fst.len() } else { snd.len() };
    if len != 0 {
        (match_cont_trim(fst, snd, fun) as f64)/(len as f64)
    } else {
        0.0
    }
}

pub fn match_norm_sim_trim<T: PartialEq>(fst: &[T], snd: &[T],
                                         fun: |f: &T, s: &T| -> bool) -> f64 {
    let len = if fst.len() > snd.len() { fst.len() } else { snd.len() };
    if len != 0 {
        (match_cont_sim_trim(fst, snd, fun) as f64)/(len as f64)
    } else {
        0.0
    }
}

