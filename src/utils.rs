use crate::attribute::instruction::Instruction;
use crate::object::{ObjectMap, Objectref};
use crate::operand::Item;
use crate::option::{OBJECT_ID, RJ_OPTION, STRING_KEY_VALUE_POOL, STRING_VALUE_KEY_POOL};
use crate::stackframe::Stackframe;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::iter;
use std::path::Path;

pub fn read_file<'a, P: AsRef<Path>>(
    input: &'a P,
    buffer: &'a mut Vec<u8>,
) -> Result<&'a mut [u8]> {
    let mut f = File::open(input)?;
    f.read_to_end(buffer)?;
    Ok(buffer.as_mut_slice())
}

pub fn extract_x_byte_as_vec(input: &mut [u8], index: usize, x: usize) -> (Vec<u8>, usize) {
    let mut result = Vec::with_capacity(x);
    for i in 0..x {
        result.push(input[index + i]);
    }
    (result, index + x)
}

pub fn extract_x_byte_as_usize(input: &mut [u8], index: usize, x: usize) -> (usize, usize) {
    let mut result: usize = 0;
    for i in 0..x {
        result += (input[index + i] as usize) << (x - i - 1) * 8;
    }
    (result, index + x)
}

pub fn devide_i64_to_two_i32(input: i64) -> (i32, i32) {
    (((input >> 32) << 32) as i32, (input & 0xFFFFFFFF) as i32)
}

pub fn emit_debug_info(instruction: &Instruction, stackframe: Option<&Stackframe>) {
    match RJ_OPTION.lock().unwrap().debug_mode {
        1 => {
            println!("instruction: {}", instruction,);
        }
        2 => {
            println!(
                "instruction: {}
operand_stack:
{}
",
                instruction,
                stackframe.unwrap().operand_stack
            );
        }
        _ => {}
    };
}

pub fn iniailize_primitive_array(type_index: usize, length: usize) -> Vec<(Item, Item)> {
    let default_val = match type_index {
        // TBoolean
        4 => (Item::Boolean(false), Item::Null),
        // // TChar
        // 5 => ,
        // // TFloat
        // 6 => ,
        // // TDouble
        // 7 => ,
        // // TByte
        // 8 => ,
        // // TShort
        // 9 => ,
        // TInt
        10 => (Item::Int(0), Item::Null),
        // TLong
        11 => (Item::Long(0), Item::Long(0)),
        _ => unreachable!("type_index range should 4 - 11"),
    };
    let mut initialize_vec = vec![];
    initialize_vec.extend(iter::repeat(default_val).take(length));
    initialize_vec
}

pub fn initialize_objectref_array(
    object_map: &mut ObjectMap,
    class_name_id: usize,
    length: usize,
) -> Vec<usize> {
    let mut initialize_vec = Vec::with_capacity(length);
    for _ in 0..length {
        let id = *OBJECT_ID.lock().unwrap();
        *OBJECT_ID.lock().unwrap() = id + 1;
        initialize_vec.push(id);
        object_map.insert(
            id,
            Objectref::new(class_name_id, RefCell::new(HashMap::new()), false),
        );
    }

    initialize_vec
}

pub fn insert_string_pool(value: String) -> usize {
    {
        let string_value_key_map = &*STRING_VALUE_KEY_POOL.lock().unwrap();
        if let Some(id) = string_value_key_map.get(&value) {
            return *id;
        }
    }

    let string_id = *OBJECT_ID.lock().unwrap();
    *OBJECT_ID.lock().unwrap() = string_id + 1;

    let string_value_key_map = &mut *STRING_VALUE_KEY_POOL.lock().unwrap();
    string_value_key_map.insert(value.clone(), string_id);

    let string_key_value_map = &mut *STRING_KEY_VALUE_POOL.lock().unwrap();
    string_key_value_map.insert(string_id, value);
    string_id
}

pub fn get_string_from_string_pool(id: &usize) -> String {
    let string_hash_map = &*STRING_KEY_VALUE_POOL.lock().unwrap();
    string_hash_map
        .get(id)
        .expect("exist string in string_pool")
        .to_string()
}

#[macro_export]
macro_rules! add_flags {
    ($flags:expr, $num:expr, $flag:expr) => {
        if $num & $flag as usize != 0 {
            $flags.push($flag)
        }
    };
}

#[test]
pub fn test_extract_x_byte_as_vec() {
    let mut input = vec![1, 2, 3, 4];
    assert_eq!(extract_x_byte_as_vec(&mut input, 1, 2), (vec![2, 3], 3));

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(
        extract_x_byte_as_vec(&mut input, 3, 4),
        (vec![4, 5, 6, 7], 7)
    );
}

#[test]
pub fn test_extract_x_byte_as_usize() {
    let mut input = vec![1, 2, 3, 4];
    assert_eq!(
        extract_x_byte_as_usize(&mut input, 1, 2),
        ((2 << 8) as usize + 3, 3)
    );
    assert_eq!(input[3], 4);

    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let fourth = 1 << (8 * 3);
    let third = 2 << (8 * 2);
    let second = 3 << (8 * 1);
    let first = 4 << (8 * 0);

    assert_eq!(
        extract_x_byte_as_usize(&mut input, 0, 4),
        (first + second + third + fourth, 4)
    );
    assert_eq!(input[4], 5);
}
