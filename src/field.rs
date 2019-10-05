use crate::attribute::Attribute;
use crate::utils::extract_x_byte_as_usize;

#[derive(Debug)]
pub struct Field {
    pub access_flags: FieldAccessFlag, // u2
    pub name_index: u16,               // u2
    pub descriptor_index: u16,         // u2
    pub attributes_count: u16,         // u2
    pub attribute_info: Vec<Attribute>,
}

impl Field {
    pub fn new(inputs: &mut Vec<u8>, index: usize) -> (Field, usize) {
        let (access_flags, index) = extract_x_byte_as_usize(inputs, index, 2);
        let access_flags = FieldAccessFlag::from(access_flags);

        let (name_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let name_index = name_index as u16;
        let (descriptor_index, index) = extract_x_byte_as_usize(inputs, index, 2);
        let descriptor_index = descriptor_index as u16;
        let (attributes_count, index) = extract_x_byte_as_usize(inputs, index, 2);
        let attributes_count = attributes_count as u16;

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
