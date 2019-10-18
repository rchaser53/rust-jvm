use std::collections::HashMap;
use std::fmt;

use crate::operand::{OperandStack, OperandStackItem};

#[derive(Debug)]
pub struct BuiltIn<'a> {
    pub class_name: &'a str,
    pub methods: HashMap<&'a str, BuiltInMethod>,
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
