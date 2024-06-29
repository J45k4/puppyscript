#[cfg(test)]
mod tests {
    use crate::parsing::*;
    use crate::types::*;

    use super::*;

	#[test]
	fn test_simple_plus_expr() {
		let code = r#"
			a = 1 + 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Lit(Value::Int(1))),
								op: Op::Plus,
								right: Box::new(ASTNode::Lit(Value::Int(2))),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_minus_expr() {
		let code = r#"
			a = 1 - 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Lit(Value::Int(1))),
								op: Op::Minus,
								right: Box::new(ASTNode::Lit(Value::Int(2))),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_mul_expr() {
		let code = r#"
			a = 1 * 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Lit(Value::Int(1))),
								op: Op::Mul,
								right: Box::new(ASTNode::Lit(Value::Int(2))),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_div_expr() {
		let code = r#"
			a = 1 / 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Lit(Value::Int(1))),
								op: Op::Div,
								right: Box::new(ASTNode::Lit(Value::Int(2))),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_paren_expr() {
		let code = r#"
			a = (1 + 2) * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(
									ASTNode::BinOp(
										BinOp {
											left: Box::new(ASTNode::Lit(Value::Int(1))),
											op: Op::Plus,
											right: Box::new(ASTNode::Lit(Value::Int(2))),
										}
									)
								),
								op: Op::Mul,
								right: Box::new(ASTNode::Lit(Value::Int(3))),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_expr_ordering() {
		let code = r#"
			a = 1 + 2 * 3
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::BinOp(
							BinOp {
								left: Box::new(ASTNode::Lit(Value::Int(1))),
								op: Op::Plus,
								right: Box::new(
									ASTNode::BinOp(
										BinOp {
											left: Box::new(ASTNode::Lit(Value::Int(2))),
											op: Op::Mul,
											right: Box::new(ASTNode::Lit(Value::Int(3))),
										}
									)
								),
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_without_args() {
		let code = r#"
			a = foo()
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(ASTNode::Ident("foo".to_string())),
								args: vec![],
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_with_num_arg() {
		let code = r#"
			a = foo(1)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expeted = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(ASTNode::Ident("foo".to_string())),
								args: vec![
									ASTNode::Lit(Value::Int(1)),
								],
							}
						)
					)
				}
			)
		];

		assert_eq!(ast, expeted);
	}

	#[test]
	fn test_double_call() {
		let code = r#"
			a = foo(1)(2)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(
						ASTNode::Call(
							Call {
								callee: Box::new(
									ASTNode::Call(
										Call {
											callee: Box::new(ASTNode::Ident("foo".to_string())),
											args: vec![
												ASTNode::Lit(Value::Int(1)),
											],
										}
									)
								),
								args: vec![
									ASTNode::Lit(Value::Int(2)),
								],
							}

						)
					),
				},
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_prob_access() {
		let code = r#"
			foo.bar
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ProbAccess(
				ProbAccess {
					object: Box::new(ASTNode::Ident("foo".to_string())),
					property: "bar".to_string(),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_method_call() {
		let code = r#"
			foo.bar(1)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Call(
				Call {
					callee: Box::new(
						ASTNode::ProbAccess(
							ProbAccess {
								object: Box::new(ASTNode::Ident("foo".to_string())),
								property: "bar".to_string(),
							}
						)
					),
					args: vec![
						ASTNode::Lit(Value::Int(1)),
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_call_with_callback() {
		let code = r#"
			foo(() => 5)
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Call(
				Call {
					callee: Box::new(ASTNode::Ident("foo".to_string())),
					args: vec![
						ASTNode::Fun(
							Fun {
								params: vec![],
								body: vec![
									ASTNode::Lit(Value::Int(5)),
								],
							}
						),
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_array() {
		let code = r#"
			l = []
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("l".to_string())),
					right: Box::new(
						ASTNode::Array(
							Array {
								items: vec![],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_array_with_many_numbers() {
		let code = r#"
			l = [1, 2, 3]
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("l".to_string())),
					right: Box::new(
						ASTNode::Array(
							Array {
								items: vec![
									ASTNode::Lit(Value::Int(1)),
									ASTNode::Lit(Value::Int(2)),
									ASTNode::Lit(Value::Int(3)),
								],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_named_instance() {
		let code = r#"
			Ball {}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("Ball".to_string()),
					props: vec![],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_named_instance_fields() {
		let code = r#"
			Ball {
				x: 1,
				y: 2,
				name: "nakki"
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("Ball".to_string()),
					props: vec![
						Property {
							name: "x".to_string(),
							value: Box::new(ASTNode::Lit(Value::Int(1))),
						},
						Property {
							name: "y".to_string(),
							value: Box::new(ASTNode::Lit(Value::Int(2))),
						},
						Property {
							name: "name".to_string(),
							value: Box::new(ASTNode::Lit(Value::Str("nakki".to_string()))),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_fun_empty_param_and_body() {
		let code = r#"
			foo = () => {}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![],
								body: vec![],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_simple_fun() {
		let code = r#"
			foo = (a, b) => {
				a + b
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![
									Param { name: "a".to_string() },
									Param { name: "b".to_string() }
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Ident("b".to_string())),
										}
									)
								],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_obj_field_fun() {
		let code = r#"
			Div {
				on_click: () => {}
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("Div".to_string()),
					props: vec![
						Property {
							name: "on_click".to_string(),
							value: Box::new(
								ASTNode::Fun(
									Fun {
										params: vec![],
										body: vec![],
									}
								)
							),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_fun_without_paren() {
		let code = r#"
			foo = a => {
				a + 1
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![
									Param { name: "a".to_string() }
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Lit(Value::Int(1))),
										}
									)
								],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_fun_without_block() {
		let code = r#"
			foo = a => a + 1
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("foo".to_string())),
					right: Box::new(
						ASTNode::Fun(
							Fun {
								params: vec![
									Param { name: "a".to_string() }
								],
								body: vec![
									ASTNode::BinOp(
										BinOp {
											op: Op::Plus,
											left: Box::new(ASTNode::Ident("a".to_string())),
											right: Box::new(ASTNode::Lit(Value::Int(1))),
										}
									)
								],
							}
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_return_expr() {
		let code = r#"
			return 1 + 5
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Ret(
				Ret {
					value: Box::new(
						Some(
							ASTNode::BinOp(
								BinOp {
									op: Op::Plus,
									left: Box::new(ASTNode::Lit(Value::Int(1))),
									right: Box::new(ASTNode::Lit(Value::Int(5))),
								}
							)
						)
					),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_return() {
		let code = r#"
			return
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Ret(
				Ret {
					value: Box::new(None),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_parse_obj_instance_in_array() {
		let code = r#"
			[
				Div { }
			]
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Array(
				Array {
					items: vec![
						ASTNode::ObjIns(
							ObjIns {
								name: Some("Div".to_string()),
								props: vec![],
							}
						)
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_parse_vertex() {
		let code = r#"
			Vertex { x: -0.6, y: 0.1, color: "black" }
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("Vertex".to_string()),
					props: vec![
						Property {
							name: "x".to_string(),
							value: Box::new(ASTNode::Lit(Value::Float(-0.6))),
						},
						Property {
							name: "y".to_string(),
							value: Box::new(ASTNode::Lit(Value::Float(0.1))),
						},
						Property {
							name: "color".to_string(),
							value: Box::new(ASTNode::Lit(Value::Str("black".to_string()))),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	pub fn test_ident_with_number() {
		let code = r#"
			H1 {
				text: "Todo"
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("H1".to_string()),
					props: vec![
						Property {
							name: "text".to_string(),
							value: Box::new(ASTNode::Lit(Value::Str("Todo".to_string()))),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_type_works_as_filename() {
		// Maybe in the future there will be type keyword
		let code = r#"
			Input {
				type: "text"
			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::ObjIns(
				ObjIns {
					name: Some("Input".to_string()),
					props: vec![
						Property {
							name: "type".to_string(),
							value: Box::new(ASTNode::Lit(Value::Str("text".to_string()))),
						},
					],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_equal_op() {
		let code = r#"
			5 == 2
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::BinOp(
				BinOp {
					op: Op::Eq,
					left: Box::new(ASTNode::Lit(Value::Int(5))),
					right: Box::new(ASTNode::Lit(Value::Int(2))),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_if_parsing() {
		let code = r#"
			if a == 5 {

			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::If(
				If {
					cond: Box::new(
						ASTNode::BinOp(
							BinOp {
								op: Op::Eq,
								left: Box::new(ASTNode::Ident("a".to_string())),
								right: Box::new(ASTNode::Lit(Value::Int(5))),
							}
						)
					),
					body: vec![],
					els: None
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_if_else_parsing() {
		let code = r#"
			if a == 5 {

			} else {

			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::If(
				If {
					cond: Box::new(
						ASTNode::BinOp(
							BinOp {
								op: Op::Eq,
								left: Box::new(ASTNode::Ident("a".to_string())),
								right: Box::new(ASTNode::Lit(Value::Int(5))),
							}
						)
					),
					body: vec![],
					els: None
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_empty_for() {
		let code = r#"
			for {

			}
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::For(
				For {
					cond: ForCond::None,
					body: vec![],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn test_for_from_it() {
		let code = r#"
			for i in iterator {

			}
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		let expected = vec![
			ASTNode::For(
				For {
					cond: ForCond::FromIt {
						ident: "i".to_string(),
						it: Box::new(ASTNode::Ident("iterator".to_string())),
					},
					body: vec![],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn for_array() {
		let code = r#"
			for i in [1, 2, 3] {

			}
		"#;

		let ast = Parser::new(code)
			.set_loglevel(1)
			.parse();

		let expected = vec![
			ASTNode::For(
				For {
					cond: ForCond::FromIt {
						ident: "i".to_string(),
						it: Box::new(
							ASTNode::Array(
								Array {
									items: vec![
										ASTNode::Lit(Value::Int(1)),
										ASTNode::Lit(Value::Int(2)),
										ASTNode::Lit(Value::Int(3)),
									],
								}
							)
						),
					},
					body: vec![],
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn single_line_comments() {
		let code = r#"
		// This is a comment
		a = 1 // This is also a comment
		"#;

		let ast = Parser::new(code)
			.parse();

		let expected = vec![
			ASTNode::Assign(
				Assign {
					left: Box::new(ASTNode::Ident("a".to_string())),
					right: Box::new(ASTNode::Lit(Value::Int(1))),
				}
			)
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn list_map() {
		let code = "[1].map(p => p * 2)";
		let ast = Parser::new(code).set_loglevel(1).parse();
		let expected = vec![ASTNode::Call(Call {
			callee: Box::new(ASTNode::ProbAccess(ProbAccess {
				object: Box::new(ASTNode::Array(Array {
					items: vec![ASTNode::Lit(Value::Int(1))],
				})),
				property: "map".to_string(),
			})),
			args: vec![ASTNode::Fun(Fun {
				params: vec![Param {
					name: "p".to_string(),
				}],
				body: vec![ASTNode::BinOp(BinOp {
					op: Op::Mul,
					left: Box::new(ASTNode::Ident("p".to_string())),
					right: Box::new(ASTNode::Lit(Value::Int(2))),
				})],
			})],
		})];

		assert_eq!(ast, expected);
	}

	#[test]
	fn unamed_object() {
		let code = r#"
			obj = {
				x: 1,
				y: 2,
			}
		"#;

		let ast = Parser::new(code).set_loglevel(1).parse();
		let expected = vec![ASTNode::Assign(Assign {
			left: Box::new(ASTNode::Ident("obj".to_string())),
			right: Box::new(ASTNode::ObjIns(ObjIns {
				name: None,
				props: vec![
					Property {
						name: "x".to_string(),
						value: Box::new(ASTNode::Lit(Value::Int(1))),
					},
					Property {
						name: "y".to_string(),
						value: Box::new(ASTNode::Lit(Value::Int(2))),
					},
				],
			})),
		})];

		assert_eq!(ast, expected);
	}

	#[test]
	fn object_same_name_assigment() {
		let code = r#"
		camera = Camera {}

		player = Node {
			camera
		}"#;

		let ast = Parser::new(code).parse();

		let expected = vec![
			ASTNode::Assign(Assign {
				left: Box::new(ASTNode::Ident("camera".to_string())),
				right: Box::new(ASTNode::ObjIns(ObjIns {
					name: Some("Camera".to_string()),
					props: vec![],
				})),
			}),
			ASTNode::Assign(Assign {
				left: Box::new(ASTNode::Ident("player".to_string())),
				right: Box::new(ASTNode::ObjIns(ObjIns {
					name: Some("Node".to_string()),
					props: vec![Property {
						name: "camera".to_string(),
						value: Box::new(ASTNode::Ident("camera".to_string())),
					}],
				})),
			}),
		];

		assert_eq!(ast, expected);
	}

	#[test]
	fn match_parsing() {
		let code = r#"
		match a {
			1 => 2,
			_ => 3
		}"#;

		let ast = Parser::new(code).set_loglevel(1).parse();
		// println!("{:?}", ast);
	}
}
