use crate::common::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
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
        // TODO: Need a way to have sort of Expression::Identifier as type here
        args: Box<Vec<Expression>>
    },
}

impl Expression {
    pub fn extract_string(expr: &Self) -> Option<String> {
        match expr {
            Self::String(value) => Some(String::from(value)),
            Self::Identifier(value) => Some(String::from(value)),
            _ => None,
        }
    }
}