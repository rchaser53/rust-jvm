use crate::operand::{OperandStack, OperandStackItem};

use crate::stackframe::{Stackframe, StarckframeItem};

use crate::order::{Opecode, Order};

#[derive(Debug)]
pub struct ProgramContext {
    pub orders: Vec<Order>,
    pub operand_stack: OperandStack,
    pub stack_frames: Vec<Stackframe>,
    pub constant_pool: ConstantPool,
    pub program_count: usize,
}
impl ProgramContext {
    pub fn new(orders: Vec<Order>) -> ProgramContext {
        ProgramContext {
            orders,
            operand_stack: OperandStack::new(),
            stack_frames: vec![],
            constant_pool: ConstantPool::new(),
            program_count: 0,
        }
    }

    pub fn executes_programs(&mut self) {
        let order_item_number = self.orders.len() - 1;
        while order_item_number > self.program_count {
            self.execute();
            self.program_count += 1;
        }
    }

    pub fn execute(&mut self) {
        let order = &self.orders[self.program_count];
        match order.opecode {
            Opecode::Iadd => {
                let val = self.operand_stack.iadd();
                self.operand_stack.stack.push(OperandStackItem::I32(val));
            }
            Opecode::Iconst => {
                self.operand_stack.iconst(order.operand);
            }
            Opecode::Ireturn => {
                // TODO: how should I handle this value?
                let _ = self.operand_stack.stack.pop();
            }
            Opecode::IfIcmple => {
                let left = self.operand_stack.stack.pop();
                let right = self.operand_stack.stack.pop();
                if left > right {
                    if let OperandStackItem::I32(val) = order.operand {
                        self.program_count = val as usize;
                    }
                }
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
