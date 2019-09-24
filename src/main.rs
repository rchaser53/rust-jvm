use std::collections::HashMap;
use std::fmt;
use std::ops::Fn;

#[derive(Debug)]
pub enum OperandStackItem {
    Null,
    I32(i32),
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

pub fn convert_operand_stackframe(operand_stack_item: OperandStackItem) -> StarckframeItem {
    match operand_stack_item {
        OperandStackItem::I32(value) => StarckframeItem::I32(value),
        OperandStackItem::Null => StarckframeItem::Null,
    }
}

#[derive(Debug)]
pub enum StarckframeItem {
    Null,
    I32(i32),
}

#[derive(Debug)]
pub struct Stackframe {
    pub local_variables: Vec<StarckframeItem>,
}

impl Stackframe {
    pub fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
        }
    }

    pub fn istore(&mut self, operand_stack: &mut OperandStack, index: usize) {
        if let Some(val) = operand_stack.stack.pop() {
            self.local_variables
                .insert(index, convert_operand_stackframe(val));
        }
    }
}

#[derive(Debug)]
pub struct ConstantPool(Vec<StarckframeItem>);
impl ConstantPool {
    pub fn new() -> ConstantPool {
        ConstantPool(vec![StarckframeItem::Null])
    }
}

#[derive(Debug)]
pub struct ProgramContext {
    pub orders: Vec<Order>,
    pub operand_stack: OperandStack,
    pub stack_frames: Vec<Stackframe>,
}
impl ProgramContext {
    pub fn new(orders: Vec<Order>) -> ProgramContext {
        ProgramContext {
            orders,
            operand_stack: OperandStack::new(),
            stack_frames: vec![],
        }
    }

    pub fn executes_programs(&mut self) {
        self.orders.reverse();
        while let Some(order) = self.orders.pop() {
            self.execute(order);
        }
    }

    pub fn execute(&mut self, order: Order) {
        if let Order { opecode, operand } = order {
            match opecode {
                Opecode::Iadd => {
                    let val = self.operand_stack.iadd();
                    self.operand_stack.stack.push(OperandStackItem::I32(val));
                }
                Opecode::Iconst => {
                    self.operand_stack.iconst(operand);
                }
                Opecode::Ireturn => {
                    // TODO: how should I handle this value?
                    let _ = self.operand_stack.stack.pop();
                }
            };
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub opecode: Opecode,
    pub operand: OperandStackItem,
}
impl Order {
    pub fn new(opecode: Opecode, operand: OperandStackItem) -> Order {
        Order { opecode, operand }
    }
}

#[derive(Debug)]
pub enum Opecode {
    Iadd,
    Iconst,
    Ireturn,
}

fn main() {
    let mut program_context = ProgramContext::new(vec![
        Order::new(Opecode::Iconst, OperandStackItem::I32(1)),
        Order::new(Opecode::Iconst, OperandStackItem::I32(2)),
        Order::new(Opecode::Iadd, OperandStackItem::I32(2)),
    ]);
    program_context.executes_programs();

    // operand_stack.iconst(OperandStackItem::I32(1));
    // stackframe.istore(&mut operand_stack, 0);

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
