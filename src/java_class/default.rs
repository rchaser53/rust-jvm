use crate::java_class::{
    builtin::{BuiltIn, BuiltInMethod, BuitlInCodeType},
    JavaClass,
};
use std::collections::HashMap;

pub fn setup_class_map() -> HashMap<String, JavaClass> {
    let mut class_map = HashMap::new();
    let (print_stream_name, print_stream) = create_print_stream();
    let (java_lang_object_name, java_lang_object) = create_java_lang_object();
    let (java_lang_integer_name, java_lang_integer) = create_java_lang_integer();
    let (java_lang_system_name, java_lang_system) = create_java_lang_system();

    class_map.insert(print_stream_name, print_stream);
    class_map.insert(java_lang_object_name, java_lang_object);
    class_map.insert(java_lang_integer_name, java_lang_integer);
    class_map.insert(java_lang_system_name, java_lang_system);
    class_map
}

fn create_print_stream() -> (String, JavaClass) {
    let print_stream_name = String::from("java/io/PrintStream");
    let mut print_stream = BuiltIn::new(print_stream_name.clone());
    let println_name = String::from("println");
    let println = BuiltInMethod::new(println_name.clone(), BuitlInCodeType::Println);
    print_stream.methods.insert(println_name, println);
    (print_stream_name, JavaClass::BuiltIn(print_stream))
}

fn create_java_lang_object() -> (String, JavaClass) {
    let java_lang_object_name = String::from("java/lang/Object");
    let mut java_lang_object = BuiltIn::new(java_lang_object_name.clone());
    let init_name = String::from("<init>");
    let init = BuiltInMethod::new(init_name.clone(), BuitlInCodeType::JavaLangObjectInit);
    java_lang_object.methods.insert(init_name, init);
    (java_lang_object_name, JavaClass::BuiltIn(java_lang_object))
}

fn create_java_lang_system() -> (String, JavaClass) {
    let java_lang_system_name = String::from("java/lang/System");
    let mut java_lang_system = BuiltIn::new(java_lang_system_name.clone());
    let init_name = String::from("<init>");
    let init = BuiltInMethod::new(init_name.clone(), BuitlInCodeType::JavaLangSystemInit);
    java_lang_system.methods.insert(init_name, init);
    (java_lang_system_name, JavaClass::BuiltIn(java_lang_system))
}

fn create_java_lang_integer() -> (String, JavaClass) {
    let java_lang_integer_name = String::from("java/lang/Integer");
    let mut java_lang_integer = BuiltIn::new(java_lang_integer_name.clone());
    let to_string_name = String::from("toString");
    let to_string = BuiltInMethod::new(
        to_string_name.clone(),
        BuitlInCodeType::JavaLangObjectToString,
    );
    java_lang_integer.methods.insert(to_string_name, to_string);
    (
        java_lang_integer_name,
        JavaClass::BuiltIn(java_lang_integer),
    )
}
