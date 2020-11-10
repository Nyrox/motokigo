use super::*;

#[derive(Debug)]
pub struct GenerateGLSL {
	pub functions: Vec<(String, String)>,
	pub current_fn: Option<String>,
	pub prelude: String,
	pub indent: usize,
}
impl GenerateGLSL {
	pub fn new() -> Self {
		GenerateGLSL {
			functions: vec![],
			current_fn: None,
			prelude: String::new(),
			indent: 0,
		}
	}

	pub fn finish(self) -> String {
		let mut output = String::new();
		output.push_str(&self.prelude);
		output.push('\n');
		self.functions.into_iter().map(|(_, s)| s).for_each(|s| {
			output.push_str(&s);
			output.push('\n');
		});
		output
	}
}

impl GenerateGLSL {
	pub fn consume(&mut self, program: Program) {
		self.prelude.push_str("#version 330 core\n\n");

		for i in program.in_parameters.iter() {
			self.consume_in_parameter(i);
		}

		for i in program.functions.iter() {
			self.consume_func_decl(i);
		}

		let main_fn = program.get_function("main".to_owned());
		main_fn.map(|f| self.generate_main_shim(f));
	}

	pub fn consume_in_parameter(&mut self, param: &InParameterDeclaration) {
		let glsl_type = get_glsl_type(&param.type_kind);
		self.prelude
			.push_str(&format!("in {} {};\n", glsl_type, param.ident.item));
	}

	pub fn consume_func_decl(&mut self, decl: &FunctionDeclaration) {
		let glsl_type = get_glsl_type(&decl.ret_type);

		let func_ident = match decl.ident.item.as_str() {
			"main" => "__impl_main".to_owned(),
			i => i.to_owned(),
		};

		let func_body = self.generate_statements(&decl.statements);

		let func_params = decl
			.params
			.iter()
			.map(|(tk, ident)| format!("{} {}", get_glsl_type(tk), ident.item))
			.collect::<Vec<String>>()
			.join(", ");

		let func_text = format!("{} {}({}) {{\n{}\n}}\n", glsl_type, func_ident, func_params, func_body);

		self.functions.push((func_ident, func_text));
	}

	pub fn generate_main_shim(&mut self, main: &FunctionDeclaration) {
		// todo implement structs
		let glsl_type = get_glsl_type(&main.ret_type.item);

		self.prelude.push_str(&format!("out {} {};\n", glsl_type, "out_0"));

		let shim_text = format!(
			"void main() {{\n\t{} rt = __impl_main();\n\tout_0 = rt;\n}}\n",
			glsl_type
		);

		self.functions.push(("main".to_owned(), shim_text));
	}

	pub fn generate_statements(&mut self, statements: &Vec<Statement>) -> String {
		self.add_indent();
		let func_body = statements
			.iter()
			.map(|s| match s {
				Statement::VariableDeclaration(_, id, expr) => {
					let glsl_type = get_glsl_type(&expr.typekind().unwrap());
					format!(
						"{}{} {} = {};",
						self.indent_string(),
						glsl_type,
						id.item,
						self.generate_expr(expr)
					)
				}
				Statement::Assignment(id, expr) => {
					format!("{}{} = {};", self.indent_string(), id.item, self.generate_expr(expr))
				}
				Statement::Return(_, expr) => format!("{}return {};", self.indent_string(), self.generate_expr(expr)),
				Statement::Conditional(conditional) => {
					fn generate_conditional(this: &mut GenerateGLSL, c: &Conditional) -> String {
						let mut result = String::new();
						let stmts = this.generate_statements(&c.body);

						if let Some(cond) = &c.cond {
							let expr = this.generate_expr(&cond);
							result.extend(
								format!("if (bool({})) {{\n{}\n{}}}", expr, stmts, this.indent_string()).chars(),
							);

							if let Some(next) = &c.alternate {
								result.extend(" else ".chars());
								result.extend(generate_conditional(this, next.as_ref()).chars());
							}
						} else {
							result.extend(format!("{{\n{}\n{}}}", stmts, this.indent_string()).chars());
						}

						result
					}

					format!("{}{}", self.indent_string(), generate_conditional(self, conditional))
				}
				Statement::Loop(ident, from, to, body) => {
					let typekind = get_glsl_type(&from.typekind().unwrap());
					format!(
						"{}for ({} {}={}; {} < {}; i++) {{\n{}\n{}}}",
						self.indent_string(),
						typekind,
						ident.item,
						self.generate_expr(from),
						ident.item,
						self.generate_expr(to),
						self.generate_statements(&body),
						self.indent_string()
					)
				}
			})
			.collect::<Vec<String>>()
			.join("\n");
		self.rem_indent();

		func_body
	}

	pub fn generate_expr(&mut self, expr: &Expr) -> String {
		match expr {
			Expr::Symbol(s) => s.resolved.clone().unwrap().0,
			Expr::FuncCall((f, args)) => {
				let arg_types = args.iter().map(|e| e.typekind().unwrap()).collect::<Vec<_>>();

				let args: Vec<_> = args.iter().map(|e| self.generate_expr(e)).collect();

				if let Some((_, builtin)) = crate::builtins::get_builtin_fn(f.raw.as_ref(), &arg_types) {
					builtin.generate(self, args)
				} else {
					format!("{}({})", f.raw.clone().item, args.join(", "))
				}
			}
			Expr::FieldAccess(s, f, _) => {
				format!("{}.{}", s.resolved.clone().unwrap().0, f.item.clone())
			}
			Expr::Literal(l) => l.to_string(),
			Expr::Grouped(e) => format!("({})", self.generate_expr(e)),
			Expr::StructConstruction(tk, fields) => {
				unimplemented!()
			}
		}
	}

	pub fn add_indent(&mut self) {
		self.indent += 1;
	}

	pub fn rem_indent(&mut self) {
		self.indent -= 1;
	}

	pub fn indent_string(&self) -> String {
		(0..self.indent).map(|_| "\t").collect()
	}
}
