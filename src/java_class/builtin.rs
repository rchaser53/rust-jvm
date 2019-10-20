use std::collections::HashMap;

use crate::constant::ConstantPool;
use crate::operand::OperandStack;
use crate::stackframe::Stackframe;

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
        operand_stack: &mut OperandStack,
    ) {
        match self.code_type {
            BuitlInCodeType::Println => {
                if let Some(item) = stackframe.local_variables.last() {
                    println!("{:?}", item);
                } else {
                    unreachable!("should exits some parameter to {:?}", self)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum BuitlInCodeType {
    Println,
}
