use crate::operand::Item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub type ObjectMap = HashMap<usize, Objectref>;
#[derive(PartialEq, Clone, Debug)]
pub struct Objectref {
    pub class_name: String,
    pub field_map: RefCell<HashMap<(String, usize), (Item, Item)>>,
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
        write!(
            f,
            "object_ref:
class {}:
{}",
            self.class_name,
            val_strs.join("\n")
        )
    }
}
