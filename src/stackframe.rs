use crate::operand::{Item, OperandStack};

#[derive(Debug)]
pub struct Stackframe {
    pub local_variables: Vec<Item>,
    pub operand_stack: OperandStack,
}

impl Stackframe {
    pub fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
            operand_stack: OperandStack::new(),
        }
    }
}
