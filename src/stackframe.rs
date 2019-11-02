use crate::operand::{Item, OperandStack};

#[derive(Debug)]
pub struct Stackframe {
    pub local_variables: Vec<Item>,
    pub operand_stack: OperandStack,
}

impl Stackframe {
    pub fn new(variables_number: usize) -> Self {
        Stackframe {
            local_variables: Vec::with_capacity(variables_number),
            operand_stack: OperandStack::new(),
        }
    }

    pub fn update_object_ref(
        &mut self,
        class_name: String,
        field_name: String,
        vals: (Item, Item),
    ) {
        let target = self
            .local_variables
            .iter()
            .find(|variable| {
                if let Item::Objectref(ref object_ref) = variable {
                    object_ref.class_name == class_name
                        && object_ref.field_map.borrow().get(&field_name).is_some()
                } else {
                    false
                }
            })
            .expect(&format!(
                "should exist object_ref
// stackframe:
// {:?}",
                self
            ));

        if let Item::Objectref(object_ref) = target {
            object_ref
                .field_map
                .borrow_mut()
                .insert(field_name.clone(), vals.clone());
        } else {
            unreachable!("not come here")
        }
    }
}
