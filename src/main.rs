mod operand;
use operand::OperandStackItem;

mod stackframe;
use stackframe::StarckframeItem;

mod order;
use order::{Opecode, Order};

mod context;
use crate::context::{ConstantPool, ProgramContext};

struct Interface;
struct Field;
struct Method;
struct Attribute;

struct ClassFile {
    magic: u16,                 // u4
    minor_version: u8,          // u2
    major_version: u8,          // u2
    constant_pool_count: u8,    // u2
    cp_info: ConstantPool,      // cp_info        constant_pool[constant_pool_count-1];
    access_flags: u8,           // u2
    this_class: u8,             // u2
    super_class: u8,            // u2
    interfaces_count: u8,       // u2
    interfaces: Vec<Interface>, // u2             interfaces[interfaces_count];
    fields_count: u8,           // u2
    fields: Vec<Field>,         // field_info     fields[fields_count];
    methods_count: u8,          // u2
    methods: Vec<Method>,       // method_info    methods[methods_count];
    attributes_count: u8,       // u2
    attributes: Vec<Attribute>, // attribute_info attributes[attributes_count];
}

fn main() {
    let mut program_context = ProgramContext::new(vec![
        Order::new(Opecode::Iconst, OperandStackItem::I32(1)),
        Order::new(Opecode::Iconst, OperandStackItem::I32(2)),
        Order::new(Opecode::Iadd, OperandStackItem::I32(2)),
    ]);
    program_context.executes_programs();

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
