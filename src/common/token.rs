use std::fmt;
use super::token_type::TokenType;

pub struct Token {
    pub kind: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenType, line: usize) -> Self {
        Self { kind, line }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token {{ kind: {:?}, line: {:?}}}", self.kind, self.line)
    }
}
