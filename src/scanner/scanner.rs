use super::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    position: usize,
}

enum CommentType {
    Line,
    Block,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        println!("--- Source Provided ---\n{}", source);
        println!("-----------------------\n");
        Scanner {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        
        while !self.is_end() {
            if self.is_whitespace(self.position) {
                self.increment_position();
                continue;
            }

            if let Some(comment_type) = self.is_comment() {
                match comment_type {
                    CommentType::Line => {
                        self.skip_line_comment();
                        continue;
                    },
                    CommentType::Block => {
                        if let Err(error_message) = self.skip_block_comment() {
                            panic!("{}", error_message);
                        }
                        continue;
                    },
                }
            }

            let c = self.peek();
            let token_type: TokenType = TokenType::from(c);
            match token_type {
                TokenType::Unsupported(_) => {
                    if ('0'..='9').contains(&c) || '.' == c {
                        tokens.push(self.consume_number());
                        continue;
                    }

                    if ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_' {
                        tokens.push(self.consume_identifier());
                        continue;
                    }
                },
                kind => tokens.push(Token::new(kind, self.position /* TODO: should be line */)),
            }

            self.increment_position();
        }

        tokens.push(Token::new(TokenType::Eof, self.position /* TODO: should be line */));
        tokens
    }

    fn is_whitespace(&self, position: usize) -> bool {
        match self.source[position] {
            '\n' | ' ' | '\t' | '\r' => true,
            _ => false,
        }
    }

    fn is_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn increment_position(&mut self) {
        self.position += 1;
    }
    
    fn peek(&self) -> char {
        self.source[self.position]
    }

    fn peek_next(&self) -> Option<char> {
        if self.source.len() <= self.position + 1 {
            return None;
        }

        Some(self.source[self.position + 1])
    }

    fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while !self.is_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            identifier.push(self.peek());
            self.position += 1;
        }

        let token_type = TokenType::from(&identifier.chars().collect::<Vec<char>>()[..]);

        Token::new(token_type, self.position /* TODO: should be line */)
    }

    fn consume_number(&mut self) -> Token {
        let start = self.position;
        self.position += 1;

        while self.peek().is_ascii_digit() {
            self.position += 1;
        }

        if self.peek() == '.' {
            self.position += 1;
            while !self.is_end() && self.peek().is_ascii_digit() {
                self.position += 1;
            }
        }

        let num_str: String = self.source[start..self.position]
            .iter()
            .collect();

        let token_type ;
        if num_str.ends_with('.') {
            token_type = TokenType::Number(num_str[..num_str.len() - 1].parse().expect("number ending with . should be parsed"));
        } else {
            token_type = TokenType::Number(num_str.parse().expect("consume_number: can't parse a number"));
        }

        Token::new(token_type, self.position /* TODO: should be line */)
    }

    fn is_comment(&self) -> Option<CommentType> {
        let c = self.peek();
        let c_next = match self.peek_next() {
            Some(c) => c,
            None => return None,
        };
        if c == '/' {
            if c_next == '/' {
                return Some(CommentType::Line);
            }

            if c_next == '*' {
                return Some(CommentType::Block);
            }
        }

        None
    }

    fn skip_line_comment(&mut self) {
        while !self.is_end() && self.peek() != '\n' {
            self.increment_position();
        }

        self.increment_position();
    }

    fn skip_block_comment(&mut self) -> Result<(), String> {
        self.position += 2;

        let mut depth = 1;
        while depth > 0 && !self.is_end() {
            match (self.peek(), self.peek_next()) {
                ('/', Some('*')) => {
                    self.increment_position();
                    self.increment_position();
                    depth += 1;
                },
                ('*', Some('/')) => {
                    self.increment_position();
                    self.increment_position();
                    depth -= 1;
                },
                _ => self.increment_position(),
            }
        }

        if depth != 0 {
            return Err("unterminated block comment".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_token_types(source: &str) -> Vec<TokenType> {
        let mut scanner = Scanner::new(source.into());
        scanner.tokenize().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_general_cases() {
        assert_eq!(
            get_token_types("1 + 5 * (1 + 9);"),
            vec![
                TokenType::Number(1.0 as f64),
                TokenType::Plus,
                TokenType::Number(5.0 as f64),
                TokenType::Star,
                TokenType::LeftParen,
                TokenType::Number(1.0 as f64),
                TokenType::Plus,
                TokenType::Number(9.0 as f64),
                TokenType::RightParen,
                TokenType::Semicolon,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_single_char_tokens() {
        assert_eq!(
            get_token_types("( ) + - * / = ;"),
            vec![
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Star,
                TokenType::Slash,
                TokenType::Equals,
                TokenType::Semicolon,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_number_literals() {
        assert_eq!(
            get_token_types("123 12.3 0 0.0 .123 123."),
            vec![
                TokenType::Number(123 as f64),
                TokenType::Number(12.3 as f64),
                TokenType::Number(0 as f64),
                TokenType::Number(0.0 as f64),
                TokenType::Number(0.123 as f64),
                TokenType::Number(123 as f64),
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_identifiers_and_keywords() {
        assert_eq!(
            get_token_types("let const hello _hello x0"),
            vec![
                TokenType::KeywordLet,
                TokenType::KeywordConst,
                TokenType::Identifier("hello".into()),
                TokenType::Identifier("_hello".into()),
                TokenType::Identifier("x0".into()),
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            get_token_types("// Line comment \nlet x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Equals,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(
            get_token_types("let /* block comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Equals,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_nested_block_comment() {
        assert_eq!(
            get_token_types("let /* block /* nested */ comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Equals,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_multi_line_nested_block_comment() {
        assert_eq!(
            get_token_types("let /* block \n * \n * /* nested */ \n * \n * comment */ x = 10;"),
            vec![
                TokenType::KeywordLet,
                TokenType::Identifier("x".into()),
                TokenType::Equals,
                TokenType::Number(10.0),
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(get_token_types(""), vec![TokenType::Eof]);

        assert_eq!(get_token_types("   \t\n\r"), vec![TokenType::Eof]);
    }
}