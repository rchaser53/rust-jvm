use crate::utils::devide_i64_to_two_i32;
use std::cell::RefCell;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Item {
    Null,
    Int(i32),
    Long(i32),
    String(String),
    Boolean(bool),
    Classref(String),
    Fieldref(usize),
    Objectref(Objectref),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Objectref {
    pub class_name: String,
    pub field_map: RefCell<HashMap<String, (Item, Item)>>,
}

impl fmt::Display for Objectref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_map = self.field_map.borrow();
        let keys = field_map.keys();
        let mut val_strs = Vec::with_capacity(keys.len());
        for key in keys {
            let val = field_map.get(key).unwrap();
            match val.1 {
                Item::Null => val_strs.push(format!("{}: {}", key, val.0)),
                _ => val_strs.push(format!("{}: {} {}", key, val.0, val.1)),
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

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Null => write!(f, "null"),
            Item::Int(val) => write!(f, "int: {}", val),
            Item::Long(val) => write!(f, "long: {}", val),
            Item::Boolean(val) => write!(f, "boolean: {}", val),
            Item::String(val) => write!(f, "string: {}", val),
            Item::Classref(val) => write!(f, "class_ref: {}", val),
            Item::Fieldref(val) => write!(f, "field_ref: {}", val),
            Item::Objectref(val) => write!(f, "{}", val),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        match (self, other) {
            (Item::Null, Item::Null) => Some(Ordering::Equal),
            (Item::Int(left), Item::Int(right)) => Some(left.cmp(right)),
            (Item::Boolean(left), Item::Boolean(right)) => Some(left.cmp(right)),
            (Item::Long(left), Item::Long(right)) => Some(left.cmp(right)),
            (Item::Classref(left), Item::Classref(right)) => Some(left.cmp(right)),
            (Item::Fieldref(left), Item::Fieldref(right)) => Some(left.cmp(right)),
            (Item::String(left), Item::String(right)) => Some(left.cmp(right)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct OperandStack {
    pub stack: Vec<Item>,
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    fn extract_long_values(&mut self) -> (i64, i64) {
        match (
            self.stack.pop(),
            self.stack.pop(),
            self.stack.pop(),
            self.stack.pop(),
        ) {
            (
                Some(Item::Long(second_2)),
                Some(Item::Long(second_1)),
                Some(Item::Long(first_2)),
                Some(Item::Long(first_1)),
            ) => {
                let second: i64 = (((second_1 as i64) << 32) as i64) | second_2 as i64;
                let first: i64 = (((first_1 as i64) << 32) as i64) | first_2 as i64;
                (first, second)
            }
            _ => panic!("shortage item in OperandStack"),
        }
    }

    fn extract_int_values(&mut self) -> (i32, i32) {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Int(second)), Some(Item::Int(first))) => (first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn iadd(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first + second)
    }

    pub fn ladd(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first + second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn isub(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first - second)
    }

    pub fn lsub(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first + second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn imul(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first * second)
    }

    pub fn lmul(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first * second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn idiv(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first / second)
    }

    pub fn ldiv(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first / second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn irem(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first % second)
    }

    pub fn lrem(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first % second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn lcmp(&mut self) -> Item {
        let (first, second) = self.extract_long_values();
        if first > second {
            Item::Int(1)
        } else if first == second {
            Item::Int(0)
        } else {
            Item::Int(-1)
        }
    }
}

impl fmt::Display for OperandStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length = self.stack.len();
        let mut item_string_vec = Vec::with_capacity(length);
        let mut index = 0;
        for item in self.stack.iter() {
            match item {
                Item::Long(_) => {
                    item_string_vec.push(format!("#{}+#{} {}", index, index + 1, item));
                    index += 1;
                }
                _ => item_string_vec.push(format!("#{} {}", index, item)),
            };
            index += 1;
        }

        write!(
            f,
            "length: {}
item ====================
{}
========================",
            length,
            item_string_vec.join("\n")
        )
    }
}
