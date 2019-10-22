use crate::attribute::instruction::Instruction;
use crate::constant::ConstantNameAndType;
use crate::java_class::{custom::Custom, JavaClass};
use crate::method::Method;
use crate::operand::{OperandStack, OperandStackItem};
use crate::stackframe::{Stackframe, StackframeItem};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
pub struct Context {
    pub class_map: HashMap<String, JavaClass>,
    pub operand_stack: OperandStack,
    pub program_count: usize,
    pub stack_frames: Vec<Stackframe>,
}

impl Context {
    pub fn new(class_map: HashMap<String, JavaClass>) -> Context {
        Context {
            class_map,
            operand_stack: OperandStack::new(),
            program_count: 0,
            stack_frames: vec![],
        }
    }

    pub fn run_entry_file(&mut self, class_file: Custom) {
        let entry_method = if let Some(entry_method) = class_file.get_entry_method() {
            entry_method
        } else {
            unimplemented!("add handler in the case failed to find entry method")
        };

        // TBD Perhaps this method is not invoked from super_class
        let super_class_index = class_file.super_class;
        let stack_frame_item_0 = StackframeItem::Classref(super_class_index);
        self.run_method(&class_file, entry_method, stack_frame_item_0);

        self.class_map
            .insert(class_file.this_class_name(), JavaClass::Custom(class_file));
    }

    fn run_method(
        &mut self,
        class_file: &Custom,
        method: &Method,
        stack_frame_item: StackframeItem,
    ) {
        if let Some(code) = method.extract_code() {
            let mut stack_frame = Stackframe::new(code.max_locals as usize);
            stack_frame.local_variables.push(stack_frame_item);
            self.stack_frames.push(stack_frame);
            let mut index = 0;
            while let Some(instruction) = code.code.get(index) {
                // println!("{}", instruction);
                let (should_finish, update_index) = self.execute(class_file, instruction, index);
                if should_finish {
                    break;
                }
                index = update_index + 1;
            }
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
                let item = self.operand_stack.iadd();
                self.operand_stack.stack.push(item);
            }
            Instruction::Isub => {
                let item = self.operand_stack.isub();
                self.operand_stack.stack.push(item);
            }
            Instruction::Imul => {
                let item = self.operand_stack.imul();
                self.operand_stack.stack.push(item);
            }
            Instruction::Idiv => {
                let item = self.operand_stack.idiv();
                self.operand_stack.stack.push(item);
            }
            Instruction::Irem => {
                let item = self.operand_stack.irem();
                self.operand_stack.stack.push(item);
            }
            Instruction::IconstN(val) => {
                self.operand_stack
                    .iconst(OperandStackItem::Int(*val as i32));
            }
            // maybe need to fix for float or something like that
            Instruction::Bipush(val) => {
                self.operand_stack
                    .iconst(OperandStackItem::Int(*val as i32));
            }
            Instruction::Goto(pointer) => {
                return (false, *pointer);
            }
            Instruction::Iinc(index, value) => {
                if let Some(stack_frame) = self.stack_frames.last_mut() {
                    if let Some(item) = stack_frame.local_variables.get_mut(*index) {
                        if let StackframeItem::Int(val) = item {
                            mem::replace(val, *val + *value as i32);
                        }
                    }
                }
            }
            Instruction::Ifeq(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val == OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifne(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val != OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Iflt(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val < OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifge(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val >= OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifgt(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val > OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifle(if_val, else_val) => {
                let val = self.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val <= OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ificmpeq(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first == second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpne(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first != second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmplt(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first < second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpge(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first >= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpgt(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first > second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmple(if_val, else_val) => {
                let second = self.operand_stack.stack.pop();
                let first = self.operand_stack.stack.pop();
                let jump_pointer = if first <= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::IloadN(index) => {
                self.load_n(instruction, *index);
            }
            Instruction::LloadN(index) => {
                let base_index = *index;
                self.load_n(instruction, base_index);
                self.load_n(instruction, base_index + 1);
            }
            Instruction::IstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::LstoreN(index) => {
                let base_index = *index;
                self.store_n(&[base_index + 1, base_index]);
            }
            Instruction::AloadN(index) => {
                if let Some(stack_frame) = self.stack_frames.last() {
                    let value = &stack_frame.local_variables[*index];
                    self.operand_stack.stack.push(OperandStackItem::from(value));
                }
            }
            Instruction::Getstatic(index) => {
                let item = class_file.cp_info.get_operand_stack_item(*index);
                self.operand_stack.stack.push(item);
            }
            Instruction::Ireturn => {
                if let Some(item) = self.operand_stack.stack.pop() {
                    self.operand_stack.stack.clear();
                    self.operand_stack.stack.push(item);
                } else {
                    unreachable!("should exist return value on operand_stack");
                }
                return (true, index);
            }
            Instruction::Pop => {
                self.operand_stack.stack.pop();
            }
            Instruction::Invokevirtual(index)
            | Instruction::Invokespecial(index)
            | Instruction::Invokestatic(index) => {
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
                if let Some(mut class) = self.class_map.remove(&class_name) {
                    match class {
                        JavaClass::BuiltIn(ref mut builtin_class) => {
                            let method_name = class_file.cp_info.get_utf8(name_and_type.name_index);
                            let _method_descriptor =
                                class_file.cp_info.get_utf8(name_and_type.descriptor_index);
                            // TBD: should use method_descriptor
                            if let Some(method) = builtin_class.methods.get_mut(&method_name) {
                                let mut stack_frame =
                                    self.create_new_stack_frame(method.max_locals);
                                method.execute(
                                    &class_file.cp_info,
                                    &mut stack_frame,
                                    &mut self.operand_stack,
                                );
                            } else {
                                unreachable!(
                                    "{} is not found in {}",
                                    method_name, builtin_class.class_name
                                );
                            }
                        }
                        JavaClass::Custom(ref custom_class) => {
                            if let Some(method_code) = custom_class.get_method_code(
                                name_and_type.name_index,
                                name_and_type.descriptor_index,
                            ) {
                                let local_variable_length = method_code.max_locals as usize;
                                let mut _stack_frame =
                                    self.create_new_stack_frame(local_variable_length);
                            }
                        }
                    }
                    self.class_map.insert(class.this_class_name(), class);
                } else {
                    // TBD: I guess need to read the new other.class
                    unreachable!("{} is not found in class_map", class_name)
                }
            }
            Instruction::Ldc(index) => {
                let string_val = class_file.cp_info.get_string(*index);
                self.operand_stack
                    .stack
                    .push(OperandStackItem::String(string_val));
            }
            Instruction::Ldc2W(first, second) => {
                let item = class_file
                    .cp_info
                    .get_operand_stack_item((*first << 8 | *second) & 0xFFFF);
                self.operand_stack.stack.push(item);
            }
            _ => {}
        };
        (false, index + instruction.counsume_index())
    }

    fn load_n(&mut self, instruction: &Instruction, index: usize) {
        if let Some(stack_frame) = self.stack_frames.last() {
            let value = &stack_frame.local_variables[index];
            self.operand_stack.stack.push(OperandStackItem::from(value));
        } else {
            unreachable!("order: {}, should find item in {}", instruction, index);
        }
    }

    fn store_n(&mut self, indexs: &[usize]) {
        let index_size = indexs.len();
        let mut item_vec = Vec::with_capacity(index_size);
        if let Some(stack_frame) = self.stack_frames.last_mut() {
            for i in 0..index_size {
                let item = if let Some(item) = self.operand_stack.stack.pop() {
                    item
                } else {
                    unreachable!("should have item in operand_stack")
                };
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
                if stack_frame.local_variables.get(index).is_some() {
                    stack_frame.local_variables[index] = StackframeItem::from(item);
                } else {
                    stack_frame
                        .local_variables
                        .insert(index, StackframeItem::from(item));
                }
            }
        }
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
        let stack_length = self.operand_stack.stack.len();
        let mut variables: Vec<_> = self
            .operand_stack
            .stack
            .drain((stack_length - local_variable_length)..stack_length)
            .into_iter()
            .map(|operand_item| StackframeItem::from(operand_item))
            .collect();
        new_stack_frame.local_variables.append(&mut variables);
        new_stack_frame
    }

    // Instruction::AloadN(val) => write!(f, "aload_{}", val),
    // Instruction::Return => write!(f, "return"),
    // Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
    // Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
}
