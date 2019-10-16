use std::collections::HashMap;

#[derive(Debug)]
pub struct BuiltIn<'a> {
    pub class_name: &'a str,
    pub methods: HashMap<&'a str, BuiltInMethod>,
}

#[derive(Debug)]
pub struct BuiltInMethod;
