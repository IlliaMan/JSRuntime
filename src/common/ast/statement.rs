use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  ExpressionStatement {
    expression: Box<Expression>,
  },
  Declaration {
    is_const: bool,
    // TODO: Need a way to have sort of Expression::Identifier as type here
    name: Box<Expression>,
    value: Box<Option<Expression>>,
  },
  FunctionDeclaration {
    // TODO: Need a way to have sort of Expression::Identifier as type here
    name: Box<Expression>,
    // TODO: Need a way to have sort of Expression::Identifier as type here
    params: Box<Vec<Expression>>,
    body: Box<Vec<Statement>>,
  }
}
