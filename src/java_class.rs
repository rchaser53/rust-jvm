pub mod builtin;
pub mod custom;

#[derive(Debug)]
pub enum JavaClass<'a> {
    Custom(custom::Custom),
    BuiltIn(builtin::BuiltIn<'a>),
}
