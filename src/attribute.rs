use crate::constant::{ConstPoolItem, ConstantPool};
use crate::utils::{extract_x_byte_as_usize, extract_x_byte_as_vec};
use std::fmt;

#[derive(Debug)]
pub enum Attribute {
    SourceFile(SourceFile),
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods,
    ConstantValue,
    Code(Code),
    Exceptions,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    MethodParameters,
    Synthetic,
    Deprecated,
    Signature,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    LineNumberTable(LineNumberTable),
    LocalVariableTable,
    LocalVariableTypeTable,
    StackMapTable(StackMapTable),
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl Attribute {
    pub fn new(
        constant_pool: &ConstantPool,
        inputs: &mut [u8],
        index: usize,
    ) -> (Attribute, usize) {
        let (attribute_name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        if let ConstPoolItem::ConstantUtf8(item) = &constant_pool.0[attribute_name_index] {
            let attribute_name_index = attribute_name_index as u16;

            let val = String::from_utf8_lossy(item.bytes.as_slice());
            match AttributeTag::from(val.into_owned()) {
                AttributeTag::SourceFile => {
                    let (item, index) = SourceFile::new(inputs, index, attribute_name_index);
                    (Attribute::SourceFile(item), index)
                }
                AttributeTag::LineNumberTable => {
                    let (item, index) = LineNumberTable::new(inputs, index, attribute_name_index);
                    (Attribute::LineNumberTable(item), index)
                }
                AttributeTag::StackMapTable => {
                    let (item, index) = StackMapTable::new(inputs, index, attribute_name_index);
                    (Attribute::StackMapTable(item), index)
                }
                AttributeTag::Code => {
                    let (item, index) =
                        Code::new(constant_pool, inputs, index, attribute_name_index);
                    (Attribute::Code(item), index)
                }
                _ => unimplemented!(),
            }
        } else {
            panic!(
                "{:?} is not ConstantUtf8",
                constant_pool.0[attribute_name_index]
            );
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::SourceFile(val) => write!(f, "{}", val),
            Attribute::Code(val) => write!(f, "{}", val),
            Attribute::LineNumberTable(val) => write!(f, "{}", val),
            Attribute::StackMapTable(val) => write!(f, "{}", val),
            _ => unimplemented!(),
        }
    }
}

// this is a custom enum for handling
#[derive(Debug)]
pub enum AttributeTag {
    SourceFile,
    InnerClasses,
    EnclosingMethod,
    SourceDebugExtension,
    BootstrapMethods,
    ConstantValue,
    Code,
    Exceptions,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    MethodParameters,
    Synthetic,
    Deprecated,
    Signature,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    StackMapTable,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl From<String> for AttributeTag {
    fn from(input: String) -> AttributeTag {
        match input.as_str() {
            "SourceFile" => AttributeTag::SourceFile,
            "InnerClasses" => AttributeTag::InnerClasses,
            "EnclosingMethod" => AttributeTag::EnclosingMethod,
            "SourceDebugExtension" => AttributeTag::SourceDebugExtension,
            "BootstrapMethods" => AttributeTag::BootstrapMethods,
            "ConstantValue" => AttributeTag::ConstantValue,
            "Code" => AttributeTag::Code,
            "Exceptions" => AttributeTag::Exceptions,
            "RuntimeVisibleParameterAnnotations" => {
                AttributeTag::RuntimeVisibleParameterAnnotations
            }
            "RuntimeInvisibleParameterAnnotations" => {
                AttributeTag::RuntimeInvisibleParameterAnnotations
            }
            "AnnotationDefault" => AttributeTag::AnnotationDefault,
            "MethodParameters" => AttributeTag::MethodParameters,
            "Synthetic" => AttributeTag::Synthetic,
            "Deprecated" => AttributeTag::Deprecated,
            "Signature" => AttributeTag::Signature,
            "RuntimeVisibleAnnotations" => AttributeTag::RuntimeVisibleAnnotations,
            "RuntimeInvisibleAnnotations" => AttributeTag::RuntimeInvisibleAnnotations,
            "LineNumberTable" => AttributeTag::LineNumberTable,
            "LocalVariableTable" => AttributeTag::LocalVariableTable,
            "LocalVariableTypeTable" => AttributeTag::LocalVariableTypeTable,
            "StackMapTable" => AttributeTag::StackMapTable,
            "RuntimeVisibleTypeAnnotations" => AttributeTag::RuntimeVisibleTypeAnnotations,
            "RuntimeInvisibleTypeAnnotations" => AttributeTag::RuntimeInvisibleTypeAnnotations,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub attribute_name_index: u16, // u2
    pub attribute_length: u32,     // u4
    pub sourcefile_index: u16,     // u2
}

impl SourceFile {
    pub fn new(inputs: &mut [u8], index: usize, attribute_name_index: u16) -> (SourceFile, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (sourcefile_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let sourcefile_index = sourcefile_index as u16;

        let source_file = SourceFile {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        };
        (source_file, index)
    }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SourceFile: #{}", self.sourcefile_index)
    }
}

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
            let (instruction, update_index, consume_index) =
                Instruction::new(inputs, update_index, tag);
            code_loop_index += consume_index;
            index = update_index;
            code.push(instruction);
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

#[derive(Debug)]
pub enum Instruction {
    IconstN(usize),         // 0x02(-1) - 0x08(5)
    Ldc(usize),             // 0x12
    IloadN(usize),          // 0x1a(0) - 0x1d(3)
    AloadN(usize),          // 0x2a(0) - 0x2d(3)
    IstoreN(usize),         // 0x3b(0) - 0x3e(3)
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
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::IconstN(val) => write!(f, "iconst_{}", val),
            Instruction::Ldc(val) => write!(f, "ldc             #{}", val),
            Instruction::IloadN(val) => write!(f, "iload_{}", val),
            Instruction::AloadN(val) => write!(f, "aload_{}", val),
            Instruction::IstoreN(val) => write!(f, "istore_{}", val),
            Instruction::Iadd => write!(f, "iadd"),
            Instruction::Isub => write!(f, "isub"),
            Instruction::Imul => write!(f, "imul"),
            Instruction::Idiv => write!(f, "idiv"),
            Instruction::Irem => write!(f, "irem"),
            Instruction::Ificmple(_, val) => write!(f, "if_icmple       {}", val),
            Instruction::Ireturn => write!(f, "ireturn"),
            Instruction::Return => write!(f, "return"),
            Instruction::Getstatic(val) => write!(f, "getstatic     #{}", val),
            Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
            Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
            Instruction::Invokevirtual(val) => write!(f, "invokevirtual   #{}", val),
            Instruction::Invokespecial(val) => write!(f, "invokespecial   #{}", val),
        }
    }
}

impl Instruction {
    pub fn new(inputs: &mut [u8], index: usize, tag: usize) -> (Instruction, usize, usize) {
        match tag {
            // aload_n
            val @ 0x2a..0x2d => (Instruction::AloadN(val - 0x2a), index, 1),
            // ldc
            0x12 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 1);
                (Instruction::Ldc(val), index, 2)
            }
            // if_icmple
            0xa4 => {
                let (val, index) = extract_x_byte_as_vec(inputs, index, 2);
                (
                    Instruction::Ificmple(val[0] as usize, val[1] as usize),
                    index,
                    3,
                )
            }
            // putfield
            0xb5 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Putfield(val), index, 3)
            }
            // invokevirtual
            0xb6 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Invokevirtual(val), index, 3)
            }
            // invokespecial
            0xb7 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Invokespecial(val), index, 3)
            }
            // getstatic
            0xb2 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Getstatic(val), index, 3)
            }
            // getfield
            0xb4 => {
                let (val, index) = extract_x_byte_as_usize(inputs, index, 2);
                (Instruction::Getfield(val), index, 3)
            }
            // iadd
            0x60 => (Instruction::Iadd, index, 1),
            // isub
            0x64 => (Instruction::Isub, index, 1),
            // imul
            0x68 => (Instruction::Imul, index, 1),
            // idiv
            0x6c => (Instruction::Idiv, index, 1),
            // irem
            0x70 => (Instruction::Irem, index, 1),
            // ireturn
            0xac => (Instruction::Ireturn, index, 1),
            // return
            0xb1 => (Instruction::Return, index, 1),
            // iload_n
            val @ 0x1a..0x1d => (Instruction::IloadN(val - 0x1a), index, 1),
            // iload_n
            val @ 0x02..0x08 => (Instruction::IconstN(val - 0x03), index, 1),
            // istore_n
            val @ 0x3b..0x3e => (Instruction::IstoreN(val - 0x3b), index, 1),
            _ => unimplemented!("tag: {:x}", tag),
        }
    }
}

#[derive(Debug)]
pub struct LineNumberTable {
    pub attribute_name_index: u16,       // u2
    pub attribute_length: u32,           // u4
    pub line_number_table_length: usize, // u2
    pub line_number_tables: Vec<LineNumberTableItem>,
}

impl LineNumberTable {
    pub fn new(
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (LineNumberTable, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (line_number_table_length, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let mut line_number_tables = Vec::with_capacity(line_number_table_length);

        for _ in 0..line_number_table_length {
            let (start_pc, update_index) = extract_x_byte_as_usize(inputs, index, 2);
            let start_pc = start_pc as u16;

            let (line_number, update_index) = extract_x_byte_as_usize(inputs, update_index, 2);
            let line_number = line_number as u16;

            line_number_tables.push(LineNumberTableItem {
                start_pc,
                line_number,
            });
            index = update_index;
        }

        (
            LineNumberTable {
                attribute_name_index,
                attribute_length,
                line_number_table_length,
                line_number_tables,
            },
            index,
        )
    }
}

impl fmt::Display for LineNumberTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table_strs = Vec::with_capacity(self.line_number_table_length);
        for item in self.line_number_tables.iter() {
            table_strs.push(format!("{}", item));
        }
        write!(
            f,
            "LineNumberTable:
  {}",
            table_strs.join("\n  ")
        )
    }
}

#[derive(Debug)]
pub struct LineNumberTableItem {
    pub start_pc: u16,    // u2
    pub line_number: u16, // u2
}

impl fmt::Display for LineNumberTableItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  line {}: {}", self.line_number, self.start_pc)
    }
}

#[derive(Debug)]
pub struct StackMapTable {
    attribute_name_index: u16, // u2
    attribute_length: u32,     // u4
    number_of_entries: usize,  // u2
    stack_map_frame: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(
        inputs: &mut [u8],
        index: usize,
        attribute_name_index: u16,
    ) -> (StackMapTable, usize) {
        let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
        let attribute_length = attribute_length as u32;

        let (number_of_entries, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let mut stack_map_frame = Vec::with_capacity(number_of_entries);

        for _ in 0..number_of_entries {
            let (frame, update_index) = StackMapFrame::new(inputs, index);
            stack_map_frame.push(frame);
            index = update_index;
        }
        (
            StackMapTable {
                attribute_name_index,
                attribute_length,
                number_of_entries,
                stack_map_frame,
            },
            index,
        )
    }
}

impl fmt::Display for StackMapTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack_map_frame_strs = Vec::with_capacity(self.number_of_entries);
        for item in self.stack_map_frame.iter() {
            stack_map_frame_strs.push(format!("frame_type = {}", item));
        }

        write!(
            f,
            "StackMapTable: number_of_entries = {}
  frame_type = {}",
            self.number_of_entries,
            stack_map_frame_strs.join("\n  ")
        )
    }
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(usize),
    SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
    ChopFrame,
    SameFrameExtended,
    AppendFrame,
    FullFrame,
}

impl StackMapFrame {
    pub fn new(inputs: &mut [u8], index: usize) -> (StackMapFrame, usize) {
        let (tag, index) = extract_x_byte_as_usize(inputs, index, 1);
        match tag {
            0..63 => (StackMapFrame::SameFrame(tag), index),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for StackMapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackMapFrame::SameFrame(val) => write!(f, "{}   /* same */", val),
            _ => unimplemented!(),
        }
    }
}
