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
    pub constant_pool_count: usize, // u2
    pub cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    pub access_flags: AccessFlag,   // u2
    pub this_class: usize,          // u2
    pub super_class: usize,         // u2
    pub interfaces_count: usize,    // u2
    pub interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    pub fields_count: usize,        // u2
    pub fields: Vec<Field>,         // field_info     fields[fields_count];
    pub methods_count: usize,       // u2
    pub methods: Vec<Method>,       // method_info    methods[methods_count];
    pub attributes_count: usize,    // u2
    pub attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

impl ClassFile {
    pub fn new(input: &mut [u8], index: usize) -> (ClassFile, usize) {
        let (magic, index) = extract_x_byte_as_usize(input, index, 4);
        let magic = magic as u32;

        let (minor_version, index) = extract_x_byte_as_usize(input, index, 2);
        let minor_version = minor_version as u16;
        let (major_version, index) = extract_x_byte_as_usize(input, index, 2);
        let major_version = major_version as u16;

        let (constant_pool_count, index) = extract_x_byte_as_usize(input, index, 2);
        let (cp_info, index) = ConstantPool::new(input, index, constant_pool_count);

        let (access_flags_num, index) = extract_x_byte_as_usize(input, index, 2);
        let access_flags = AccessFlag::from(access_flags_num);

        let (this_class, index) = extract_x_byte_as_usize(input, index, 2);
        let (super_class, index) = extract_x_byte_as_usize(input, index, 2);

        let (interfaces_count, index) = extract_x_byte_as_usize(input, index, 2);
        let interfaces = Vec::with_capacity(interfaces_count);

        let (fields_count, index) = extract_x_byte_as_usize(input, index, 2);
        let fields = Vec::with_capacity(fields_count);

        let (methods_count, index) = extract_x_byte_as_usize(input, index, 2);
        let methods = Vec::with_capacity(methods_count);

        let (attributes_count, index) = extract_x_byte_as_usize(input, index, 2);
        let attributes = Vec::with_capacity(attributes_count);

        (
            ClassFile {
                magic,
                minor_version,
                major_version,
                constant_pool_count,
                cp_info,
                access_flags,
                this_class,
                super_class,
                interfaces_count,
                interfaces,
                fields_count,
                fields,
                methods_count,
                methods,
                attributes_count,
                attributes,
            },
            index,
        )
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

impl From<usize> for AccessFlag {
    fn from(num: usize) -> AccessFlag {
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
        let class_file = ClassFile::new(buffer, 0);

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
