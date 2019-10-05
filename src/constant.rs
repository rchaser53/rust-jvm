use crate::utils::*;

#[test]
fn constant_pool_utf8() {
    let mut inputs = vec![
        0x01, // utf8
        0x00, 0x0A, // 0x0a
        0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6C, 0x65, // SourceFile
    ];
    let result = ConstantPool::new(&mut inputs, 0, 1);

    assert_eq!(
        result,
        (
            ConstantPool(vec![
                ConstPoolItem::ConstantNull,
                ConstPoolItem::ConstantUtf8(ConstantUtf8 {
                    tag: ConstPoolTag::ConstantUtf8,
                    length: 0x0a,
                    bytes: vec![0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6C, 0x65]
                })
            ]),
            inputs.len()
        )
    );
}

#[derive(Debug, PartialEq)]
pub struct ConstantPool(pub Vec<ConstPoolItem>);
impl ConstantPool {
    pub fn new(inputs: &mut Vec<u8>, mut index: usize, length: usize) -> (ConstantPool, usize) {
        let mut items = vec![ConstPoolItem::ConstantNull];
        for _ in 0..length {
            let tag = inputs[index];
            index += 1;

            let (item, update_index) = match ConstPoolTag::from(tag) {
                ConstPoolTag::ConstantClass => {
                    let (item, update_index) =
                        ConstantClass::create_and_update_index(inputs, index);
                    (ConstPoolItem::ConstantClass(item), update_index)
                }
                ConstPoolTag::ConstantMethodref => {
                    let (item, update_index) =
                        ConstantMethodref::create_and_update_index(inputs, index);
                    (ConstPoolItem::ConstantMethodref(item), update_index)
                }
                ConstPoolTag::ConstantNameAndType => {
                    let (item, update_index) =
                        ConstantNameAndType::create_and_update_index(inputs, index);
                    (ConstPoolItem::ConstantNameAndType(item), update_index)
                }
                ConstPoolTag::ConstantUtf8 => {
                    let (item, update_index) = ConstantUtf8::create_and_update_index(inputs, index);
                    (ConstPoolItem::ConstantUtf8(item), update_index)
                }
                _ => unimplemented!(),
            };
            index = update_index;
            items.push(item);
        }

        (ConstantPool(items), index)
    }
}

#[derive(Debug, PartialEq)]
pub enum ConstPoolTag {
    ConstantNull = 0, // custom tag for index 0
    ConstantClass = 7,
    ConstantFieldref = 9,
    ConstantMethodref = 10,
    ConstantInterfaceMethodref = 11,
    ConstantString = 8,
    ConstantInteger = 3,
    ConstantFloat = 4,
    ConstantLong = 5,
    ConstantDouble = 6,
    ConstantNameAndType = 12,
    ConstantUtf8 = 1,
    ConstantMethodHandle = 15,
    ConstantMethodType = 16,
    ConstantInvokeDynamic = 18,
}

impl From<u8> for ConstPoolTag {
    fn from(num: u8) -> ConstPoolTag {
        match num {
            7 => ConstPoolTag::ConstantClass,
            9 => ConstPoolTag::ConstantFieldref,
            10 => ConstPoolTag::ConstantMethodref,
            11 => ConstPoolTag::ConstantInterfaceMethodref,
            8 => ConstPoolTag::ConstantString,
            3 => ConstPoolTag::ConstantInteger,
            4 => ConstPoolTag::ConstantFloat,
            5 => ConstPoolTag::ConstantLong,
            6 => ConstPoolTag::ConstantDouble,
            12 => ConstPoolTag::ConstantNameAndType,
            1 => ConstPoolTag::ConstantUtf8,
            15 => ConstPoolTag::ConstantMethodHandle,
            16 => ConstPoolTag::ConstantMethodType,
            18 => ConstPoolTag::ConstantInvokeDynamic,
            _ => panic!("failed to convert {} to ConstPoolTag", num),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConstPoolItem {
    ConstantNull,
    ConstantClass(ConstantClass),
    ConstantFieldref,
    ConstantMethodref(ConstantMethodref),
    ConstantInterfaceMethodref,
    ConstantString,
    ConstantInteger,
    ConstantFloat,
    ConstantLong,
    ConstantDouble,
    ConstantNameAndType(ConstantNameAndType),
    ConstantUtf8(ConstantUtf8),
    ConstantMethodHandle,
    ConstantMethodType,
    ConstantInvokeDynamic,
}

#[derive(Debug, PartialEq)]
pub struct ConstantNameAndType {
    pub tag: ConstPoolTag,
    pub name_index: usize,       // u2
    pub descriptor_index: usize, // u2
}

impl ConstantNameAndType {
    pub fn create_and_update_index(
        inputs: &mut Vec<u8>,
        mut index: usize,
    ) -> (ConstantNameAndType, usize) {
        let output = extract_x_byte_as_usize(inputs, index, 2);
        let name_index = output.0;
        index = output.1;

        let output = extract_x_byte_as_usize(inputs, index, 2);
        let descriptor_index = output.0;
        index = output.1;

        (
            ConstantNameAndType {
                tag: ConstPoolTag::ConstantNameAndType,
                name_index,
                descriptor_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantClass {
    pub tag: ConstPoolTag,
    pub name_index: usize, // u2
}

impl ConstantClass {
    pub fn create_and_update_index(
        inputs: &mut Vec<u8>,
        mut index: usize,
    ) -> (ConstantClass, usize) {
        let output = extract_x_byte_as_usize(inputs, index, 2);
        let name_index = output.0;
        index = output.1;
        (
            ConstantClass {
                tag: ConstPoolTag::ConstantClass,
                name_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantMethodref {
    pub tag: ConstPoolTag,
    pub class_index: usize,         // u2
    pub name_and_type_index: usize, // u2
}

impl ConstantMethodref {
    pub fn create_and_update_index(
        inputs: &mut Vec<u8>,
        mut index: usize,
    ) -> (ConstantMethodref, usize) {
        let output = extract_x_byte_as_usize(inputs, index, 2);
        let class_index = output.0;
        index = output.1;

        let output = extract_x_byte_as_usize(inputs, index, 2);
        let name_and_type_index = output.0;
        index = output.1;

        (
            ConstantMethodref {
                tag: ConstPoolTag::ConstantMethodref,
                class_index,
                name_and_type_index,
            },
            index,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantUtf8 {
    pub tag: ConstPoolTag,
    pub length: usize, // u2
    pub bytes: Vec<u8>,
}

impl ConstantUtf8 {
    pub fn create_and_update_index(
        inputs: &mut Vec<u8>,
        mut index: usize,
    ) -> (ConstantUtf8, usize) {
        let output = extract_x_byte_as_usize(inputs, index, 2);
        let utf8_length = output.0;
        index = output.1;

        let output = extract_x_byte_as_vec(inputs, index, utf8_length);
        let bytes = output.0;
        index = output.1;

        (
            ConstantUtf8 {
                tag: ConstPoolTag::ConstantUtf8,
                length: utf8_length,
                bytes,
            },
            index,
        )
    }
}
