use crate::attribute::code::Code;
use crate::attribute::instruction::Instruction;
use crate::constant::{ConstantNameAndType, ConstantPool};
use crate::java_class::{custom::Custom, JavaClass};
use crate::operand::OperandStackItem;
use crate::option::RJ_OPTION;
use crate::stackframe::{Stackframe, StackframeItem};
use crate::utils::read_file;
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
}

impl<'a> Context<'a> {
    pub fn new(class_map: HashMap<String, JavaClass>, root_path: &'a str) -> Context {
        Context {
            class_map,
            program_count: 0,
            stack_frames: vec![],
            root_path,
        }
    }

    pub fn run_entry_file(&mut self, class_file: Custom) {
        let entry_method = class_file
            .get_entry_method()
            .expect("add handler in the case failed to find entry method");

        // TBD Perhaps this method is not invoked from super_class
        let super_class_index = class_file.super_class;
        let stack_frame_item_0 = StackframeItem::Classref(super_class_index);

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
            if RJ_OPTION.lock().unwrap().is_debug {
                println!("{}", instruction);
            }
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
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.iadd();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Ladd => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                if let OperandStackItem::Long(val) = stackframe.operand_stack.ladd() {
                    // TBD should fix this
                    let first = 0;
                    let second = val & 0xFFFFFFFF;
                    stackframe
                        .operand_stack
                        .stack
                        .push(OperandStackItem::Long(first));
                    stackframe
                        .operand_stack
                        .stack
                        .push(OperandStackItem::Long(second));
                }
            }
            Instruction::Isub => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.isub();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Imul => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.imul();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Idiv => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.idiv();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::Irem => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = stackframe.operand_stack.irem();
                stackframe.operand_stack.stack.push(item);
            }
            Instruction::IconstN(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Int(*val as i32));
            }
            Instruction::LconstN(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Long(0));
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Long(*val as i64));
            }
            // maybe need to fix for float or something like that
            Instruction::Bipush(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Int(*val as i32));
            }
            Instruction::Sipush(val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Int(*val as i32));
            }
            Instruction::Goto(pointer) => {
                return (false, *pointer);
            }
            Instruction::Iinc(index, value) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                if let Some(item) = stackframe.local_variables.get_mut(*index) {
                    if let StackframeItem::Int(val) = item {
                        mem::replace(val, *val + *value as i32);
                    }
                }
            }
            Instruction::Ifeq(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val == OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifne(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val != OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Iflt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val < OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifge(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val >= OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifgt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val > OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ifle(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let val = stackframe.operand_stack.stack.pop().unwrap();
                let jump_pointer = if val <= OperandStackItem::Int(0) {
                    *if_val
                } else {
                    *else_val
                };
                return (false, jump_pointer);
            }
            Instruction::Ificmpeq(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first == second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpne(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first != second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmplt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first < second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpge(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first >= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmpgt(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first > second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::Ificmple(if_val, else_val) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let second = stackframe.operand_stack.stack.pop();
                let first = stackframe.operand_stack.stack.pop();
                let jump_pointer = if first <= second { *if_val } else { *else_val };
                return (false, jump_pointer);
            }
            Instruction::IloadN(index) => {
                self.load_n(*index);
            }
            Instruction::LloadN(index) => {
                let base_index = *index;
                self.load_n(base_index);
                self.load_n(base_index + 1);
            }
            Instruction::IstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::LstoreN(index) => {
                let base_index = *index;
                self.store_n(&[base_index + 1, base_index]);
            }
            Instruction::AloadN(index) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let value = &stackframe.local_variables[*index];
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::from(value));
            }
            Instruction::AstoreN(index) => {
                self.store_n(&[*index]);
            }
            Instruction::Getstatic(index) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                class_file
                    .cp_info
                    .create_and_set_operand_stack_item(&mut stackframe.operand_stack.stack, *index);
            }
            Instruction::Areturn | Instruction::Ireturn => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let item = if let Some(item) = stackframe.operand_stack.stack.pop() {
                    stackframe.operand_stack.stack.clear();
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
                if let Some(stackframe) = self.stack_frames.last_mut() {
                    stackframe.operand_stack.stack.pop();
                }
            }
            Instruction::Dup => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                let last = if let Some(last) = stackframe.operand_stack.stack.last() {
                    last.clone()
                } else {
                    unreachable!("should have an item at least");
                };
                stackframe.operand_stack.stack.push(last);
            }
            Instruction::Invokevirtual(index)
            | Instruction::Invokespecial(index)
            | Instruction::Invokestatic(index) => {
                let (class_name, name_and_type) = self.get_related_method_info(class_file, *index);
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
                    let class_name = class_name.to_string() + ".class";
                    let class_path = Path::new(self.root_path).join(&class_name);
                    if let Ok(buffer) = read_file(&class_path, &mut vec![]) {
                        let (new_class_file, _pc_count) = Custom::new(buffer, 0);
                        let mut new_class_file = JavaClass::Custom(new_class_file);

                        self.call_other_class_method(
                            &mut new_class_file,
                            &class_file.cp_info,
                            &method_name,
                            &method_descriptor,
                        );
                        self.class_map
                            .insert(class_name.to_string(), new_class_file);
                    } else {
                        unimplemented!(
                            "need to add handler for the case failed to find the class file: {}",
                            class_name
                        )
                    }
                }
            }
            Instruction::Ldc(index) => {
                let string_val = class_file.cp_info.get_string(*index);
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::String(string_val));
            }
            Instruction::Ldc2W(first, second) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                class_file.cp_info.create_and_set_operand_stack_item(
                    &mut stackframe.operand_stack.stack,
                    (*first << 8 | *second) & 0xFFFF,
                );
            }
            Instruction::New(index) => {
                let stackframe = self
                    .stack_frames
                    .last_mut()
                    .expect("should exist stack_frame");
                // TBD need to implement correctly
                stackframe
                    .operand_stack
                    .stack
                    .push(OperandStackItem::Objectref(*index));
            }
            Instruction::Return => {}
            _ => {}
        };
        (false, index + instruction.counsume_index())
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
                    let local_variable_length = method_code.max_locals as usize;
                    let stack_frame = self.create_new_stack_frame(local_variable_length);
                    self.stack_frames.push(stack_frame);
                    self.run_method(custom_class, method_code);
                }
            }
        }
    }

    fn load_n(&mut self, index: usize) {
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");
        let value = &stackframe.local_variables[index];
        stackframe
            .operand_stack
            .stack
            .push(OperandStackItem::from(value));
    }

    fn store_n(&mut self, indexs: &[usize]) {
        let index_size = indexs.len();
        let mut item_vec = Vec::with_capacity(index_size);
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");
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
                stackframe.local_variables[index] = StackframeItem::from(item);
            } else {
                stackframe
                    .local_variables
                    .insert(index, StackframeItem::from(item));
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
        let stackframe = self
            .stack_frames
            .last_mut()
            .expect("should exist stack_frame");
        let mut variables: Vec<_> = stackframe
            .operand_stack
            .stack
            .iter()
            .rev()
            .map(|operand_item| StackframeItem::from(operand_item))
            .collect();
        stackframe.operand_stack.stack.clear();
        new_stack_frame.local_variables.append(&mut variables);

        new_stack_frame
    }

    // Instruction::Getfield(val) => write!(f, "getfield        #{}", val),
    // Instruction::Putfield(val) => write!(f, "putfield        #{}", val),
}
