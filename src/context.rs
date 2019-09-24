use crate::operand::{OperandStack, OperandStackItem};

use crate::stackframe::{Stackframe, StarckframeItem};

use crate::order::{Opecode, Order};

#[derive(Debug)]
pub struct ProgramContext {
    pub orders: Vec<Order>,
    pub operand_stack: OperandStack,
    pub stack_frames: Vec<Stackframe>,
    pub constant_pool: ConstantPool,
}
impl ProgramContext {
    pub fn new(orders: Vec<Order>) -> ProgramContext {
        ProgramContext {
            orders,
            operand_stack: OperandStack::new(),
            stack_frames: vec![],
            constant_pool: ConstantPool::new(),
        }
    }

    pub fn executes_programs(&mut self) {
        self.orders.reverse();
        while let Some(order) = self.orders.pop() {
            self.execute(order);
        }
    }

    pub fn execute(&mut self, order: Order) {
        let Order { opecode, operand } = order;
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

#[derive(Debug)]
pub struct ConstantPool(Vec<StarckframeItem>);
impl ConstantPool {
    pub fn new() -> ConstantPool {
        ConstantPool(vec![StarckframeItem::Null])
    }
}
