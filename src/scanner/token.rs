use std::fmt;

pub struct Token {
    pub kind: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenType, line: usize) -> Self {
        Self {
            kind,
            line,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Token {{ kind: {:?}, line: {:?}}}", self.kind, self.line)
    }
}


#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen, // (
    RightParen, // )
    Plus,
    Minus,
    Star,
    Slash,
    Equals,

    // Literals
    Number(f64),
    
    // Binding Keywords
    KeywordLet,
    KeywordConst,

    Identifier(String),
    Unsupported(String),
    Semicolon,
    Eof,
}

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Star,
            '/' => Self::Slash,
            '=' => Self::Equals,
            ';' => Self::Semicolon,
            _ => Self::Unsupported(String::from(value))
        }
    }
}

// TODO: won't work with identifiers and numbers
// identifiers, numbers and keywords should be factored out into separate structs
// to implement From for each of them
impl From<&[char]> for TokenType {
    fn from(value: &[char]) -> Self {
        let value: String = value.iter().collect();
        match value.as_str() {
            "let" => Self::KeywordLet,
            "const" => Self::KeywordConst,
            _ => Self::Identifier(String::from(value))
        }
    }
}