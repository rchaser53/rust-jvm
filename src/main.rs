#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::ops::{Fn, FnOnce};

enum Methods {
    ReturnI32(Box<Fn(i32) -> i32>),
}

fn foo(input: i32) -> i32 {
    input
}

fn main() {
    let a = Methods::ReturnI32(Box::new(foo));
    if let Methods::ReturnI32(b_a) = a {
        let aa = b_a(11);
        println!("{}", aa);
    }
}
