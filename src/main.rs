#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod attribute;
mod class_file;
mod constant;
mod context;
mod field;
mod operand;
mod order;
mod stackframe;
mod utils;

use crate::utils::read_file;

fn main() {
    if let Ok(buffer) = read_file("Helloworld.class", &mut vec![]) {
        let class_file = class_file::ClassFile::new(buffer, 0);

        println!("{}", class_file.0.cp_info);
    }
}

/*
* 1 + 2;
*/
// bipush 1
// bipush 2
// iadd

/*
 *  int i;
 *  i = 0;
 */
//  iconst_0
//  istore_1
//
