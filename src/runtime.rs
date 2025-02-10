use crate::{parser::{Expression, Statement}, scanner::token::TokenType};
use std::collections::{HashMap, HashSet};

pub struct Runtime {
    environment: Environment,
}

pub struct Environment {
    variables: HashMap<String, RuntimeValue>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            if let Err(error_message) = self.evaluate_statement(statement) {
              println!("runtime>: {}", error_message);
            }
        }
    }

    fn evaluate_statement(&mut self, statement: Statement) -> Result<(), String> {
      match statement {
        Statement::Declaration { is_const, name, value} => {
          let value = match &*value {
            Some(expr) => self.evalutate_expression(expr)?,
            None => RuntimeValue::Undefined,
          };

          let name = match &*name {
            Expression::Identifier(name) => String::from(name),
            _ => panic!("parser bug: name can only Expression::Identifier, got {:?}", name),
          };

          if self.environment.variables.contains_key(&name) {
            return Err(format!("variable {} already declared", name));
          }

          if is_const {
            self.environment.constants.insert(name.clone());
          }

          println!("runtime>: created {:?} = {:?}", name, value);
          self.environment.variables.insert(name, value);
        },
        Statement::ExpressionStatement { expression} => {
          let value = self.evalutate_expression(&expression)?;

          println!("runtime>: {:?}", value);
        },
      }

      Ok(())
    }

    fn evalutate_expression(&self, expression: &Expression) -> Result<RuntimeValue, String> {
      match expression {
        Expression::Number(value) => Ok(RuntimeValue::Number(*value)),
        Expression::String(value) => Ok(RuntimeValue::String(String::from(value))),
        Expression::Boolean(value) => Ok(RuntimeValue::Boolean(*value)),
        Expression::Null => Ok(RuntimeValue::Null),
        Expression::Undefined => Ok(RuntimeValue::Undefined),
        Expression::Identifier(value) => {
          match self.environment.variables.get(value) {
            Some(value) => Ok(value.clone()),
            None => Ok(RuntimeValue::Undefined),
          }
        }
        Expression::Grouping { expression } => self.evalutate_expression(&**expression),
        Expression::Unary { operator, right } => {
          let right_value = self.evalutate_expression(right.as_ref())?;
          match (operator, right_value) {
            (TokenType::Minus, RuntimeValue::Number(n)) => Ok(RuntimeValue::Number(-n)),
            _ => Err(format!("invalid unary operator: {:?}", operator)),
          }
        }
        Expression::Binary { left, operator, right } => {
          let left_value = self.evalutate_expression(left.as_ref())?;
          let right_value = self.evalutate_expression(right.as_ref())?;

          match (left_value.clone(), operator, right_value.clone()) {
            (RuntimeValue::Number(a), TokenType::Star, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a * b)),
            (RuntimeValue::Number(a), TokenType::Slash, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a / b)),
            (RuntimeValue::Number(a), TokenType::Plus, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a + b)),
            (RuntimeValue::Number(a), TokenType::Minus, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a - b)),
            _ => Err(format!("unhandled binary expression: {:?} {:?} {:?}", left_value, operator, right_value)),
          }
        },
        Expression::Comparison { left, operator, right } => {
          let left_value = self.evalutate_expression(left.as_ref())?;
          let right_value = self.evalutate_expression(right.as_ref())?;

          // TODO: comparing every type vs every time is not efficient and too much boilerplate
          match (left_value.clone(), right_value.clone()) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::String(a), RuntimeValue::String(b)) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::Null, RuntimeValue::Null) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(true)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(true)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(true)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(true)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::Undefined, RuntimeValue::Undefined) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(true)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(true)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::Null, RuntimeValue::Undefined) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(true)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(true)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            (RuntimeValue::Undefined, RuntimeValue::Null) => {
              match operator {
                TokenType::Equal => Ok(RuntimeValue::Boolean(true)),
                TokenType::NotEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(true)),
                TokenType::GreaterThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThan => Ok(RuntimeValue::Boolean(false)),
                TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(false)),
                _ => Err(format!("{:?} {:?} {:?} comparison expression is not supported", left_value, operator, right_value)),
              }
            },
            _ => Err(format!("unhandled comparison expression: {:?} {:?} {:?}", left_value, operator, right_value)),
          }
        }
      }
    }
}