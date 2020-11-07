pub type Ident = String;

use std::fmt;

pub type VResult = Result<(), Box<dyn std::error::Error>>;

pub trait Visitor {
    fn post_in_parameter(&mut self, _t: &mut InParameterDeclaration) -> VResult {
        Ok(())
    }
    fn post_expr(&mut self, _t: &mut Expr) -> VResult {
        Ok(())
    }
    fn type_kind(&mut self, _t: &mut TypeKind) -> VResult {
        Ok(())
    }
    fn symbol(&mut self, _t: &mut Symbol) -> VResult {
        Ok(())
    }
    fn function_decl(&mut self, _t: &mut FunctionDeclaration) -> VResult {
        Ok(())
    }
    fn post_statement(&mut self, _t: &mut Statement) -> VResult {
        Ok(())
    }
    fn post_func_call(&mut self, _t: &mut FuncCall) -> VResult {
        Ok(())
    }
}

pub trait Visitable {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult;
}

impl<T: Visitable> Visitable for Vec<T> {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        for t in self {
            t.visit(v)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub from: Position,
    pub to: Position,
}

impl<T: Copy> Copy for Spanned<T> {}

impl<T> Spanned<T> {
    pub fn new(item: T, from: Position, to: Position) -> Spanned<T> {
        Spanned { item, from, to }
    }

    pub fn encompass<A, B>(item: T, s1: Spanned<A>, s2: Spanned<B>) -> Spanned<T> {
        Spanned {
            item,
            from: s1.from,
            to: s2.to,
        }
    }

    pub fn map<U, F>(&self, f: F) -> Spanned<U>
    where
        F: FnOnce(&T) -> U,
    {
        Spanned {
            from: self.from,
            to: self.to,
            item: f(&self.item),
        }
    }
}

use std::ops::Deref;

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

#[derive(Clone)]
pub struct Reference<T, R> {
    pub raw: T,
    pub resolved: Option<R>,
}

impl<T, R> Reference<T, R> {
    pub fn unresolved(raw: T) -> Self {
        Reference {
            raw,
            resolved: None,
        }
    }
}

impl<T: fmt::Debug, R: fmt::Debug> fmt::Debug for Reference<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(r) = &self.resolved {
            write!(f, "{:?} => {:?}", self.raw, r)
        } else {
            write!(f, "{:?}", self.raw)
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Literal {
    IntegerLiteral(i64),
    DecimalLiteral(f64),
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::IntegerLiteral(i) => format!("{}", i),
            Literal::DecimalLiteral(f) => format!("{}", f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Void,
    I32,
    F32,
    TypeRef(String),
    Vector(Box<TypeKind>, usize),
    Matrix(Box<TypeKind>, usize, usize),
}

impl TypeKind {
    pub fn size(&self) -> usize {
        match self {
            TypeKind::Void => 0,
            TypeKind::I32 => 4,
            TypeKind::F32 => 4,
            TypeKind::Vector(typeKind, size) => typeKind.size() * size,
            TypeKind::Matrix(typeKind, m, n) => typeKind.size() * m * n,
            _ => unimplemented!("{:?}", self),
        }
    }
}

impl Visitable for TypeKind {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        v.type_kind(self)
    }
}

pub type Symbol = Reference<Spanned<Ident>, (Ident, TypeKind)>;

impl Visitable for Symbol {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        v.symbol(self)
    }
}

pub type FuncCall = (Reference<Spanned<Ident>, (Ident, TypeKind)>, Vec<Box<Expr>>);

impl Visitable for FuncCall {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        for e in self.1.iter_mut() {
            e.visit(v)?;
        }
        v.post_func_call(self)
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    FuncCall(FuncCall),
    Literal(Spanned<Literal>),
    Symbol(Symbol),
}

impl Expr {
    pub fn typekind(&self) -> Option<TypeKind> {
        match self {
            Expr::FuncCall((def, _)) => def.resolved.clone().map(|(_, tk)| tk),
            Expr::Symbol(s) => s.resolved.clone().map(|(_, tk)| tk),
            Expr::Literal(l) => match l.item {
                Literal::DecimalLiteral(_) => Some(TypeKind::F32),
                Literal::IntegerLiteral(_) => Some(TypeKind::I32),
            },
        }
    }

    pub fn expect_typekind(&self) -> TypeKind {
        self.typekind().expect(&format!(
            "Expected expr {:#?} to be typed by this point.",
            self
        ))
    }

    pub fn span(&self) -> Spanned<()> {
        match self {
            Self::FuncCall(fc) => fc.0.raw.map(|x| ()),
            Self::Literal(lit) => lit.map(|x| ()),
            Self::Symbol(sym) => sym.raw.map(|x| ()),
        }
    }
}

impl Visitable for Expr {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        match self {
            Expr::FuncCall(func) => {
                for e in func.1.iter_mut() {
                    e.visit(v)?
                }
                v.post_func_call(func)?;
            }
            Expr::Symbol(s) => s.visit(v)?,
            _ => (),
        }

        v.post_expr(self)
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(Spanned<Ident>, Expr),
    VariableDeclaration(bool, Spanned<Ident>, Expr),
    Return(Spanned<()>, Expr),
    Conditional(Conditional),
}

impl Visitable for Statement {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        match self {
            Statement::Assignment(_, expr) => expr.visit(v)?,
            Statement::VariableDeclaration(_, _, expr) => expr.visit(v)?,
            Statement::Return(_, expr) => expr.visit(v)?,
            Statement::Conditional(cond) => cond.visit(v)?,
        }

        v.post_statement(self)
    }
}

#[derive(Clone, Debug)]
pub struct Conditional {
    pub cond: Option<Expr>,
    pub body: Vec<Statement>,
    pub alternate: Option<Box<Conditional>>,
}

impl Visitable for Conditional {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        if let Some(cond) = &mut self.cond {
            cond.visit(v)?;
        }
        self.body.visit(v)?;
        if let Some(alt) = self.alternate.as_mut() {
            alt.visit(v)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub ident: Spanned<Ident>,
    pub param_types: Vec<Spanned<Ident>>,
    pub statements: Vec<Statement>,
    pub ret_type: Spanned<TypeKind>,
}

impl Visitable for FunctionDeclaration {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        v.function_decl(self)?;
        self.ret_type.item.visit(v)?;
        self.statements.visit(v)
    }
}

#[derive(Clone, Debug)]
pub struct InParameterDeclaration {
    pub type_kind: Spanned<TypeKind>,
    pub ident: Spanned<Ident>,
}

impl Visitable for InParameterDeclaration {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        self.type_kind.item.visit(v)?;
        v.post_in_parameter(self)
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub in_parameters: Vec<InParameterDeclaration>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            functions: Vec::new(),
            in_parameters: Vec::new(),
        }
    }

    pub fn get_function(&self, ident: Ident) -> Option<&FunctionDeclaration> {
        self.functions.iter().find(|f| *f.ident == ident)
    }
}

impl Visitable for Program {
    fn visit(&mut self, v: &mut dyn Visitor) -> VResult {
        (|| {
            self.in_parameters.visit(v)?;
            self.functions.visit(v)
        })()?;

        Ok(())
    }
}
