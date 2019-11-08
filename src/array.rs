use crate::operand::Item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct ArrayMap {
    pub id: usize,
    pub map: HashMap<usize, Array>,
}
impl ArrayMap {
    pub fn new() -> ArrayMap {
        ArrayMap {
            id: 0,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: Array) -> usize {
        let id = self.id;
        self.id += 1;
        self.map.insert(id, value);
        id
    }

    pub fn get(&self, id: &usize) -> Option<&Array> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: &usize) -> Option<&mut Array> {
        self.map.get_mut(id)
    }
}

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
