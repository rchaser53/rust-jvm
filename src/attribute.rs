#[derive(Debug)]
pub enum Attribute {
    SourceFile(SourceFile),
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
