use std::collections::HashMap;
use std::fmt;
use std::ops::Fn;

#[derive(Debug)]
enum StackItem {
    I32(i32),
}

#[derive(Debug)]
struct OperandStack {
    stack: Vec<StackItem>,
}

impl OperandStack {
    fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    fn iadd(&mut self) -> i32 {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(left), Some(right)) => OperandStack::add_two_item(left, right),
            _ => panic!("shortage item in OperandStack"),
        }
    }

    fn add_two_item(left: StackItem, right: StackItem) -> i32 {
        match (&left, &right) {
            (StackItem::I32(left), StackItem::I32(right)) => left + right,
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    fn bipush(&mut self, item: StackItem) {
        self.stack.push(item);
    }
}

fn main() {
    let mut operand_stack = OperandStack::new();
    operand_stack.bipush(StackItem::I32(1));
    operand_stack.bipush(StackItem::I32(2));
    let result = operand_stack.iadd();
    dbg!(&result);
}
