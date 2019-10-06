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

use crate::utils::read_file;

fn main() {
    if let Ok(buffer) = read_file("A.class", &mut vec![]) {
        let class_file = class_file::ClassFile::new(buffer, 0);

        println!("{}", class_file.0.cp_info);
    }
}
