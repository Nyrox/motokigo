use super::*;
use crate::compiler::program_data::ProgramData;
use std::mem;

#[derive(Debug, Clone)]
pub struct VMProgram {
    pub code: Vec<MemoryCell>,
    pub data: ProgramData,
}

impl VMProgram {
    pub fn new() -> Self {
        VMProgram {
            code: Vec::new(),
            data: ProgramData::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    return_addr: usize,
    stack_base: usize,
}

#[derive(Clone)]
pub struct VirtualMachine<'a> {
    pub program: &'a VMProgram,
    pub stack: Vec<u8>,
    pub call_stack: Vec<StackFrame>,
    pub isp: usize,
    pub stack_base: usize,
    pub breakpoints: Vec<u16>,
}

const INITIAL_STACK_CAPACITY: usize = 128; // should be large enough?

impl<'a> VirtualMachine<'a> {
    pub fn new(program: &'a VMProgram) -> VirtualMachine<'a> {
        let mut stack =
            Vec::with_capacity(program.data.static_section_size + INITIAL_STACK_CAPACITY);
        stack.resize(program.data.static_section_size, 0);

        VirtualMachine {
            program,
            stack,
            isp: 0,
            call_stack: Vec::with_capacity(8), // should always be enough
            stack_base: program.data.static_section_size,
            breakpoints: vec![],
        }
    }

    pub fn set_in_float(&mut self, ident: &str, val: f32) {
        let offset = self
            .program
            .data
            .global_symbols
            .get(ident)
            .unwrap()
            .stack_offset
            .unwrap();

        unsafe {
            self.write_stack(offset, val);
        }
    }

    pub fn set_global<T: bytemuck::Pod>(&mut self, ident: &str, val: T) {
        let offset = self
            .program
            .data
            .global_symbols
            .get(ident)
            .unwrap()
            .stack_offset
            .unwrap();

        unsafe {
            self.write_stack(offset, val);
        }
    }

    pub fn get_global<T: bytemuck::Pod>(&mut self, ident: &str) -> T {
        let offset = self
            .program
            .data
            .global_symbols
            .get(ident)
            .unwrap()
            .stack_offset
            .unwrap();

        unsafe { self.load_stack(offset) }
    }

    pub fn get_out_float(&mut self, ident: &str) -> f32 {
        let offset = self
            .program
            .data
            .global_symbols
            .get(ident)
            .unwrap()
            .stack_offset
            .unwrap();

        unsafe { self.load_stack(offset) }
    }

    pub fn run_fn(mut self, id: &str, breakpoints: Vec<u16>) -> VMState<'a> {
        let fnc = self.program.data.functions.get(id).unwrap();
        self.isp = fnc.address.unwrap();
        self.stack_base = self.stack.len();
        self.breakpoints = breakpoints;

        self.resume()
    }

    pub fn resume(mut self) -> VMState<'a> {
        loop {
            let (op, p) = self.program.code[self.isp].get_inst();
            self.isp = self.isp + 1;

            match op.unwrap() {
                OpCode::StmtMarker => {
                    if self.breakpoints.contains(&p) {
                        return VMState::BreakpointEncountered(VMBreakpointState(self, p));
                    }
                }
                OpCode::Call => {
                    self.call_stack.push(StackFrame {
                        return_addr: self.isp + 1,
                        stack_base: self.stack_base,
                    });

                    self.stack_base = self.stack.len() - self.program.code[self.isp].data as usize;
                    self.isp = p as usize;
                }
                OpCode::JmpZero => {
                    let cond: u32 = unsafe { self.pop_stack() };
                    if cond == 0u32 {
                        self.isp = p as usize;
                    }
                }
                OpCode::JmpNotZero => {
                    let cond: u32 = unsafe { self.pop_stack() };
                    if cond != 0u32 {
                        self.isp = p as usize;
                    }
                }
                OpCode::Jmp => {
                    self.isp = p as usize;
                }
                OpCode::CallBuiltIn => {
                    crate::builtins::call_builtin_fn(p as usize, &mut self);
                }
                OpCode::Const4 => {
                    self.push_stack_raw(self.program.code[self.isp].data);
                    self.isp = self.isp + 1;
                }
                OpCode::Mov4 => unsafe {
                    let val = self.pop_stack::<u32>();
                    self.write_stack(self.stack_base + p as usize, val);
                },
                OpCode::Mov4Global => unsafe {
                    let val = self.pop_stack::<u32>();
                    self.write_stack(p as usize, val);
                },
                OpCode::Load4 => unsafe {
                    let val = self.load_stack::<u32>(self.stack_base + p as usize);
                    self.push_stack_raw(std::mem::transmute(val));
                },
                OpCode::Load4Global => unsafe {
                    let val = self.load_stack::<u32>(p as usize);
                    self.push_stack_raw(std::mem::transmute(val));
                },
                OpCode::Void => self.push_stack_raw(0),
                OpCode::Ret => {
                    // we need to figure out the amount of bytes to buffer
                    // as the return value. we can retrieve this by seeing how many
                    // bytes the vm has on it's current stack frame, compared to the
                    // declared stack-length of the Ret-Parameter.
                    let frame_len = self.stack.len() - self.stack_base;
                    let rv_len = p as usize;

                    let rv = self.pop_bytes(rv_len);

                    // now we pop locals off the stack
                    self.pop_bytes(frame_len - p as usize);

                    self.push_bytes(&rv);

                    if let Some(sf) = self.call_stack.pop() {
                        // ret
                        self.stack_base = sf.stack_base;
                        self.isp = sf.return_addr;
                    } else {
                        // returned from main
                        return VMState::VMRunFinished(VMRunFinishedState(self));
                    }
                }
                o => unimplemented!("{:?}", o),
            }
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.stack.extend_from_slice(bytes);
    }

    pub fn pop_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
        self.stack.drain((self.stack.len() - num_bytes)..).collect()
    }

    pub fn read_stack_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.stack[offset..(offset + len)]
    }

    pub unsafe fn write_stack<T: bytemuck::Pod>(&mut self, offset: usize, val: T) {
        let mut ptr = self.stack.as_mut_ptr().offset(offset as isize);

        for v in bytemuck::bytes_of(&val) {
            *ptr = *v;
            ptr = ptr.offset(1);
        }
    }

    pub unsafe fn load_stack<T: bytemuck::Pod>(&mut self, offset: usize) -> T {
        let ptr = self.stack.as_ptr().offset(offset as isize) as *const T;
        *ptr
    }

    pub fn push_stack<T: bytemuck::Pod>(&mut self, data: T) {
        self.stack.extend_from_slice(bytemuck::bytes_of(&data));
    }

    pub fn push_stack_raw(&mut self, data: u32) {
        self.stack.extend_from_slice(bytemuck::bytes_of(&data));
    }

    pub unsafe fn pop_stack<T>(&mut self) -> T
    where
        T: Copy + std::fmt::Debug,
    {
        let ptr = self
            .stack
            .as_ptr()
            .offset((self.stack.len() - mem::size_of::<T>()) as isize);
        let v = *(ptr as *const T);

        self.stack.resize(self.stack.len() - mem::size_of::<T>(), 0);
        v
    }
}
