use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  ExpressionStatement {
    expression: Box<Expression>,
  },
  Declaration {
    is_const: bool,
    name: String,
    value: Box<Option<Expression>>,
  },
  FunctionDeclaration {
    name: String,
    params: Vec<String>,
    body: Box<Vec<Statement>>,
  },
  Return {
    expression: Box<Expression>,
  }
}
