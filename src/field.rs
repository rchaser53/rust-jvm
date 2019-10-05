use crate::attribute::Attribute;

#[derive(Debug)]
pub struct Field {
    pub access_flags: FieldAccessFlag, // u2
    pub name_index: u16,               // u2
    pub descriptor_index: u16,         // u2
    pub attributes_count: u16,         // u2
    pub attribute_info: Vec<Attribute>,
}

#[derive(Debug)]
pub enum FieldAccessFlag {
    Unknown,
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccVolatitle,
    AccTransient,
    AccSynthetic,
    AccEnum,
}

impl From<u16> for FieldAccessFlag {
    fn from(num: u16) -> FieldAccessFlag {
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
