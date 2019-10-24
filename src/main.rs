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
use crate::java_class::{
    builtin::{BuiltIn, BuiltInMethod, BuitlInCodeType},
    custom::Custom,
    JavaClass,
};
use crate::utils::read_file;

#[macro_use]
extern crate lazy_static;
extern crate clap;
use clap::App;

use option::RJ_OPTION;
use std::collections::HashMap;

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

fn setup_class_map() -> HashMap<String, JavaClass> {
    let mut class_map = HashMap::new();
    let (print_stream_name, print_stream) = create_print_stream();
    let (java_lang_object_name, java_lang_object) = create_java_lang_object();
    let (java_lang_integer_name, java_lang_integer) = create_java_lang_integer();

    class_map.insert(print_stream_name, print_stream);
    class_map.insert(java_lang_object_name, java_lang_object);
    class_map.insert(java_lang_integer_name, java_lang_integer);
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

fn create_java_lang_object() -> (String, JavaClass) {
    let java_lang_object_name = String::from("java/lang/Object");
    let mut java_lang_object = BuiltIn::new(java_lang_object_name.clone());
    let init_name = String::from("<init>");
    let init = BuiltInMethod::new(init_name.clone(), BuitlInCodeType::JavaLangObjectInit);
    java_lang_object.methods.insert(init_name, init);
    (java_lang_object_name, JavaClass::BuiltIn(java_lang_object))
}

fn create_java_lang_integer() -> (String, JavaClass) {
    let java_lang_integer_name = String::from("java/lang/Integer");
    let mut java_lang_integer = BuiltIn::new(java_lang_integer_name.clone());
    let to_string_name = String::from("toString");
    let to_string = BuiltInMethod::new(
        to_string_name.clone(),
        BuitlInCodeType::JavaLangObjectToString,
    );
    java_lang_integer.methods.insert(to_string_name, to_string);
    (
        java_lang_integer_name,
        JavaClass::BuiltIn(java_lang_integer),
    )
}
