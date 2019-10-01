#[derive(Debug)]
pub struct ConstantPool(Vec<ConstPoolItem>);
impl ConstantPool {
    pub fn new(inputs: &mut Vec<u8>, index: usize, length: usize) -> ConstantPool {
        let mut items = vec![ConstPoolItem::ConstantNull];
        for _ in 0..length {
          let tag = inputs[index];
          index += 1;

          match ConstPoolTag::from(tag) {
              ConstPoolTag::ConstantClass => {
              },
              ConstPoolTag::ConstantMethodref => {
              },
              ConstPoolTag::ConstantNameAndType => {
              },
              ConstPoolTag::ConstantUtf8 => {
                let mut next_index = index + 1;
                let utf8_length = (inputs[index] << 8 + inputs[next_index]) as usize;
                next_index += 1;
                let bytes = inputs[next_index..(next_index + utf8_length)].to_vec();

                items.push(ConstPoolItem::ConstantUtf8(ConstantUtf8 {
                    tag: ConstPoolTag::ConstantUtf8,
                    length: utf8_length,
                    bytes,
                }));
                index = next_index + utf8_length + 1;
              },
          }
        }

        ConstantPool(items)
    }
}

#[derive(Debug)]
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
    pub fn new(tag_byte: u8, byte_iter: &mut std::slice::Iter<'_, u8>) -> ConstPoolItem {
        match ConstPoolTag::from(tag_byte) {
            ConstPoolTag::ConstantClass => {
                let hi = byte_iter.next().unwrap() << 2 * 8;
                let lo = byte_iter.next().unwrap();
                ConstPoolItem::ConstantClass(ConstantClass {
                    tag: ConstPoolTag::ConstantClass,
                    name_index: (hi + lo) as usize,
                })
            }
            ConstPoolTag::ConstantMethodref => {
                let hi_class = byte_iter.next().unwrap() << 2 * 8;
                let lo_class = byte_iter.next().unwrap();
                let hi_name_and_type = byte_iter.next().unwrap() << 2 * 8;
                let lo_name_and_type = byte_iter.next().unwrap() << 2 * 8;
                ConstPoolItem::ConstantMethodref(ConstantMethodref {
                    tag: ConstPoolTag::ConstantMethodref,
                    class_index: (hi_class + lo_class) as usize,
                    name_and_type_index: (hi_name_and_type + lo_name_and_type) as usize,
                })
            }
            ConstPoolTag::ConstantUtf8 => {
                let hi = byte_iter.next().unwrap() << 2 * 8;
                let lo = byte_iter.next().unwrap();
                let length = (hi + lo) as usize;
                let mut bytes = Vec::with_capacity(length);
                for _ in 0..length {
                    bytes.push(*byte_iter.next().unwrap());
                }

                ConstPoolItem::ConstantUtf8(ConstantUtf8 {
                    tag: ConstPoolTag::ConstantUtf8,
                    length,
                    bytes,
                })
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantClass {
    pub tag: ConstPoolTag,
    pub name_index: usize, // u2
}

#[derive(Debug)]
pub struct ConstantMethodref {
    pub tag: ConstPoolTag,
    pub class_index: usize,         // u2
    pub name_and_type_index: usize, // u2
}

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub tag: ConstPoolTag,
    pub length: usize, // u2
    pub bytes: Vec<u8>,
}
