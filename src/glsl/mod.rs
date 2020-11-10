use crate::ast::*;

pub mod compiler;
use compiler::GenerateGLSL;

pub trait BuiltInCallableGLSL {
	fn generate(&self, _g: &mut GenerateGLSL, _a: Vec<String>) -> String;
}

fn get_glsl_type(tk: &TypeKind) -> String {
	match tk {
		TypeKind::F32 => "float".to_owned(),
		TypeKind::I32 => "int".to_owned(),
		TypeKind::Vector(_, size) => format!("vec{}", size),
		TypeKind::Matrix(_, m, n) => {
			if m == n {
				format!("mat{}", m)
			} else {
				format!("mat{}x{}", m, n)
			}
		}
		TypeKind::Void => "void".to_owned(),
		TypeKind::Struct(s) => s.borrow().ident.item.clone(),
		t => {
			dbg!(t);
			unimplemented!()
		}
	}
}

pub fn generate_glsl(program: Program) -> String {
	let mut generator = GenerateGLSL::new();
	generator.consume(program);
	generator.finish()
}
