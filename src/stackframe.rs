use crate::operand::{OperandStack, OperandStackItem};

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

pub fn convert_operand_stackframe(operand_stack_item: OperandStackItem) -> StarckframeItem {
    match operand_stack_item {
        OperandStackItem::I32(value) => StarckframeItem::I32(value),
        OperandStackItem::Null => StarckframeItem::Null,
    }
}
