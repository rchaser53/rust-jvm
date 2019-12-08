#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod array;
mod attribute;
mod constant;
mod context;
mod field;
mod java_class;
mod method;
mod object;
mod operand;
mod option;
mod order;
mod stackframe;
mod string_pool;
mod utils;
mod wasm;

use crate::context::Context;
use crate::java_class::{custom::Custom, default::setup_class_map};
use crate::string_pool::StringPool;

use crate::option::RJ_OPTION;
use crate::wasm::get_file_content;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use std::path::Path;

#[macro_use]
extern crate lazy_static;

pub fn execute(file_name: String, debug_mode: usize) {
    RJ_OPTION.lock().unwrap().debug_mode = debug_mode;
    let class_name = file_name + ".class";
    let buffer = get_file_content(&class_name);
    let mut string_pool = StringPool::new();
    let (class_file, _pc_count) = Custom::new(&mut string_pool, &buffer, 0);
    let class_map = setup_class_map(&mut string_pool);
    let parent_path = if let Some(parent_path) = Path::new(&class_name).parent() {
        parent_path.to_str().unwrap()
    } else {
        "./"
    };

    let mut context = Context::new(&mut string_pool, class_map, &class_file, parent_path);
    context.run_entry_file(&mut string_pool, class_file);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_wasm(class_name: &str) {
    let mut string_pool = StringPool::new();
    let inputs = get_file_content(class_name);
    let (class_file, _pc_count) = Custom::new(&mut string_pool, &inputs, 0);
    let class_map = setup_class_map(&mut string_pool);
    let parent_path = "";

    let mut context = Context::new(&mut string_pool, class_map, &class_file, parent_path);
    context.run_entry_file(&mut string_pool, class_file);
}
