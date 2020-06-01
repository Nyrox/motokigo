use super::*;
use crate::ast::*;

#[derive(Debug)]
pub struct GenerateGLSL {
    pub functions: Vec<(String, String)>,
    pub current_fn: Option<String>,
    pub prelude: String,
}
impl GenerateGLSL {
    pub fn new() -> Self {
        GenerateGLSL {
            functions: vec![],
            current_fn: None,
            prelude: String::new(),
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

        let func_text = format!("{} {}() {{\n{}\n}}\n", glsl_type, func_ident, func_body);

        self.functions.push((func_ident, func_text));
    }

    pub fn generate_main_shim(&mut self, main: &FunctionDeclaration) {
        // todo implement structs
        let glsl_type = get_glsl_type(&main.ret_type.item);

        self.prelude
            .push_str(&format!("out {} {};\n", glsl_type, "out_0"));

        let shim_text = format!(
            "void main() {{\n\t{} rt = __impl_main();\n\tout_0 = rt;\n}}\n",
            glsl_type
        );

        self.functions.push(("main".to_owned(), shim_text));
    }

    pub fn generate_statements(&mut self, statements: &Vec<Statement>) -> String {
        let func_body = statements.iter().map(|s| match s {
            Statement::Assignment(id, expr) => {
                let glsl_type = get_glsl_type(&expr.get_type().unwrap());
                format!(
                    "\t{} {} = {};",
                    glsl_type,
                    id.item,
                    self.generate_expr(expr)
                )
            }
            Statement::Return(_, expr) => format!("\treturn {};", self.generate_expr(expr)),
        });

        func_body.collect::<Vec<String>>().join("\n")
    }

    pub fn generate_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Symbol(s) => s.resolved.clone().unwrap().0,
            Expr::FuncCall((f, args)) => {
                let arg_types = args
                    .iter()
                    .map(|e| e.get_type().unwrap())
                    .collect::<Vec<_>>();

                let args: Vec<_> = args.iter().map(|e| self.generate_expr(e)).collect();

                if let Some((_, builtin)) =
                    crate::builtins::get_builtin_fn(f.raw.as_ref(), &arg_types)
                {
                    builtin.generate(self, args)
                } else {
                    format!("{}()", f.raw.clone().item)
                }
            }
            Expr::Literal(l) => l.to_string(),
        }
    }
}
