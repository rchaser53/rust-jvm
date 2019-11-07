use crate::attribute::code::Code;
use crate::constant::{ConstPoolItem, ConstantPool};
use crate::utils::{extract_x_byte_as_usize, get_string_from_string_pool};

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

            match AttributeTag::from(get_string_from_string_pool(&item.id)) {
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
    SameFrame(SameFrame),
    SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
    ChopFrame(ChopFrame),
    SameFrameExtended,
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
pub struct SameFrame {
    frame_type: usize,
}

#[derive(Debug)]
pub struct ChopFrame {
    frame_type: usize,
    offset_delta: usize,
}

#[derive(Debug)]
pub struct AppendFrame {
    frame_type: usize,
    offset_delta: usize,
    locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct FullFrame {
    frame_type: usize,
    offset_delta: usize,               // u2
    number_of_locals: usize,           // u2
    locals: Vec<VerificationTypeInfo>, // locals[number_of_locals]
    number_of_stack_items: usize,      // u2
    stack: Vec<VerificationTypeInfo>,  // stack[number_of_stack_items]
}

impl StackMapFrame {
    pub fn new(inputs: &mut [u8], index: usize) -> (StackMapFrame, usize) {
        let (frame_type, index) = extract_x_byte_as_usize(inputs, index, 1);
        match frame_type {
            0..=63 => (StackMapFrame::SameFrame(SameFrame { frame_type }), index),
            248..=250 => {
                let (offset_delta, index) = extract_x_byte_as_usize(inputs, index, 2);
                (
                    StackMapFrame::ChopFrame(ChopFrame {
                        frame_type,
                        offset_delta,
                    }),
                    index,
                )
            }
            252..=254 => {
                let (offset_delta, index) = extract_x_byte_as_usize(inputs, index, 2);
                let length = (frame_type as i32) - 251;
                let (locals, index) = if length > 0 {
                    extract_verification_type_info(inputs, index, length as usize)
                } else {
                    (vec![], index)
                };

                (
                    StackMapFrame::AppendFrame(AppendFrame {
                        frame_type,
                        offset_delta,
                        locals,
                    }),
                    index,
                )
            }
            255 => {
                let (offset_delta, index) = extract_x_byte_as_usize(inputs, index, 2);
                let (number_of_locals, index) = extract_x_byte_as_usize(inputs, index, 2);
                let (locals, index) =
                    extract_verification_type_info(inputs, index, number_of_locals);
                let (number_of_stack_items, index) = extract_x_byte_as_usize(inputs, index, 2);
                let (stack, index) =
                    extract_verification_type_info(inputs, index, number_of_stack_items);

                (
                    StackMapFrame::FullFrame(FullFrame {
                        frame_type,
                        offset_delta,
                        number_of_locals,
                        locals,
                        number_of_stack_items,
                        stack,
                    }),
                    index,
                )
            }
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for StackMapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackMapFrame::SameFrame(SameFrame { frame_type }) => {
                write!(f, "{}   /* same */", frame_type)
            }
            StackMapFrame::ChopFrame(ChopFrame {
                frame_type,
                offset_delta,
            }) => write!(
                f,
                "{}   /* chop */
  offset_delta = {}",
                frame_type, offset_delta
            ),
            StackMapFrame::AppendFrame(AppendFrame {
                frame_type,
                offset_delta,
                locals,
            }) => write!(
                f,
                "{}   /* append */
    offset_delta = {}
    locals = [{}]",
                frame_type,
                offset_delta,
                locals
                    .iter()
                    .map(|local| format!("{}", local))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            StackMapFrame::FullFrame(FullFrame {
                frame_type,
                offset_delta,
                locals,
                stack,
                ..
            }) => write!(
                f,
                "{}   /* full_frame */
    offset_delta = {}
    locals = [{}]
    stack = [{}]",
                frame_type,
                offset_delta,
                format!(
                    "{}",
                    locals
                        .iter()
                        .map(|local| format!("{}", local))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                format!(
                    "{}",
                    stack
                        .iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            ),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo,                  // 0
    IntegerVariableInfo,              // 1
    FloatVariableInfo,                // 2
    DoubleVariableInfo,               // 3
    LongVariableInfo,                 // 4
    NullVariableInfo,                 // 5
    UninitializedThisVariableInfo,    // 6
    ObjectVariableInfo(usize),        // 7, u2(cpool_index)
    UninitializedVariableInfo(usize), // 8, u2(offset)
}

impl fmt::Display for VerificationTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerificationTypeInfo::TopVariableInfo => write!(f, "top"),
            VerificationTypeInfo::IntegerVariableInfo => write!(f, "int"),
            VerificationTypeInfo::FloatVariableInfo => write!(f, "float"),
            VerificationTypeInfo::DoubleVariableInfo => write!(f, "double"),
            VerificationTypeInfo::LongVariableInfo => write!(f, "long"),
            VerificationTypeInfo::NullVariableInfo => write!(f, "null"),
            VerificationTypeInfo::UninitializedThisVariableInfo => write!(f, "uninitialized_this"),
            VerificationTypeInfo::ObjectVariableInfo(index) => {
                write!(f, "object_variable: #{}", index)
            }
            VerificationTypeInfo::UninitializedVariableInfo(index) => {
                write!(f, "uninitialized_variable: #{}", index)
            }
        }
    }
}

pub fn extract_verification_type_info(
    inputs: &mut [u8],
    original_index: usize,
    length: usize,
) -> (Vec<VerificationTypeInfo>, usize) {
    let mut index = original_index;
    let mut result = Vec::with_capacity(length);
    for _ in 0..length {
        let (tag, update_index) = extract_x_byte_as_usize(inputs, index, 1);
        let (type_info, update_index) = match tag {
            0 => (VerificationTypeInfo::TopVariableInfo, update_index),
            1 => (VerificationTypeInfo::IntegerVariableInfo, update_index),
            2 => (VerificationTypeInfo::FloatVariableInfo, update_index),
            3 => (VerificationTypeInfo::DoubleVariableInfo, update_index),
            4 => (VerificationTypeInfo::LongVariableInfo, update_index),
            5 => (VerificationTypeInfo::NullVariableInfo, update_index),
            6 => (
                VerificationTypeInfo::UninitializedThisVariableInfo,
                update_index,
            ),
            7 => {
                let (cpool_index, update_index) = extract_x_byte_as_usize(inputs, update_index, 2);
                (
                    VerificationTypeInfo::ObjectVariableInfo(cpool_index),
                    update_index,
                )
            }
            8 => {
                let (offset, update_index) = extract_x_byte_as_usize(inputs, update_index, 2);
                (
                    VerificationTypeInfo::UninitializedVariableInfo(offset),
                    update_index,
                )
            }
            _ => unreachable!(
                "should be below 8 for verification_type_info. actual {}",
                tag
            ),
        };
        result.push(type_info);
        index = update_index;
    }
    (result, index)
}
