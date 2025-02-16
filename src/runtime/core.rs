use super::environment::Environment;
use super::values::RuntimeValue;
use crate::common::{*, ast::*};

pub struct Runtime {
    environment: Environment,
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
          let value: RuntimeValue = match &*value {
            Some(expr) => self.evalutate_expression(expr)?,
            _ => RuntimeValue::Undefined,
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
        Statement::FunctionDeclaration { name, params, body } => {
          let function_name = match &*name {
            Expression::Identifier(name) => String::from(name),
            _ => panic!("parser bug: function name can only Expression::Identifier, got {:?}", name),
          };

          println!("runtime>: created {:?}({:?})", function_name, params);
          self.environment.functions.insert(function_name, Statement::FunctionDeclaration { name, params, body });
        },
        Statement::Return { .. } => return Err("return statements can't be used outside of functions".into()),
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
            Expression::Identifier(value) => match self.environment.variables.get(value) {
                Some(value) => Ok(value.clone()),
                None => Ok(RuntimeValue::Undefined),
            },
            Expression::Grouping { expression } => self.evalutate_expression(&**expression),
            Expression::Unary { operator, right } => {
                let right_value = self.evalutate_expression(right.as_ref())?;
                match (operator, right_value) {
                    (TokenType::Minus, RuntimeValue::Number(n)) => Ok(RuntimeValue::Number(-n)),
                    _ => Err(format!("invalid unary operator: {:?}", operator)),
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evalutate_expression(left.as_ref())?;
                let right_value = self.evalutate_expression(right.as_ref())?;

                match (left_value.clone(), operator, right_value.clone()) {
                    (RuntimeValue::Number(a), TokenType::Star, RuntimeValue::Number(b)) => {
                        Ok(RuntimeValue::Number(a * b))
                    }
                    (RuntimeValue::Number(a), TokenType::Slash, RuntimeValue::Number(b)) => {
                        Ok(RuntimeValue::Number(a / b))
                    }
                    (RuntimeValue::Number(a), TokenType::Plus, RuntimeValue::Number(b)) => {
                        Ok(RuntimeValue::Number(a + b))
                    }
                    (RuntimeValue::Number(a), TokenType::Minus, RuntimeValue::Number(b)) => {
                        Ok(RuntimeValue::Number(a - b))
                    }
                    _ => Err(format!(
                        "unhandled binary expression: {:?} {:?} {:?}",
                        left_value, operator, right_value
                    )),
                }
            }
            Expression::Comparison {
                left,
                operator,
                right,
            } => {
                let left_value = self.evalutate_expression(left.as_ref())?;
                let right_value = self.evalutate_expression(right.as_ref())?;

                // TODO: comparing every type vs every time is not efficient and too much boilerplate
                match (left_value.clone(), right_value.clone()) {
                    (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                        self.compare_numbers(a, b, operator)
                    }
                    (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => {
                        self.compare_booleans(a, b, operator)
                    }
                    (RuntimeValue::String(a), RuntimeValue::String(b)) => {
                        self.compare_strings(&a, &b, operator)
                    }
                    (RuntimeValue::Null, RuntimeValue::Null) => self.compare_nulls(operator),
                    (RuntimeValue::Undefined, RuntimeValue::Undefined) => {
                        self.compare_undefinds(operator)
                    }
                    (RuntimeValue::Null, RuntimeValue::Undefined) => {
                        self.compare_null_undefined(operator)
                    }
                    (RuntimeValue::Undefined, RuntimeValue::Null) => {
                        self.compare_null_undefined(operator)
                    }
                    _ => Err(format!(
                        "unhandled comparison expression: {:?} {:?} {:?}",
                        left_value, operator, right_value
                    )),
                }
            },
            Expression::Call { callee, args } => self.call_function(callee, &*args),
            // Expression::Return { expression } => self.evalutate_expression(expression),
        }
    }

    fn call_function(&self, callee: &String, args: &Vec<Expression>) -> Result<RuntimeValue, String> {
      let function = self.get_function(callee)?;
      let evaluated_args = self.evaluate_arguments(args)?;
      println!("runtime>: function {:?} called with {:?}", callee, evaluated_args);
      
      let (body, params) = match function {
        Statement::FunctionDeclaration { body, params, .. } => (body, params),
        _ => return Err(String::from("expected Statement::FunctionDeclaration as function"))
      };
      
      let mut local_scope = Environment::new();
      for (key, value) in &self.environment.functions {
        local_scope.functions.insert(key.clone(), value.clone());
      }
      self.bind_params(params, &evaluated_args, &mut local_scope)?;
      
      let result = self.execute_function_body(local_scope, body)?;
      println!("runtime>: function {:?} returned {:?}", callee, result);
      Ok(result)
    }

    fn get_function(&self, callee: &String) -> Result<&Statement, String> {
      self.environment.functions
        .get(callee)
        .ok_or(format!("function {:?} is not defined (hoisting is not supported)", callee))
    }

    fn evaluate_arguments(&self, args: &Vec<Expression>) -> Result<Vec<RuntimeValue>, String> {
      args.iter()
        .map(|arg| self.evalutate_expression(arg))
        .collect()
    }

    fn bind_params(&self, params: &Vec<Expression>, values: &Vec<RuntimeValue>, environment: &mut Environment) -> Result<(), String> {
      for (i, param) in params.iter().enumerate() {
        let param_name = match param {
            Expression::Identifier(param) => String::from(param),
            _ => return Err(String::from("param must be Identifier")),
        };

        let value = values.get(i).cloned().unwrap_or(RuntimeValue::Undefined);
        environment.variables.insert(param_name, value);
      }

      Ok(())
    } 

    fn execute_function_body(&self, local_scope: Environment, body: &Vec<Statement>) -> Result<RuntimeValue, String> {
      let mut runtime = Runtime { environment: local_scope };

      for statement in body.iter() {
        if let Statement::Return { expression } = statement {
            return runtime.evalutate_expression(expression);
        }
        
        if let Err(error) = runtime.evaluate_statement(statement.clone()) {
          return Err(error);
        }
      }
    
      Ok(RuntimeValue::Undefined)
    }

    fn compare_numbers(&self, a: f64, b: f64, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
            TokenType::NotEqual | TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
            TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
            TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
            TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
            TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err(format!("invalid operator for numbers: {:?}", operator)),
        }
    }

    fn compare_booleans(&self, a: bool, b: bool, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
            TokenType::NotEqual | TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
            TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
            TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
            TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
            TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err(format!("invalid operator for booleans: {:?}", operator)),
        }
    }

    fn compare_strings(&self, a: &str, b: &str, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictEqual => Ok(RuntimeValue::Boolean(a == b)),
            TokenType::NotEqual | TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(a != b)),
            TokenType::GreaterThan => Ok(RuntimeValue::Boolean(a > b)),
            TokenType::GreaterThanOrEqual => Ok(RuntimeValue::Boolean(a >= b)),
            TokenType::LessThan => Ok(RuntimeValue::Boolean(a < b)),
            TokenType::LessThanOrEqual => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err(format!("invalid operator for strings: {:?}", operator)),
        }
    }

    fn compare_undefinds(&self, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictEqual => Ok(RuntimeValue::Boolean(true)),
            TokenType::NotEqual | TokenType::StrictNotEqual | TokenType::GreaterThan => {
                Ok(RuntimeValue::Boolean(false))
            }
            TokenType::GreaterThanOrEqual | TokenType::LessThan | TokenType::LessThanOrEqual => {
                Ok(RuntimeValue::Boolean(false))
            }
            _ => Err(format!("invalid operator for undefinds: {:?}", operator)),
        }
    }

    fn compare_nulls(&self, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(true)),
            TokenType::NotEqual | TokenType::StrictEqual | TokenType::GreaterThan => {
                Ok(RuntimeValue::Boolean(false))
            }
            TokenType::GreaterThanOrEqual | TokenType::LessThan | TokenType::LessThanOrEqual => {
                Ok(RuntimeValue::Boolean(false))
            }
            _ => Err(format!("invalid operator for nulls: {:?}", operator)),
        }
    }

    fn compare_null_undefined(&self, operator: &TokenType) -> Result<RuntimeValue, String> {
        match operator {
            TokenType::Equal | TokenType::StrictNotEqual => Ok(RuntimeValue::Boolean(true)),
            TokenType::NotEqual | TokenType::StrictEqual | TokenType::GreaterThan => {
                Ok(RuntimeValue::Boolean(false))
            }
            TokenType::GreaterThanOrEqual | TokenType::LessThan | TokenType::LessThanOrEqual => {
                Ok(RuntimeValue::Boolean(false))
            }
            _ => Err(format!(
                "invalid operator for null and undefined: {:?}",
                operator
            )),
        }
    }
}