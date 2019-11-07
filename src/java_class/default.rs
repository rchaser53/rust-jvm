use crate::java_class::{
    builtin::{BuiltIn, BuiltInMethod, BuitlInCodeType},
    JavaClass,
};
use crate::utils::insert_string_pool;
use std::collections::HashMap;

pub fn setup_class_map() -> HashMap<usize, JavaClass> {
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

fn create_print_stream() -> (usize, JavaClass) {
    let class_name_id = insert_string_pool(String::from("java/io/PrintStream"));
    let mut print_stream = BuiltIn::new(class_name_id);
    let println_name_id = insert_string_pool(String::from("println"));
    let println = BuiltInMethod::new(println_name_id, BuitlInCodeType::Println);
    print_stream.methods.insert(println_name_id, println);
    (class_name_id, JavaClass::BuiltIn(print_stream))
}

fn create_java_lang_object() -> (usize, JavaClass) {
    let java_lang_object_name_id = insert_string_pool(String::from("java/lang/Object"));
    let mut java_lang_object = BuiltIn::new(java_lang_object_name_id);
    let init_name_id = insert_string_pool(String::from("<init>"));
    let init = BuiltInMethod::new(init_name_id, BuitlInCodeType::JavaLangObjectInit);
    java_lang_object.methods.insert(init_name_id, init);
    (
        java_lang_object_name_id,
        JavaClass::BuiltIn(java_lang_object),
    )
}

fn create_java_lang_system() -> (usize, JavaClass) {
    let java_lang_system_name_id = insert_string_pool(String::from("java/lang/System"));
    let mut java_lang_system = BuiltIn::new(java_lang_system_name_id);
    let init_name_id = insert_string_pool(String::from("<init>"));
    let init = BuiltInMethod::new(init_name_id, BuitlInCodeType::JavaLangSystemInit);
    java_lang_system.methods.insert(init_name_id, init);
    (
        java_lang_system_name_id,
        JavaClass::BuiltIn(java_lang_system),
    )
}

fn create_java_lang_integer() -> (usize, JavaClass) {
    let java_lang_integer_name_id = insert_string_pool(String::from("java/lang/Integer"));
    let mut java_lang_integer = BuiltIn::new(java_lang_integer_name_id);
    let to_string_name_id = insert_string_pool(String::from("toString"));
    let to_string = BuiltInMethod::new(to_string_name_id, BuitlInCodeType::JavaLangObjectToString);
    java_lang_integer
        .methods
        .insert(to_string_name_id, to_string);
    (
        java_lang_integer_name_id,
        JavaClass::BuiltIn(java_lang_integer),
    )
}
