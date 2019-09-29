use crate::stackframe::StarckframeItem;

#[derive(Debug)]
pub struct ConstantPool(Vec<ConstPoolItem>);
impl ConstantPool {
    pub fn new(mut inputs: &[u8]) -> ConstantPool
    {
        let mut items = vec![ConstPoolItem::ConstantNull];
        let mut byte_iter = inputs.into_iter();
        while let Some(byte) = byte_iter.next() {
            match ConstPoolTag::from(*byte) {
                ConstPoolTag::ConstantClass => {
                    let hi = byte_iter.next().unwrap() << 2 * 8;
                    let lo = byte_iter.next().unwrap();
                    let item = ConstantClass {
                        tag: ConstPoolTag::ConstantClass,
                        name_index: (hi + lo) as usize
                    };
                    items.push(ConstPoolItem::ConstantClass(item));
                }
                ConstPoolTag::ConstantMethodref => {
                    let hi_class = byte_iter.next().unwrap() << 2 * 8;
                    let lo_class = byte_iter.next().unwrap();
                    let hi_name_and_type = byte_iter.next().unwrap() << 2 * 8;
                    let lo_name_and_type = byte_iter.next().unwrap() << 2 * 8;
                    let item = ConstantMethodref {
                        tag: ConstPoolTag::ConstantMethodref,
                        class_index: (hi_class + lo_class) as usize,
                        name_and_type_index: (hi_name_and_type + lo_name_and_type) as usize
                    };
                    items.push(ConstPoolItem::ConstantMethodref(item));
                }
                ConstPoolTag::ConstantUtf8 => {
                    let hi = byte_iter.next().unwrap() << 2 * 8;
                    let lo = byte_iter.next().unwrap();
                    let length = (hi + lo) as usize;
                    let mut bytes = Vec::with_capacity(length);
                    for _ in 0..length {
                        bytes.push(*byte_iter.next().unwrap());
                    }
                    let item = ConstantUtf8 {
                        tag: ConstPoolTag::ConstantUtf8,
                        length,
                        bytes
                    };
                    items.push(ConstPoolItem::ConstantUtf8(item));
                }
                _ => unimplemented!()
            }
        }
        ConstantPool(items)
    }
}

#[derive(Debug)]
pub enum ConstPoolTag {
    ConstantNull = 0,               // custom tag for index 0 
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

#[derive(Debug)]
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
    ConstantNameAndType,
    ConstantUtf8(ConstantUtf8),
    ConstantMethodHandle,
    ConstantMethodType,
    ConstantInvokeDynamic, 
}

impl ConstPoolItem {
  // pub fn new(input: &mut IntoIterator<[u8]>) -> ConstPoolItem {
  //     input.next()
  // }
}

#[derive(Debug)]
pub struct ConstantClass {
    pub tag: ConstPoolTag,
    pub name_index: usize               // u2
}

#[derive(Debug)]
pub struct ConstantMethodref {
    pub tag: ConstPoolTag,
    pub class_index: usize,             // u2
    pub name_and_type_index: usize,     // u2
}

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub tag: ConstPoolTag,
    pub length: usize,                  // u2
    pub bytes: Vec<u8>
}