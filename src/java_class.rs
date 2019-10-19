pub mod builtin;
pub mod custom;

#[derive(Debug)]
pub enum JavaClass {
    Custom(custom::Custom),
    BuiltIn(builtin::BuiltIn),
}
