use crate::operand::Item;

#[derive(Debug)]
pub struct Order {
    pub opecode: Opecode,
    pub operand: Item,
}
impl Order {
    pub fn new(opecode: Opecode, operand: Item) -> Order {
        Order { opecode, operand }
    }
}

#[derive(Debug)]
pub enum Opecode {
    Iadd,
    Iconst,
    Ireturn,
    IfIcmple,
}
