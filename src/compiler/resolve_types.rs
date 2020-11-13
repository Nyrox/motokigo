use crate::{
	ast::*,
	compiler::program_data::{FuncMeta, ProgramData, SymbolMeta},
};

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
	UnknownType(Spanned<Ident>),
	GenericError(String), // For now
}

use std::{cell::RefCell, error, fmt, rc::Rc};

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
				} else if name.starts_with("Mat") {
					let m = name.chars().nth(3).and_then(|x| x.to_digit(10));
					let n = name.chars().nth(5).and_then(|x| x.to_digit(10));
					if let Some(m) = m {
						if let Some(n) = n {
							*tk = TypeKind::Matrix(Box::new(TypeKind::F32), m as usize, n as usize);
						} else {
							*tk = TypeKind::Matrix(Box::new(TypeKind::F32), m as usize, m as usize);
						}
					}
				} else if &name.item == "Float" {
					*tk = TypeKind::F32;
				} else if &name.item == "Int" {
					*tk = TypeKind::I32;
				} else if let Some(s) = self.program_data.struct_declarations.get(&name.item) {
					*tk = TypeKind::Struct(s.clone());
				} else {
					Err(Box::new(TypeError::UnknownType(name.clone())))?;
				}

				Ok(())
			}
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

	fn struct_declaration(&mut self, s: &mut StructDeclaration) -> VResult {
		s.size = Some(s.members.iter().map(|(_, tk)| tk.size()).sum());

		self.program_data
			.struct_declarations
			.insert(s.ident.item.clone(), Rc::new(RefCell::new(s.clone())));

		Ok(())
	}

	fn function_decl(&mut self, func: &mut FunctionDeclaration) -> VResult {
		self.current_scope = Some(func.ident.item.clone());

		let mut fnc = FuncMeta::new();
		fnc.return_type = Some(func.ret_type.item.clone());
		let mut param_offset = 0;

		for (tk, ident) in &func.params {
			fnc.symbols.insert(
				ident.item.clone(),
				SymbolMeta {
					type_kind: tk.item.clone(),
					is_static: false,
					is_mutable: false,
					stack_offset: Some(param_offset),
				},
			);

			param_offset += tk.size();
		}
		fnc.param_types = func.params.iter().map(|x| x.0.item.clone()).collect();

		self.program_data.functions.insert(func.ident.item.clone(), fnc);

		Ok(())
	}

	fn pre_statement(&mut self, stmt: &mut Statement) -> VResult {
		match stmt {
			Statement::Loop(ident, _, _, _) => {
				let scope = self.current_scope();

				scope.symbols.insert(
					ident.item.clone(),
					SymbolMeta {
						type_kind: TypeKind::I32,
						is_static: false,
						is_mutable: false,
						stack_offset: None,
					},
				);
			}
			_ => {}
		}

		Ok(())
	}

	fn post_statement(&mut self, stmt: &mut Statement) -> VResult {
		match stmt {
			Statement::VariableDeclaration(is_mut, ident, rhs) => {
				let scope = self.current_scope();

				scope.symbols.insert(
					ident.item.clone(),
					SymbolMeta {
						type_kind: rhs
							.typekind()
							.expect(&format!("Expected expr {:#?} to be typed by this point.", rhs)),
						is_static: false,
						is_mutable: *is_mut,
						stack_offset: None,
					},
				);
			}
			Statement::Assignment(ident, rhs) => {
				let scope = self.current_scope();

				if let Some(s) = scope.symbols.get(&ident.item) {
					if !s.is_mutable {
						Err(Box::new(TypeError::AssignmentToImmutable(ident.clone())))?;
					}

					let rhs_t = rhs.expect_typekind();
					if s.type_kind != rhs_t {
						Err(Box::new(TypeError::TypeError(
							ident.clone(),
							s.type_kind.clone(),
							rhs_t,
						)))?;
					}
				} else {
					Err(Box::new(TypeError::UnknownSymbol(ident.clone())))?;
				}
			}
			Statement::Return(_, rhs) => {
				let scope = self.current_scope();

				if &rhs.typekind() != &scope.return_type {
					return Err(Box::new(TypeError::TypeError(
						rhs.span()
							.map(|_| String::from("Return type does not match type of return expression")),
						rhs.expect_typekind(),
						scope.return_type.clone().unwrap(),
					)));
				}
			}
			Statement::Conditional(conditional) => {
				let c = conditional.cond.as_ref().unwrap();
				match c.expect_typekind() {
					TypeKind::I32 | TypeKind::F32 => {}
					t => {
						Err(Box::new(TypeError::TypeError(
							c.span().map(|_| String::from("ur bad")),
							TypeKind::F32,
							t,
						)))?;
					}
				}
			}
			Statement::Loop(_, from, to, _) => {
				let l = from.expect_typekind();
				if l != TypeKind::I32 {
					Err(Box::new(TypeError::TypeError(
						from.span().map(|_| String::from("ur bad")),
						TypeKind::I32,
						l,
					)))?
				}
				let r = to.expect_typekind();
				if r != TypeKind::I32 {
					Err(Box::new(TypeError::TypeError(
						to.span().map(|_| String::from("ur bad")),
						TypeKind::I32,
						r,
					)))?
				}
			}
		}

		Ok(())
	}

	fn post_func_call(&mut self, func: &mut FuncCall) -> VResult {
		let arg_types = func.1.iter().map(|e| e.typekind().unwrap()).collect::<Vec<_>>();

		if let Some((_, builtin)) = crate::builtins::get_builtin_fn(func.0.raw.as_ref(), &arg_types) {
			func.0.resolved = Some((func.0.raw.item.clone(), builtin.return_type()));
			Ok(())
		} else if let Some(f) = self.program_data.functions.get(func.0.raw.as_str()) {
			// TODO: Implement function overloading in user land
			if false
				== arg_types
					.iter()
					.zip(f.param_types.iter())
					.fold(true, |acc, (a, b)| acc && a == b)
			{
				Err(Box::new(TypeError::UnknownFunction(func.0.raw.clone(), arg_types)))?
			}

			func.0.resolved = Some((func.0.raw.item.clone(), f.return_type.clone().unwrap()));
			Ok(())
		} else {
			Err(Box::new(TypeError::UnknownFunction(func.0.raw.clone(), arg_types)))
		}
	}

	fn post_expr(&mut self, e: &mut Expr) -> VResult {
		match e {
			Expr::FieldAccess(s, f, t, so) => match &s.resolved.as_ref().unwrap().1 {
				TypeKind::Struct(s) => {
					let s = s.borrow();
					if let Some(field) = s.members.iter().find(|(mn, _)| &mn.item == &f.item) {
						*t = Some(field.1.item.clone());
						*so = Some(
							s.members
								.iter()
								.map_while(|(n, tk)| (&n.item != &f.item).then_some(tk.size()))
								.sum(),
						);
					} else {
						Err(Box::new(TypeError::GenericError(format!(
							"Field {:?} does not exist on struct {:?}",
							f, s.ident
						))))?;
					}
                }
                TypeKind::Vector(tk, size) => {
                    fn all_in_set(s: &String, set: Vec<char>) -> bool {
                        for c in s.chars() {
                            if !set.contains(&c) {
                                return false;
                            }
                        }
                        true
                    }

                    let len = f.len();
                    if len > 4 {
                        Err(Box::new(TypeError::GenericError(format!(
							"{:?} is not a valid vector accessor",
							f
						))))?;
                    } else {
                        if all_in_set(f, vec!['x', 'y', 'z', 'w']) || all_in_set(f, vec!['r', 'g', 'b', 'a']) {
                            if len == 1 {
                                *t = Some(*tk.clone());
                            } else {
                                *t = Some(TypeKind::Vector(tk.clone(), len));
                            }
                            *so = None //TODO: Set this?
                        } else {
                            Err(Box::new(TypeError::GenericError(
                                "Vector components are not from the same set".to_owned()
                            )))?;
                        }
                    }
                }
				_ => Err(Box::new(TypeError::GenericError(format!(
					"Field access on type {:?} is not valid",
					s.resolved.as_ref().unwrap().1.clone()
				))))?,
			},
			Expr::FuncCall(_) => {}
			Expr::Grouped(_) => {}
			Expr::Literal(_) => {}
			Expr::StructConstruction(name, s, _) => {
				if let Some(newt) = self.program_data.struct_declarations.get(&name.item) {
					*s = Some(newt.clone());
				} else {
					Err(TypeError::UnknownType(name.clone()))?;
				}
			}
			Expr::Symbol(_) => {}
		}

		Ok(())
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
