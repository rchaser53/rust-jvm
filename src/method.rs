use crate::attribute::Attribute;
use crate::constant::ConstantPool;
use crate::utils::*;
use std::fmt;

#[derive(Debug)]
pub struct Method {
    pub access_flags: MethodAccessFlags, // u2
    pub name_index: usize,               // u2
    pub descriptor_index: usize,         // u2
    pub attributes_count: usize,         // u2
    pub attribute_info: Vec<Attribute>,
}

impl Method {
    pub fn new(constant_pool: &ConstantPool, inputs: &mut [u8], index: usize) -> (Method, usize) {
        let (access_flag_num, index) = extract_x_byte_as_usize(inputs, index, 2);
        let access_flags = extract_access_flags(access_flag_num);
        let (name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let (descriptor_index, index) = extract_x_byte_as_usize(inputs, index, 2);

        let (attributes_count, mut index) = extract_x_byte_as_usize(inputs, index, 2);
        let mut attribute_info = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let (attribute, updated_index) = Attribute::new(constant_pool, inputs, index);
            index = updated_index;
            attribute_info.push(attribute);
        }

        (
            Method {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attribute_info,
            },
            index,
        )
    }
}

impl fmt::Display for Method {
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
  {}",
            self.name_index,
            self.descriptor_index,
            self.access_flags,
            attribute_strs.join("\n\n")
        )
    }
}

fn extract_access_flags(num: usize) -> MethodAccessFlags {
    let mut access_flags = vec![];
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccPublic);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccPrivate);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccProtected);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccStatic);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccFinal);

    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccSynchronized);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccBridge);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccVarargs);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccNative);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccAbstract);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccStrict);
    crate::add_flags!(&mut access_flags, num, MethodAccessFlag::AccSynthetic);

    MethodAccessFlags(access_flags)
}

#[derive(Debug, PartialEq)]
pub enum MethodAccessFlag {
    AccPublic = 0x0001,
    AccPrivate = 0x0002,
    AccProtected = 0x0004,
    AccStatic = 0x0008,
    AccFinal = 0x0010,
    AccSynchronized = 0x0020,
    AccBridge = 0x0040,
    AccVarargs = 0x0080,
    AccNative = 0x0100,
    AccAbstract = 0x0400,
    AccStrict = 0x0800,
    AccSynthetic = 0x1000,
}

impl fmt::Display for MethodAccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodAccessFlag::AccPublic => write!(f, "ACC_PUBLIC"),
            MethodAccessFlag::AccPrivate => write!(f, "ACC_PRIVATE"),
            MethodAccessFlag::AccProtected => write!(f, "ACC_PROTECTED"),
            MethodAccessFlag::AccStatic => write!(f, "ACC_STATIC"),
            MethodAccessFlag::AccFinal => write!(f, "ACC_FINAL"),
            MethodAccessFlag::AccSynchronized => write!(f, "ACC_SYNCHRONIZED"),
            MethodAccessFlag::AccBridge => write!(f, "ACC_BRIDGE"),
            MethodAccessFlag::AccVarargs => write!(f, "ACC_VARARGS"),
            MethodAccessFlag::AccNative => write!(f, "ACC_NATIVE"),
            MethodAccessFlag::AccAbstract => write!(f, "ACC_ABSTRACT"),
            MethodAccessFlag::AccStrict => write!(f, "ACC_STRICT"),
            MethodAccessFlag::AccSynthetic => write!(f, "ACC_SYNTHETIC"),
        }
    }
}

impl From<usize> for MethodAccessFlag {
    fn from(num: usize) -> MethodAccessFlag {
        match num {
            0x0001 => MethodAccessFlag::AccPublic,
            0x0002 => MethodAccessFlag::AccPrivate,
            0x0004 => MethodAccessFlag::AccProtected,
            0x0008 => MethodAccessFlag::AccStatic,
            0x0010 => MethodAccessFlag::AccFinal,
            0x0020 => MethodAccessFlag::AccSynchronized,
            0x0040 => MethodAccessFlag::AccBridge,
            0x0080 => MethodAccessFlag::AccVarargs,
            0x0100 => MethodAccessFlag::AccNative,
            0x0400 => MethodAccessFlag::AccAbstract,
            0x0800 => MethodAccessFlag::AccStrict,
            0x1000 => MethodAccessFlag::AccSynthetic,
            _ => panic!("failed to convert {} to MethodAccessFlag", num),
        }
    }
}

#[derive(Debug)]
pub struct MethodAccessFlags(pub Vec<MethodAccessFlag>);
impl fmt::Display for MethodAccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::with_capacity(self.0.len());
        for item in self.0.iter() {
            result.push(format!("{}", item));
        }
        if result.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", result.join(", "))
        }
    }
}
