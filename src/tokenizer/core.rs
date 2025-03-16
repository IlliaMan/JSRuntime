use crate::common::*;

pub struct Tokenizer {
    source: Vec<char>,
    position: usize,
}

enum CommentType {
    Line,
    Block,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        println!("--- Source Provided ---\n{}", source);
        println!("-----------------------\n");
        Self {
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
                    }
                    CommentType::Block => {
                        if let Err(error_message) = self.skip_block_comment() {
                            panic!("{}", error_message);
                        }
                        continue;
                    }
                }
            }

            if let Some(token) = self.consume_if_comparison_operator() {
                tokens.push(token);
            }

            if self.is_end() {
                break;
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

                    if c == '"' || c == '\'' || c == '`' {
                        tokens.push(self.consume_string());
                        continue;
                    }
                }
                kind => tokens.push(Token::new(
                    kind,
                    self.position, /* TODO: should be line */
                )),
            }

            self.increment_position();
        }

        tokens.push(Token::new(
            TokenType::Eof,
            self.position, /* TODO: should be line */
        ));
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
            self.increment_position();
        }

        let token_type = TokenType::from(&identifier.chars().collect::<Vec<char>>()[..]);

        Token::new(token_type, self.position /* TODO: should be line */)
    }

    fn consume_string(&mut self) -> Token {
        let mut string = String::new();
        let quote = self.peek();
        self.increment_position();

        while !self.is_end() && self.peek() != quote {
            string.push(self.peek());
            self.increment_position();
        }

        if self.is_end() {
            panic!("string {} has no closing quote {}", string, quote);
        }

        let token_type = TokenType::Literal(Literal::String(String::from(string)));
        self.increment_position();

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

        let num_str: String = self.source[start..self.position].iter().collect();

        let token_type;
        if num_str.ends_with('.') {
            token_type = TokenType::Literal(Literal::Number(
                num_str[..num_str.len() - 1]
                    .parse()
                    .expect("number ending with . should be parsed"),
            ));
        } else {
            token_type = TokenType::Literal(Literal::Number(
                num_str
                    .parse()
                    .expect("consume_number: can't parse a number"),
            ));
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

    fn consume_if_comparison_operator(&mut self) -> Option<Token> {
        let c = self.peek();
        let c_next = match self.peek_next() {
            Some(c) => c,
            None => return None,
        };

        let token_type = match (c, c_next) {
            ('>', '=') => {
                self.increment_position();
                self.increment_position();
                Some(TokenType::GreaterThanOrEqual)
            }
            ('<', '=') => {
                self.increment_position();
                self.increment_position();
                Some(TokenType::LessThanOrEqual)
            }
            ('<', _) => {
                self.increment_position();
                Some(TokenType::LessThan)
            }
            ('>', _) => {
                self.increment_position();
                Some(TokenType::GreaterThan)
            }
            ('!', '=') => {
                self.increment_position();
                match self.peek_next() {
                    Some('=') => {
                        self.increment_position();
                        self.increment_position();
                        Some(TokenType::StrictNotEqual)
                    }
                    _ => {
                        self.increment_position();
                        Some(TokenType::NotEqual)
                    }
                }
            }
            ('=', '=') => {
                self.increment_position();
                match self.peek_next() {
                    Some('=') => {
                        self.increment_position();
                        self.increment_position();
                        Some(TokenType::StrictEqual)
                    }
                    _ => {
                        self.increment_position();
                        Some(TokenType::Equal)
                    }
                }
            }
            _ => None,
        };

        match token_type {
            Some(token_type) => Some(Token::new(
                token_type,
                self.position, /* TODO: should be line */
            )),
            None => None,
        }
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
                }
                ('*', Some('/')) => {
                    self.increment_position();
                    self.increment_position();
                    depth -= 1;
                }
                _ => self.increment_position(),
            }
        }

        if depth != 0 {
            return Err("unterminated block comment".into());
        }

        Ok(())
    }
}
