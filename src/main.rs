#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod attribute;
mod constant;
mod context;
mod field;
mod operand;
mod order;
mod stackframe;
mod utils;

use crate::attribute::Attribute;
use crate::constant::ConstantPool;
use crate::field::Field;
use crate::utils::{extract_x_byte_as_usize, read_file};

#[derive(Debug)]
struct Interface;

#[derive(Debug)]
struct Method {
    pub access_flags: u16,     // u2
    pub name_index: u16,       // u2
    pub descriptor_index: u16, // u2
    pub attributes_count: u16, // u2
    pub attribute_info: Vec<Attribute>,
}

#[derive(Debug)]
struct ClassFile {
    pub magic: u32,                 // u4
    pub minor_version: u16,         // u2
    pub major_version: u16,         // u2
    pub constant_pool_count: u16,   // u2
    pub cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    pub access_flags: AccessFlag,   // u2
    pub this_class: u16,            // u2
    pub super_class: u16,           // u2
    pub interfaces_count: u16,      // u2
    pub interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    pub fields_count: u16,          // u2
    pub fields: Vec<Field>,         // field_info     fields[fields_count];
    pub methods_count: u16,         // u2
    pub methods: Vec<Method>,       // method_info    methods[methods_count];
    pub attributes_count: u16,      // u2
    pub attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

impl ClassFile {
    // pub fn new(input: &mut Vec<u8>) -> ClassFile {
    pub fn new(input: &mut [u8]) {
        let index = 0;

        let (_, index) = extract_x_byte_as_usize(input, index, 4);
        let (minor_version, index) = extract_x_byte_as_usize(input, index, 2);
        let (major_version, index) = extract_x_byte_as_usize(input, index, 2);
        let (constant_pool_count, index) = extract_x_byte_as_usize(input, index, 2);
        let (constant_pool, index) = ConstantPool::new(input, index, constant_pool_count);

        println!("{}", constant_pool);
    }
}

#[derive(Debug)]
pub enum AccessFlag {
    AccPublic,
    AccFinal,
    AccSuper,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
}

impl From<u16> for AccessFlag {
    fn from(num: u16) -> AccessFlag {
        match num {
            0x0001 => AccessFlag::AccPublic,
            0x0010 => AccessFlag::AccFinal,
            0x0020 => AccessFlag::AccSuper,
            0x0200 => AccessFlag::AccInterface,
            0x0400 => AccessFlag::AccAbstract,
            0x1000 => AccessFlag::AccSynthetic,
            0x2000 => AccessFlag::AccAnnotation,
            0x4000 => AccessFlag::AccEnum,
            _ => panic!("failed to convert {} to AccessFlag", num),
        }
    }
}

fn main() {
    if let Ok(buffer) = read_file("A.class", &mut vec![]) {
        ClassFile::new(buffer);
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
