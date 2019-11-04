use crate::array::{Array, ArrayMap};
use crate::attribute::code::Code;
use crate::attribute::instruction::Instruction;
use crate::constant::{ConstantNameAndType, ConstantPool};
use crate::field::{BaseType, FieldDescriptor};
use crate::java_class::{custom::Custom, JavaClass};
use crate::object::{ObjectMap, Objectref};
use crate::operand::Item;
use crate::option::OBJECT_ID;
use crate::stackframe::Stackframe;
use crate::utils::{
    emit_debug_info, iniailize_primitive_array, initialize_objectref_array, read_file,
};

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use std::path::Path;

#[derive(Debug)]
pub struct Context<'a> {
    pub class_map: HashMap<String, JavaClass>,
    pub program_count: usize,
    pub stack_frames: Vec<Stackframe>,
    pub root_path: &'a str,
    pub static_fields: StaticFields,
    pub object_map: ObjectMap,
    pub array_map: ArrayMap,
}

pub type ClassMap = HashMap<String, JavaClass>;
// class_name, field_name
pub type StaticFields = HashMap<(String, String), (Item, Item)>;

impl<'a> Context<'a> {
    pub fn new(class_map: ClassMap, class_file: &Custom, root_path: &'a str) -> Context<'a> {
        let mut static_fields = setup_static_fields(&class_map);
        set_static_fields(&class_file, &mut static_fields);

        Context {
            class_map,
            program_count: 0,
            stack_frames: vec![],
            root_path,
            static_fields,
            object_map: HashMap::new(),
            array_map: HashMap::new(),
        }
    }

    pub fn run_entry_file(&mut self, class_file: Custom) {
        let entry_method = class_file
            .get_entry_method()
            .expect("add handler in the case failed to find entry method");

        // TBD Perhaps this method is not invoked from super_class
        let super_class_index = class_file.super_class;
        let super_class_ref = class_file.cp_info.get_class_ref(super_class_index);
        let super_class_name = class_file.cp_info.get_utf8(super_class_ref.name_index);
        let stack_frame_item_0 = Item::Classref(super_class_name);

        if let Some(code) = class_file.get_clinit_code() {
            let stack_frame = Stackframe::new(code.max_locals as usize);
            self.stack_frames.push(stack_frame);
            self.call_custom_class_method(&class_file, code);
        }

        let code = entry_method
            .extract_code()
            .expect("should exist code in method");
        let mut stack_frame = Stackframe::new(code.max_locals as usize);
        stack_frame.local_variables.push(stack_frame_item_0);
        self.stack_frames.push(stack_frame);
        self.run_method(&class_file, code);

        self.class_map
            .insert(class_file.this_class_name(), JavaClass::Custom(class_file));
    }

    fn run_method(&mut self, class_file: &Custom, code: &Code) {
        let mut index = 0;
        while let Some(instruction) = code.code.get(index) {
            emit_debug_info(instruction, self.stack_frames.last());
            let (should_finish, update_index) = self.execute(class_file, instruction, index);
            if should_finish {
                break;
            }
            index = update_index + 1;
        }
        self.stack_frames.pop();
    }

    pub fn execute(
        &mut self,
        class_file: &Custom,
        instruction: &Instruction,
        index: usize,
    ) -> (bool, usize) {
        match instruction {
            Instruction::Iadd => {
                let stackframe = self.get_last_stackframe();
                let item = stackframe.operand_stack.iadd();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Ladd => {
                let stackframe = self.get_last_stackframe();
                let (first, second) = stackframe.operand_stack.ladd();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Isub => {
                let stackframe = self.get_last_stackframe();
                let item = stackframe.operand_stack.isub();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lsub => {
                let stackframe = self.get_last_stackframe();
                let (first, second) = stackframe.operand_stack.lsub();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Imul => {
                let stackframe = self.get_last_stackframe();
                let item = stackframe.operand_stack.imul();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lmul => {
                let stackframe = self.get_last_stackframe();
                let (first, second) = stackframe.operand_stack.lmul();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Idiv => {
                let stackframe = self.get_last_stackframe();
                let item = stackframe.operand_stack.idiv();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Ldiv => {
                let stackframe = self.get_last_stackframe();
                let (first, second) = stackframe.operand_stack.ldiv();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::Irem => {
                let stackframe = self.get_last_stackframe();
                let item = stackframe.operand_stack.irem();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Lrem => {
                let stackframe = self.get_last_stackframe();
                let (first, second) = stackframe.operand_stack.lrem();
                stackframe.operand_stack.stack.push(first);
                stackframe.operand_stack.stack.push(second);
            }
            Instruction::IconstN(val) => {
                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Int(*val as i32));
            }
            Instruction::LconstN(val) => {
                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Long(0));
                operand_stack.push(Item::Long(*val as i32));
            }
            // maybe need to fix for float or something like that
            Instruction::Bipush(val) => {
                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Int(*val as i32));
            }
            Instruction::Sipush(val) => {
                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Int(*val as i32));
            }
            Instruction::Lookupswitch(vals) => {
                let operand_stack = self.get_operand_stack();
                if let Some(Item::Int(target_key)) = operand_stack.pop() {
                    if let Some(jump_pointer) = vals.iter().find(|(optional_key, _)| {
                        if let Some(key) = *optional_key {
                            key == target_key as usize
                        } else {
                            false
                        }
                    }) {
                        return (false, jump_pointer.1);
                    } else {
                        return (false, vals.first().expect("should exist default value").1);
                    }
                } else {
                    unreachable!("should exist operan_item");
                }
            }
            Instruction::Goto(pointer) => {
                return (false, *pointer);
            }
            Instruction::Iinc(index, value) => {
                let stackframe = self.get_last_stackframe();
                if let Some(item) = stackframe.local_variables.get_mut(*index) {
                    if let Item::Int(val) = item {
                        mem::replace(val, *val + *value as i32);
                    }
                }
            }
            Instruction::Lcmp => {
                let stackframe = self.get_last_stackframe();
                let val = stackframe.operand_stack.lcmp();
                stackframe.operand_stack.stack.push(val);
            }
            Instruction::Ifeq(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val == Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifne(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val != Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Iflt(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val < Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifge(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val >= Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifgt(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val > Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifle(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let val = operand_stack.pop().unwrap();
                let jump_pointer = if val <= Item::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ificmpeq(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first == second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpne(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first != second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmplt(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first < second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpge(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first >= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpgt(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first > second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmple(if_val, else_val) => {
                let operand_stack = self.get_operand_stack();
                let second = operand_stack.pop();
                let first = operand_stack.pop();
                let jump_pointer = if first <= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Iload(index) => {
                self.load_n(*index);
            }
            Instruction::IloadN(index) => {
                self.load_n(*index);
            }
            Instruction::LloadN(index) => {
                let base_index = *index;
                self.load_n(base_index);
                self.load_n(base_index + 1);
            }
            Instruction::Istore(index) => {
                self.store_n(&[*index]);
            }
            Instruction::IstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::LstoreN(index) => {
                let base_index = *index;
                self.store_n(&[base_index + 1, base_index]);
            }
            Instruction::AloadN(index) => {
                self.load_n(*index);
            }
            Instruction::Iaload => {
                let operand_stack = self.get_operand_stack();
                match (operand_stack.pop(), operand_stack.pop()) {
                    (Some(Item::Int(index)), Some(Item::Arrayref(array_ref_id))) => {
                        let array_cell = self
                            .array_map
                            .get_mut(&array_ref_id)
                            .expect("should exist item in array_map");
                        let item = match array_cell {
                            Array::Primitive(items) => items.borrow()[index as usize].clone(),
                            _ => unimplemented!(),
                        };
                        let operand_stack = self.get_operand_stack();
                        operand_stack.push(item.0);
                        match item.1 {
                            Item::Null => {}
                            _ => operand_stack.push(item.1),
                        };
                    }
                    _ => panic!("should exist two items in operand_stack"),
                };
            }
            Instruction::Aaload => {
                let operand_stack = self.get_operand_stack();
                match (operand_stack.pop(), operand_stack.pop()) {
                    (Some(Item::Int(index)), Some(Item::Arrayref(array_ref_id))) => {
                        let array_cell = self
                            .array_map
                            .get_mut(&array_ref_id)
                            .expect("should exist item in array_map");
                        let object_id = match array_cell {
                            Array::Custom(items) => items.borrow()[index as usize],
                            _ => unimplemented!(),
                        };
                        let operand_stack = self.get_operand_stack();
                        operand_stack.push(Item::Objectref(object_id));
                    }
                    _ => unreachable!("should exist two items in operand_stack"),
                };
            }
            Instruction::Iastore => {
                let operand_stack = self.get_operand_stack();
                match (
                    operand_stack.pop(),
                    operand_stack.pop(),
                    operand_stack.pop(),
                ) {
                    (Some(value), Some(Item::Int(index)), Some(Item::Arrayref(array_ref_id))) => {
                        if let Some(array_cell) = self.array_map.get_mut(&array_ref_id) {
                            match array_cell {
                                Array::Primitive(items) => {
                                    // TBD need to fix this
                                    items.borrow_mut()[index as usize] = (value, Item::Null);
                                }
                                _ => unimplemented!(),
                            };
                        }
                    }
                    _ => unreachable!("should exist three items in operand_stack"),
                };
            }
            Instruction::Aastore => {
                let operand_stack = self.get_operand_stack();
                match (
                    operand_stack.pop(),
                    operand_stack.pop(),
                    operand_stack.pop(),
                ) {
                    (
                        Some(Item::Objectref(ref_id)),
                        Some(Item::Int(index)),
                        Some(Item::Arrayref(array_ref_id)),
                    ) => {
                        if let Some(array_cell) = self.array_map.get_mut(&array_ref_id) {
                            match array_cell {
                                Array::Custom(items) => {
                                    items.borrow_mut()[index as usize] = ref_id;
                                }
                                _ => unimplemented!(),
                            };
                        }
                    }
                    _ => panic!("should exist three items in operand_stack"),
                };
            }
            Instruction::AstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::Putstatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, field_name) = self.get_class_and_field_name(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);

                let operand_stack = self.get_operand_stack();
                let (first, second) = match operand_stack.pop() {
                    Some(second @ Item::Long(_)) => {
                        let first = operand_stack.pop().unwrap();
                        (first, second)
                    }
                    first @ _ => (first.unwrap(), Item::Null),
                };
                self.static_fields
                    .insert((class_name, field_name), (first, second));
            }
            Instruction::Getstatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, field_name) = self.get_class_and_field_name(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);

                let err_message = format!(
                    "Getstatic failed. {}.{} is not found",
                    &class_name, &field_name
                );
                let items = self
                    .static_fields
                    .get_mut(&(class_name, field_name))
                    .expect(&err_message)
                    .clone();

                let operand_stack = self.get_operand_stack();
                match items.0 {
                    Item::Long(_) => {
                        operand_stack.push(items.0);
                        operand_stack.push(items.1);
                    }
                    _ => operand_stack.push(items.0),
                };
            }
            Instruction::Areturn | Instruction::Ireturn => {
                let operand_stack = self.get_operand_stack();
                let item = if let Some(item) = operand_stack.pop() {
                    operand_stack.clear();
                    item
                } else {
                    unreachable!("should exist return value on operand_stack")
                };
                let length = self.stack_frames.len();
                if let Some(stackframe) = self.stack_frames.get_mut(length - 2) {
                    stackframe.operand_stack.stack.push(item);
                } else {
                    unreachable!("should exist over two stack_frame");
                }
                return (true, index);
            }
            Instruction::Pop => {
                let operand_stack = self.get_operand_stack();
                operand_stack.pop();
            }
            Instruction::Dup => {
                let operand_stack = self.get_operand_stack();
                let last = if let Some(last) = operand_stack.last() {
                    last.clone()
                } else {
                    unreachable!("should have an item at least");
                };
                operand_stack.push(last);
            }
            Instruction::Invokevirtual(index) | Instruction::Invokespecial(index) => {
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
                self.call_method(&class_file, class_name, name_and_type);
            }
            Instruction::Invokestatic(index) => {
                let this_class_name = class_file.this_class_name();
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
                self.initilize_class_static_info(&this_class_name, &class_name);
                self.call_method(&class_file, class_name, name_and_type);
            }
            Instruction::Putfield(index) => {
                let (class_name, field_name) = class_file.cp_info.get_class_and_field_name(*index);
                let vals = self.get_field_tupple();

                let item = self
                    .get_operand_stack()
                    .pop()
                    .expect("should exist item in operand_stack");
                let obj_id = match item {
                    Item::Objectref(obj_id) => {
                        let obj_ref = self
                            .object_map
                            .get_mut(&obj_id)
                            .expect("should exist object_ref in object_map");
                        assert!(
                            class_name == obj_ref.class_name,
                            "should be equal class_name"
                        );
                        obj_ref
                            .field_map
                            .borrow_mut()
                            .insert((field_name.clone(), obj_id), vals);
                        obj_id
                    }
                    item @ _ => unreachable!("should be Objectref. actual: {}", item),
                };
                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Objectref(obj_id));
            }
            Instruction::Getfield(index) => {
                let (class_name, field_name) = class_file.cp_info.get_class_and_field_name(*index);

                let item = self
                    .get_operand_stack()
                    .pop()
                    .expect("should exist item in operand_stack");
                let (first, second) = match item {
                    Item::Objectref(obj_id) => {
                        let obj_ref = self
                            .object_map
                            .get(&obj_id)
                            .expect("should exist objectref in object_map");

                        assert!(
                            class_name == obj_ref.class_name,
                            "should be equal class_name"
                        );
                        let field_map = obj_ref.field_map.borrow();
                        let (first, second) = field_map
                            .get(&(field_name, obj_id))
                            .expect("should exist item")
                            .clone();
                        (first, second)
                    }
                    item @ _ => unreachable!("should be Objectref. actual: {}", item),
                };

                let operand_stack = self.get_operand_stack();
                match first {
                    Item::Long(_) => {
                        operand_stack.push(first);
                        operand_stack.push(second.clone());
                    }
                    item @ _ => {
                        operand_stack.push(item);
                    }
                }
            }
            Instruction::Ldc(index) => {
                let string_val = class_file.cp_info.get_string(*index);
                let operand_stack = self.get_operand_stack();

                operand_stack.push(Item::String(string_val));
            }
            Instruction::Ldc2W(first, second) => {
                let mut operand_stack = self.get_operand_stack();
                class_file.cp_info.create_and_set_operand_stack_item(
                    &mut operand_stack,
                    (*first << 8 | *second) & 0xFFFF,
                );
            }
            Instruction::New(index) => {
                let this_class_name = class_file.this_class_name();
                let class_ref = class_file.cp_info.get_class_ref(*index);
                let class_name = class_file.cp_info.get_utf8(class_ref.name_index);
                self.initilize_class_static_info(&this_class_name, &class_name);

                let (id, object_ref) = if let Some(JavaClass::Custom(target_class)) =
                    self.class_map.get(&class_name)
                {
                    let mut field_map = HashMap::new();
                    for (index, field) in target_class.fields.iter().enumerate() {
                        let field_name = target_class.cp_info.get_utf8(field.name_index);
                        let descriptor = target_class.cp_info.get_utf8(field.descriptor_index);
                        let vals =
                            create_uninitialized_item(&FieldDescriptor::from(descriptor.as_ref()));
                        field_map.insert((field_name, index), vals);
                    }
                    let operand_stack = self.get_operand_stack();
                    let id = *OBJECT_ID.lock().unwrap();
                    *OBJECT_ID.lock().unwrap() = id + 1;

                    operand_stack.push(Item::Objectref(id));
                    (
                        id,
                        Objectref::new(class_name, RefCell::new(field_map), true),
                    )
                } else {
                    unreachable!("not come here")
                };
                self.object_map.insert(id, object_ref);
            }
            Instruction::Newarray(type_index) => {
                let id = *OBJECT_ID.lock().unwrap();
                *OBJECT_ID.lock().unwrap() = id + 1;

                if let Some(Item::Int(length)) = self.get_operand_stack().pop() {
                    let default_array = iniailize_primitive_array(*type_index, length as usize);
                    self.array_map
                        .insert(id, Array::Primitive(RefCell::new(default_array)));
                } else {
                    unreachable!("should exist item in operand_stack")
                }

                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Arrayref(id));
            }
            // class, array, or interface type
            Instruction::Anewarray(index) => {
                let id = *OBJECT_ID.lock().unwrap();
                *OBJECT_ID.lock().unwrap() = id + 1;

                let class_name = class_file.cp_info.get_class_ref_name(*index);
                if let Some(Item::Int(length)) = self.get_operand_stack().pop() {
                    let default_array = initialize_objectref_array(
                        &mut self.object_map,
                        class_name,
                        length as usize,
                    );
                    self.array_map
                        .insert(id, Array::Custom(RefCell::new(default_array)));
                } else {
                    unreachable!("should exist item in operand_stack")
                }

                let operand_stack = self.get_operand_stack();
                operand_stack.push(Item::Arrayref(id));
            }
            Instruction::Return => {
                let operand_stack = self.get_operand_stack();
                operand_stack.clear();
            }
            Instruction::Multianewarray(index, dimentions) => {
                let operand_stack = self.get_operand_stack();
                let operand_stack_len = operand_stack.len();
                let mut counts: Vec<usize> = operand_stack
                    .drain(operand_stack_len - dimentions..operand_stack_len)
                    .map(|item| {
                        if let Item::Int(val) = item {
                            val as usize
                        } else {
                            unreachable!("Item should be int")
                        }
                    })
                    .collect();

                let dimentions = *dimentions;
                let class_array_name = class_file.cp_info.get_class_ref_name(*index);
                let class_name = &class_array_name[dimentions..];
                match &class_name[0..1] {
                    // for class
                    "L" => {
                        let _actual_class_name = &class_name[1..];
                        unimplemented!("need to implement custom class")
                    }
                    discriptor @ _ => {
                        let initial_val =
                            create_uninitialized_item(&FieldDescriptor::from(discriptor));
                        let first_count = counts.first().unwrap().clone();
                        let multi_dimentions_id = create_multi_dimentions_array(
                            &mut self.array_map,
                            &mut counts,
                            0,
                            first_count,
                            initial_val,
                        );

                        let operand_stack = self.get_operand_stack();
                        operand_stack.push(Item::Arrayref(multi_dimentions_id));
                    }
                };
            }
            _ => {}
        };
        (false, index + instruction.counsume_index())
    }

    fn get_field_tupple(&mut self) -> (Item, Item) {
        let operand_stack = self.get_operand_stack();
        let first = operand_stack
            .pop()
            .expect("should exist operand stack item");
        let second = match first {
            Item::Long(_) => operand_stack
                .pop()
                .expect("should exist operand stack item"),
            _ => Item::Null,
        };
        (first, second)
    }

    fn get_last_stackframe(&mut self) -> &mut Stackframe {
        self.stack_frames
            .last_mut()
            .expect("should exist stack_frame")
    }

    fn get_operand_stack(&mut self) -> &mut Vec<Item> {
        let stackframe = self.get_last_stackframe();
        &mut stackframe.operand_stack.stack
    }

    fn call_method(
        &mut self,
        class_file: &Custom,
        class_name: String,
        name_and_type: &ConstantNameAndType,
    ) {
        let method_name = class_file.cp_info.get_utf8(name_and_type.name_index);
        let method_descriptor = class_file.cp_info.get_utf8(name_and_type.descriptor_index);

        if let Some(mut class) = self.class_map.remove(&class_name) {
            self.call_other_class_method(
                &mut class,
                &class_file.cp_info,
                &method_name,
                &method_descriptor,
            );
            self.class_map.insert(class.this_class_name(), class);
        } else {
            let new_class_file = self.create_custom_class(&class_name);
            let mut new_class_file = JavaClass::Custom(new_class_file);

            self.call_other_class_method(
                &mut new_class_file,
                &class_file.cp_info,
                &method_name,
                &method_descriptor,
            );
            self.class_map.insert(class_name, new_class_file);
        }
    }

    fn initilize_class_static_info(&mut self, this_class_name: &str, class_name: &str) {
        if this_class_name != class_name && self.class_map.get_mut(class_name).is_none() {
            let new_class_file = self.create_custom_class(&class_name);
            if let Some(code) = new_class_file.get_clinit_code() {
                self.call_custom_class_method(&new_class_file, code);
            }

            self.class_map
                .insert(class_name.to_string(), JavaClass::Custom(new_class_file));
        }
    }

    fn create_custom_class(&mut self, class_name: &str) -> Custom {
        let class_name = class_name.to_string() + ".class";
        let class_path = Path::new(self.root_path).join(&class_name);
        let mut buffer = vec![];
        let buffer = read_file(&class_path, &mut buffer).expect(&format!(
            "need to add handler for the case failed to find the class file: {}",
            class_name
        ));
        let (new_class_file, _pc_count) = Custom::new(buffer, 0);
        // TBD should be set initial value
        set_static_fields(&new_class_file, &mut self.static_fields);
        new_class_file
    }

    fn call_other_class_method(
        &mut self,
        class_file: &mut JavaClass,
        caller_cp_info: &ConstantPool,
        method_name: &str,
        method_descriptor: &str,
    ) {
        match class_file {
            JavaClass::BuiltIn(ref mut builtin_class) => {
                let method = builtin_class.methods.get_mut(method_name).expect(&format!(
                    "{} is not found in {}",
                    method_name, builtin_class.class_name
                ));
                let parameter_length = method.parameter_length(&method_descriptor);
                let stack_frame = self.create_new_stack_frame(parameter_length);
                self.stack_frames.push(stack_frame);
                method.execute(&caller_cp_info, &mut self.stack_frames);
            }
            JavaClass::Custom(ref custom_class) => {
                if let Some(method_code) =
                    custom_class.get_method_code_by_string(method_name, method_descriptor)
                {
                    self.call_custom_class_method(custom_class, &method_code);
                }
            }
        }
    }

    fn call_custom_class_method(&mut self, class: &Custom, code: &Code) {
        let local_variable_length = code.max_locals as usize;
        let stack_frame = self.create_new_stack_frame(local_variable_length);
        self.stack_frames.push(stack_frame);
        self.run_method(class, code);
    }

    fn load_n(&mut self, index: usize) {
        let stackframe = self.get_last_stackframe();
        let value = stackframe
            .local_variables
            .get(index)
            .expect("should exist local variable");
        stackframe
            .operand_stack
            .stack
            .push(Item::from(value.clone()));
    }

    fn store_n(&mut self, indexs: &[usize]) {
        let index_size = indexs.len();
        let mut item_vec = Vec::with_capacity(index_size);
        let stackframe = self.get_last_stackframe();
        for i in 0..index_size {
            let item = stackframe
                .operand_stack
                .stack
                .pop()
                .expect("should have item in operand_stack");
            item_vec.push((indexs[i], item));
        }
        item_vec.sort_by(|before, after| {
            if before.0 > after.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for (index, item) in item_vec.into_iter() {
            if stackframe.local_variables.get(index).is_some() {
                stackframe.local_variables[index] = Item::from(item);
            } else {
                stackframe.local_variables.insert(index, Item::from(item));
            }
        }
    }

    // (class_name, field_name)
    fn get_class_and_field_name(&mut self, class_file: &Custom, index: usize) -> (String, String) {
        let field_ref = class_file.cp_info.get_field_ref(index);
        let class_ref = class_file.cp_info.get_class_ref(field_ref.class_index);
        let name_and_type = class_file
            .cp_info
            .get_name_and_type(field_ref.name_and_type_index);
        (
            class_file.cp_info.get_utf8(class_ref.name_index),
            class_file.cp_info.get_utf8(name_and_type.name_index),
        )
    }

    fn get_related_method_info<'b>(
        &mut self,
        class_file: &'b Custom,
        index: usize,
    ) -> (String, &'b ConstantNameAndType) {
        let method_ref = class_file.cp_info.get_method_ref(index);
        let class_ref = class_file.cp_info.get_class_ref(method_ref.class_index);
        let name_and_type = class_file
            .cp_info
            .get_name_and_type(method_ref.name_and_type_index);
        let class_name = class_file.cp_info.get_utf8(class_ref.name_index);
        (class_name, name_and_type)
    }

    fn create_new_stack_frame(&mut self, local_variable_length: usize) -> Stackframe {
        let mut new_stack_frame = Stackframe::new(local_variable_length);
        let stackframe = self.get_last_stackframe();

        // TBD need to fix this
        let mut variables: Vec<_> = stackframe
            .operand_stack
            .stack
            .iter()
            .rev()
            .map(|operand_item| Item::from(operand_item.clone()))
            .collect();
        let mut variables = variables.drain(0..local_variable_length).rev().collect();
        new_stack_frame.local_variables.append(&mut variables);

        // TBD need to fix this
        for _ in 0..local_variable_length {
            let _ = stackframe.operand_stack.stack.pop();
        }

        new_stack_frame
    }
}

pub fn set_static_fields(class: &Custom, static_fields: &mut StaticFields) {
    for field in class.fields.iter() {
        let field_name = class.cp_info.get_utf8(field.name_index);
        let value = create_uninitialized_item(&class.get_descriptor(field.descriptor_index));
        static_fields.insert((class.this_class_name(), field_name), value);
    }
}

pub fn setup_static_fields(class_map: &ClassMap) -> StaticFields {
    let mut static_fields = HashMap::new();
    for key in class_map.keys() {
        if let Some(JavaClass::Custom(class)) = class_map.get(key) {
            set_static_fields(&class, &mut static_fields);
        }
    }

    static_fields.insert(
        (String::from("java/lang/System"), String::from("out")),
        (
            Item::Classref(String::from("java/io/PrintStream")),
            Item::Null,
        ),
    );

    static_fields
}

// TBD need to create system to express uninitialized value
pub fn create_uninitialized_item(descriptor: &FieldDescriptor) -> (Item, Item) {
    match descriptor {
        FieldDescriptor::BaseType(BaseType::I) => (Item::Int(0), Item::Null),
        FieldDescriptor::BaseType(BaseType::J) => (Item::Long(0), Item::Long(0)),
        FieldDescriptor::BaseType(BaseType::Z) => (Item::Boolean(true), Item::Null),
        _ => unimplemented!("should implement"),
    }
}

pub fn create_multi_dimentions_array(
    array_map: &mut ArrayMap,
    counts: &mut Vec<usize>,
    current_index: usize,
    current_size: usize,
    initial_value: (Item, Item),
) -> usize {
    let next_size = if let Some(next_size) = counts.get_mut(current_index) {
        next_size.clone()
    } else {
        // leaf child
        let mut items = Vec::with_capacity(current_size);
        for _ in 0..current_size {
            items.push(initial_value.clone());
        }
        let id = *OBJECT_ID.lock().unwrap();
        *OBJECT_ID.lock().unwrap() = id + 1;
        array_map.insert(id, Array::Primitive(RefCell::new(items)));
        return id;
    };

    let mut ids = Vec::with_capacity(current_size);
    for _ in 0..current_size {
        let input_id = create_multi_dimentions_array(
            array_map,
            counts,
            current_index + 1,
            next_size,
            initial_value.clone(),
        );
        ids.push(input_id);
    }

    let id = *OBJECT_ID.lock().unwrap();
    *OBJECT_ID.lock().unwrap() = id + 1;
    array_map.insert(id, Array::Array(RefCell::new(ids)));

    id
}
