use crate::ast::*;

pub mod compiler;
use compiler::GenerateGLSL;

pub trait BuiltInCallableGLSL {
    fn generate(&self, _g: &mut GenerateGLSL, _a: Vec<String>) -> String;
}

fn get_glsl_type(tk: &TypeKind) -> String {
    match tk {
        TypeKind::F32 => "float".to_owned(),
        TypeKind::Vector(_, size) => format!("vec{}", size),
        TypeKind::Void => "void".to_owned(),
        t => {
            dbg!(t);
            unimplemented!()
        }
    }
}

pub fn generate_glsl(mut program: Program) -> String {
    let mut generator = GenerateGLSL::new();
    generator.consume(program);
    generator.finish()
}
