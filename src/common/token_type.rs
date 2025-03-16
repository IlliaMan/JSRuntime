use super::literal::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,  // (
    RightParen, // )
    LeftCurlyBrace, // {
    RightCurlyBrace, // }
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    Comma,

    // Comparison Operators
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Literal(Literal),

    // Binding Keywords
    KeywordLet,
    KeywordConst,

    Function,
    Return,
    Identifier(String),
    Unsupported(String),
    Semicolon,
    Eof,
}

impl TokenType {
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self,
            TokenType::Equal
                | TokenType::NotEqual
                | TokenType::StrictEqual
                | TokenType::StrictNotEqual
                | TokenType::GreaterThan
                | TokenType::GreaterThanOrEqual
                | TokenType::LessThan
                | TokenType::LessThanOrEqual
        )
    }
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
            '=' => Self::Assign,
            ';' => Self::Semicolon,
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            '{' => Self::LeftCurlyBrace,
            '}' => Self::RightCurlyBrace,
            ',' => Self::Comma,
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
            "true" => Self::Literal(Literal::Boolean(true)),
            "false" => Self::Literal(Literal::Boolean(false)),
            "null" => Self::Literal(Literal::Null),
            "undefined" => Self::Literal(Literal::Undefined),
            "function" => Self::Function,
            "return" => Self::Return,
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "===" => Self::StrictEqual,
            "!==" => Self::StrictNotEqual,
            ">=" => Self::GreaterThanOrEqual,
            "<=" => Self::LessThanOrEqual,
            _ => Self::Identifier(String::from(value)),
        }
    }
}
