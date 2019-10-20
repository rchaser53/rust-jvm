use crate::utils::{extract_x_byte_as_usize, extract_x_byte_as_vec};
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    IconstN(usize),         // 0x02(-1) - 0x08(5)
    Bipush(usize),          // 0x10
    Ldc(usize),             // 0x12
    IloadN(usize),          // 0x1a(0) - 0x1d(3)
    AloadN(usize),          // 0x2a(0) - 0x2d(3)
    IstoreN(usize),         // 0x3b(0) - 0x3e(3)
    Pop,                    // 0x57
    Iadd,                   // 0x60
    Isub,                   // 0x64
    Imul,                   // 0x68
    Idiv,                   // 0x6C
    Irem,                   // 0x70
    Ificmple(usize, usize), // 0xa4
    Ireturn,                // 0xac
    Return,                 // 0xb1
    Getstatic(usize),       // 0xb2
    Getfield(usize),        // 0xb4
    Putfield(usize),        // 0xb5
    Invokevirtual(usize),   // 0xb6
    Invokespecial(usize),   // 0xb7
    Invokestatic(usize),    // 0xb8
    Noope,                  // custom command for Ificmple etc.
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::IconstN(val) => write!(f, "iconst_{}", val),
            Instruction::Bipush(val) => write!(f, "bipush         {}", val),
            Instruction::Ldc(val) => write!(f, "ldc             #{}", val),
            Instruction::IloadN(val) => write!(f, "iload_{}", val),
            Instruction::AloadN(val) => write!(f, "aload_{}", val),
            Instruction::IstoreN(val) => write!(f, "istore_{}", val),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Ificmple(_, val) => write!(f, "if_icmple       {}", val),
            Instruction::Ireturn => write!(f, "ireturn"),
            Instruction::Return => write!(f, "return"),
            Instruction::Getstatic(val) => write!(f, "getstatic       #{}", val),
            Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
            Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
            Instruction::Invokevirtual(val) => write!(f, "invokevirtual   #{}", val),
            Instruction::Invokespecial(val) => write!(f, "invokespecial   #{}", val),
            Instruction::Invokestatic(val) => write!(f, "invokestatic   #{}", val),
            Instruction::Noope => write!(f, "noope"),
        }
    }
}

impl Instruction {
    pub fn create_and_push(
        codes: &mut Vec<Instruction>,
        inputs: &mut [u8],
        index: usize,
        tag: usize,
    ) -> (usize, usize) {
        match tag {
            // iconst_n
            val @ 0x02..0x08 => {
                codes.push(Instruction::IconstN(val - 0x03));
                (index, 1)
            }
            // bipush
            0x10 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Bipush(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // ldc
            0x12 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                codes.push(Instruction::Ldc(val));
                codes.push(Instruction::Noope);
                (index, 2)
            }
            // iload_n
            val @ 0x1a..0x1d => {
                codes.push(Instruction::IloadN(val - 0x1a));
                (index, 1)
            }
            // aload_n
            val @ 0x2a..0x2d => {
                codes.push(Instruction::AloadN(val - 0x2a));
                (index, 1)
            }
            // istore_n
            val @ 0x3b..0x3e => {
                codes.push(Instruction::IstoreN(val - 0x3b));
                (index, 1)
            }
            // pop
            0x57 => {
                codes.push(Instruction::Pop);
                (index, 1)
            }
            // iadd
            0x60 => {
                codes.push(Instruction::Iadd);
                (index, 1)
            }
            // isub
            0x64 => {
                codes.push(Instruction::Isub);
                (index, 1)
            }
            // imul
            0x68 => {
                codes.push(Instruction::Imul);
                (index, 1)
            }
            // idiv
            0x6c => {
                codes.push(Instruction::Idiv);
                (index, 1)
            }
            // irem
            0x70 => {
                codes.push(Instruction::Irem);
                (index, 1)
            }
            // if_icmple
            0xa4 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                codes.push(Instruction::Ificmple(val[0] as usize, val[1] as usize));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ireturn
            0xac => {
                codes.push(Instruction::Ireturn);
                (index, 1)
            }
            // return
            0xb1 => {
                codes.push(Instruction::Return);
                (index, 1)
            }
            // getstatic
            0xb2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Getstatic(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // getfield
            0xb4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Getfield(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // putfield
            0xb5 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Putfield(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokevirtual
            0xb6 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokevirtual(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokespecial
            0xb7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokespecial(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // invokestatic
            0xb8 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                codes.push(Instruction::Invokestatic(val));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            _ => unimplemented!("tag: {:x}", tag),
        }
    }

    pub fn counsume_index(&self) -> usize {
        match self {
            Instruction::Bipush(_) | Instruction::Ldc(_) => 1,
            Instruction::Ificmple(_, _)
            | Instruction::Getstatic(_)
            | Instruction::Getfield(_)
            | Instruction::Putfield(_)
            | Instruction::Invokevirtual(_)
            | Instruction::Invokespecial(_)
            | Instruction::Invokestatic(_) => 2,
            _ => 0,
        }
    }
}
