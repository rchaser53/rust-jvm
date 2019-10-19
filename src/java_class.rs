pub mod builtin;
pub mod custom;

#[derive(Debug)]
pub enum JavaClass {
    BuiltIn(builtin::BuiltIn),
    Custom(custom::Custom),
}

impl JavaClass {
    pub fn this_class_name(&self) -> String {
        match self {
            JavaClass::BuiltIn(builtin) => builtin.class_name.clone(),
            JavaClass::Custom(custom) => custom.this_class_name(),
        }
    }
}
