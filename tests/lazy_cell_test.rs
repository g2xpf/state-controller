extern crate state_controller;

use state_controller::utils::{LazyCell, Resource};
use std::{thread, time};

#[derive(Debug)]
struct A;

fn sleep_millis(msec: u64) {
    thread::sleep(time::Duration::from_millis(msec));
}

#[test]
fn lazy_cell_test() {
    let mut lc = LazyCell::new();
    lc.load(|| {
        sleep_millis(3000);
        A
    });

    // wait for loading the resource A
    loop {
        if lc.poll() {
            break;
        }
        sleep_millis(16);
    }

    let mut res = Resource::empty();

    lc.write(&mut res);

    println!("{:?}", res);
}

#[test]
#[should_panic]
fn lazy_cell_test_panic() {
    let mut lc = LazyCell::new();
    lc.load(|| {
        sleep_millis(3000);
        A
    });

    let mut res = Resource::empty();

    lc.write(&mut res);

    println!("{:?}", res);
}
