mod attribute;
mod constant;
mod context;
mod operand;
mod order;
mod stackframe;
mod utils;

use crate::attribute::Attribute;
use crate::constant::ConstantPool;
use crate::utils::read_file;

#[derive(Debug)]
struct Interface;
#[derive(Debug)]
struct Field;

#[derive(Debug)]
struct Method {
    pub access_flags: u16,     // u2
    pub name_index: u16,       // u2
    pub descriptor_index: u16, // u2
    pub attributes_count: u16, // u2
    pub attribute_info: Vec<Attribute>,
}

#[derive(Debug)]
struct ClassFile {
    pub magic: u32,                 // u4
    pub minor_version: u16,         // u2
    pub major_version: u16,         // u2
    pub constant_pool_count: u16,   // u2
    pub cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    pub access_flags: u16,          // u2
    pub this_class: u16,            // u2
    pub super_class: u16,           // u2
    pub interfaces_count: u16,      // u2
    pub interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    pub fields_count: u16,          // u2
    pub fields: Vec<Field>,         // field_info     fields[fields_count];
    pub methods_count: u16,         // u2
    pub methods: Vec<Method>,       // method_info    methods[methods_count];
    pub attributes_count: u16,      // u2
    pub attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

impl ClassFile {
    // pub fn new(input: Vec<u8>) -> ClassFile {
    //     let mut index = 0;

    //     let (_, mut index) = ClassFile::extract_magic(&mut input, index);
    //     let (minor_version, mut index) = ClassFile::extract_u16(&mut input, index);
    //     let (major_version, mut index) = ClassFile::extract_u16(&mut input, index);
    //     let (constant_pool_count, mut index) = ClassFile::extract_u16(&mut input, index);

    //     let constant_pool = ConstantPool::new(input, index);
    // }
}

#[derive(Debug)]
pub enum AccessFlag {
    AccPublic,
    AccFinal,
    AccSuper,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
}

impl From<u16> for AccessFlag {
    fn from(num: u16) -> AccessFlag {
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

fn main() {
    if let Ok(buffer) = read_file("A.class", &mut vec![]) {
        dbg!(&buffer);
    }
    // let a: ConstPoolTag = b.try_into().unwrap_or(ConstPoolTag::ConstantUtf8);
    // let mut program_context = ProgramContext::new(vec![
    //     Order::new(Opecode::Iconst, OperandStackItem::I32(1)),
    //     Order::new(Opecode::Iconst, OperandStackItem::I32(2)),
    //     Order::new(Opecode::Iadd, OperandStackItem::I32(2)),
    // ]);
    // program_context.executes_programs();

    // operand_stack.iconst(OperandStackItem::I32(1));
    // stackframe.istore(&mut operand_stack, 0);

    // operand_stack.bipush(OperandStackItem::I32(1));
    // operand_stack.bipush(OperandStackItem::I32(2));
    // let result = operand_stack.iadd();
}

/*
* 1 + 2;
*/
// bipush 1
// bipush 2
// iadd

/*
 *  int i;
 *  i = 0;
 */
//  iconst_0
//  istore_1
//
