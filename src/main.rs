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
extern crate clap;
use clap::App;

use option::RJ_OPTION;

fn main() {
    let matches = App::new("rj")
        .version("0.1")
        .author("rchaser53 <tayoshizawa29@gmail.com>")
        .about("toy jvm implemented by Rust")
        .args_from_usage(
            "
            <INPUT>              'Sets the input file to use'
            --debug              'emits the debug information'",
        )
        .get_matches();
    RJ_OPTION.lock().unwrap().is_debug = matches.occurrences_of("debug") == 1;

    if let Some(file_name) = matches.value_of("INPUT") {
        let class_name = file_name.to_string() + ".class";
        if let Ok(buffer) = read_file(&class_name, &mut vec![]) {
            let (class_file, _pc_count) = Custom::new(buffer, 0);
            let class_map = setup_class_map();
            let mut context = Context::new(class_map);
            context.run_entry_file(class_file);
        } else {
            unimplemented!("need to add handler for the case failed to find the class file")
        }
    } else {
        println!("should input the file");
    }
}
