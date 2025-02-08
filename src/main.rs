use std::{env, fs, io, path::Path};
use scanner::Scanner;

mod scanner {
    use std::fmt;
    pub struct Scanner {
        source: Vec<char>,
        position: usize,
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

                let  c= self.source[self.position];
                let token_type = TokenType::from(c);
                match token_type {
                    TokenType::Unsupported(_) => {
                        if ('0'..'9').contains(&c) || '.' == c {
                            tokens.push(self.consume_number());
                            continue;
                        }

                        if ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || c == '_' {
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

        fn consume_identifier(&mut self) -> Token {
            let mut identifier = String::new();

            while !self.is_end() && (self.source[self.position].is_ascii_alphanumeric() || self.source[self.position] == '_') {
                identifier.push(self.source[self.position]);
                self.position += 1;
            }

            let token_type = TokenType::from(&identifier.chars().collect::<Vec<char>>()[..]);

            Token::new(token_type, self.position /* TODO: should be line */)
        }

        fn consume_number(&mut self) -> Token {
            let start = self.position;
            self.position += 1;

            while self.source[self.position].is_ascii_digit() {
                self.position += 1;
            }

            if self.source[self.position] == '.' {
                self.position += 1;
                while !self.is_end() && self.source[self.position].is_ascii_digit() {
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
    }

    // #[derive(Debug)]
    pub struct Token {
        kind: TokenType,
        line: usize,
    }

    impl Token {
        fn new(kind: TokenType, line: usize) -> Self {
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
    enum TokenType {
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
}

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("<path> is not provided");
    let path = Path::new(&path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("js") {
        return Err(io::Error::new(io::ErrorKind::Other,"only .js files are accepted"));
    }

    let source = fs::read_to_string(path)?;
    let mut scanner = Scanner::new(source);

    let tokens = scanner.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
