#![feature(const_generics)]

pub mod ast;
pub mod builtins;
pub mod compiler;
pub mod glsl;
pub mod parser;
pub mod scanner;
pub mod vm;


mod tests {
	use super::*;


	#[test]
	pub fn test_everything() {
		let programs = [
			"./examples/basic.mk",
		];

		for p in programs.iter() {
			let file = std::fs::read_to_string(p).unwrap();
			let program = parser::parse(file);
			let program = compiler::compile(program);

		}
	}
}
