use crate::attribute::instruction::Instruction;
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::operand::{OperandStack, OperandStackItem};
use crate::stackframe::{Stackframe, StarckframeItem};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context<'a> {
    pub class_map: HashMap<&'a str, &'a ClassFile>,
    pub operand_stack: OperandStack,
    pub program_count: usize,
    pub stack_frames: Vec<Stackframe>,
}

impl<'a> Context<'a> {
    pub fn new(class_map: HashMap<&'a str, &'a ClassFile>) -> Context<'_> {
        Context {
            class_map,
            operand_stack: OperandStack::new(),
            program_count: 0,
            stack_frames: vec![],
        }
    }

    pub fn run_entry_file(&mut self, class_file: &ClassFile) {
        let entry_method = if let Some(entry_method) = class_file.get_entry_method() {
            entry_method
        } else {
            unimplemented!("add handler in the case failed to find entry method")
        };

        // TBD Perhaps this method is not invoked from super_class
        let super_class_index = class_file.super_class;
        let stack_frame_item_0 = StarckframeItem::Classref(super_class_index);
        self.run_method(class_file, entry_method, stack_frame_item_0);
    }

    pub fn run_method(
        &mut self,
        class_file: &ClassFile,
        method: &Method,
        stack_frame_item: StarckframeItem,
    ) {
        let mut stack_frame = Stackframe::new(0);
        stack_frame.local_variables.push(stack_frame_item);
        self.stack_frames.push(stack_frame);

        if let Some(code) = method.extract_code() {
            for instruction in code.code.iter() {
                println!("{}", instruction);
                let should_finish = self.execute(class_file, instruction);
                if should_finish {
                    break;
                }
            }
        }

        self.stack_frames.pop();
    }

    pub fn execute(&mut self, class_file: &ClassFile, instruction: &Instruction) -> bool {
        //     // let order = &self.orders[self.program_count];
        match instruction {
            Instruction::Iadd => {
                let item = self.operand_stack.iadd();
                self.operand_stack.stack.push(item);
            }
            Instruction::Isub => {
                let item = self.operand_stack.isub();
                self.operand_stack.stack.push(item);
            }
            Instruction::Imul => {
                let item = self.operand_stack.imul();
                self.operand_stack.stack.push(item);
            }
            Instruction::Idiv => {
                let item = self.operand_stack.idiv();
                self.operand_stack.stack.push(item);
            }
            Instruction::Irem => {
                let item = self.operand_stack.irem();
                self.operand_stack.stack.push(item);
            }
            Instruction::IconstN(val) => {
                self.operand_stack
                    .iconst(OperandStackItem::I32(*val as i32));
            }
            // maybe need to fix for float or something like that
            Instruction::Bipush(val) => {
                self.operand_stack
                    .iconst(OperandStackItem::I32(*val as i32));
            }
            Instruction::Ificmple(if_val, else_val) => {
                let left = self.operand_stack.stack.pop();
                let right = self.operand_stack.stack.pop();
                if left > right {
                    self.program_count = *if_val;
                } else {
                    self.program_count = *else_val;
                }
            }
            Instruction::IloadN(index) => {
                if let Some(stack_frame) = self.stack_frames.last() {
                    let value = &stack_frame.local_variables[*index];
                    self.operand_stack.stack.push(OperandStackItem::from(value));
                }
            }
            Instruction::IstoreN(index) => {
                if let Some(stack_frame) = self.stack_frames.last_mut() {
                    if let Some(item) = self.operand_stack.stack.pop() {
                        stack_frame.local_variables[*index] = StarckframeItem::from(item);
                    }
                }
            }
            Instruction::AloadN(index) => {
                if let Some(stack_frame) = self.stack_frames.last() {
                    let value = &stack_frame.local_variables[*index];
                    self.operand_stack.stack.push(OperandStackItem::from(value));
                }
            }
            Instruction::Getstatic(index) => {
                let item = class_file.cp_info.get_operand_stack_item(*index);
                self.operand_stack.stack.push(item);
            }
            Instruction::Ireturn => {
                if let Some(item) = self.operand_stack.stack.pop() {
                    self.operand_stack.stack.clear();
                    self.operand_stack.stack.push(item);
                } else {
                    unreachable!("should exist return value on operand_stack");
                }
                return true;
            }
            Instruction::Pop => {
                self.operand_stack.stack.pop();
            }
            _ => {}
        };
        false
    }

    // Instruction::Ldc(val) => write!(f, "ldc             #{}", val),
    // Instruction::AloadN(val) => write!(f, "aload_{}", val),
    // Instruction::Return => write!(f, "return"),
    // Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
    // Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
    // Instruction::Invokevirtual(val) => write!(f, "invokevirtual   #{}", val),
    // Instruction::Invokespecial(val) => write!(f, "invokespecial   #{}", val),
}

// #[derive(Debug)]
// pub struct ProgramContext {
//     pub orders: Vec<Order>,
//     pub operand_stack: OperandStack,
//     pub stack_frames: Vec<Stackframe>,
//     pub constant_pool: ConstantPool,
//     pub program_count: usize,
// }
// impl ProgramContext {
//     // pub fn new(orders: Vec<Order>) -> ProgramContext {
//     //     ProgramContext {
//     //         orders,
//     //         operand_stack: OperandStack::new(),
//     //         stack_frames: vec![],
//     //         // TBD
//     //         constant_pool: ConstantPool::new(&mut []),
//     //         program_count: 0,
//     //     }
//     // }

//     pub fn executes_programs(&mut self) {
//         let order_item_number = self.orders.len() - 1;
//         while order_item_number > self.program_count {
//             self.execute();
//             self.program_count += 1;
//         }
//     }

// }
