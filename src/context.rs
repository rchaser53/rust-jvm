use crate::attribute::instruction::Instruction;
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::operand::OperandStack;

#[derive(Debug)]
pub struct Context {
    pub operand_stack: OperandStack,
    pub program_count: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            operand_stack: OperandStack::new(),
            program_count: 0,
        }
    }

    pub fn run_entry_file(&mut self, class_file: &ClassFile) {
        let entry_method = if let Some(entry_method) = class_file.get_entry_method() {
            entry_method
        } else {
            unimplemented!("add handler in the case failed to find entry method")
        };

        self.run_method(entry_method);
    }

    pub fn run_method(&mut self, method: &Method) {
        if let Some(code) = method.extract_code() {
            for instruction in code.code.iter() {
                self.execute(instruction);
                println!("{}", instruction);
            }
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
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

            // Instruction::IconstN(val) => {
            //     self.operand_stack.iconst(order.operand);
            // }
            // Instruction::Ireturn => {
            // TODO: how should I handle this value?
            //     let _ = self.operand_stack.stack.pop();
            // }
            // Instruction::Ificmple => {
            //     let left = self.operand_stack.stack.pop();
            //     let right = self.operand_stack.stack.pop();
            //     if left > right {
            //         if let OperandStackItem::I32(val) = order.operand {
            //             self.program_count = val as usize;
            //         }
            //     }
            // }
            _ => unimplemented!(),
        };
    }

    // Instruction::Ldc(val) => write!(f, "ldc             #{}", val),
    // Instruction::IloadN(val) => write!(f, "iload_{}", val),
    // Instruction::AloadN(val) => write!(f, "aload_{}", val),
    // Instruction::IstoreN(val) => write!(f, "istore_{}", val),
    // Instruction::Return => write!(f, "return"),
    // Instruction::Getstatic(val) => write!(f, "getstatic     #{}", val),
    // Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
    // Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
    // Instruction::Invokevirtual(val) => write!(f, "invokevirtual   #{}", val),
    // Instruction::Invokespecial(val) => write!(f, "invokespecial   #{}", val),
}

// use crate::operand::{OperandStack, OperandStackItem};

// use crate::constant::ConstantPool;
// use crate::order::{Opecode, Order};
// use crate::stackframe::Stackframe;

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
