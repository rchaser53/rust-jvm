use crate::operand::OperandStackItem;

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
