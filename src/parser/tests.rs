use super::*;
use crate::common::{*, ast::*};

#[test]
fn parse_literals() {
    let tokens = vec![
        Token::new(TokenType::Literal(Literal::Number(5.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Literal(Literal::Boolean(true)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Literal(Literal::Boolean(false)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Literal(Literal::String("hello".into())), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Literal(Literal::Null), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Literal(Literal::Undefined), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      vec![
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::Number(5.0)))
        },
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::Boolean(true)))
        },
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::Boolean(false)))
        },
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::String("hello".into())))
        },
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::Null))
        },
        Statement::ExpressionStatement {
          expression: Box::new(Expression::Literal(Literal::Undefined))
        },
      ]
    );
}

#[test]
fn parse_empty_program() {
    let tokens = vec![Token::new(TokenType::Eof, 1)];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
fn parse_binary_expression() {
    let tokens = vec![
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Slash, 1),
        Token::new(TokenType::Literal(Literal::Number(8.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![Statement::ExpressionStatement {
            expression: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: TokenType::Slash,
                right: Box::new(Expression::Literal(Literal::Number(8.0))),
            }) 
        }]
    );
}

#[test]
fn parse_grouping() {
    let tokens = vec![
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Plus, 1),
        Token::new(TokenType::Literal(Literal::Number(8.0)), 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Star, 1),
        Token::new(TokenType::Literal(Literal::Number(3.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![
          Statement::ExpressionStatement {
            expression: Box::new(Expression::Binary { 
                left: Box::new(Expression::Grouping {
                    expression: Box::new(Expression::Binary {
                        left: Box::new(Expression::Literal(Literal::Number(1.0))),
                        operator: TokenType::Plus,
                        right: Box::new(Expression::Literal(Literal::Number(8.0))),
                    }),
                }),
                operator: TokenType::Star,
                right: Box::new(Expression::Literal(Literal::Number(3.0)))
            })
          }
        ]
    );
}

#[test]
fn test_valid_declaration_with_initializer() {
    let tokens = vec![
      Token::new(TokenType::KeywordLet, 1),
      Token::new(TokenType::Identifier("x".into()), 1),
      Token::new(TokenType::Assign, 1),
      Token::new(TokenType::Literal(Literal::Number(5.0)), 1),
      Token::new(TokenType::Semicolon, 1),
      Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![Statement::Declaration {
            is_const: false,
            name: "x".into(),
            value: Box::new(Some(Expression::Literal(Literal::Number(5.0))))
        }]
    );
}

#[test]
fn test_valid_declaration_without_initializer() {
    let tokens = vec![
      Token::new(TokenType::KeywordConst, 1),
      Token::new(TokenType::Identifier("x".into()), 1),
      Token::new(TokenType::Semicolon, 1),
      Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![Statement::Declaration {
            is_const: true,
            name: "x".into(),
            value: Box::new(None)
        }]
    );
}

#[test]
fn test_identifiers_as_right_hand_side_values() {
    let tokens = vec![
      Token::new(TokenType::KeywordConst, 1),
      Token::new(TokenType::Identifier("x".into()), 1),
      Token::new(TokenType::Assign, 1),
      Token::new(TokenType::Literal(Literal::Number(5.0)), 1),
      Token::new(TokenType::Semicolon, 1),
      Token::new(TokenType::KeywordLet, 1),
      Token::new(TokenType::Identifier("y".into()), 1),
      Token::new(TokenType::Assign, 1),
      Token::new(TokenType::Identifier("x".into()), 1),
      Token::new(TokenType::Semicolon, 1),
      Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
}

 #[test]
fn test_operator_precedence_multiplication() {
    let tokens = vec![
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Plus, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::Star, 1),
        Token::new(TokenType::Literal(Literal::Number(3.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: TokenType::Plus,
                right: Box::new(
                    Expression::Binary { 
                        left: Box::new(Expression::Literal(Literal::Number(2.0))),
                        operator: TokenType::Star,
                        right: Box::new(Expression::Literal(Literal::Number(3.0)))
                    }
                )
            })
        }
    ]);
}

#[test]
fn test_operator_precedence_division() {
    let tokens = vec![
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Minus, 1),
        Token::new(TokenType::Literal(Literal::Number(10.0)), 1),
        Token::new(TokenType::Slash, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: TokenType::Minus,
                right: Box::new(
                    Expression::Binary { 
                        left: Box::new(Expression::Literal(Literal::Number(10.0))),
                        operator: TokenType::Slash,
                    right: Box::new(Expression::Literal(Literal::Number(2.0)))
                    }
                )
            })
        }
    ]);
}

#[test]
fn test_operator_precedence_parentheses() {
    let tokens = vec![
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Plus, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Star, 1),
        Token::new(TokenType::Literal(Literal::Number(3.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Binary {
                left: Box::new(Expression::Grouping {
                    expression: Box::new(Expression::Binary { 
                        left: Box::new(Expression::Literal(Literal::Number(1.0))),
                        operator: TokenType::Plus,
                        right: Box::new(Expression::Literal(Literal::Number(2.0)))
                    })},
                ),
                operator: TokenType::Star,
                right: Box::new(Expression::Literal(Literal::Number(3.0)))
            })
        }
    ]);
}

#[test]
fn test_comparison_operators() {
    let tokens = vec![
        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Equal, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::Semicolon, 1),

        Token::new(TokenType::Literal(Literal::String("hello".into())), 1),
        Token::new(TokenType::NotEqual, 1),
        Token::new(TokenType::Literal(Literal::Number(4.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        
        Token::new(TokenType::Literal(Literal::Boolean(false)), 1),
        Token::new(TokenType::StrictEqual, 1),
        Token::new(TokenType::Literal(Literal::Boolean(true)), 1),
        Token::new(TokenType::Semicolon, 1),
        
        Token::new(TokenType::Literal(Literal::Boolean(false)), 1),
        Token::new(TokenType::StrictNotEqual, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::Semicolon, 1),

        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::GreaterThan, 1),
        Token::new(TokenType::Literal(Literal::Number(2.0)), 1),
        Token::new(TokenType::Semicolon, 1),
        
        Token::new(TokenType::Literal(Literal::Null), 1),
        Token::new(TokenType::LessThan, 1),
        Token::new(TokenType::Literal(Literal::Undefined), 1),
        Token::new(TokenType::Semicolon, 1),
        
        Token::new(TokenType::Literal(Literal::Boolean(false)), 1),
        Token::new(TokenType::GreaterThanOrEqual, 1),
        Token::new(TokenType::Literal(Literal::Undefined), 1),
        Token::new(TokenType::Semicolon, 1),
        
        Token::new(TokenType::Literal(Literal::String("hello".into())), 1),
        Token::new(TokenType::LessThanOrEqual, 1),
        Token::new(TokenType::Literal(Literal::Null), 1),
        Token::new(TokenType::Semicolon, 1),

        Token::new(TokenType::Eof, 1)
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: TokenType::Equal,
                right: Box::new(Expression::Literal(Literal::Number(2.0)))
            })
        },

        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::String("hello".into()))),
                operator: TokenType::NotEqual,
                right: Box::new(Expression::Literal(Literal::Number(4.0)))
            })
        },
        
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Boolean(false))),
                operator: TokenType::StrictEqual,
                right: Box::new(Expression::Literal(Literal::Boolean(true)))
            })
        },
        
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Boolean(false))),
                operator: TokenType::StrictNotEqual,
                right: Box::new(Expression::Literal(Literal::Number(2.0)))
            })
        },
        
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Number(1.0))),
                operator: TokenType::GreaterThan,
                right: Box::new(Expression::Literal(Literal::Number(2.0)))
            })
        },
        
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Null)),
                operator: TokenType::LessThan,
                right: Box::new(Expression::Literal(Literal::Undefined))
            })
        },

        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::Boolean(false))),
                operator: TokenType::GreaterThanOrEqual,
                right: Box::new(Expression::Literal(Literal::Undefined))
            })
        },
        
        Statement::ExpressionStatement { 
            expression: Box::new(Expression::Comparison { 
                left: Box::new(Expression::Literal(Literal::String("hello".into()))),
                operator: TokenType::LessThanOrEqual,
                right: Box::new(Expression::Literal(Literal::Null))
            })
        }
    ]);
}

#[test]
fn test_function_declaration_no_params() {
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::LeftCurlyBrace, 1),
        Token::new(TokenType::Return, 1),
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::RightCurlyBrace, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::FunctionDeclaration {
            name: "hello".into(),
            params: vec![],
            body: Box::new(vec![
                Statement::Return  {
                    expression: Box::new(Expression::Identifier("hello".into())) 
                }
            ])
        }
    ]);
}

#[test]
fn test_function_declaration_empty_body() {
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::LeftCurlyBrace, 1),
        Token::new(TokenType::RightCurlyBrace, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::FunctionDeclaration {
            name: "hello".into(),
            params: vec![],
            body: Box::new(vec![
                Statement::Return { 
                    expression: Box::new(Expression::Literal(Literal::Undefined))
                }
            ])
        }
    ]);
}

#[test]
fn test_function_declaration_with_return_nothing() {
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::LeftCurlyBrace, 1),
        Token::new(TokenType::Return, 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::RightCurlyBrace, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::FunctionDeclaration {
            name: "hello".into(),
            params: vec![],
            body: Box::new(vec![
                Statement::Return { expression: Box::new(Expression::Literal(Literal::Undefined)) }
            ])
        }
    ]);
}

#[test]
fn test_function_declaration_with_params() {
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("add".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Identifier("x".into()), 1),
        Token::new(TokenType::Comma, 1),
        Token::new(TokenType::Identifier("y".into()), 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::LeftCurlyBrace, 1),
        Token::new(TokenType::Return, 1),
        Token::new(TokenType::Identifier("x".into()), 1),
        Token::new(TokenType::Plus, 1),
        Token::new(TokenType::Identifier("y".into()), 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::RightCurlyBrace, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Statement::FunctionDeclaration {
            name: "add".into(),
            params: vec!["x".into(), "y".into()],
            body: Box::new(vec![
                Statement::Return { 
                    expression: Box::new(Expression::Binary { 
                        left: Box::new(Expression::Identifier("x".into())),
                        operator: TokenType::Plus,
                        right: Box::new(Expression::Identifier("y".into()))
                    })}
                ])
            }
        ]
    );
}

#[test]
fn test_function_declaration_with_invalid_syntax() {
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("get".into()), 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Return, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    
    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());

    let tokens = vec![
        Token::new(TokenType::Function, 1),
        Token::new(TokenType::Identifier("add".into()), 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Comma, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::LeftCurlyBrace, 1),
        Token::new(TokenType::RightCurlyBrace, 1),
        Token::new(TokenType::Eof, 1),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_valid_function_calls() {
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Statement::ExpressionStatement {
            expression: Box::new(Expression::Call {
                callee: "hello".into(),
                args: Box::new(vec![])
            })
        }
    ]);
    
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 2),
        Token::new(TokenType::Identifier("name".into()), 3),
        Token::new(TokenType::Comma, 4),
        Token::new(TokenType::Identifier("surname".into()), 5),
        Token::new(TokenType::RightParen, 6),
        Token::new(TokenType::Semicolon, 7),
        Token::new(TokenType::Eof, 8)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Statement::ExpressionStatement {
            expression: Box::new(Expression::Call {
                callee: "hello".into(),
                args: Box::new(vec![
                    Expression::Identifier("name".into()),
                    Expression::Identifier("surname".into()),
                ])
            })
        }
    ]);
    
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),

        Token::new(TokenType::Identifier("name".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Comma, 1),

        Token::new(TokenType::Literal(Literal::Number(1.0)), 1),
        Token::new(TokenType::Comma, 1),

        Token::new(TokenType::Literal(Literal::String("surname".into())), 1),

        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Statement::ExpressionStatement {
            expression: Box::new(Expression::Call {
                callee: "hello".into(),
                args: Box::new(vec![
                    Expression::Call {
                        callee: "name".into(),
                        args: Box::new(vec![]),
                    },
                    Expression::Literal(Literal::Number(1.0)),
                    Expression::Literal(Literal::String("surname".into()))
                ])
            })
        }
    ]);
}

#[test]
fn test_invalid_function_calls() {
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Semicolon, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Comma, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    
    let tokens = vec![
        Token::new(TokenType::Identifier("hello".into()), 1),
        Token::new(TokenType::LeftParen, 1),
        Token::new(TokenType::Literal(Literal::Boolean(true)), 1),
        Token::new(TokenType::Comma, 1),
        Token::new(TokenType::RightParen, 1),
        Token::new(TokenType::Eof, 1)
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}
