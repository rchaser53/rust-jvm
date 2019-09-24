mod operand;
use operand::OperandStackItem;

mod stackframe;
use stackframe::StarckframeItem;

mod order;
use order::{Opecode, Order};

mod context;
use crate::context::ProgramContext;

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
