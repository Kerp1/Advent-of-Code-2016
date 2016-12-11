#![feature(unboxed_closures)]

extern crate regex;
extern crate crypto;

mod common;
mod day7;

fn main() {
    println!("day7::task1() = {:?}", day7::task1());
    println!("day7::task2() = {:?}", day7::task2());
}
