use crate::parser::{Expression, Parser, Statement};
use crate::scanner::{token::TokenType, Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literals() {
        let tokens = vec![
            Token::new(TokenType::Number(5.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Boolean(true), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Boolean(false), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::String("hello".into()), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Null, 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Undefined, 1),
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
                    expression: Box::new(Expression::Number(5.0))
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Boolean(true))
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Boolean(false))
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::String("hello".into()))
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Null)
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Undefined)
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
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::Number(8.0), 1),
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
                    left: Box::new(Expression::Number(1.0)),
                    operator: TokenType::Slash,
                    right: Box::new(Expression::Number(8.0)),
                })
            }]
        );
    }

    #[test]
    fn parse_grouping() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(8.0), 1),
            Token::new(TokenType::RightParen, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Number(3.0), 1),
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
                    left: Box::new(Expression::Grouping {
                        expression: Box::new(Expression::Binary {
                            left: Box::new(Expression::Number(1.0)),
                            operator: TokenType::Plus,
                            right: Box::new(Expression::Number(8.0)),
                        }),
                    }),
                    operator: TokenType::Star,
                    right: Box::new(Expression::Number(3.0))
                })
            }]
        );
    }

    #[test]
    fn test_valid_declaration_with_initializer() {
        let tokens = vec![
            Token::new(TokenType::KeywordLet, 1),
            Token::new(TokenType::Identifier("x".into()), 1),
            Token::new(TokenType::Assign, 1),
            Token::new(TokenType::Number(5.0), 1),
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
                name: Box::new(Expression::Identifier("x".into())),
                value: Box::new(Some(Expression::Number(5.0)))
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
                name: Box::new(Expression::Identifier("x".into())),
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
            Token::new(TokenType::Number(5.0), 1),
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
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Number(3.0), 1),
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
                    left: Box::new(Expression::Number(1.0)),
                    operator: TokenType::Plus,
                    right: Box::new(Expression::Binary {
                        left: Box::new(Expression::Number(2.0)),
                        operator: TokenType::Star,
                        right: Box::new(Expression::Number(3.0))
                    })
                })
            }]
        );
    }

    #[test]
    fn test_operator_precedence_division() {
        let tokens = vec![
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Minus, 1),
            Token::new(TokenType::Number(10.0), 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::Number(2.0), 1),
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
                    left: Box::new(Expression::Number(1.0)),
                    operator: TokenType::Minus,
                    right: Box::new(Expression::Binary {
                        left: Box::new(Expression::Number(10.0)),
                        operator: TokenType::Slash,
                        right: Box::new(Expression::Number(2.0))
                    })
                })
            }]
        );
    }

    #[test]
    fn test_operator_precedence_parentheses() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::RightParen, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Number(3.0), 1),
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
                    left: Box::new(Expression::Grouping {
                        expression: Box::new(Expression::Binary {
                            left: Box::new(Expression::Number(1.0)),
                            operator: TokenType::Plus,
                            right: Box::new(Expression::Number(2.0))
                        })
                    },),
                    operator: TokenType::Star,
                    right: Box::new(Expression::Number(3.0))
                })
            }]
        );
    }

    #[test]
    fn test_comparison_operators() {
        let tokens = vec![
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::String("hello".into()), 1),
            Token::new(TokenType::NotEqual, 1),
            Token::new(TokenType::Number(4.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Boolean(false), 1),
            Token::new(TokenType::StrictEqual, 1),
            Token::new(TokenType::Boolean(true), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Boolean(false), 1),
            Token::new(TokenType::StrictNotEqual, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::GreaterThan, 1),
            Token::new(TokenType::Number(2.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Null, 1),
            Token::new(TokenType::LessThan, 1),
            Token::new(TokenType::Undefined, 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Boolean(false), 1),
            Token::new(TokenType::GreaterThanOrEqual, 1),
            Token::new(TokenType::Undefined, 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::String("hello".into()), 1),
            Token::new(TokenType::LessThanOrEqual, 1),
            Token::new(TokenType::Null, 1),
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
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Number(1.0)),
                        operator: TokenType::Equal,
                        right: Box::new(Expression::Number(2.0))
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::String("hello".into())),
                        operator: TokenType::NotEqual,
                        right: Box::new(Expression::Number(4.0))
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Boolean(false)),
                        operator: TokenType::StrictEqual,
                        right: Box::new(Expression::Boolean(true))
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Boolean(false)),
                        operator: TokenType::StrictNotEqual,
                        right: Box::new(Expression::Number(2.0))
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Number(1.0)),
                        operator: TokenType::GreaterThan,
                        right: Box::new(Expression::Number(2.0))
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Null),
                        operator: TokenType::LessThan,
                        right: Box::new(Expression::Undefined)
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::Boolean(false)),
                        operator: TokenType::GreaterThanOrEqual,
                        right: Box::new(Expression::Undefined)
                    })
                },
                Statement::ExpressionStatement {
                    expression: Box::new(Expression::Comparison {
                        left: Box::new(Expression::String("hello".into())),
                        operator: TokenType::LessThanOrEqual,
                        right: Box::new(Expression::Null)
                    })
                }
            ]
        );
    }
}
