use crate::attribute::defs::Attribute;
use crate::attribute::instruction::Instruction;
use crate::constant::ConstantPool;
use crate::utils::extract_x_byte_as_usize;
use std::fmt;

#[derive(Debug)]
pub struct Code {
    pub attribute_name_index: u16, // u2
    pub attribute_length: u32,     // u4
    pub max_stack: u16,            // u2
    pub max_locals: u16,           // u2
    pub code_length: usize,        // u4
    pub code: Vec<Instruction>,
    pub exception_table_length: usize, // u2
    pub exception_table: Vec<ExceptionTableItem>,
    pub attributes_count: usize, // u2
    pub attribute_info: Vec<Attribute>,
}

impl Code {
    pub fn new(
        constant_pool: &ConstantPool,
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (Code, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (max_stack, index) = extract_x_byte_as_usize(inputs, index, 2);
        let max_stack = max_stack as u16;

        let (max_locals, index) = extract_x_byte_as_usize(inputs, index, 2);
        let max_locals = max_locals as u16;

        let (code_length, mut index) = extract_x_byte_as_usize(inputs, index, 4);
        let mut code = Vec::with_capacity(code_length);
        let mut code_loop_index = 0;

        while code_length - code_loop_index > 0 {
            let (tag, update_index) = extract_x_byte_as_usize(inputs, index, 1);
            let (update_index, consume_index) =
                Instruction::create_and_push(&mut code, inputs, update_index, tag);
            code_loop_index += consume_index;
            index = update_index;
        }

        let (exception_table_length, index) = extract_x_byte_as_usize(inputs, index, 2);
        let exception_table = Vec::with_capacity(exception_table_length);

        let (attributes_count, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let mut attribute_info = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let (attribute, update_index) = Attribute::new(constant_pool, inputs, index);
            index = update_index;
            attribute_info.push(attribute);
        }

        (
            Code {
                attribute_name_index,
                attribute_length,
                max_stack,
                max_locals,
                code_length,
                code,
                exception_table_length,
                exception_table,
                attributes_count,
                attribute_info,
            },
            index,
        )
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut code_strs = Vec::with_capacity(self.code_length);
        for (index, code) in self.code.iter().enumerate() {
            if let Instruction::Noope = *code {
                continue;
            }
            code_strs.push(format!("{}: {}", index, code));
        }
        let mut attribute_strs = Vec::with_capacity(self.attributes_count);
        for item in self.attribute_info.iter() {
            attribute_strs.push(format!("{}", item));
        }

        write!(
            f,
            "Code:
  stack:{}, locals={}, args_size=?
    {}
  {}",
            self.max_stack,
            self.max_locals,
            code_strs.join("\n    "),
            attribute_strs.join("\n  "),
        )
    }
}

#[derive(Debug)]
pub struct ExceptionTableItem {
    pub start_pc: u16,   //u2
    pub end_pc: u16,     //u2
    pub handler_pc: u16, //u2
    pub catch_type: u16, //u2
}
