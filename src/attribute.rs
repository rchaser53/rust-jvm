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
    Aload(usize),
    Invokespecial(u16, u16),
    Return,
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
