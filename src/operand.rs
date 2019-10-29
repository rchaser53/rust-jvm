use crate::stackframe::StackframeItem;
use crate::utils::devide_i64_to_two_i32;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum OperandStackItem {
    Null,
    Int(i32),
    Long(i32),
    String(String),
    Boolean(bool),
    Classref(String),
    Fieldref(usize),
    Objectref(usize),
}

impl From<&StackframeItem> for OperandStackItem {
    fn from(item: &StackframeItem) -> OperandStackItem {
        match item {
            StackframeItem::Int(value) => OperandStackItem::Int(*value),
            StackframeItem::Long(value) => OperandStackItem::Long(*value),
            StackframeItem::Boolean(value) => OperandStackItem::Boolean(value.clone()),
            StackframeItem::String(value) => OperandStackItem::String(value.clone()),
            StackframeItem::Classref(value) => OperandStackItem::Classref(value.clone()),
            StackframeItem::Fieldref(index) => OperandStackItem::Fieldref(*index),
            StackframeItem::Objectref(index) => OperandStackItem::Objectref(*index),
            StackframeItem::Null => OperandStackItem::Null,
        }
    }
}

impl fmt::Display for OperandStackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandStackItem::Null => write!(f, "null"),
            OperandStackItem::Int(val) => write!(f, "int: {}", val),
            OperandStackItem::Long(val) => write!(f, "long: {}", val),
            OperandStackItem::Boolean(val) => write!(f, "boolean: {}", val),
            OperandStackItem::String(val) => write!(f, "string: {}", val),
            OperandStackItem::Classref(val) => write!(f, "class_ref: {}", val),
            OperandStackItem::Fieldref(val) => write!(f, "field_ref: {}", val),
            OperandStackItem::Objectref(val) => write!(f, "object_ref: {}", val),
        }
    }
}

impl PartialOrd for OperandStackItem {
    fn partial_cmp(&self, other: &OperandStackItem) -> Option<Ordering> {
        match (self, other) {
            (OperandStackItem::Null, OperandStackItem::Null) => Some(Ordering::Equal),
            (OperandStackItem::Int(left), OperandStackItem::Int(right)) => Some(left.cmp(right)),
            (OperandStackItem::Boolean(left), OperandStackItem::Boolean(right)) => {
                Some(left.cmp(right))
            }
            (OperandStackItem::Long(left), OperandStackItem::Long(right)) => Some(left.cmp(right)),
            (OperandStackItem::Classref(left), OperandStackItem::Classref(right)) => {
                Some(left.cmp(right))
            }
            (OperandStackItem::Fieldref(left), OperandStackItem::Fieldref(right)) => {
                Some(left.cmp(right))
            }
            (OperandStackItem::Objectref(left), OperandStackItem::Objectref(right)) => {
                Some(left.cmp(right))
            }
            (OperandStackItem::String(left), OperandStackItem::String(right)) => {
                Some(left.cmp(right))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct OperandStack {
    pub stack: Vec<OperandStackItem>,
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
                Some(OperandStackItem::Long(second_2)),
                Some(OperandStackItem::Long(second_1)),
                Some(OperandStackItem::Long(first_2)),
                Some(OperandStackItem::Long(first_1)),
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
            (Some(OperandStackItem::Int(second)), Some(OperandStackItem::Int(first))) => {
                (first, second)
            }
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn iadd(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_int_values();
        OperandStackItem::Int(first + second)
    }

    pub fn ladd(&mut self) -> (OperandStackItem, OperandStackItem) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first + second);
        (
            OperandStackItem::Long(first),
            OperandStackItem::Long(second),
        )
    }

    pub fn isub(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_int_values();
        OperandStackItem::Int(first - second)
    }

    pub fn lsub(&mut self) -> (OperandStackItem, OperandStackItem) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first + second);
        (
            OperandStackItem::Long(first),
            OperandStackItem::Long(second),
        )
    }

    pub fn imul(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_int_values();
        OperandStackItem::Int(first * second)
    }

    pub fn lmul(&mut self) -> (OperandStackItem, OperandStackItem) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first * second);
        (
            OperandStackItem::Long(first),
            OperandStackItem::Long(second),
        )
    }

    pub fn idiv(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_int_values();
        OperandStackItem::Int(first / second)
    }

    pub fn ldiv(&mut self) -> (OperandStackItem, OperandStackItem) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first / second);
        (
            OperandStackItem::Long(first),
            OperandStackItem::Long(second),
        )
    }

    pub fn irem(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_int_values();
        OperandStackItem::Int(first % second)
    }

    pub fn lrem(&mut self) -> (OperandStackItem, OperandStackItem) {
        let (first, second) = self.extract_long_values();
        let (first, second) = devide_i64_to_two_i32(first % second);
        (
            OperandStackItem::Long(first),
            OperandStackItem::Long(second),
        )
    }

    pub fn lcmp(&mut self) -> OperandStackItem {
        let (first, second) = self.extract_long_values();
        if first > second {
            OperandStackItem::Int(1)
        } else if first == second {
            OperandStackItem::Int(0)
        } else {
            OperandStackItem::Int(-1)
        }
    }
}

impl fmt::Display for OperandStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length = self.stack.len();
        let mut item_string_vec = Vec::with_capacity(length);
        for item in self.stack.iter() {
            item_string_vec.push(format!("{}", item));
        }

        write!(
            f,
            "length: {}
item:
{}",
            length,
            item_string_vec.join("\n")
        )
    }
}
