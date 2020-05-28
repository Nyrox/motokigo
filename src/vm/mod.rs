use std::mem;

use crate::compiler::program_data::ProgramData;

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    ConstF32,
    Void,
    Mov4,
    Load4,
    Mov4Global,
    Load4Global,

    Ret,
    Call,
    CallBuiltIn,
    Jmp,
    JmpIf,

    LenPlaceholder,
}

#[derive(Clone)]
pub struct MemoryCell {
    pub data: u32,
}

impl std::fmt::Debug for MemoryCell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (op, ax) = self.get_inst();

        let mut fmt = fmt.debug_tuple("MemoryCell");

        if op.is_some() {
            fmt.field(&op.unwrap());
            fmt.field(&ax);
        }
        fmt.finish()
    }
}

impl MemoryCell {
    pub fn raw(data: u32) -> Self {
        MemoryCell { data }
    }

    pub fn plain_inst(inst: OpCode) -> Self {
        MemoryCell {
            data: inst as u16 as u32,
        }
    }

    pub fn with_data(inst: OpCode, data: u16) -> Self {
        MemoryCell {
            data: (inst as u16 as u32) | ((data as u32) << 16),
        }
    }

    pub fn get_inst(&self) -> (Option<OpCode>, u16) {
        let op = self.data as u16;
        let op = if op < OpCode::LenPlaceholder as u16 {
            Some(unsafe { std::mem::transmute(op) })
        } else {
            None
        };

        (op, (self.data >> 16) as u16)
    }
}

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

pub struct VirtualMachine<'a> {
    program: &'a VMProgram,
    pub stack: Vec<u8>,
    isp: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(program: &'a VMProgram) -> VirtualMachine<'a> {
        VirtualMachine {
            program,
            stack: vec![0; program.data.min_stack_size],
            isp: 0,
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

    pub fn run_fn(&mut self, id: &str) {
        let fnc = self.program.data.functions.get(id).unwrap();
        self.isp = fnc.address.unwrap();
        let mut depth = 0;

        let mut stack_base = self.stack.len();

        loop {
            let (op, p) = self.program.code[self.isp].get_inst();
            self.isp = self.isp + 1;

            match op.unwrap() {
                OpCode::Call => {
                    depth += 1;
                    self.push_stack_raw(self.isp as u32);
                    self.push_stack_raw(stack_base as u32);
                    stack_base = self.stack.len();
                    self.isp = p as usize;
                }
                OpCode::CallBuiltIn => {
                    crate::builtins::call_builtin_fn(p as usize, self);
                }
                OpCode::ConstF32 => {
                    self.push_stack_raw(self.program.code[self.isp].data);
                    self.isp = self.isp + 1;
                }
                OpCode::Mov4 => unsafe {
                    let val = self.pop_stack::<u32>();
                    self.write_stack(stack_base + p as usize, val);
                },
                OpCode::Mov4Global => unsafe {
                    let val = self.pop_stack::<u32>();
                    self.write_stack(p as usize, val);
                },
                OpCode::Load4 => unsafe {
                    let val = self.load_stack::<u32>(stack_base + p as usize);
                    self.push_stack_raw(std::mem::transmute(val));
                },
                OpCode::Load4Global => unsafe {
                    let val = self.load_stack::<u32>(p as usize);
                    self.push_stack_raw(std::mem::transmute(val));
                },
                OpCode::Void => self.push_stack_raw(0),
                OpCode::Ret => unsafe {
                    // we need to figure out the amount of bytes to buffer
                    // as the return value. we can retrieve this by seeing how many
                    // bytes the vm has on it's current stack frame, compared to the
                    // declared stack-length of the Ret-Parameter.
                    let frame_len = self.stack.len() - stack_base;
                    let rv_len = frame_len - p as usize;

                    let rv = self.pop_bytes(rv_len);

                    // now we pop locals off the stack
                    for _ in 0..p {
                        self.pop_bytes(0);
                    }

                    // check if we are in main, if so we are done
                    if depth == 0 {
                        self.push_bytes(&rv);
                        return;
                    }
                    // restore previous stack frame
                    stack_base = self.pop_stack::<u32>() as usize;
                    self.isp = self.pop_stack::<u32>() as usize;

                    self.push_bytes(&rv);
                },
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

        let _ = self.stack.split_off(self.stack.len() - mem::size_of::<T>());
        v
    }
}
