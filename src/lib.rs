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
        let programs = ["basic.mk"];

        std::fs::create_dir("./debug").ok();

        for p in programs.iter() {
            let file = std::fs::read_to_string(format!("./examples/{}", p)).unwrap();
            let mut program = parser::parse(file).unwrap();
            compiler::resolve_types::resolve(
                &mut program,
                &mut compiler::program_data::ProgramData::new(),
            )
            .unwrap();

            //compiler::compile(program.clone());
            let glsl = glsl::generate_glsl(program);

            std::fs::write(format!("./debug/{}", p), glsl).unwrap();
        }
    }
}
