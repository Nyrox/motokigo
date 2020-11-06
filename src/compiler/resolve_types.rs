use crate::ast::*;
use crate::compiler::program_data::{FuncMeta, ProgramData, SymbolMeta};

#[derive(Debug)]
pub struct ResolveTypes<'a> {
    program_data: &'a mut ProgramData,
    current_scope: Option<Ident>,
}

impl<'a> ResolveTypes<'a> {
    pub fn new(program_data: &'a mut ProgramData) -> Self {
        ResolveTypes {
            program_data,
            current_scope: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeError {
    UnknownSymbol(Spanned<Ident>),
	UnknownFunction(Spanned<Ident>, Vec<TypeKind>),
	AssignmentToImmutable(Spanned<Ident>),
	TypeError(Spanned<Ident>, TypeKind, TypeKind),
}

use std::error;
use std::fmt;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type Error: {:?}", self)
    }
}

impl error::Error for TypeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl<'a> ResolveTypes<'a> {
    pub fn current_scope(&mut self) -> &mut FuncMeta {
        let fn_name = self.current_scope.clone().unwrap();

        self.program_data.functions.get_mut(&fn_name).unwrap()
    }
}

impl<'a> Visitor for ResolveTypes<'a> {
    fn type_kind(&mut self, tk: &mut TypeKind) -> VResult {
        match tk {
            TypeKind::TypeRef(name) => {
                if name.starts_with("Vec") {
                    if let Some(n) = name.chars().nth(3).and_then(|x| x.to_digit(10)) {
                        *tk = TypeKind::Vector(Box::new(TypeKind::F32), n as usize);
                    }
                }
                else if name.starts_with("Mat") {
                    let m = name.chars().nth(3).and_then(|x| x.to_digit(10));
                    let n = name.chars().nth(5).and_then(|x| x.to_digit(10));
                    if let Some(m) = m {
                        if let Some(n) = n {
                            *tk = TypeKind::Matrix(Box::new(TypeKind::F32), m as usize, n as usize);
                        }
                        else {
                            *tk = TypeKind::Matrix(Box::new(TypeKind::F32), m as usize, m as usize);
                        }
                    }
                }
                else if name == "Float" {
                    *tk = TypeKind::F32;
				}
                Ok(())
            },
            _ => Ok(()),
        }
    }

    fn symbol(&mut self, s: &mut Symbol) -> VResult {
        // it's impossible to parse a symbol outside of a function, so this is safe
        let scope = self.current_scope();

        if let Some(def) = scope.symbols.get(s.raw.item.as_str()) {
            s.resolved = Some((s.raw.item.clone(), def.type_kind.clone()));
        } else if let Some(def) = self.program_data.global_symbols.get(s.raw.item.as_str()) {
            s.resolved = Some((s.raw.item.clone(), def.type_kind.clone()));
        } else {
            Err(Box::new(TypeError::UnknownSymbol(s.raw.clone())))?;
        }
        Ok(())
    }

    fn function_decl(&mut self, func: &mut FunctionDeclaration) -> VResult {
        self.current_scope = Some(func.ident.item.clone());
        self.program_data
            .functions
            .insert(func.ident.item.clone(), FuncMeta::new());

        Ok(())
    }

    fn post_statement(&mut self, stmt: &mut Statement) -> VResult {
        match stmt {
            Statement::VariableDeclaration(is_mut, ident, rhs) => {
                let scope = self.current_scope();

                scope.symbols.insert(
                    ident.item.clone(),
                    SymbolMeta {
                        type_kind: rhs.typekind().expect(&format!(
                            "Expected expr {:#?} to be typed by this point.",
                            rhs
                        )),
						is_static: false,
						is_mutable: *is_mut,
                        stack_offset: None,
                    },
                );
			}
			Statement::Assignment(ident, rhs) =>  {
				let scope = self.current_scope();

				if let Some(s) = scope.symbols.get(&ident.item) {
					if !s.is_mutable {
						Err(Box::new(TypeError::AssignmentToImmutable(ident.clone())))?;
					}

					let rhs_t = rhs.expect_typekind();
					if s.type_kind != rhs_t {
						Err(Box::new(TypeError::TypeError(ident.clone(), s.type_kind.clone(), rhs_t)))?;
					}
				} else {
					Err(Box::new(TypeError::UnknownSymbol(ident.clone())))?;
				}
			}
            Statement::Return(_, _) => {}
            Statement::Conditional(conditional) => { 
                let c = conditional.cond.as_ref().unwrap();
                match c.expect_typekind() {
                    TypeKind::I32 | TypeKind::F32 => {},
                    t => { Err(Box::new(TypeError::TypeError(c.span().map(|_| String::from("ur bad")), TypeKind::F32, t)))?; }
                }
            }
        }

        Ok(())
    }

    fn post_func_call(&mut self, func: &mut FuncCall) -> VResult {
        let arg_types = func
            .1
            .iter()
            .map(|e| e.typekind().unwrap())
            .collect::<Vec<_>>();

        if let Some((_, builtin)) = crate::builtins::get_builtin_fn(func.0.raw.as_ref(), &arg_types)
        {
            func.0.resolved = Some((func.0.raw.item.clone(), builtin.return_type()));
            Ok(())
        } else if let Some(_) = self.program_data.functions.get(func.0.raw.as_str()) {
            Ok(())
        } else {
            Err(Box::new(TypeError::UnknownFunction(
                func.0.raw.clone(),
                arg_types,
            )))
        }
    }

    fn post_in_parameter(&mut self, param: &mut InParameterDeclaration) -> VResult {
        self.program_data.global_symbols.insert(
            param.ident.item.clone(),
            SymbolMeta {
                type_kind: param.type_kind.item.clone(),
                stack_offset: None,
				is_static: true,
				is_mutable: false,
            },
        );

        Ok(())
    }
}

pub fn resolve<'a>(ast: &'a mut Program, data: &'a mut ProgramData) -> VResult {
    let mut rt = ResolveTypes::new(data);
    ast.visit(&mut rt)
}
