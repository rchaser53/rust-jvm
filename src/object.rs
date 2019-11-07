use crate::operand::Item;
use crate::option::STRING_POOL;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub type ObjectMap = HashMap<usize, Objectref>;
#[derive(PartialEq, Clone, Debug)]
pub struct Objectref {
    pub class_name: usize,
    // field_name, object_ref_id
    pub field_map: RefCell<HashMap<(usize, usize), (Item, Item)>>,
    pub is_initialized: bool,
}

pub type FieldMap = RefCell<HashMap<(usize, usize), (Item, Item)>>;

impl Objectref {
    pub fn new(class_name: usize, field_map: FieldMap, is_initialized: bool) -> Objectref {
        Objectref {
            class_name,
            field_map,
            is_initialized,
        }
    }
}

impl fmt::Display for Objectref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_map = self.field_map.borrow();
        let keys = field_map.keys();
        let mut val_strs = Vec::with_capacity(keys.len());
        for key in keys {
            let val = field_map.get(key).unwrap();
            match val.1 {
                Item::Null => val_strs.push(format!("{}.{}: {}", key.0, key.1, val.0)),
                _ => val_strs.push(format!("{}.{}: {} {}", key.0, key.1, val.0, val.1)),
            };
        }

        let string_hash_map = &*STRING_POOL.lock().unwrap();
        write!(
            f,
            "object_ref:
class {}:
{}",
            string_hash_map.get(&self.class_name).unwrap(),
            val_strs.join("\n")
        )
    }
}
