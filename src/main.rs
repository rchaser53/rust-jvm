#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod attribute;
mod class_file;
mod constant;
mod context;
mod field;
mod method;
mod operand;
mod order;
mod stackframe;
mod utils;

use crate::context::Context;

fn main() {
    let mut context = Context::new("HelloWorld.class");
    context.run_entry_file();
}
