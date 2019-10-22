use crate::utils::{extract_x_byte_as_usize, extract_x_byte_as_vec};
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    IconstN(usize),         // 0x02(-1) - 0x08(5)
    Bipush(usize),          // 0x10
    Ldc(usize),             // 0x12
    Ldc2W(usize, usize),    // 0x14
    IloadN(usize),          // 0x1a(0) - 0x1d(3)
    LloadN(usize),          // 0x1e(0) - 0x21(3)
    AloadN(usize),          // 0x2a(0) - 0x2d(3)
    IstoreN(usize),         // 0x3b(0) - 0x3e(3)
    LstoreN(usize),         // 0x3f(0) - 0x42(3)
    Pop,                    // 0x57
    Iadd,                   // 0x60
    Isub,                   // 0x64
    Imul,                   // 0x68
    Idiv,                   // 0x6C
    Irem,                   // 0x70
    Iinc(usize, usize),     // 0x84
    Ifeq(usize, usize),     // 0x99
    Ifne(usize, usize),     // 0x9a
    Iflt(usize, usize),     // 0x9b
    Ifge(usize, usize),     // 0x9c
    Ifgt(usize, usize),     // 0x9d
    Ifle(usize, usize),     // 0x9e
    Ificmpeq(usize, usize), // 0x9f
    Ificmpne(usize, usize), // 0xa0
    Ificmplt(usize, usize), // 0xa1
    Ificmpge(usize, usize), // 0xa2
    Ificmpgt(usize, usize), // 0xa3
    Ificmple(usize, usize), // 0xa4
    Goto(usize),            // 0xa7
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
            Instruction::Ldc2W(a, b) => write!(f, "ldc2_w         #{},{}", a, b),
            Instruction::IloadN(val) => write!(f, "iload_{}", val),
            Instruction::LloadN(val) => write!(f, "lload_{}", val),
            Instruction::AloadN(val) => write!(f, "aload_{}", val),
            Instruction::IstoreN(val) => write!(f, "istore_{}", val),
            Instruction::LstoreN(val) => write!(f, "lstore_{}", val),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Iinc(a, b) => write!(f, "iinc        {}, {}", a, b),
            Instruction::Ifeq(a, b) => write!(f, "if_eq       {}, {}", a, b),
            Instruction::Ifne(a, b) => write!(f, "if_ne       {}, {}", a, b),
            Instruction::Iflt(a, b) => write!(f, "if_lt       {}, {}", a, b),
            Instruction::Ifge(a, b) => write!(f, "if_ge       {}, {}", a, b),
            Instruction::Ifgt(a, b) => write!(f, "if_gt       {}, {}", a, b),
            Instruction::Ifle(a, b) => write!(f, "if_le       {}, {}", a, b),
            Instruction::Ificmpeq(a, b) => write!(f, "if_icmpeq   {}, {}", a, b),
            Instruction::Ificmpne(a, b) => write!(f, "if_icmpne   {}, {}", a, b),
            Instruction::Ificmplt(a, b) => write!(f, "if_icmplt   {}, {}", a, b),
            Instruction::Ificmpge(a, b) => write!(f, "if_icmpge   {}, {}", a, b),
            Instruction::Ificmpgt(a, b) => write!(f, "if_icmpgt   {}, {}", a, b),
            Instruction::Ificmple(a, b) => write!(f, "if_icmple   {}, {}", a, b),
            Instruction::Goto(val) => write!(f, "goto          {}", val),
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
            val @ 0x02..=0x08 => {
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
            // ldc2_w
            0x14 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                codes.push(Instruction::Ldc2W(val[0] as usize, val[1] as usize));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // iload_n
            val @ 0x1a..=0x1d => {
                codes.push(Instruction::IloadN(val - 0x1a));
                (index, 1)
            }
            // lload_n
            val @ 0x1e..=0x21 => {
                codes.push(Instruction::LloadN(val - 0x1e));
                (index, 1)
            }
            // aload_n
            val @ 0x2a..=0x2d => {
                codes.push(Instruction::AloadN(val - 0x2a));
                (index, 1)
            }
            // istore_n
            val @ 0x3b..=0x3e => {
                codes.push(Instruction::IstoreN(val - 0x3b));
                (index, 1)
            }
            val @ 0x3f..=0x42 => {
                codes.push(Instruction::LstoreN(val - 0x3f));
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
            // iinc
            0x84 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                codes.push(Instruction::Iinc(val[0] as usize, val[1] as usize));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifeq
            0x99 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifeq(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifne
            0x9a => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifne(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // iflt
            0x9b => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Iflt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifge
            0x9c => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifge(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifgt
            0x9d => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifgt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // ifle
            0x9e => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ifle(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpeq
            0x9f => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpeq(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpne
            0xa0 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpne(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmplt
            0xa1 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmplt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpge
            0xa2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpge(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmpgt
            0xa3 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmpgt(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // if_icmple
            0xa4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Ificmple(
                    (val + code_length - 1) & 0xffff,
                    code_length + 2,
                ));
                codes.push(Instruction::Noope);
                codes.push(Instruction::Noope);
                (index, 3)
            }
            // goto
            0xa7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                let code_length = codes.len();
                codes.push(Instruction::Goto((val + code_length - 1) & 0xFFFF));
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
            | Instruction::Iinc(_, _)
            | Instruction::Ldc2W(_, _)
            | Instruction::LstoreN(_)
            | Instruction::LloadN(_)
            | Instruction::Invokestatic(_) => 2,
            Instruction::IconstN(_)
            | Instruction::IstoreN(_)
            | Instruction::IloadN(_)
            | Instruction::Irem
            | Instruction::Return
            | Instruction::Iadd => 0,
            a => unimplemented!("{}", a),
        }
    }
}
