use std::collections::HashMap;

use crate::constant::ConstantPool;
use crate::operand::OperandStack;
use crate::stackframe::{Stackframe, StackframeItem};

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
pub enum BuiltInLocal {
    Println,
}

#[derive(Debug)]
pub struct BuiltInMethod {
    pub name: String,
    pub code_type: BuitlInCodeType,
}

impl BuiltInMethod {
    pub fn new(name: String, code_type: BuitlInCodeType) -> BuiltInMethod {
        BuiltInMethod { name, code_type }
    }

    pub fn max_locals(&self, descriptor: &str) -> usize {
        match self.code_type {
            BuitlInCodeType::Println => match descriptor {
                "(J)V" | "(D)V" => 2,
                _ => 1,
            },
        }
    }

    pub fn execute(
        &mut self,
        constant_pool: &ConstantPool,
        stackframe: &mut Stackframe,
        _operand_stack: &mut OperandStack,
    ) {
        match self.code_type {
            BuitlInCodeType::Println => {
                if let Some(item) = stackframe.local_variables.get(1) {
                    match item {
                        StackframeItem::Fieldref(index) => {
                            println!("{}", constant_pool.get_fieldref_as_utf8(*index));
                        }
                        StackframeItem::String(value) => {
                            println!("{}", value);
                        }
                        StackframeItem::Int(value) => {
                            println!("{}", value);
                        }
                        StackframeItem::Long(first) => {
                            if let Some(StackframeItem::Long(second)) =
                                stackframe.local_variables.get(2)
                            {
                                println!("{}", (first << 8 | second) & 0xFFFF as i64);
                            } else {
                                unreachable!("should exist long second item")
                            }
                        }
                        _ => unimplemented!(),
                    };
                } else {
                    unreachable!("should have a argument for println")
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
}
