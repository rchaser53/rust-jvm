use crate::operand::Item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub type ArrayMap = HashMap<usize, Array>;

#[derive(Debug)]
pub enum PrimitiveArrayType {
    TBoolean = 4,
    TChar = 5,
    TFloat = 6,
    TDouble = 7,
    TByte = 8,
    TShort = 9,
    TInt = 10,
    TLong = 11,
}

#[derive(Clone, Debug)]
pub enum Array {
    Primitive(RefCell<Vec<(Item, Item)>>),
    Array(RefCell<Vec<usize>>),
    Custom(RefCell<Vec<usize>>),
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
