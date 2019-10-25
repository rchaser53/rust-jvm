use crate::stackframe::StackframeItem;
use std::cmp::{Ordering, PartialOrd};

#[derive(PartialEq, Clone, Debug)]
pub enum OperandStackItem {
    Null,
    Int(i32),
    Long(i64),
    String(String),
    Utf8(usize),
    Classref(usize),
    Fieldref(usize),
    Objectref(usize),
}

impl From<&StackframeItem> for OperandStackItem {
    fn from(item: &StackframeItem) -> OperandStackItem {
        match item {
            StackframeItem::Int(value) => OperandStackItem::Int(*value),
            StackframeItem::Long(value) => OperandStackItem::Long(*value),
            StackframeItem::String(value) => OperandStackItem::String(value.clone()),
            StackframeItem::Utf8(index) => OperandStackItem::Utf8(*index),
            StackframeItem::Classref(index) => OperandStackItem::Classref(*index),
            StackframeItem::Fieldref(index) => OperandStackItem::Fieldref(*index),
            StackframeItem::Objectref(index) => OperandStackItem::Objectref(*index),
            StackframeItem::Null => OperandStackItem::Null,
        }
    }
}

impl PartialOrd for OperandStackItem {
    fn partial_cmp(&self, other: &OperandStackItem) -> Option<Ordering> {
        match (self, other) {
            (OperandStackItem::Null, OperandStackItem::Null) => Some(Ordering::Equal),
            (OperandStackItem::Int(left), OperandStackItem::Int(right)) => Some(left.cmp(right)),
            (OperandStackItem::Long(left), OperandStackItem::Long(right)) => Some(left.cmp(right)),
            (OperandStackItem::Utf8(left), OperandStackItem::Utf8(right)) => Some(left.cmp(right)),
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

    pub fn iadd(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(second), Some(first)) => OperandStack::add_two_item(first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn ladd(&mut self) -> OperandStackItem {
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
                let second = (second_1 << 16 | second_2) & 0xFFFFFFFF;
                let first = (first_1 << 16 | first_2) & 0xFFFFFFFF;
                OperandStack::add_two_item(
                    OperandStackItem::Long(first),
                    OperandStackItem::Long(second),
                )
            }
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn isub(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(second), Some(first)) => OperandStack::sub_two_item(first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn imul(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(second), Some(first)) => OperandStack::mul_two_item(first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn idiv(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(second), Some(first)) => OperandStack::div_two_item(first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn irem(&mut self) -> OperandStackItem {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(second), Some(first)) => OperandStack::rem_two_item(first, second),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn add_two_item(first: OperandStackItem, second: OperandStackItem) -> OperandStackItem {
        match (&first, &second) {
            (OperandStackItem::Int(first), OperandStackItem::Int(second)) => {
                OperandStackItem::Int(first + second)
            }
            (OperandStackItem::Long(first), OperandStackItem::Long(second)) => {
                OperandStackItem::Long(first + second)
            }
            _ => panic!(
                "first:{:?} and second:{:?} types are not matched",
                first, second
            ),
        }
    }

    pub fn sub_two_item(first: OperandStackItem, second: OperandStackItem) -> OperandStackItem {
        match (&first, &second) {
            (OperandStackItem::Int(first), OperandStackItem::Int(second)) => {
                OperandStackItem::Int(first - second)
            }
            _ => panic!(
                "first:{:?} and second:{:?} types are not matched",
                first, second
            ),
        }
    }

    pub fn mul_two_item(first: OperandStackItem, second: OperandStackItem) -> OperandStackItem {
        match (&first, &second) {
            (OperandStackItem::Int(first), OperandStackItem::Int(second)) => {
                OperandStackItem::Int(first * second)
            }
            _ => panic!(
                "first:{:?} and second:{:?} types are not matched",
                first, second
            ),
        }
    }

    pub fn div_two_item(first: OperandStackItem, second: OperandStackItem) -> OperandStackItem {
        match (&first, &second) {
            (OperandStackItem::Int(first), OperandStackItem::Int(second)) => {
                OperandStackItem::Int(first / second)
            }
            _ => panic!(
                "first:{:?} and second:{:?} types are not matched",
                first, second
            ),
        }
    }

    pub fn rem_two_item(first: OperandStackItem, second: OperandStackItem) -> OperandStackItem {
        match (&first, &second) {
            (OperandStackItem::Int(first), OperandStackItem::Int(second)) => {
                OperandStackItem::Int(first % second)
            }
            _ => panic!(
                "first:{:?} and second:{:?} types are not matched",
                first, second
            ),
        }
    }
}
