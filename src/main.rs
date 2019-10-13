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

use crate::class_file::ClassFile;
use crate::context::Context;
use crate::utils::read_file;

fn main() {
    let class_name = "HelloWorld.class";
    if let Ok(buffer) = read_file(class_name, &mut vec![]) {
        let (class_file, pc_count) = ClassFile::new(buffer, 0);
        let mut context = Context::new();
        context.run_entry_file(&class_file);
    } else {
        unimplemented!("need to add handler for the case failed to find the class file")
    }
}
