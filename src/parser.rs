use crate::ast::*;

use crate::scanner::*;
use std::iter::{Iterator, Peekable};

pub fn parse(input: impl AsRef<str>) -> Result<Program, ParsingError> {
	let tokens = Scanner::new(input.as_ref().chars()).scan_all()?;

	let mut tokens = tokens.into_iter().peekable();

	parse_program(&mut tokens)
}

type ItemType = Spanned<Token>;

#[derive(Debug, Clone)]
pub enum ParsingError {
	UnexpectedToken(ItemType),
	UnexpectedEndOfInput,
	ScanningError(ScanningError),
}

impl From<ScanningError> for ParsingError {
	fn from(e: ScanningError) -> ParsingError {
		ParsingError::ScanningError(e)
	}
}

type ParsingResult<T> = Result<T, ParsingError>;

pub trait TokenSource: std::clone::Clone {
	fn next(&mut self) -> Option<ItemType>;
	fn peek(&mut self) -> Option<&ItemType>;

	fn expect_next(&mut self) -> ParsingResult<ItemType> {
		match self.next() {
			None => Err(ParsingError::UnexpectedEndOfInput),
			Some(t) => Ok(t),
		}
	}

	fn expect_token(&mut self, token: Token) -> ParsingResult<ItemType> {
		match self.expect_next()? {
			t if *t == token => Ok(t),
			t => Err(ParsingError::UnexpectedToken(t)),
		}
	}

	fn expect_identifier(&mut self) -> ParsingResult<Spanned<String>> {
		let token = self.expect_next()?;
		match token.item {
			Token::Identifier(s) => Ok(Spanned::new(s, token.from, token.to)),
			_ => Err(ParsingError::UnexpectedToken(token)),
		}
	}

	fn expect_typekind(&mut self) -> ParsingResult<Spanned<TypeKind>> {
		let token = self.expect_next()?;

		let res = token.map(|t| match t {
			Token::Float => Ok(TypeKind::F32),
			Token::Int => Ok(TypeKind::I32),
			Token::Void => Ok(TypeKind::Void),
			Token::Identifier(i) => Ok(TypeKind::TypeRef(token.map(|_| i.clone()))),
			t => Err(ParsingError::UnexpectedToken(token.map(|_| t.clone()))),
		});

		match res.item {
			Ok(t) => Ok(token.map(|_| t)),
			Err(e) => Err(e),
		}
	}

	fn maybe_expect(&mut self, token: Token) -> Option<ItemType> {
		match TokenSource::peek(self)? {
			t if t.item == token => Some(self.expect_next().unwrap()),
			_ => None,
		}
	}
}

impl<T> TokenSource for Peekable<T>
where
	T: Iterator<Item = ItemType> + Clone,
{
	fn next(&mut self) -> Option<ItemType> {
		std::iter::Iterator::next(self)
	}

	fn peek(&mut self) -> Option<&ItemType> {
		self.peek()
	}
}

pub fn parse_program(tokens: &mut impl TokenSource) -> ParsingResult<Program> {
	let mut program = Program::new();

	'parsing: loop {
		let token = tokens.peek();
		if token.is_none() {
			break 'parsing;
		}
		let token = token.unwrap();

		match &token.item {
			Token::In => {
				tokens.expect_token(Token::In)?;
				let type_kind = tokens.expect_typekind()?;

				let ident = tokens.expect_identifier()?;
				program.in_parameters.push(InParameterDeclaration { type_kind, ident, is_uniform: false });
				continue;
			}
			Token::Uniform => {
				tokens.expect_token(Token::Uniform)?;
				let type_kind = tokens.expect_typekind()?;

				let ident = tokens.expect_identifier()?;
				program.in_parameters.push(InParameterDeclaration { type_kind, ident, is_uniform: true });
				continue;
			}
			Token::Struct => {
				tokens.expect_token(Token::Struct)?;
				let ident = tokens.expect_identifier()?;

				tokens.expect_token(Token::LeftBrace)?;

				let mut members = Vec::new();
				loop {
					let tk = tokens.expect_typekind()?;
					let field_ident = tokens.expect_identifier()?;

					members.push((field_ident, tk));

					if tokens.maybe_expect(Token::Comma).is_none() {
						tokens.expect_token(Token::RightBrace)?;
						break;
					}
					if tokens.maybe_expect(Token::RightBrace).is_some() {
						break;
					}
				}

				program.struct_declarations.push(StructDeclaration {
					ident,
					members,
					size: None,
				})
			}
			// func declarations
			_ => {
				let tk = tokens.expect_typekind()?;
				let ident = tokens.expect_identifier()?;

				// arg list
				tokens.expect_token(Token::LeftParen)?;
				let mut params = Vec::new();
				while let Some(tok) = tokens.peek() {
					if tok.item == Token::RightParen {
						break;
					}
					let tk = tokens.expect_typekind()?;
					let ident = tokens.expect_identifier()?;
					params.push((tk, ident));

					if tokens.maybe_expect(Token::Comma).is_some() {
						if let Some(next) = tokens.peek() {
							if next.item == Token::RightParen {
								Err(ParsingError::UnexpectedToken(next.clone()))?
							}
						}
					}
				}
				tokens.expect_token(Token::RightParen)?;

				// body
				tokens.expect_token(Token::LeftBrace)?;

				let statements = parse_statements(tokens)?;

				program.functions.push(FunctionDeclaration {
					ident,
					params,
					statements,
					ret_type: tk,
				});
			}
		}
	}

	Ok(program)
}

pub fn parse_statements(tokens: &mut impl TokenSource) -> ParsingResult<Vec<Statement>> {
	let mut output = Vec::new();

	'parsing: loop {
		let token = tokens.next();
		if token.is_none() {
			break 'parsing;
		}
		let token = token.unwrap();

		match &token.item {
			Token::Return => {
				output.push(Statement::Return(token.map(|_| ()), parse_expr_bp(tokens, 0)?));
			}
			Token::Let => {
				let is_mut = tokens.maybe_expect(Token::Mut).is_some();
				let ident = tokens.expect_identifier()?;

				tokens.expect_token(Token::Equals)?;

				output.push(Statement::VariableDeclaration(is_mut, ident, parse_expr_bp(tokens, 0)?));
			}
			Token::Identifier(s) => {
				tokens.expect_token(Token::Equals)?;

				output.push(Statement::Assignment(
					token.map(|_| s.clone()),
					parse_expr_bp(tokens, 0)?,
				));
			}
			Token::If => {
				output.push(Statement::Conditional(parse_conditional(tokens)?));
			}
			Token::For => {
				let ident = tokens.expect_identifier()?;
				tokens.expect_token(Token::Equals)?;
				let from = parse_expr_bp(tokens, 0)?;

				tokens.expect_token(Token::To)?;
				let to = parse_expr_bp(tokens, 0)?;

				tokens.expect_token(Token::LeftBrace)?;
				let body = parse_statements(tokens)?;

				output.push(Statement::Loop(ident, from, to, body));
			}
			Token::RightBrace => break 'parsing,
			_ => {
				return Err(ParsingError::UnexpectedToken(token));
			}
		}
	}

	Ok(output)
}

pub fn parse_conditional(tokens: &mut impl TokenSource) -> ParsingResult<Conditional> {
	// cond
	let cond = parse_expr_bp(tokens, 0)?;

	// body
	tokens.expect_token(Token::LeftBrace)?;
	let body = parse_statements(tokens)?;

	// recurse
	let alt = if tokens.maybe_expect(Token::Else).is_some() {
		if let Some(_) = tokens.maybe_expect(Token::If) {
			// else if
			Some(parse_conditional(tokens)?)
		} else {
			// else
			tokens.expect_token(Token::LeftBrace)?;
			let last_body = parse_statements(tokens)?;
			Some(Conditional {
				cond: None,
				body: last_body,
				alternate: None,
			})
		}
	} else {
		None
	};

	Ok(Conditional {
		cond: Some(cond),
		body,
		alternate: alt.map(Box::new),
	})
}

pub fn infix_binding_power(t: &Token) -> Option<(u8, u8)> {
	match t {
		Token::Plus => Some((1, 2)),
		Token::Minus => Some((1, 2)),
		Token::Star => Some((3, 4)),
		Token::Slash => Some((3, 4)),
		Token::Less => Some((0, 1)),
		Token::LessEq => Some((0, 1)),
		Token::Greater => Some((0, 1)),
		Token::GreaterEq => Some((0, 1)),
		Token::EqualsEquals => Some((0, 1)),
		_ => None,
	}
}

pub fn prefix_binding_power(t: &Token) -> Option<((), u8)> {
	match t {
		Token::Minus => Some(((), 5)),
		_ => None,
	}
}

pub fn parse_expr_bp(lexer: &mut impl TokenSource, min_bp: u8) -> ParsingResult<Expr> {
	let token = lexer.expect_next()?;
	// atoms
	let mut lhs = match &token.item {
		Token::FloatLiteral(f) => Expr::Literal(token.map(|_| Literal::DecimalLiteral(*f))),
		Token::IntegerLiteral(i) => Expr::Literal(token.map(|_| Literal::IntegerLiteral(*i))),
		Token::Identifier(i) => match lexer.peek() {
			Some(t) if t.item == Token::LeftParen => {
				lexer.next();

				let mut exprs = Vec::new();
				loop {
					let e = parse_expr_bp(lexer, 0)?;
					exprs.push(Box::new(e));
					match lexer.expect_next()? {
						t if t.item == Token::RightParen => {
							break;
						}
						t if t.item == Token::Comma => {
							continue;
						}
						t => Err(ParsingError::UnexpectedToken(t))?,
					}
				}

				Expr::FuncCall((Reference::unresolved(token.map(|_| i.clone())), exprs))
			}
			Some(t) if t.item == Token::LeftBrace => {
				let mut lookahead = lexer.clone();
				lookahead.next();
				match (lookahead.next().map(|x| x.item), lookahead.next().map(|x| x.item)) {
					(Some(Token::Identifier(_)), Some(Token::Colon)) => (),
					_ => return Ok(Expr::Symbol(Reference::unresolved(token.map(|_| i.clone()))))
				}

				lexer.next();
				let mut fields = Vec::new();
				loop {
					let i = lexer.expect_identifier()?;
					lexer.expect_token(Token::Colon)?;
					let rhs = parse_expr_bp(lexer, 0)?;

					fields.push((i, Box::new(rhs)));

					if lexer.maybe_expect(Token::Comma).is_none() {
						lexer.expect_token(Token::RightBrace)?;
						break;
					}
					if lexer.maybe_expect(Token::RightBrace).is_some() {
						break;
					}
				}

				Expr::StructConstruction(token.map(|_| i.clone()), None, fields)
			}
			Some(t) if t.item == Token::Dot => {
				lexer.next();

				let field = lexer.expect_identifier()?;

				Expr::FieldAccess(Reference::unresolved(token.map(|_| i.clone())), field, None, None)
			}
			_ => Expr::Symbol(Reference::unresolved(token.map(|_| i.clone()))),
		},
		Token::LeftParen => {
			let e = parse_expr_bp(lexer, 0)?;
			lexer.expect_token(Token::RightParen)?;
			Expr::Grouped(Box::new(e))
		}
		t if prefix_binding_power(t).is_some() => {
			let ((), r_bp) = prefix_binding_power(t).unwrap();

			let rhs = parse_expr_bp(lexer, r_bp)?;
			let fnc = match t {
				Token::Minus => "__op_unary_neg",
				_ => unreachable!(), // at this point we know we have a valid unary operator, so this is fine
			};

			Expr::FuncCall((
				Reference::unresolved(token.map(|_| fnc.to_owned())),
				vec![Box::new(rhs)],
			))
		}
		_ => return Err(ParsingError::UnexpectedToken(token)),
	};

	loop {
		let (t, (l_bp, r_bp)) = match lexer.peek() {
			Some(t) if infix_binding_power(t).is_some() => (t.clone(), infix_binding_power(t).unwrap()),
			_ => break,
		};
		if l_bp < min_bp {
			break;
		}

		lexer.next().unwrap();
		let rhs = parse_expr_bp(lexer, r_bp)?;

		let fnc = match &t.item {
			Token::Plus => "__op_binary_add",
			Token::Minus => "__op_binary_sub",
			Token::Star => "__op_binary_mul",
			Token::Slash => "__op_binary_div",
			Token::Less => "__op_binary_less",
			Token::LessEq => "__op_binary_less_equal",
			Token::Greater => "__op_binary_greater",
			Token::GreaterEq => "__op_binary_greater_equal",
			Token::EqualsEquals => "__op_binary_equality",
			_ => unreachable!(),
		};

		lhs = Expr::FuncCall((
			Reference::unresolved(t.map(|_| fnc.to_owned())),
			vec![Box::new(lhs), Box::new(rhs)],
		));
		continue;
	}

	Ok(lhs)
}
