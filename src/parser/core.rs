use crate::common::ast::*;
use crate::common::{Token, TokenType, Literal};

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

    fn statement(&mut self) -> Result<Statement, String> {
        let token = self.peek();

        match token.kind {
            TokenType::KeywordLet | TokenType::KeywordConst => self.declaration(),
            TokenType::Function => self.function_declaration(),
            TokenType::Return => self.return_statement(),
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
        let name = Expression::extract_string(&name)
            .ok_or_else(|| format!("Expected declaration name to be Expression::Identifier but got {:?}", name))?;

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

        Ok(Statement::Declaration { is_const, name, value: Box::new(value) })
    }

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

    fn function_declaration(&mut self) -> Result<Statement, String> {
        let _ = self.consume_token();
        let identifier = self.identifier()?;
        let name = Expression::extract_string(&identifier)
            .ok_or_else(|| format!("Expected declaration name to be Expression::Identifier but got {:?}", identifier))?;
        
        self.consume_token_type(TokenType::LeftParen, "expected '(' after function name")?;
        let mut params = vec![];
        if self.peek().kind != TokenType::RightParen {
            params = self.function_params()?;
        }

        self.consume_token_type(TokenType::RightParen, "expected ')' after function arguments")?;
        let mut body = self.function_body()?;
        if body.len() == 0 {
            body = vec![ Statement::Return { expression: Box::new(Expression::Literal(Literal::Undefined))}];
        }
        
        Ok(Statement::FunctionDeclaration { name, params, body: Box::new(body) })
    }

    fn return_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume_token();

        if self.peek().kind == TokenType::Semicolon {
            let _ = self.consume_token();
            return Ok(Statement::Return { expression: Box::new(Expression::Literal(Literal::Undefined)) });
        }
        
        let expr = self.comparison()?;
        self.consume_token_type(TokenType::Semicolon, "expected ';' after return statement")?;
        
        Ok(Statement::Return { expression: Box::new(expr) })
    }

    fn function_params(&mut self) -> Result<Vec<String>, String> {
        let mut params = vec![];

        let param = self.identifier()?;
        let param: String = Expression::extract_string(&param)
            .ok_or_else(|| format!("Expected function parameter to be Expression::Identifier but got {:?}", param))?;

        params.push(param);
        
        while self.peek().kind == TokenType::Comma {
            self.consume_token();
            let param = self.identifier()?;
            let param: String = Expression::extract_string(&param)
                .ok_or_else(|| format!("Expected function parameter to be Expression::Identifier but got {:?}", param))?;

            params.push(param);
        }

        Ok(params)
    }

    fn function_body(&mut self) -> Result<Vec<Statement>, String> {
        self.consume_token_type(TokenType::LeftCurlyBrace, "Expected '{' to begin function body.")?;

        let mut statements = vec![];
        if self.peek().kind != TokenType::RightCurlyBrace {
            statements = self.function_body_content()?;
        }
        
        self.consume_token_type(TokenType::RightCurlyBrace, "Expected '}' to end function body.")?;

        Ok(statements)
    }

    fn function_body_content(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];
        let mut is_return_found = false;
        while self.peek().kind != TokenType::RightCurlyBrace {
            let statement = match self.peek().kind {
                TokenType::Function => return Err(format!("line {}: Functions inside functions are not yet supported", self.peek().line)),
                TokenType::Return => {
                    is_return_found = true;
                    self.return_statement()
                },
                TokenType::KeywordLet | TokenType::KeywordConst => self.declaration(),
                _ => self.expression_statement(),
            };
            statements.push(statement?);
        }

        if !is_return_found {
            statements.push(Statement::Return { expression: Box::new(Expression::Literal(Literal::Undefined)) });
        }

        Ok(statements)
    }
    
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

    fn factor(&mut self) -> Result<Expression, String> {
        let token = self.peek();

        match token.kind {
            TokenType::Literal(_) => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(),
            TokenType::Identifier(_) => {
                let mut expr = self.identifier()?;
                if self.peek().kind == TokenType::LeftParen {
                    self.go_to_previous_token();
                    expr = self.call()?;
                }

                Ok(expr)
            },
            _ => Err(format!("line {}: Expected factor (number, '(', unary -) but got {:?}", token.line, token.kind))
        }
    }

    fn call(&mut self) -> Result<Expression, String> {
        let identifier: Expression = self.identifier()?;
        let identifier = Expression::extract_string(&identifier)
            .ok_or_else(|| format!("Expected function name as Expression::Identifier but got {:?}", identifier))?;

        self.consume_token_type(TokenType::LeftParen, format!("expected '(' for {:?} function call", identifier).as_str())?;
        if self.peek().kind == TokenType::RightParen {
            self.consume_token();
            return Ok(Expression::Call { callee: identifier, args: Box::new(vec![]) });
        }

        let args = self.arguments()?;
        self.consume_token_type(TokenType::RightParen, format!("expected ')' for {:?} function call", identifier).as_str())?;
        Ok(Expression::Call { callee: identifier, args: Box::new(args) })
    } 
    
    fn arguments(&mut self) -> Result<Vec<Expression>, String> {
        let mut args = vec![];
        args.push(self.comparison()?);

        while self.peek().kind == TokenType::Comma {
            self.consume_token();
            args.push(self.comparison()?);
        }

        Ok(args)
    }

    fn literal(&mut self) -> Result<Expression, String> {
        let token = self.consume_token();

        match &token.kind {
            TokenType::Literal(literal) => Ok(Expression::Literal(literal.clone())),
            _ => Err(format!("line {}: Expected number literal but got {:?}", token.line, token.kind)),
        }
    }

    fn grouping(&mut self) -> Result<Expression, String> {
        self.consume_token_type(TokenType::LeftParen, "expected '(' to start grouping")?;
        let expr = self.expression()?;
        self.consume_token_type(TokenType::RightParen, "expected ')' to close grouping")?;
        Ok(Expression::Grouping {
            expression: Box::new(expr),
        })
    }

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

    fn go_to_previous_token(&mut self) {
        if self.position != 0 {
            self.position -= 1;
        }
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
