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
use crate::context::{Context, JavaClass};
use crate::utils::read_file;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

fn main() {
    let class_name = "HelloWorld.class";
    if let Ok(buffer) = read_file(class_name, &mut vec![]) {
        let (class_file, _pc_count) = ClassFile::new(buffer, 0);
        let class_name = class_file.this_class_name();
        let java_class_file = JavaClass::CustomClass(class_file);

        let mut class_map = HashMap::new();
        class_map.insert(class_name.as_str(), &java_class_file);
        let mut context = Context::new(class_map);

        if let JavaClass::CustomClass(ref class_file) = java_class_file {
            context.run_entry_file(&class_file);
        }
    } else {
        unimplemented!("need to add handler for the case failed to find the class file")
    }
}
