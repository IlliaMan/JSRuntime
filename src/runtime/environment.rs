use std::collections::{HashMap, HashSet};
use super::values::RuntimeValue;
use crate::common::ast::Statement;

pub struct Environment {
    pub variables: HashMap<String, RuntimeValue>,
    pub constants: HashSet<String>,
    // TODO: move out Statement::FunctionDeclaration from enum for less ambiguity
    pub functions: HashMap<String, Statement>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashSet::new(),
            functions: HashMap::new(),
        }
    }
}
