use std::cmp::{Ordering, PartialOrd};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum OperandStackItem {
    Null,
    I32(i32),
}

impl PartialOrd for OperandStackItem {
    fn partial_cmp(&self, other: &OperandStackItem) -> Option<Ordering> {
        match (self, other) {
            (OperandStackItem::Null, OperandStackItem::Null) => Some(Ordering::Equal),
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => Some(left.cmp(right)),
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

    pub fn iadd(&mut self) -> i32 {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::add_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    pub fn add_two_item(left: OperandStackItem, right: OperandStackItem) -> i32 {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => left + right,
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
