use crate::scanner::{token::TokenType, Token};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
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
            _ => {
                return Err(format!(
                    "line {}: declaration expects let or const instead of {:?}",
                    token.line, token.kind
                ))
            }
        };

        let name = self.identifier()?;
        let mut value = None;

        let token = self.consume_token();
        match token.kind {
            TokenType::Assign => {
                value = Some(self.comparison()?);

                self.consume_token_type(TokenType::Semicolon, "expected ';' after declaration")?;
            }
            TokenType::Semicolon => (),
            _ => {
                return Err(format!(
                    "line {}: declaration expects '=' or ';' instead of {:?}",
                    token.line, token.kind
                ))
            }
        };

        Ok(Statement::Declaration {
            is_const,
            name: Box::new(name),
            value: Box::new(value),
        })
    }

    // EXPRESSION_STATEMENT -> EXPRESSION TokenTyp::Semicolon
    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.comparison()?;
        self.consume_token_type(
            TokenType::Semicolon,
            "expected ';' after expression statement",
        )?;
        Ok(Statement::ExpressionStatement {
            expression: Box::new(expr),
        })
    }

    // COMPARISON -> EXPRESSION (COMPARISON_OPERATOR EXPRESSION)*
    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.expression()?;

        while self.peek().kind.is_comparison_operator() {
            let operator_token = self.consume_token();
            // workaround: cannot borrow `*self` as mutable more than once
            let operator_token: Token =
                Token::new(operator_token.kind.clone(), operator_token.line);

            let right_operand = self.expression()?;
            expr = Expression::Comparison {
                left: Box::new(expr),
                operator: operator_token.kind,
                right: Box::new(right_operand),
            };
        }

        Ok(expr)
    }

    // EXPRESSION -> TERM ((TokenType::Plus | TokenType::Minus) TERM)*
    fn expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.term()?;

        while matches!(self.peek().kind, TokenType::Plus | TokenType::Minus) {
            let operator_token = self.consume_token();
            // workaround: cannot borrow `*self` as mutable more than once
            let operator_token: Token =
                Token::new(operator_token.kind.clone(), operator_token.line);

            let right_operand = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: operator_token.kind,
                right: Box::new(right_operand),
            };
        }

        Ok(expr)
    }

    // TERM -> FACTOR ((TokenType::Star | TokenType::Division) FACTOR)*
    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;

        while matches!(self.peek().kind, TokenType::Star | TokenType::Slash) {
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
            TokenType::Number(_)
            | TokenType::String(_)
            | TokenType::Boolean(_)
            | TokenType::Null
            | TokenType::Undefined => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(),
            TokenType::Identifier(_) => self.identifier(),
            _ => Err(format!(
                "line {}: Expected factor (number, '(', unary -) but got {:?}",
                token.line, token.kind
            )),
        }
    }

    // LITERAL -> TokenType::Number
    fn literal(&mut self) -> Result<Expression, String> {
        let token = self.consume_token();

        match &token.kind {
            TokenType::Number(value) => Ok(Expression::Number(*value)),
            TokenType::String(value) => Ok(Expression::String(String::from(value))),
            TokenType::Boolean(value) => Ok(Expression::Boolean(*value)),
            TokenType::Null => Ok(Expression::Null),
            TokenType::Undefined => Ok(Expression::Undefined),
            _ => Err(format!(
                "line {}: Expected number literal but got {:?}",
                token.line, token.kind
            )),
        }
    }

    // GROUPING -> TokenType::LeftParen EXPRESSION Token::RightParen
    fn grouping(&mut self) -> Result<Expression, String> {
        self.consume_token_type(TokenType::LeftParen, "expected '(' to start grouping")?;
        let expr = self.expression()?;
        self.consume_token_type(TokenType::RightParen, "expected ')' to close grouping")?;
        Ok(Expression::Grouping {
            expression: Box::new(expr),
        })
    }

    // UNARY -> TokenType::Minus FACTOR
    fn unary(&mut self) -> Result<Expression, String> {
        let operator_token = self.consume_token();
        // workaround: cannot borrow `*self` as mutable more than once
        let operator_token = Token::new(operator_token.kind.clone(), operator_token.line);
        let factor = self.factor()?;

        match operator_token.kind {
            TokenType::Minus => Ok(Expression::Unary {
                operator: TokenType::Minus,
                right: Box::new(factor),
            }),
            _ => Err(format!(
                "line {}: Invalid unary operator {:?}",
                operator_token.line, operator_token.kind
            )),
        }
    }

    // IDENTIFIER -> TokenType::Identifier
    fn identifier(&mut self) -> Result<Expression, String> {
        let token = self.consume_token();
        match token.kind {
            TokenType::Identifier(ref name) => Ok(Expression::Identifier(String::from(name))),
            _ => Err(format!(
                "line {}: identifier expected instead of {:?}",
                token.line, token.kind
            )),
        }
    }

    fn consume_token_type(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, String> {
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

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExpressionStatement {
        expression: Box<Expression>,
    },
    Declaration {
        is_const: bool,
        // TODO: Need a way to have sort of Expression::Identifier as type here
        name: Box<Expression>,
        value: Box<Option<Expression>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    Identifier(String),
    Grouping {
        expression: Box<Expression>,
    },
    Comparison {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
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