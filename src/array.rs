use crate::operand::Item;

use std::collections::HashMap;
use std::fmt;

pub type ArrayMap = HashMap<usize, Array>;

#[derive(Debug)]
pub enum PrimitiveArrayType {
    TBoolean = 4,
    TChar = 5,
    TFloat = 6,
    TDouble = 7,
    TByte = 8,
    TShort = 9,
    TInt = 10,
    TLong = 11,
}

#[derive(Debug)]
pub enum Array {
    Primitive(Vec<Item>),
    Array(Vec<Array>),
    Custom(usize),
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

// #[derive(PartialEq, Clone, Debug)]
// pub struct Arrayref {
//     pub class_name: String,
//     pub field_map: RefCell<HashMap<(String, usize), (Item, Item)>>,
// }

// impl fmt::Display for Arrayref {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let field_map = self.field_map.borrow();
//         let keys = field_map.keys();
//         let mut val_strs = Vec::with_capacity(keys.len());
//         for key in keys {
//             let val = field_map.get(key).unwrap();
//             match val.1 {
//                 Item::Null => val_strs.push(format!("{}.{}: {}", key.0, key.1, val.0)),
//                 _ => val_strs.push(format!("{}.{}: {} {}", key.0, key.1, val.0, val.1)),
//             };
//         }
//         write!(
//             f,
//             "object_ref:
// class {}:
// {}",
//             self.class_name,
//             val_strs.join("\n")
//         )
//     }
// }
