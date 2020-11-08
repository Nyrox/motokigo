use crate::ast::*;
use crate::vm::*;

pub mod program_data;
pub mod resolve_types;

use program_data::{FuncMeta, ProgramData, SymbolMeta};

pub static mut COUNTER: i32 = 0;

pub fn compile(mut ast: Program) -> VMProgram {
    let mut program_data = ProgramData::new();

    resolve_types::resolve(&mut ast, &mut program_data).unwrap();

    codegen(ast, program_data)
}

pub fn codegen(ast: Program, mut data: ProgramData) -> VMProgram {
    let mut program = VMProgram::new();
    program.data = data.clone();

    let mut static_section = 0;

    for i in ast.in_parameters.iter() {
        data.global_symbols.insert(
            i.ident.item.clone(),
            SymbolMeta {
                stack_offset: Some(static_section),
                type_kind: i.type_kind.clone().item,
                is_static: true,
                is_mutable: false,
            },
        );

        static_section += 12;
    }

    program.data = data.clone();

    for f in ast.functions.iter() {
        let mut fnc = data.functions.get_mut(f.ident.item.as_str()).unwrap();
        fnc.address = Some(program.code.len());

        for s in f.statements.iter() {
            generate_statement(&mut program, &ast, &mut fnc, s);
        }

        if !fnc.has_return {
            program.code.push(MemoryCell::plain_inst(OpCode::Void));
            program
                .code
                .push(MemoryCell::with_data(OpCode::Ret, fnc.stack_offset as u16));
        }
    }

    program.data = data;
    program.data.static_section_size = static_section;
    program.data.min_stack_size = static_section + 1024;
    program
}

pub fn generate_statement(
    program: &mut VMProgram,
    ast: &Program,
    fnc: &mut FuncMeta,
    statement: &Statement,
) {
    match statement {
        Statement::VariableDeclaration(_, i, expr) => {
            generate_expr(program, &ast, fnc, &expr);

            if let Some(o) = program.data.global_symbols.get(&i.item) {
                program.code.push(MemoryCell::with_data(
                    OpCode::Mov4Global,
                    o.stack_offset.unwrap() as u16,
                ));
            } else {
                fnc.symbols.get_mut(i.item.as_str()).unwrap().stack_offset = Some(fnc.stack_offset);
                fnc.stack_offset += expr.typekind().unwrap().size();
            }

            program.code.push(MemoryCell::with_data(
                OpCode::StmtMarker,
                i.from.line as u16,
            ));
        }
        Statement::Assignment(i, expr) => {
            generate_expr(program, &ast, fnc, &expr);

            if let Some(o) = program.data.global_symbols.get(&i.item) {
                program.code.push(MemoryCell::with_data(
                    OpCode::Mov4Global,
                    o.stack_offset.unwrap() as u16,
                ));
            } else if let Some(o) = fnc.symbols.get(&i.item) {
                program.code.push(MemoryCell::with_data(OpCode::Mov4, o.stack_offset.unwrap() as u16));
            } else {
                panic!("[ICE: Assignment to unknown symbol after typechecking]");
            }

            program.code.push(MemoryCell::with_data(
                OpCode::StmtMarker,
                i.from.line as u16,
            ));
        }
        Statement::Return(span, expr) => {
            generate_expr(program, &ast, fnc, &expr);

            program.code.push(MemoryCell::with_data(
                OpCode::StmtMarker,
                span.from.line as u16,
            ));
            program
                .code
                .push(MemoryCell::with_data(OpCode::Ret, fnc.stack_offset as u16));
            fnc.has_return = true;
        }
        Statement::Conditional(cond) => {
            fn generate_conditional_branch(
                program: &mut VMProgram,
                ast: &Program,
                fnc: &mut FuncMeta,
                cond: &Conditional,
            ) {
                if let Some(c) = &cond.cond {
                    generate_expr(program, ast, fnc, &c);
                    let label = program.code.len();
                    program.code.push(MemoryCell::with_data(OpCode::CondJmp, 0));
                    for s in cond.body.iter() {
                        generate_statement(program, ast, fnc, s);
                    }
                    program.code[label] =
                        MemoryCell::with_data(OpCode::CondJmp, program.code.len() as u16);
                } else {
                    for s in cond.body.iter() {
                        generate_statement(program, ast, fnc, s);
                    }
                }

                if let Some(c) = &cond.alternate {
                    generate_conditional_branch(program, ast, fnc, &c);
                }
            }

            generate_conditional_branch(program, ast, fnc, &cond);
        }
    };
}

pub fn generate_expr(program: &mut VMProgram, ast: &Program, fnc: &FuncMeta, expr: &Expr) {
    match expr {
        Expr::FuncCall((id, args)) => {
            for arg in args {
                generate_expr(program, ast, fnc, arg);
            }

            let arg_types = &args
                .iter()
                .map(|e| e.typekind().unwrap())
                .collect::<Vec<_>>();

            if let Some((func, _)) = crate::builtins::get_builtin_fn(id.raw.as_ref(), arg_types) {
                program
                    .code
                    .push(MemoryCell::with_data(OpCode::CallBuiltIn, func as u16));
            } else if let Some(func) = program.data.functions.get(id.raw.as_str()) {
                program.code.push(MemoryCell::with_data(
                    OpCode::Call,
                    func.address.unwrap() as u16,
                ));
            } else {
                panic!("Unrecognized function: {:?}: ({:?})", id, arg_types);
            }
        }
        Expr::Literal(l) => match l.item {
            Literal::DecimalLiteral(f) => {
                program.code.push(MemoryCell::plain_inst(OpCode::ConstF32));
                program
                    .code
                    .push(MemoryCell::raw(unsafe { std::mem::transmute(f as f32) }));
            }
            _ => unimplemented!(),
        },
        Expr::Symbol(s) => {
            let symbol = {
                if let Some(symbol) = fnc.symbols.get(s.raw.as_str()) {
                    symbol
                } else if let Some(symbol) = program.data.global_symbols.get(s.raw.as_str()) {
                    symbol
                } else {
                    panic!("Unknown symbol: {:?}", s);
                }
            };

            let offset = symbol.stack_offset.unwrap();
            let size = symbol.type_kind.size();

            let instruction = match symbol.is_static {
                true => OpCode::Load4Global,
                false => OpCode::Load4,
            };

            for i in 0..(size / 4) {
                program.code.push(MemoryCell::with_data(
                    instruction,
                    (offset + (i * 4)) as u16,
                ))
            }
        }
    }
}
