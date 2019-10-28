use crate::attribute::defs::Attribute;
use crate::utils::extract_x_byte_as_usize;
use std::fmt;

#[derive(Debug)]
pub struct Field {
    pub access_flags: FieldAccessFlags, // u2
    pub name_index: usize,              // u2
    pub descriptor_index: usize,        // u2
    pub attributes_count: usize,        // u2
    pub attribute_info: Vec<Attribute>,
}

impl Field {
    pub fn new(inputs: &mut [u8], index: usize) -> (Field, usize) {
        let (access_flags, index) = extract_x_byte_as_usize(inputs, index, 2);
        let access_flags = extract_access_flags(access_flags);

        let (name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (descriptor_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (attributes_count, index) = extract_x_byte_as_usize(inputs, index, 2);

        (
            Field {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attribute_info: vec![],
            },
            index,
        )
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attribute_strs = Vec::with_capacity(self.attributes_count);
        for item in self.attribute_info.iter() {
            attribute_strs.push(format!("{}", item));
        }

        write!(
            f,
            "  name: #{}
  descriptor: #{}
  flags: {}
  attribute:
    {}",
            self.name_index,
            self.descriptor_index,
            self.access_flags,
            attribute_strs.join("\n  ")
        )
    }
}

#[derive(Debug)]
pub enum FieldDescriptor {
    BaseType(BaseType),
    // L ClassName ;	reference	an instance of class ClassName
    ObjectType(String),
    ArrayType(Box<FieldDescriptor>),
}

#[derive(Debug)]
pub enum BaseType {
    B, // byte
    C, // char
    D, // double
    F, // float
    I, // int
    J, // long
    S, // short
    Z, // boolean
}

fn extract_access_flags(num: usize) -> FieldAccessFlags {
    let mut access_flags = vec![];
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccPublic);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccPrivate);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccProtected);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccStatic);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccFinal);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccVolatitle);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccTransient);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccSynthetic);
    crate::add_flags!(&mut access_flags, num, FieldAccessFlag::AccEnum);

    FieldAccessFlags(access_flags)
}

#[derive(Debug)]
pub struct FieldAccessFlags(Vec<FieldAccessFlag>);
impl fmt::Display for FieldAccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::with_capacity(self.0.len());
        for item in self.0.iter() {
            result.push(format!("{}", item));
        }
        write!(f, "{}", result.join(", "))
    }
}

#[derive(Debug)]
pub enum FieldAccessFlag {
    Unknown = 0x0000,
    AccPublic = 0x0001,
    AccPrivate = 0x0002,
    AccProtected = 0x0004,
    AccStatic = 0x0008,
    AccFinal = 0x0010,
    AccVolatitle = 0x0040,
    AccTransient = 0x0080,
    AccSynthetic = 0x1000,
    AccEnum = 0x4000,
}

impl fmt::Display for FieldAccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldAccessFlag::Unknown => write!(f, ""),
            FieldAccessFlag::AccPublic => write!(f, "ACC_PUBLIC"),
            FieldAccessFlag::AccPrivate => write!(f, "ACC_PRIVATE"),
            FieldAccessFlag::AccProtected => write!(f, "ACC_PROTECTED"),
            FieldAccessFlag::AccStatic => write!(f, "ACC_STATIC"),
            FieldAccessFlag::AccFinal => write!(f, "ACC_FINAL"),
            FieldAccessFlag::AccVolatitle => write!(f, "ACC_VOLATITLE"),
            FieldAccessFlag::AccTransient => write!(f, "ACC_TRANSIENT"),
            FieldAccessFlag::AccSynthetic => write!(f, "ACC_SYNTHETIC"),
            FieldAccessFlag::AccEnum => write!(f, "ACC_ENUM"),
        }
    }
}

impl From<usize> for FieldAccessFlag {
    fn from(num: usize) -> FieldAccessFlag {
        match num {
            0x0000 => FieldAccessFlag::Unknown, // custom
            0x0001 => FieldAccessFlag::AccPublic,
            0x0002 => FieldAccessFlag::AccPrivate,
            0x0004 => FieldAccessFlag::AccProtected,
            0x0008 => FieldAccessFlag::AccStatic,
            0x0010 => FieldAccessFlag::AccFinal,
            0x0040 => FieldAccessFlag::AccVolatitle,
            0x0080 => FieldAccessFlag::AccTransient,
            0x1000 => FieldAccessFlag::AccSynthetic,
            0x4000 => FieldAccessFlag::AccEnum,
            _ => panic!("failed to convert {} to FieldAccessFlag", num),
        }
    }
}
