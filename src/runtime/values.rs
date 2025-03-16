use crate::common::Literal;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
}

impl From<Literal> for RuntimeValue {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Boolean(value) => RuntimeValue::Boolean(value),
            Literal::Null => RuntimeValue::Null,
            Literal::Number(value) => RuntimeValue::Number(value),
            Literal::String(value) => RuntimeValue::String(value),
            Literal::Undefined => RuntimeValue::Undefined,
        }
    }
}