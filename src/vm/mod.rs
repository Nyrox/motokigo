mod machine;
use std::mem;

use crate::ast::TypeKind;

mod exports {
    use super::machine;
    pub use machine::{VMProgram, VirtualMachine};
}
pub use exports::*;

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

    StmtMarker,
    LenPlaceholder,
}

pub struct VMBreakpointState<'a>(VirtualMachine<'a>, u16);
pub struct VMRunFinishedState<'a>(VirtualMachine<'a>);

pub enum VMState<'a> {
    BreakpointEncountered(VMBreakpointState<'a>),
    VMRunFinished(VMRunFinishedState<'a>),
}

impl<'a> VMState<'a> {
    pub fn unwrap_vm(self) -> VirtualMachine<'a> {
        match self {
            VMState::BreakpointEncountered(VMBreakpointState(vm, _)) => vm,
            VMState::VMRunFinished(VMRunFinishedState(vm)) => vm,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackView {
    pub current_fn: String,
    pub symbols: Vec<(String, (TypeKind, Vec<u8>))>,
}

impl<'a> VMBreakpointState<'a> {
    pub fn breakpoint(&self) -> u16 {
        self.1
    }

    pub fn resume(self) -> VMState<'a> {
        self.0.resume()
    }

    pub fn generate_stack_view(&self) -> StackView {
        let data = &self.0.program.data;
        let (current_fn, func_meta) = data
            .functions
            .iter()
            .filter(|(_, func)| func.address.unwrap() < self.0.isp)
            .max_by_key(|(_, f)| f.address.unwrap())
            .unwrap();

        let symbols = func_meta
            .symbols
            .iter()
            .flat_map(|(id, s)| {
                let tk = s.type_kind.clone();

                if self.0.stack.len() > (self.0.stack_base + s.stack_offset.unwrap()) {
                    let bytes = self
                        .0
                        .read_stack_bytes(self.0.stack_base + s.stack_offset.unwrap(), tk.size());

                    Some((id.clone(), (tk, bytes.to_vec())))
                } else {
                    None
                }
            })
            .collect();

        StackView {
            current_fn: current_fn.clone(),
            symbols: symbols,
        }
    }
}

impl<'a> VMRunFinishedState<'a> {
    pub fn reset(self) -> VirtualMachine<'a> {
        // TODO
        self.0
    }
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
            Some(unsafe { mem::transmute(op) })
        } else {
            None
        };

        (op, (self.data >> 16) as u16)
    }
}
