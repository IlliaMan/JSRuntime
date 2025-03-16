use crate::common::{TokenType, Literal};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Grouping {
        expression: Box<Expression>,
    },
    Comparison {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    Unary {
        operator: TokenType,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    Call {
        callee: String,
        args: Box<Vec<Expression>>
    },
}

impl Expression {
    pub fn extract_string(expr: &Self) -> Option<String> {
        match expr {
            Self::Literal(literal) => {
                match literal {
                    Literal::String(value) => Some(String::from(value)),
                    _ => None,
                }
            },
            Self::Identifier(value) => Some(String::from(value)),
            _ => None,
        }
    }
}