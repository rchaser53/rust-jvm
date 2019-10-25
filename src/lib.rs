#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod attribute;
mod constant;
mod context;
mod field;
mod java_class;
mod method;
mod operand;
mod option;
mod order;
mod stackframe;
mod utils;

use crate::context::Context;
use crate::java_class::{custom::Custom, default::setup_class_map};
use crate::utils::read_file;

#[macro_use]
extern crate lazy_static;

use option::RJ_OPTION;

pub fn execute(file_name: String, is_debug: bool) {
    RJ_OPTION.lock().unwrap().is_debug = is_debug;
    let class_name = file_name + ".class";
    if let Ok(buffer) = read_file(&class_name, &mut vec![]) {
        let (class_file, _pc_count) = Custom::new(buffer, 0);
        let class_map = setup_class_map();
        let mut context = Context::new(class_map);
        context.run_entry_file(class_file);
    } else {
        unimplemented!("need to add handler for the case failed to find the class file")
    }
}
