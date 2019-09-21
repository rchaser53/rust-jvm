use std::collections::HashMap;
use std::fmt;
use std::ops::Fn;

#[derive(Debug)]
enum OperandStackItem {
    I32(i32),
}

#[derive(Debug)]
struct OperandStack {
    stack: Vec<OperandStackItem>,
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

    fn add_two_item(left: OperandStackItem, right: OperandStackItem) -> i32 {
        match (&left, &right) {
            (OperandStackItem::I32(left), OperandStackItem::I32(right)) => left + right,
            _ => panic!(
                "left:{:?} and right:{:?} types are not matched",
                left, right
            ),
        }
    }

    fn bipush(&mut self, item: OperandStackItem) {
        self.stack.push(item);
    }

    fn iconst(&mut self, item: OperandStackItem) {
        self.stack.push(item);
    }
}

fn convert_operand_stackframe(operand_stack_item: OperandStackItem) -> StarckframeItem {
    match operand_stack_item {
        OperandStackItem::I32(value) => StarckframeItem::I32(value),
    }
}

#[derive(Debug)]
enum StarckframeItem {
    I32(i32),
}

#[derive(Debug)]
struct Stackframe {
    local_variables: Vec<StarckframeItem>,
}

impl Stackframe {
    fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
        }
    }

    fn istore(&mut self, operand_stack: &mut OperandStack, index: usize) {
        if let Some(val) = operand_stack.stack.pop() {
            self.local_variables
                .insert(index, convert_operand_stackframe(val));
        }
    }
}

fn main() {
    let mut operand_stack = OperandStack::new();
    let mut stackframe = Stackframe::new(1);

    operand_stack.iconst(OperandStackItem::I32(1));
    stackframe.istore(&mut operand_stack, 0);

    dbg!(&stackframe);

    // operand_stack.bipush(OperandStackItem::I32(1));
    // operand_stack.bipush(OperandStackItem::I32(2));
    // let result = operand_stack.iadd();
}

/*
* 1 + 2;
*/
// bipush 1
// bipush 2
// iadd

/*
 *  int i;
 *  i = 0;
 */
//  iconst_0
//  istore_1
//
