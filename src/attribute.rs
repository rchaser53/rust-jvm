use crate::constant::{ConstPoolItem, ConstantPool};
use crate::utils::extract_x_byte_as_usize;

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
    StackMapTable,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
}

impl Attribute {
    pub fn new(
        constant_pool: &mut ConstantPool,
        inputs: &mut Vec<u8>,
        index: usize,
    ) -> (Attribute, usize) {
        let (attribute_name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        if let ConstPoolItem::ConstantUtf8(item) = &constant_pool.0[attribute_name_index] {
            let attribute_name_index = attribute_name_index as u16;

            let val = String::from_utf8_lossy(item.bytes.as_slice());
            match AttributeTag::from(val.into_owned()) {
                AttributeTag::SourceFile => {
                    let (attribute_length, index) = extract_x_byte_as_usize(inputs, index, 4);
                    let attribute_length = attribute_length as u32;

                    let (sourcefile_index, index) = extract_x_byte_as_usize(inputs, index, 2);
                    let sourcefile_index = sourcefile_index as u16;

                    let source_file = SourceFile {
                        attribute_name_index,
                        attribute_length,
                        sourcefile_index,
                    };
                    (Attribute::SourceFile(source_file), index)
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
    attribute_name_index: u16, // u2
    attribute_length: u32,     // u4
    sourcefile_index: u16,     // u2
}

#[derive(Debug)]
pub struct Code {
    attribute_name_index: u16, // u2
    attribute_length: u32,     // u4
    max_stack: u16,            // u2
    max_locals: u16,           // u2
    code_length: u32,          // u4
    code: Vec<Instruction>,
    exception_table_length: u16, // u2
    exception_table: Vec<ExceptionTableItem>,
    attributes_count: u16, // u2
    attribute_info: Vec<Attribute>,
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
    Aload(usize),           // 0x2a(0) - 0x2d(3)
    Ificmple(usize, usize), // A4
    Invokespecial(u16, u16),
    Getfield(u16, u16), // B4
    Iadd,               // 0x60
    Return,             // 0xac
    IloadN(usize),      // 0x1a(0) - 0x1d(3)
    IconstN(usize),     // 0x02(-1) - 0x08(5)
    IstoreN(usize),     // 0x3b(0) - 0x3e(3)
}

#[derive(Debug)]
pub struct LineNumberTable {
    pub attribute_name_index: u16,     // u2
    pub attribute_length: u32,         // u4
    pub line_number_table_length: u16, // u2
    pub line_number_tables: Vec<LineNumberTableItem>,
}

#[derive(Debug)]
pub struct LineNumberTableItem {
    pub start_pc: u16,    // u2
    pub line_number: u16, // u2
}

//  0: iload_1
//  1: iconst_2
//  2: if_icmple     12
//  5: aload_0
//  6: getfield      #2                  // Field x:I
//  9: iconst_2
// 10: iadd
// 11: istore_1
// 12: iload_1
// 13: ireturn
