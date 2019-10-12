use crate::attribute::code::Code;
use crate::constant::{ConstPoolItem, ConstantPool};
use crate::utils::extract_x_byte_as_usize;
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
