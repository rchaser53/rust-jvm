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
pub struct BuiltInMethod {
    pub name: String,
    pub descriptor: String,
    pub code_type: BuitlInCodeType,
    pub max_locals: usize,
}

impl BuiltInMethod {
    pub fn new(
        name: String,
        descriptor: String,
        code_type: BuitlInCodeType,
        max_locals: usize,
    ) -> BuiltInMethod {
        BuiltInMethod {
            name,
            descriptor,
            code_type,
            max_locals,
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
                let mut param_iter = stackframe.local_variables.iter();
                while let Some(item) = param_iter.next() {
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
                        _ => unimplemented!(),
                    };
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
}
