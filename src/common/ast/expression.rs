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
        // TODO: Need a way to have sort of Expression::Identifier as type here
        callee: Box<Expression>,
        // TODO: Need a way to have sort of Expression::Identifier as type here
        args: Box<Vec<Expression>>
    },
    Return {
        expression: Box<Expression>,
    },
}
