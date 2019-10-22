#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod attribute;
mod constant;
mod context;
mod field;
mod java_class;
mod method;
mod operand;
mod order;
mod stackframe;
mod utils;

use crate::context::Context;
use crate::java_class::{
    builtin::{BuiltIn, BuiltInMethod, BuitlInCodeType},
    custom::Custom,
    JavaClass,
};
use crate::utils::read_file;
extern crate lazy_static;

use std::collections::HashMap;

fn main() {
    let class_name = "HelloWorld.class";
    if let Ok(buffer) = read_file(class_name, &mut vec![]) {
        let (class_file, _pc_count) = Custom::new(buffer, 0);
        let class_map = setup_class_map();
        let mut context = Context::new(class_map);
        context.run_entry_file(class_file);
    } else {
        unimplemented!("need to add handler for the case failed to find the class file")
    }
}

fn setup_class_map() -> HashMap<String, JavaClass> {
    let mut class_map = HashMap::new();
    let (print_stream_name, print_stream) = create_print_stream();
    class_map.insert(print_stream_name, print_stream);
    class_map
}

fn create_print_stream() -> (String, JavaClass) {
    let print_stream_name = String::from("java/io/PrintStream");
    let mut print_stream = BuiltIn::new(print_stream_name.clone());
    let println_name = String::from("println");
    let println = BuiltInMethod::new(println_name.clone(), BuitlInCodeType::Println);
    print_stream.methods.insert(println_name, println);
    (print_stream_name, JavaClass::BuiltIn(print_stream))
}
