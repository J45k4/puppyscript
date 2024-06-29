use std::ops::Range;

use logos::{skip, Logos};
use crate::types::*;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
	#[token(" ", skip)]
	#[token("\t", skip)]
	#[token("\n", skip)]
	#[token("\r", skip)]
	Whitespace,
	#[regex("//[^\n]*", skip)]
	Comment_,
	#[token("for")]
	For,
	#[token("in")]
	In,
	#[token("if")]
	If,
	#[token("else")]
	Else,
	// #[token("type")]
	// Type,
	#[token("=>")]
	Arrow,
	#[token("{")]
	OpenBrace,
	#[token("}")]
	CloseBrace,
	#[token("(")]
	OpenParen,
	#[token(")")] 
	CloseParen,
	#[token("[")]
	OpenBracket,
	#[token("]")]
	CloseBracket,
	#[token("::")]
	DoubleColon,
	#[token(":")]
	Colon,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,
	#[token("==")]
	Eq,
	#[token("=")]
	Assign,
	#[token("true")]
	True,
	#[token("false")]
	False,
	#[regex(r#""[^"]*""#, |t| t.slice()[1..t.slice().len()-1].to_string())]
	String(String),
	#[regex(r"-?[0-9]+", |t| t.slice().parse::<i64>().ok(), priority = 2)]
	Int(i64),
	#[regex(r"-?[0-9]*\.[0-9]+", |t| t.slice().parse::<f64>().ok())]
	Float(f64),
	#[token("struct")]
	Struct,
	#[token("Int")]
	IntDef,
	#[token("Float")]
	FloatDef,
	#[token("String")]
	StringDef,
	#[token("return")]
	Ret,
	#[token("match")]
	Match,
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("_")]
	Underscore,
	#[token("*")]
	Multiply,
	#[token("/")]
	Divide,
	#[regex(r"[A-Za-z_0-9]+", |t| t.slice().to_string())]
	Ident(String),
}

pub struct Parser {
	tokens: Vec<(Token, Range<usize>)>,
	i: usize,
	loglevel: usize,
	callstack: Vec<String>,
	input: String
}

impl Parser {
	pub fn new(input: &str) -> Parser {
		println!("new parser");

		let lexer = Token::lexer(input);

        let tokens = lexer.spanned()
			.map(|(token, span)| (token.unwrap(), span.into()))
			.collect();

		Parser {
			input: input.to_string(),
			i: 0,
			loglevel: 0,
			callstack: Vec::new(),
			tokens: tokens
		}
	}

	pub fn set_loglevel(mut self, level: usize) -> Self {
		self.loglevel = level;

		self
	}

	pub fn parse(&mut self) -> Vec<ASTNode> {
		self.parse_block()
	}

	fn peek(&self, i: usize) -> Option<Token> {
		if self.loglevel > 0 {
			self.log(&format!("peek: {} {:?}", i, self.tokens.get(self.i + i)));
		}

		match self.tokens.get(self.i + i) {
			Some((token, _)) => Some(token.clone()),
			None => None,
		}
	}

	fn peek_unwrap(&self, i: usize) -> Token {
		match self.peek(i) {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Unexpected end of input");
			},
		}
	}

	fn eat(&mut self) -> Option<Token> {
		if self.loglevel > 0 {
			self.log(&format!("eat: {} val: {:?}", self.i, self.tokens.get(self.i)));
		}

		let token = match self.tokens.get(self.i) {
			Some((token, _)) => token.clone(),
			None => return None,
		};

		self.i += 1;

		Some(token.clone())
	}

	fn expect_eat(&mut self, token: Token) {
		if self.loglevel > 0 {
			self.log(&format!("expect_eat: {:?}", token));
		}

		let next = match self.eat() {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Unexpected end of input");
			},
		};

		if next != token {
			println!("{}", self.curr_loc());
			panic!("Expected {:?} but got {:?}", token, next);
		}
	}

	fn skip(&mut self, n: usize) {
		if self.loglevel > 0 {
			self.log(&format!("skip: {} {:?} to {:?}", n, self.tokens.get(self.i), self.tokens.get(self.i + n)));
		}

		self.i += n;
	}

	fn expect_ident(&mut self) -> String {
		if self.loglevel > 0 {
			self.log(&format!("expect_ident"));
		}

		let token = match self.eat() {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Unexpected end of input");
			},
		};

		match token {
			Token::Ident(ident) => ident,
			_ => {
				println!("{}", self.curr_loc());
				panic!("Expected ident but got {:?}", token)
			},
		}
	}

	fn log(&self, msg: &str) {
		println!("{} {}", self.callstack.join(":"), msg);
	}

	// Return the current location in the source code
	// takes few lines of context from both sides
	fn curr_loc(&self) -> String {
		let mut start = self.i;
		let mut end = self.i;

		for _ in 0..3 {
			if start > 0 {
				start -= 1;
			}
		}

		for _ in 0..3 {
			if end < self.tokens.len() {
				end += 1;
			}
		}

		let min = if start > 0 {
			self.tokens.get(start).unwrap().1.start
		} else {
			0
		};
		let max = if end < self.tokens.len() {
			self.tokens.get(end).unwrap().1.end
		} else {
			self.input.len()
		};

		let text = self.input.get(min..max).unwrap();

		text.to_string()
	}

	fn parse_block(&mut self) -> Vec<ASTNode> {
		if self.loglevel > 0 {
			self.callstack.push("parse_block".to_string());
		}
	
		let mut nodes = Vec::new();

		loop {
			match self.parse_item() {
				Some(n) => nodes.push(n),
				None => break,
			};
		}

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		nodes
	}

	fn parse_array(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_item".to_string());
		}

		self.skip(1);
		let mut items = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseBracket => {
					self.skip(1);
					break;
				},
				Token::Comma => {
					self.skip(1);
				},
				_ => {
					items.push(self.parse_item().unwrap());
				}
			}
		}

		let left = ASTNode::Array(Array { items });

		match self.peek(0) {
			Some(Token::Dot) => self.parse_prob_access(left),
			_ => left
		}
	}

	fn parse_item(&mut self) -> Option<ASTNode> {
		if self.loglevel > 0 {
			self.callstack.push("parse_item".to_string());
		}

		let token = match self.peek(0) {
			Some(token) => token.clone(),
			None => return None,
		};

		let ret = match token {
			Token::OpenBrace => {
				Some(self.parse_obj_props(None))
			}
			Token::Ident(ident) => {
				if self.loglevel > 0 {
					self.log(&format!("ident: {}", ident));
				}

				match self.peek(1) {
					Some(Token::Assign) => {
						self.skip(2);

						let a = Assign { 
							left: Box::new(ASTNode::Ident(ident.clone())), 
							right: Box::new(self.parse_item().unwrap())
						};


						Some(ASTNode::Assign(a))
					},
					Some(Token::Ident(name)) => {
						self.skip(2);
						Some(
							ASTNode::Var(
								Var {
									name: name.to_string(),
									typ: ident.to_string(),
								}
							)
						)
					},
					Some(Token::OpenBrace) => {
						Some(self.parse_obj_ins())
					},
					Some(Token::Arrow) => {
						Some(self.parse_fun())
					},
					_ => {
						Some(self.parse_expr())
					}
				}
			}
			Token::OpenBracket => Some(self.parse_array()),
			Token::OpenParen => {
				// In here we check if future tokens contain an close paren and an arrow
				// If so, we parse a function, otherwise we parse an expression
				let mut i = 1;

				while let Some(token) = self.peek(i) {
					i += 1;

					match token {
						Token::CloseParen => break,
						_ => {}
					}
				};
		
				Some(match self.peek(i) {
					Some(Token::Arrow) => {
						self.parse_fun()
					}
					_ => self.parse_expr()
				})
			}
			Token::Ret => {
				self.skip(1);
				
				Some(ASTNode::Ret(Ret {
					value: Box::new(self.parse_item()),
				}))
			}
			Token::If => {
				Some(self.parse_if())
			}
			Token::For => {
				Some(self.parse_for())
			}
			Token::Match => {
				Some(self.parse_match())
			}
			_ => Some(self.parse_expr())
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_match(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_match".to_string());
		}

		self.skip(1);

		let cond = self.parse_expr();
		self.expect_eat(Token::OpenBrace);

		// let mut cases = Vec::new();

		while let Some(token) = self.eat() {
			println!("{:?}", token);
			match token {
				Token::Underscore => {}
			}
		}

		ASTNode::Match
	}

	fn parse_for_it(&mut self) -> ASTNode {
		let token = match self.peek(0) {
			Some(token) => token.clone(),
			None => {
				println!("{}", self.curr_loc());
				panic!("Unexpected end of input");
			},
		};

		match token {
			Token::OpenBracket => self.parse_array(),
			Token::Ident(idt) => {
				self.skip(1);
				ASTNode::Ident(idt)
			}
			_ => self.parse_expr()
		}
	}

	fn parse_for(&mut self) -> ASTNode {
		self.skip(1);

		match self.peek(0) {
			Some(Token::Ident(idt)) => {
				self.skip(1);
				match self.peek(0) {
					Some(Token::In) => {
						self.skip(1);
						let it = self.parse_for_it();
						self.expect_eat(Token::OpenBrace);

						let mut body = Vec::new();

						while let Some(token) = self.peek(0) {
							match token {
								Token::CloseBrace => {
									self.skip(1);
									break;
								},
								_ => body.push(self.parse_item().unwrap()),
							}
						}

						ASTNode::For(
							For {
								cond: ForCond::FromIt {
									ident: idt.to_string(),
									it: Box::new(it),
								},
								body: body,
							}
						)
					},
					_ => {
						println!("{}", self.curr_loc());
						panic!("Expected in but got {:?}", self.peek(0));
					}
				}
			},
			Some(Token::OpenBrace) => {
				self.skip(1);
				let mut body = Vec::new();

				while let Some(token) = self.peek(0) {
					match token {
						Token::CloseBrace => {
							self.skip(1);
							break;
						},
						_ => body.push(self.parse_item().unwrap()),
					}
				}

				ASTNode::For(
					For {
						cond: ForCond::None,
						body: body,
					}
				)
			},
			_ => {
				println!("{}", self.curr_loc());
				panic!("Expected open brace got {:?}", self.peek(0));
			}
		}
	}

	fn parse_if(&mut self) -> ASTNode {
		self.skip(1);
		let cond = self.parse_expr();
		self.expect_eat(Token::OpenBrace);

		let mut body = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseBrace => {
					self.skip(1);
					break;
				},
				_ => body.push(self.parse_item().unwrap()),
			}
		}

		if let Some(Token::Else) = self.peek(0) {
			self.skip(1);

			match self.peek(0) {
				Some(Token::If) => {
					body.push(self.parse_if());
				},
				Some(Token::OpenBrace) => {
					self.skip(1);
					while let Some(token) = self.peek(0) {
						match token {
							Token::CloseBrace => {
								self.skip(1);
								break;
							},
							_ => body.push(self.parse_item().unwrap()),
						}
					}
				},
				_ => {
					println!("{}", self.curr_loc());
					panic!("Expected if or open brace got {:?}", self.peek(0));
				}
			}
		}

		ASTNode::If(
			If {
				cond: Box::new(cond),
				body: body,
				els: None,
			}
		)
	}

	fn parse_fun(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_fun".to_string());
		}

		let next = match self.peek(0) {
			Some(token) => token,
			None => {
				println!("{}", self.curr_loc());
				panic!("Expected token but got None")
			},
		};

		let mut params = Vec::new();

		match next {
			Token::OpenParen => {
				self.skip(1);

				while let Some(token) = self.peek(0) {
					match token {
						Token::CloseParen => {
							self.skip(1);
							break;
						},
						Token::Comma => {
							self.skip(1);
						},
						Token::Ident(name) => {
							self.skip(1);
							params.push(Param { name: name });
						},
						_ => panic!("Expected ident or ) but got {:?}", self.peek(0)),
					}
				}
				
			}
			Token::Ident(idt) => {
				self.skip(1);
				params.push(Param { name: idt });
			}
			_ => {
				println!("{}", self.curr_loc());
				panic!("Expected ( or ident but got {:?}", next);
			}
		}

		self.expect_eat(Token::Arrow);

		let next = self.peek_unwrap(0);

		let mut body = Vec::new();

		match next {
			Token::OpenBrace => {
				self.skip(1);
				while let Some(token) = self.peek(0) {
					match token {
						Token::CloseBrace => {
							self.skip(1);
							break;
						},
						_ => body.push(self.parse_item().unwrap()),
					}
				}
			},
			_ => {
				body.push(self.parse_item().unwrap());
			}
		}

		// self.expect_eat(Token::OpenBrace);

		let f = Fun {
			params: params,
			body: body,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ASTNode::Fun(f)
	}

	fn parse_obj_ins(&mut self) -> ASTNode {
		let name = self.expect_ident();

		if self.loglevel > 0 {
			self.callstack.push("parse_obj_ins".to_string());
			self.log(&format!("name: {}", name));
		}

		self.parse_obj_props(Some(name))
	}

	fn parse_obj_props(&mut self, name: Option<String>) -> ASTNode {
		self.expect_eat(Token::OpenBrace);

		let mut props = Vec::new();

		loop {
			match self.peek(0) {
				Some(Token::CloseBrace) => {
					self.skip(1);
					break;
				}
				Some(Token::Comma) => {
					self.skip(1);
				}
				_ => {
					let prob_name = self.expect_ident();

					let prob = match self.peek(0) {
						Some(Token::Colon) => {
							self.expect_eat(Token::Colon);
							Property {
								name: prob_name,
								value: Box::new(self.parse_item().unwrap())
							}
						},
						_ => {
							Property {
								name: prob_name.clone(),
								value: Box::new(ASTNode::Ident(prob_name))
							}
						}
					};

					props.push(prob);
				}
			}
		}

		let b = ObjIns {
			name: name,
			props,
		};

		ASTNode::ObjIns(b)
	}

	fn parse_expr(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_expr".to_string());
		}

		let left = self.parse_term();

		let next = match self.peek(0) {
			Some(t) => t,
			None => {
				if self.loglevel > 0 {
					self.callstack.pop();
				}
				return left;
			}
		};

		let ret = match next {
			Token::Plus => {
				if self.loglevel > 0 {
					self.log("Plus");
				}
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Plus,
						right: Box::new(self.parse_expr()) 
					}
				)
			},
			Token::Minus => {
				if self.loglevel > 0 {
					self.log("Minus");
				}
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Minus,
						right: Box::new(self.parse_expr()) 
					}
				)
			},
			Token::OpenParen => self.parse_call(left),
			Token::Dot => self.parse_prob_access(left),
			Token::Eq => {
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Eq,
						right: Box::new(self.parse_expr()) 
					}
				)
			},
			_ => {
				left
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_call(&mut self, caller: ASTNode) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_call".to_string());
		}

		self.skip(1);

		let mut args = Vec::new();

		while let Some(token) = self.peek(0) {
			match token {
				Token::CloseParen => {
					self.skip(1);
					break;
				},
				Token::Comma => {
					self.skip(1);
				},
				_ => {
					args.push(self.parse_item().unwrap());
				}
			}
		}

		if self.loglevel > 0 {
			self.log("call done");
		}

		let call = ASTNode::Call(
			Call {
				callee: Box::new(caller),
				args: args,
			}
		);

		let ret = match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(call)
				}
				_ => call,
			},
			None => call,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_prob_access(&mut self, left: ASTNode) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_prob_access".to_string());
		}

		self.skip(1);

		let ident = self.expect_ident();
		
		let prob_access = ASTNode::ProbAccess(
			ProbAccess {
				object: Box::new(left),
				property: ident,
			}
		);

		let ret = match self.peek(0) {
			Some(t) => match t {
				Token::OpenParen => {
					self.parse_call(prob_access)
				},
				_ => prob_access,
			},
			None => prob_access,
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_term(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_term".to_string());
		}

		let left = self.parse_factor();

		let next = match self.peek(0) {
			Some(t) => t,
			None => {
				if self.loglevel > 0 {
					self.callstack.pop();
				}
				return left;
			}
		};

		let ret = match next {
			Token::Multiply => {
				if self.loglevel > 0 {
					self.log("Multiply");
				}
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Mul,
						right: Box::new(self.parse_factor()) 
					}
				)
			},
			Token::Divide => {
				if self.loglevel > 0 {
					self.log("Divide");
				}
				self.skip(1);
				ASTNode::BinOp(
					BinOp { 
						left: Box::new(left), 
						op: Op::Div,
						right: Box::new(self.parse_factor()) 
					}
				)
			},
			_ => {
				left
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}

	fn parse_factor(&mut self) -> ASTNode {
		if self.loglevel > 0 {
			self.callstack.push("parse_factor".to_string());
		}

		let next = match self.eat() {
			Some(t) => t,
			None => {
				panic!("Unexpected end of tokens");
			}
		};

		let ret = match next {
			Token::Ident(ident) => {
				if self.loglevel > 0 {
					self.log(&format!("Ident: {}", ident));
				}

				ASTNode::Ident(ident.to_string())
			}
			Token::String(s) => ASTNode::Lit(Value::Str(s)),
			Token::Int(num) => ASTNode::Lit(Value::Int(num)),
			Token::Float(num) => ASTNode::Lit(Value::Float(num)),
			Token::True => ASTNode::Lit(Value::Bool(true)),
			Token::False => ASTNode::Lit(Value::Bool(false)),
			Token::OpenParen => {
				let node = self.parse_expr();	
				self.expect_eat(Token::CloseParen);
				return node;
			},
			_ => {
				println!("{}", self.curr_loc());
				panic!("Unexpected token {:?}", next);
			}
		};

		if self.loglevel > 0 {
			self.callstack.pop();
		}

		ret
	}
}
