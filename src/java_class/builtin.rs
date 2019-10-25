use std::collections::HashMap;

use crate::constant::ConstantPool;
use crate::operand::OperandStackItem;
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

    pub fn parameter_length(&self, descriptor: &str) -> usize {
        match self.code_type {
            BuitlInCodeType::Println => match descriptor {
                "(J)V" | "(D)V" => 2,
                _ => 1,
            },
            BuitlInCodeType::JavaLangObjectInit => 1,
            BuitlInCodeType::JavaLangObjectToString => 1,
        }
    }

    pub fn execute(&mut self, constant_pool: &ConstantPool, stackframes: &mut Vec<Stackframe>) {
        let mut stackframe = stackframes.pop().expect("should has stack_frame");
        match self.code_type {
            BuitlInCodeType::Println => {
                if let Some(item) = stackframe.local_variables.get(0) {
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
                        StackframeItem::Long(second) => {
                            if let Some(StackframeItem::Long(first)) =
                                stackframe.local_variables.get(1)
                            {
                                println!("{}", (first << 16 | second) & 0xFFFFFFFF as i64);
                            } else {
                                unreachable!("should exist long second item")
                            }
                            let _ = stackframe.operand_stack.stack.pop();
                        }
                        _ => unimplemented!(),
                    };
                    let _ = stackframe.operand_stack.stack.pop();
                } else {
                    unreachable!("should have a argument for println")
                }
            }
            BuitlInCodeType::JavaLangObjectInit => {}
            BuitlInCodeType::JavaLangObjectToString => {
                let val = if let Some(StackframeItem::Int(val)) = stackframe.local_variables.get(0)
                {
                    val
                } else {
                    unreachable!("should have a argument for toString")
                };
                let stackframe = stackframes.last_mut().expect("should exist stackframe");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::String(format!("{}", val)));
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
    JavaLangObjectInit,
    JavaLangObjectToString,
}
