// use crate::utils::devide_u64_to_two_u32;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

pub fn devide_i64_two_usize(input: i64) -> (usize, usize) {
    let high_value = ((input >> 32) << 32) as usize;
    let low_value = (input & 0xFFFFFFFF) as usize;
    if input > 0 {
        (low_value, high_value)
    } else {
        (0xFFFFFFFF - low_value, 0xFFFFFFFF - high_value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Item {
    Null,
    Int(i32),
    Long(usize),
    Float(f32),
    String(usize),
    Boolean(bool),
    Classref(usize),
    Fieldref(usize),
    Objectref(usize),
    Arrayref(usize),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Null => write!(f, "null"),
            Item::Int(val) => write!(f, "int: {}", val),
            Item::Long(val) => write!(f, "long: {}", val),
            Item::Float(val) => write!(f, "float: {}", val),
            Item::Boolean(val) => write!(f, "boolean: {}", val),
            Item::String(val) => write!(f, "string: {}", val),
            Item::Classref(val) => write!(f, "class_ref: {}", val),
            Item::Fieldref(val) => write!(f, "field_ref: {}", val),
            Item::Objectref(val) => write!(f, "object_ref: {}", val),
            Item::Arrayref(val) => write!(f, "array_ref {}", val),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        match (self, other) {
            (Item::Null, Item::Null) => Some(Ordering::Equal),
            (Item::Int(left), Item::Int(right)) => Some(left.cmp(right)),
            (Item::Float(left), Item::Float(right)) => Some({
                if left > right {
                    Ordering::Greater
                } else if left == right {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }),
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

    fn extract_long_values_as_i64(&mut self) -> (i64, i64) {
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
                let second: u64 = (((second_1 as u64) << 32) as u64) | second_2 as u64;
                let second = if second > 0x7fffffffffffffff {
                    -1 * ((second ^ 0xffffffffffffffff) + 1) as i64
                } else {
                    second as i64
                };
                let first: u64 = (((first_1 as u64) << 32) as u64) | first_2 as u64;
                let first = if first > 0x7fffffffffffffff {
                    -1 * ((first ^ 0xffffffffffffffff) + 1) as i64
                } else {
                    first as i64
                };
                (first, second)
            }
            (second_2, second_1, first_2, first_1) => panic!(
                "failed to extract long values
first: {:?}, {:?}
second: {:?}, {:?}",
                first_1, first_2, second_1, second_2
            ),
        }
    }

    fn extract_int_values(&mut self) -> (i32, i32) {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Int(second)), Some(Item::Int(first))) => (first, second),
            (second, first) => panic!(
                "failed to extract int values
first: {:?}
second: {:?}",
                second, first
            ),
        }
    }

    fn extract_float_values(&mut self) -> (f32, f32) {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Item::Float(second)), Some(Item::Float(first))) => (first, second),
            (second, first) => panic!(
                "failed to extract float values
first: {:?}
second: {:?}",
                second, first
            ),
        }
    }

    pub fn iadd(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first + second)
    }

    pub fn fadd(&mut self) -> Item {
        let (first, second) = self.extract_float_values();
        Item::Float(first + second)
    }

    pub fn ladd(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values_as_i64();
        let (first, second) = devide_i64_two_usize(first + second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn isub(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first - second)
    }

    pub fn fsub(&mut self) -> Item {
        let (first, second) = self.extract_float_values();
        Item::Float(first - second)
    }

    pub fn lsub(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values_as_i64();
        let (first, second) = devide_i64_two_usize(first + second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn imul(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first * second)
    }

    pub fn fmul(&mut self) -> Item {
        let (first, second) = self.extract_float_values();
        Item::Float(first * second)
    }

    pub fn lmul(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values_as_i64();
        let (first, second) = devide_i64_two_usize(first * second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn idiv(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first / second)
    }

    pub fn fdiv(&mut self) -> Item {
        let (first, second) = self.extract_float_values();
        Item::Float(first / second)
    }

    pub fn ldiv(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values_as_i64();
        let (first, second) = devide_i64_two_usize(first / second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn irem(&mut self) -> Item {
        let (first, second) = self.extract_int_values();
        Item::Int(first % second)
    }

    pub fn lrem(&mut self) -> (Item, Item) {
        let (first, second) = self.extract_long_values_as_i64();
        let (first, second) = devide_i64_two_usize(first % second);
        (Item::Long(first), Item::Long(second))
    }

    pub fn lcmp(&mut self) -> Item {
        let (first, second) = self.extract_long_values_as_i64();
        self.compare_value(first, second)
    }

    pub fn fcmp(&mut self) -> Item {
        let (first, second) = self.extract_float_values();
        self.compare_value(first, second)
    }

    fn compare_value<T>(&self, first: T, second: T) -> Item
    where
        T: PartialOrd,
    {
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
