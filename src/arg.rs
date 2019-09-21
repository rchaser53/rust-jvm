#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt;
use std::ops::Fn;

pub struct Argument {
    kind: ArgumentKind,
    value: ArgumentValue,
}
impl Argument {
    pub fn new(kind: ArgumentKind, value: ArgumentValue) -> Self {
        Argument { kind, value }
    }
}

impl fmt::Debug for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub enum ArgumentKind {
    I32,
    F32,
    String,
    Array(Box<ArgumentKind>),
}

impl fmt::Debug for ArgumentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentKind::I32 => write!(f, "i32"),
            ArgumentKind::F32 => write!(f, "f32"),
            ArgumentKind::String => write!(f, "String"),
            _ => unimplemented!(),
        }
    }
}

pub enum ArgumentValue {
    I32(i32),
    F32(f32),
    String(String),
    Array(Box<ArgumentValue>),
}

impl fmt::Debug for ArgumentValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentValue::I32(v) => write!(f, "{}", v),
            ArgumentValue::F32(v) => write!(f, "{}", v),
            ArgumentValue::String(v) => write!(f, "{}", v),
            _ => unimplemented!(),
        }
    }
}

pub enum Method {
    ReturnVoid(Vec<ArgumentKind>, Box<dyn Fn(Argument)>),
}

fn print(input: Argument) {
    println!("{:?}", input.value);
}

fn main() {
    let a = Method::ReturnVoid(vec![], Box::new(|a| println!("{:?}", a)));
    if let Method::ReturnVoid(_, b_a) = a {
        // b_a("11");
    }
}

lazy_static! {
    static ref CLASS_MAP: HashMap<String, JavaClass> = {
        let mut map = HashMap::new();

        let mut java_class = HashMap::new();
        java_class.insert(
            String::from("println"),
            Method::ReturnVoid(vec![ArgumentKind::String], Box::new(print)),
        );
        let print_stream = JavaClass::new(java_class);

        map.insert(String::from("java/io/PrintStream"), print_stream);
        map
    };
}

pub struct JavaClass {
    methods: HashMap<String, Method>,
}

impl JavaClass {
    pub fn new(methods: HashMap<String, Method>) -> Self {
        JavaClass { methods }
    }
}
unsafe impl Sync for JavaClass {}
