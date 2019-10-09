use crate::attribute::{Attribute, Code};
use crate::constant::ConstantPool;
use crate::field::Field;
use crate::method::{Method, MethodAccessFlag};
use crate::utils::*;
use std::fmt;

#[derive(Debug)]
pub struct Interface(usize);

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,                 // u4
    pub minor_version: u16,         // u2
    pub major_version: u16,         // u2
    pub constant_pool_count: usize, // u2
    pub cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    pub access_flags: AccessFlags,  // u2
    pub this_class: usize,          // u2
    pub super_class: usize,         // u2
    pub interfaces_count: usize,    // u2
    pub interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    pub fields_count: usize,        // u2
    pub fields: Vec<Field>,         // field_info     fields[fields_count];
    pub methods_count: usize,       // u2
    pub methods: Vec<Method>,       // method_info    methods[methods_count];
    pub attributes_count: usize,    // u2
    pub attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

impl ClassFile {
    pub fn new(input: &mut [u8], index: usize) -> (ClassFile, usize) {
        let (magic, index) = extract_x_byte_as_usize(input, index, 4);
        let magic = magic as u32;

        let (minor_version, index) = extract_x_byte_as_usize(input, index, 2);
        let minor_version = minor_version as u16;
        let (major_version, index) = extract_x_byte_as_usize(input, index, 2);
        let major_version = major_version as u16;

        let (constant_pool_count, index) = extract_x_byte_as_usize(input, index, 2);
        let (cp_info, index) = ConstantPool::new(input, index, constant_pool_count);

        let (access_flags_num, index) = extract_x_byte_as_usize(input, index, 2);
        let access_flags = extract_access_flags(access_flags_num);

        let (this_class, index) = extract_x_byte_as_usize(input, index, 2);
        let (super_class, index) = extract_x_byte_as_usize(input, index, 2);

        let (interfaces_count, mut index) = extract_x_byte_as_usize(input, index, 2);
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            let (interface_index, updated_index) = extract_x_byte_as_usize(input, index, 2);
            index = updated_index;
            interfaces.push(Interface(interface_index));
        }

        let (fields_count, mut index) = extract_x_byte_as_usize(input, index, 2);
        let mut fields = Vec::with_capacity(fields_count);
        for _ in 0..fields_count {
            let (field, updated_index) = Field::new(input, index);
            index = updated_index;
            fields.push(field);
        }

        let (methods_count, mut index) = extract_x_byte_as_usize(input, index, 2);
        let mut methods = Vec::with_capacity(methods_count);
        for _ in 0..methods_count {
            let (method, updated_index) = Method::new(&cp_info, input, index);
            index = updated_index;
            methods.push(method);
        }

        let (attributes_count, mut index) = extract_x_byte_as_usize(input, index, 2);
        let mut attributes = Vec::with_capacity(attributes_count);

        for _ in 0..attributes_count {
            let (attribute, updated_index) = Attribute::new(&cp_info, input, index);
            index = updated_index;
            attributes.push(attribute);
        }

        (
            ClassFile {
                magic,
                minor_version,
                major_version,
                constant_pool_count,
                cp_info,
                access_flags,
                this_class,
                super_class,
                interfaces_count,
                interfaces,
                fields_count,
                fields,
                methods_count,
                methods,
                attributes_count,
                attributes,
            },
            index,
        )
    }

    pub fn run_entry_file(&self) {
        if let Some(main_index) = self.cp_info.get_main_index() {
            if let Some(entry_method) = self.methods.iter().find(|method| {
                method
                    .access_flags
                    .0
                    .iter()
                    .find(|flag| **flag == MethodAccessFlag::AccPublic)
                    .is_some()
                    && method.name_index == main_index
            }) {
                self.run_method(entry_method);
                return;
            }
        }
        panic!("failed to find main method in {}", self);
    }

    pub fn run_method(&self, method: &Method) {
        if let Some(code) = self.extract_code(method) {
            for instruction in code.code.iter() {
                println!("{}", instruction);
            }
        }
    }

    pub fn extract_code<'a>(&self, method: &'a Method) -> Option<&'a Code> {
        if let Some(attribute) = method.attribute_info.iter().find(|attribute| {
            if let Attribute::Code(_) = attribute {
                true
            } else {
                false
            }
        }) {
            if let Attribute::Code(ref code) = attribute {
                Some(code)
            } else {
                unreachable!();
            }
        } else {
            None
        }
    }
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut field_strs = Vec::with_capacity(self.fields_count);
        for item in self.fields.iter() {
            field_strs.push(format!("{}", item));
        }

        let mut method_strs = Vec::with_capacity(self.methods_count);
        for item in self.methods.iter() {
            method_strs.push(format!("{}", item));
        }

        let mut attribute_strs = Vec::with_capacity(self.attributes_count);
        for item in self.attributes.iter() {
            attribute_strs.push(format!("{}", item));
        }
        write!(
            f,
            "minor version: {}
major version: {}
flags: {}
Constant pool:
{}

Fields:
{}
Methods:
{}

Attributes:
  {}",
            self.minor_version,
            self.major_version,
            self.access_flags,
            self.cp_info,
            field_strs.join("\n"),
            method_strs.join("\n\n"),
            attribute_strs.join("\n  ")
        )
    }
}

fn extract_access_flags(num: usize) -> AccessFlags {
    let mut access_flags = vec![];
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccPublic);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccFinal);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccSuper);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccInterface);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccAbstract);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccSynthetic);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccAnnotation);
    crate::add_flags!(&mut access_flags, num, AccessFlag::AccEnum);

    AccessFlags(access_flags)
}

#[derive(Debug)]
pub enum AccessFlag {
    AccPublic = 0x0001,
    AccFinal = 0x0010,
    AccSuper = 0x0020,
    AccInterface = 0x0200,
    AccAbstract = 0x0400,
    AccSynthetic = 0x1000,
    AccAnnotation = 0x2000,
    AccEnum = 0x4000,
}

impl fmt::Display for AccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessFlag::AccPublic => write!(f, "ACC_PUBLIC"),
            AccessFlag::AccFinal => write!(f, "ACC_FINAL"),
            AccessFlag::AccSuper => write!(f, "ACC_SUPER"),
            AccessFlag::AccInterface => write!(f, "ACC_INTERFACE"),
            AccessFlag::AccAbstract => write!(f, "ACC_ABSTRACT"),
            AccessFlag::AccSynthetic => write!(f, "ACC_SYNTHETIC"),
            AccessFlag::AccAnnotation => write!(f, "ACC_ANNOTATION"),
            AccessFlag::AccEnum => write!(f, "ACC_ENUM"),
        }
    }
}

impl From<usize> for AccessFlag {
    fn from(num: usize) -> AccessFlag {
        match num {
            0x0001 => AccessFlag::AccPublic,
            0x0010 => AccessFlag::AccFinal,
            0x0020 => AccessFlag::AccSuper,
            0x0200 => AccessFlag::AccInterface,
            0x0400 => AccessFlag::AccAbstract,
            0x1000 => AccessFlag::AccSynthetic,
            0x2000 => AccessFlag::AccAnnotation,
            0x4000 => AccessFlag::AccEnum,
            _ => panic!("failed to convert {} to AccessFlag", num),
        }
    }
}

#[derive(Debug)]
pub struct AccessFlags(Vec<AccessFlag>);
impl fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::with_capacity(self.0.len());
        for item in self.0.iter() {
            result.push(format!("{}", item));
        }
        write!(f, "flags: {}", result.join(", "))
    }
}
