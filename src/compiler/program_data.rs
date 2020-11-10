use crate::ast::TypeKind;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SymbolMeta {
	pub type_kind: TypeKind,
	pub stack_offset: Option<usize>,
	pub is_static: bool,
	pub is_mutable: bool,
}

#[derive(Clone, Debug)]
pub struct FuncMeta {
	pub symbols: HashMap<String, SymbolMeta>,
	pub address: Option<usize>,
	pub stack_offset: usize,
	pub return_type: Option<TypeKind>,
	pub param_types: Vec<TypeKind>,
}

impl FuncMeta {
	pub fn new() -> Self {
		FuncMeta {
			symbols: HashMap::new(),
			address: None,
			stack_offset: 0,
			return_type: None,
			param_types: Vec::new(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ProgramData {
	pub functions: HashMap<String, FuncMeta>,
	pub global_symbols: HashMap<String, SymbolMeta>,
	pub static_section_size: usize,
}

impl ProgramData {
	pub fn new() -> Self {
		ProgramData {
			functions: HashMap::new(),
			global_symbols: HashMap::new(),
			static_section_size: 0,
		}
	}
}
