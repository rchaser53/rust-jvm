use crate::stackframe::StarckframeItem;
use std::cmp::{Ordering, PartialOrd};

#[derive(PartialEq, Clone, Debug)]
pub enum OperandStackItem {
    Null,
    I32(i32),
    String(String),
    Utf8(usize),
    Classref(usize),
    Fieldref(usize),
}

impl From<&StarckframeItem> for OperandStackItem {
    fn from(item: &StarckframeItem) -> OperandStackItem {
        match item {
            StarckframeItem::I32(value) => OperandStackItem::I32(*value),
            StarckframeItem::String(value) => OperandStackItem::String(value.clone()),
            StarckframeItem::Utf8(index) => OperandStackItem::Utf8(*index),
            StarckframeItem::Classref(index) => OperandStackItem::Classref(*index),
            StarckframeItem::Fieldref(index) => OperandStackItem::Fieldref(*index),
            StarckframeItem::Null => OperandStackItem::Null,
        }
    }
}

impl PartialOrd for OperandStackItem {
    fn partial_cmp(&self, other: &OperandStackItem) -> Option<Ordering> {
        match (self, other) {
            (OperandStackItem::Null, OperandStackItem::Null) => Some(Ordering::Equal),
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => Some(left.cmp(right)),
            (OperandStackItem::Utf8(left), OperandStackItem::Utf8(right)) => Some(left.cmp(right)),
            (OperandStackItem::Classref(left), OperandStackItem::Classref(right)) => {
                Some(left.cmp(right))
            }
            (OperandStackItem::Fieldref(left), OperandStackItem::Fieldref(right)) => {
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

    pub fn iadd(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::add_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn isub(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::sub_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn imul(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::mul_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn idiv(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::div_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn irem(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::rem_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn add_two_item(left: OperandStackItem, right: OperandStackItem) -> OperandStackItem {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => {
                OperandStackItem::I32(left + right)
            }
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    pub fn sub_two_item(left: OperandStackItem, right: OperandStackItem) -> OperandStackItem {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => {
                OperandStackItem::I32(left - right)
            }
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    pub fn mul_two_item(left: OperandStackItem, right: OperandStackItem) -> OperandStackItem {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => {
                OperandStackItem::I32(left * right)
            }
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    pub fn div_two_item(left: OperandStackItem, right: OperandStackItem) -> OperandStackItem {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => {
                OperandStackItem::I32(left / right)
            }
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    pub fn rem_two_item(left: OperandStackItem, right: OperandStackItem) -> OperandStackItem {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => {
                OperandStackItem::I32(left & right)
            }
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    pub fn bipush(&mut self, item: OperandStackItem) {
        self.stack.push(item);
    }

    pub fn iconst(&mut self, item: OperandStackItem) {
        self.stack.push(item);
    }
}
