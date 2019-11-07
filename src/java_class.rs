pub mod builtin;
pub mod custom;
pub mod default;

#[derive(Debug)]
pub enum JavaClass {
    BuiltIn(builtin::BuiltIn),
    Custom(custom::Custom),
}

impl JavaClass {
    pub fn this_class_name(&self) -> usize {
        match self {
            JavaClass::BuiltIn(builtin) => builtin.class_name,
            JavaClass::Custom(custom) => custom.this_class_name(),
        }
    }
}
