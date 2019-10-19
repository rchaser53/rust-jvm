use std::collections::HashMap;

use crate::operand::{OperandStack, OperandStackItem};

#[derive(Debug)]
pub struct BuiltIn {
    pub class_name: String,
    pub methods: HashMap<String, BuiltInMethod>,
}

impl BuiltIn {
    pub fn new(class_name: String) -> BuiltIn {
        BuiltIn {
            class_name,
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct BuiltInMethod {
    pub name: String,
    pub descriptor_index: String,
    pub code: BuitlInCodeType,
    pub max_locals: usize,
}

impl BuiltInMethod {
    pub fn execute(&mut self, operand_stack: &mut OperandStack) {
        match self.code {
            BuitlInCodeType::Println => {
                println!("test!");
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
}
