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

#[derive(Debug, Clone, Copy)]
pub enum RuntimeValue {
    Number(f64),
    Undefined,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            self.evaluate_statement(statement)?;
        }

        Ok(())
    }

    fn evaluate_statement(&mut self, statement: Statement) -> Result<(), String> {
      match statement {
        Statement::Declaration { is_const, name, value} => {
          let value = match &*value {
            Some(expr) => self.evalutate_expression(expr)?,
            None => RuntimeValue::Undefined,
          };

          if self.environment.variables.contains_key(&name.get_value()) {
            return Err(format!("variable {} already declared", name.get_value()));
          }

          if is_const {
            self.environment.constants.insert(name.get_value());
          }

          self.environment.variables.insert(name.get_value(), value);
          println!("runtime>: created {:?} = {:?}", name.get_value(), value);
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

          match (left_value, operator, right_value) {
            (RuntimeValue::Number(a), TokenType::Star, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a * b)),
            (RuntimeValue::Number(a), TokenType::Slash, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a / b)),
            (RuntimeValue::Number(a), TokenType::Plus, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a + b)),
            (RuntimeValue::Number(a), TokenType::Minus, RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a - b)),
            _ => Err(format!("unhandled binary expression: {:?} {:?} {:?}", left_value, operator, right_value)),
          }
        },
      }
    }
}