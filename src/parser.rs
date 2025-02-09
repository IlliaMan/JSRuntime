use crate::scanner::{token::TokenType, Token};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        self.program()
    }

    // PROGRAM -> STATEMENT* TokenType::Eof
    fn program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        while !self.is_end() && self.peek().kind != TokenType::Eof {
            statements.push(self.statement()?);
        }

        if self.is_end() {
            self.consume_token_type(TokenType::Eof, "expected end of file")?;
        }

        Ok(statements)
    }

    // STATEMENT -> DECLARATION | EXPRESSION_STATEMENT
    fn statement(&mut self) -> Result<Statement, String> {
        let token = self.peek();

        match token.kind {
            TokenType::KeywordLet | TokenType::KeywordConst => self.declaration(),
            _ => self.expression_statement(),
        }
    }

    fn declaration(&mut self) -> Result<Statement, String> {
      let token = self.consume_token();
      let is_const = match token.kind {
        TokenType::KeywordLet => false,
        TokenType::KeywordConst => true,
        _ => return Err(format!("line {}: declaration expects let or const instead of {:?}", token.line, token.kind)),
      };

      let name = self.identifier()?;
      let mut value = None;

      let token = self.consume_token();
      match token.kind {
        TokenType::Equals => {
          value = Some(self.expression()?);

          self.consume_token_type(TokenType::Semicolon, "expected ';' after declaration")?;
        },
        TokenType::Semicolon => (),
        _ => return Err(format!("line {}: declaration expects '=' or ';' instead of {:?}", token.line, token.kind)),
      };

      Ok(Statement::Declaration { 
        is_const,
        name,
        value: Box::new(value),
      })
    }

    // EXPRESSION_STATEMENT -> EXPRESSION TokenTyp::Semicolon
    fn expression_statement(&mut self) -> Result<Statement, String> {
      let expr = self.expression()?;
      self.consume_token_type(TokenType::Semicolon, "expected ';' after expression statement")?;
      Ok(Statement::ExpressionStatement { expression: Box::new(expr) })
    }
    
    // EXPRESSION -> FACTOR (OPERATOR FACTOR)*
    fn expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;

        while self.is_binary_operator() {
            let operator_token = self.consume_token();
            // workaround: cannot borrow `*self` as mutable more than once
            let operator_token = Token::new(operator_token.kind.clone(), operator_token.line);

            let right_operand = self.factor()?;
            expr = Expression::Binary { 
                left: Box::new(expr),
                operator: operator_token.kind,
                right: Box::new(right_operand),
            };
        }

        Ok(expr)
    }

    // FACTOR -> LITERAL | UNARY | GROUPING
    fn factor(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        match token.kind {
            TokenType::Number(_) => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(),
            _ => Err(format!("line {}: Expected factor (number, '(', unary -) but got {:?}", token.line, token.kind))
        }
    }

    // LITERAL -> TokenType::Number
    fn literal(&mut self) -> Result<Expression, String> {
        let token = self.consume_token();

        match token.kind {
            TokenType::Number(value) => Ok(Expression::Number(value)),
            _ => Err(format!("line {}: Expected number literal but got {:?}", token.line, token.kind)),
        }
    }
    
    // GROUPING -> TokenType::LeftParen EXPRESSION Token::RightParen
    fn grouping(&mut self) -> Result<Expression, String> {
        self.consume_token_type(TokenType::LeftParen, "expected '(' to start grouping")?;
        let expr = self.expression()?;
        self.consume_token_type(TokenType::RightParen, "expected ')' to close grouping")?;
        Ok(Expression::Grouping { expression: Box::new(expr) })
    }

    // UNARY -> TokenType::Minus FACTOR
    fn unary(&mut self) -> Result<Expression, String> {
        let operator_token = self.consume_token();
        // workaround: cannot borrow `*self` as mutable more than once
        let operator_token = Token::new(operator_token.kind.clone(), operator_token.line);
        let factor = self.factor()?;

        match operator_token.kind {
            TokenType::Minus => Ok(Expression::Unary { operator: TokenType::Minus, right: Box::new(factor) }),
            _ => Err(format!("line {}: Invalid unary operator {:?}", operator_token.line, operator_token.kind)),
        }
    }
    
    // OPERATOR -> TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash
    fn is_binary_operator(&self) -> bool {
        match (*self.peek()).kind {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => true,
            _ => false,
        }
    }

    // IDENTIFIER -> TokenType::Identifier
    fn identifier(&mut self) -> Result<Identifier, String> {
      let token = self.consume_token();
      match token.kind {
        TokenType::Identifier(ref name) => Ok(Identifier(String::from(name))),
        _ => Err(format!("line {}: identifier expected instead of {:?}", token.line, token.kind)),
      }
    }

    fn consume_token_type(&mut self, token_type: TokenType, error_message: &str) -> Result<&Token, String> {
        if self.is_end() || self.peek().kind != token_type {
            return Err(String::from(error_message));
        }

        self.position += 1;
        Ok(self.previous())
    }

    fn consume_token(&mut self) -> &Token {
        if !self.is_end() {
            self.position += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &Token {
        if self.is_end() {
            // TokenType::Eof is always present
            return &self.tokens[self.tokens.len() - 1];
        }

        &self.tokens[self.position]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.position - 1]
    }

    fn is_end(&self) -> bool {
       self.position >= self.tokens.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn get_value(&self) -> String {
        String::from(&self.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
  ExpressionStatement {
    expression: Box<Expression>,
  },
  Declaration {
    is_const: bool,
    name: Identifier,
    value: Box<Option<Expression>>,
  }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: TokenType,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::token::TokenType;
    use crate::scanner::Token;

    #[test]
    fn parse_single_number() {
        let tokens = vec![
            Token::new(TokenType::Number(5.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Eof, 1),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
          result.unwrap(),
          vec![
            Statement::ExpressionStatement {
              expression: Box::new(Expression::Number(5.0))
            }
          ]
        );
    }

    #[test]
    fn parse_empty_program() {
        let tokens = vec![Token::new(TokenType::Eof, 1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[test]
    fn parse_binary_expression() {
        let tokens = vec![
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::Number(8.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Eof, 1),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![Statement::ExpressionStatement {
                expression: Box::new(Expression::Binary {
                    left: Box::new(Expression::Number(1.0)),
                    operator: TokenType::Slash,
                    right: Box::new(Expression::Number(8.0)),
                }) 
            }]
        );
    }

    #[test]
    fn parse_grouping() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(8.0), 1),
            Token::new(TokenType::RightParen, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Number(3.0), 1),
            Token::new(TokenType::Semicolon, 1),
            Token::new(TokenType::Eof, 1),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
              Statement::ExpressionStatement {
                expression: Box::new(Expression::Binary { 
                    left: Box::new(Expression::Grouping {
                        expression: Box::new(Expression::Binary {
                            left: Box::new(Expression::Number(1.0)),
                            operator: TokenType::Plus,
                            right: Box::new(Expression::Number(8.0)),
                        }),
                    }),
                    operator: TokenType::Star,
                    right: Box::new(Expression::Number(3.0))
                })
              }
            ]
        );
    }

    #[test]
    fn test_valid_declaration_with_initializer() {
        let tokens = vec![
          Token::new(TokenType::KeywordLet, 1),
          Token::new(TokenType::Identifier("x".into()), 1),
          Token::new(TokenType::Equals, 1),
          Token::new(TokenType::Number(5.0), 1),
          Token::new(TokenType::Semicolon, 1),
          Token::new(TokenType::Eof, 1),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![Statement::Declaration {
                is_const: false,
                name: Identifier("x".into()),
                value: Box::new(Some(Expression::Number(5.0)))
            }]
        );
    }

    #[test]
    fn test_valid_declaration_without_initializer() {
        let tokens = vec![
          Token::new(TokenType::KeywordConst, 1),
          Token::new(TokenType::Identifier("x".into()), 1),
          Token::new(TokenType::Semicolon, 1),
          Token::new(TokenType::Eof, 1),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![Statement::Declaration {
                is_const: true,
                name: Identifier("x".into()),
                value: Box::new(None)
            }]
        );
    }

}