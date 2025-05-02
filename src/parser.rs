use std::path::Path;
use crate::ast::{Statement, Expression, BinaryOp, UnaryOp};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    file_path: String,
    source_code: &'a str,
    file_prefix: String, // Store file prefix for function definitions
}

impl<'a> Parser<'a> {
    pub fn new(file_path: &str, source_code: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.tokenize()?;

        // Extract file prefix from path for module system
        let path = Path::new(file_path);
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main");
        
        Ok(Self {
            tokens,
            current: 0,
            file_path: file_path.to_string(),
            source_code,
            file_prefix: file_stem.to_string(),
        })
    }
    
    // Parse a complete program
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        Ok(statements)
    }

    // Used for parsing imported modules
    pub fn parse_module(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            // Only include function definitions and imports in modules
            match self.peek() {
                Token::Fn | Token::Import => {
                    statements.push(self.declaration()?);
                },
                _ => {
                    // Skip other top-level statements in modules
                    self.advance();
                }
            }
        }
        
        Ok(statements)
    }
    
    fn declaration(&mut self) -> Result<Statement, String> {
        if self.match_token(&[Token::Let]) {
            return self.let_declaration();
        } else if self.match_token(&[Token::Fn]) {
            return self.function_definition();
        } else if self.match_token(&[Token::Import]) {
            return self.import_declaration();
        }
        
        self.statement()
    }
    
    fn let_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected variable name after 'let'")?;
        
        let value = if self.match_token(&[Token::Equal]) {
            self.expression()?
        } else {
            return Err("Expected '=' after variable name in let declaration".to_string());
        };
        
        self.consume(&Token::Semicolon, "Expected ';' after variable declaration")?;
        
        Ok(Statement::Let { 
            name, 
            value 
        })
    }
    
    fn function_definition(&mut self) -> Result<Statement, String> {
        // Check if this is a main function definition
        let is_main = self.match_token(&[Token::Main]);
        
        let name = if is_main {
            String::new() // Main function has an empty name internally
        } else {
            self.consume_identifier("Expected function name after 'fn'")?
        };
        
        self.consume(&Token::LParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            // Parse first parameter
            params.push(self.consume_identifier("Expected parameter name")?);
            
            // Parse any additional parameters
            while self.match_token(&[Token::Comma]) {
                params.push(self.consume_identifier("Expected parameter name")?);
            }
        }
        
        self.consume(&Token::RParen, "Expected ')' after parameters")?;
        self.consume(&Token::LBrace, "Expected '{' before function body")?;
        
        let body = self.block()?;
        
        Ok(Statement::FunctionDef {
            file_prefix: self.file_prefix.clone(),
            is_main,
            name,
            params,
            body,
        })
    }
    
    fn import_declaration(&mut self) -> Result<Statement, String> {
        let module_name = self.consume_identifier("Expected module name after 'import'")?;
        self.consume(&Token::Semicolon, "Expected ';' after import statement")?;
        
        Ok(Statement::Import(module_name))
    }
    
    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&[Token::LBrace]) {
            return Ok(Statement::Block(self.block()?));
        } else if self.match_token(&[Token::If]) {
            return self.if_statement();
        } else if self.match_token(&[Token::While]) {
            return self.while_statement();
        } else if self.match_token(&[Token::For]) {
            return self.for_statement();
        } else if self.match_token(&[Token::Print]) {
            return self.print_statement();
        } else if self.match_token(&[Token::Return]) {
            return self.return_statement();
        } else if self.match_token(&[Token::Try]) {
            return self.try_statement();
        }
        
        self.expression_statement()
    }
    
    fn print_statement(&mut self) -> Result<Statement, String> {
        let value = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after print statement")?;
        
        Ok(Statement::Print(value))
    }
    
    fn return_statement(&mut self) -> Result<Statement, String> {
        let value = if !self.check(&Token::Semicolon) {
            self.expression()?
        } else {
            // Replace Expression::Void with a placeholder since Void doesn't exist
            Expression::Bool(false) // Placeholder for void
        };
        
        self.consume(&Token::Semicolon, "Expected ';' after return value")?;
        
        Ok(Statement::Return(value))
    }
    
    fn block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        self.consume(&Token::RBrace, "Expected '}' after block")?;
        
        Ok(statements)
    }
    
    fn if_statement(&mut self) -> Result<Statement, String> {
        self.consume(&Token::LParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after if condition")?;
        
        self.consume(&Token::LBrace, "Expected '{' before if body")?;
        let then_branch = self.block()?;
        
        let else_branch = if self.match_token(&[Token::Else]) {
            if self.match_token(&[Token::If]) {
                // Handle 'else if' as a nested if in the else branch
                let else_if_stmt = self.if_statement()?;
                Some(vec![else_if_stmt])
            } else {
                self.consume(&Token::LBrace, "Expected '{' before else body")?;
                Some(self.block()?)
            }
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Result<Statement, String> {
        self.consume(&Token::LParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after while condition")?;
        
        self.consume(&Token::LBrace, "Expected '{' before while body")?;
        let body = self.block()?;
        
        Ok(Statement::While {
            condition,
            body,
        })
    }
    
    fn for_statement(&mut self) -> Result<Statement, String> {
        self.consume(&Token::LParen, "Expected '(' after 'for'")?;
        
        // Initializer
        let init = if self.match_token(&[Token::Semicolon]) {
            None
        } else if self.match_token(&[Token::Let]) {
            Some(Box::new(self.let_declaration()?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };
        
        // Condition
        let condition = if !self.check(&Token::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&Token::Semicolon, "Expected ';' after loop condition")?;
        
        // Increment
        let update = if !self.check(&Token::RParen) {
            let expr = self.expression()?;
            Some(Box::new(Statement::Expression(expr)))
        } else {
            None
        };
        self.consume(&Token::RParen, "Expected ')' after for clauses")?;
        
        self.consume(&Token::LBrace, "Expected '{' before for body")?;
        let body = self.block()?;
        
        Ok(Statement::For {
            init,
            condition,
            update,
            body,
        })
    }
    
    fn try_statement(&mut self) -> Result<Statement, String> {
        self.consume(&Token::LBrace, "Expected '{' after 'try'")?;
        let try_body = self.block()?;
        
        self.consume(&Token::Catch, "Expected 'catch' after try block")?;
        self.consume(&Token::LBrace, "Expected '{' after 'catch'")?;
        let catch_body = self.block()?;
        
        Ok(Statement::Try {
            body: try_body,
            catch: catch_body,
        })
    }
    
    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        
        // Handle assignment expressions
        if let Expression::Binary { 
            left: box_left, 
            op: BinaryOp::Equal, 
            right 
        } = expr.clone() {
            if let Expression::Identifier(name) = *box_left {
                self.consume(&Token::Semicolon, "Expected ';' after assignment")?;
                return Ok(Statement::Assign { name, value: *right });
            }
        }
        
        self.consume(&Token::Semicolon, "Expected ';' after expression")?;
        Ok(Statement::Expression(expr))
    }
    
    // Expression parsing methods
    fn expression(&mut self) -> Result<Expression, String> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.logic_or()?;
        
        if self.match_token(&[Token::Equal]) {
            if let Expression::Identifier(name) = expr {
                let value = Box::new(self.assignment()?);
                return Ok(Expression::Binary { 
                    left: Box::new(Expression::Identifier(name)), 
                    op: BinaryOp::Equal, 
                    right: value 
                });
            } else if let Expression::ArrayAccess { array, index } = expr {
                let value = Box::new(self.assignment()?);
                // Instead of using ArrayAssign, use a function call pattern
                return Ok(Expression::Call {
                    callee: Box::new(Expression::Identifier("__array_assign".to_string())),
                    arguments: vec![*array, *index, *value],
                });
            }
            
            return Err("Invalid assignment target".to_string());
        }
        
        Ok(expr)
    }
    
    fn logic_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.logic_and()?;
        
        while self.match_token(&[Token::Or]) {
            let right = self.logic_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logic_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(&[Token::And]) {
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.comparison()?;
        
        while self.match_token(&[Token::EqualEqual, Token::NotEqual]) {
            let op = match self.previous() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.term()?;
        
        while self.match_token(&[
            Token::LessThan, 
            Token::GreaterThan, 
            Token::LessThanOrEqual, 
            Token::GreaterThanOrEqual
        ]) {
            let op = match self.previous() {
                Token::LessThan => BinaryOp::LessThan,
                Token::GreaterThan => BinaryOp::GreaterThan,
                Token::LessThanOrEqual => BinaryOp::LessThanOrEqual,
                Token::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;
        
        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match self.previous() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;
        
        while self.match_token(&[Token::Star, Token::Slash, Token::Percent]) {
            let op = match self.previous() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_token(&[Token::Minus, Token::Not]) {
            let op = match self.previous() {
                Token::Minus => UnaryOp::Negate,
                Token::Not => UnaryOp::Not,
                _ => unreachable!(),
            };
            
            let right = self.unary()?;
            return Ok(Expression::Unary {
                op,
                expr: Box::new(right),
            });
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&[Token::LParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[Token::LBracket]) {
                let index = self.expression()?;
                self.consume(&Token::RBracket, "Expected ']' after array index")?;
                expr = Expression::ArrayAccess {
                    array: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut arguments = Vec::new();
        
        if !self.check(&Token::RParen) {
            // Parse the first argument
            if arguments.len() >= 255 {
                return Err("Cannot have more than 255 arguments".to_string());
            }
            arguments.push(self.expression()?);
            
            // Parse any additional arguments
            while self.match_token(&[Token::Comma]) {
                if arguments.len() >= 255 {
                    return Err("Cannot have more than 255 arguments".to_string());
                }
                arguments.push(self.expression()?);
            }
        }
        
        self.consume(&Token::RParen, "Expected ')' after arguments")?;
        
        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }
    
    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(&[Token::Number(0)]) {
            if let Token::Number(n) = self.previous() {
                return Ok(Expression::Number(*n));
            }
        }
        
        if self.match_token(&[Token::Float(0.0)]) {
            if let Token::Float(f) = self.previous() {
                return Ok(Expression::Float(*f));
            }
        }
        
        if self.match_token(&[Token::Bool(false)]) {
            if let Token::Bool(b) = self.previous() {
                return Ok(Expression::Bool(*b));
            }
        }
        
        if self.match_token(&[Token::String("".to_string())]) {
            if let Token::String(s) = self.previous() {
                return Ok(Expression::String(s.clone()));
            }
        }
        
        if self.match_token(&[Token::LBracket]) {
            return self.array_literal();
        }
        
        if self.match_token(&[Token::Identifier("".to_string())]) {
            if let Token::Identifier(name) = self.previous() {
                return Ok(Expression::Identifier(name.clone()));
            }
        }
        
        if self.match_token(&[Token::LParen]) {
            let expr = self.expression()?;
            self.consume(&Token::RParen, "Expected ')' after expression")?;
            return Ok(expr);
        }
        
        // Parse closure/lambda expressions
        if self.match_token(&[Token::Fn]) {
            self.consume(&Token::LParen, "Expected '(' after 'fn' in closure")?;
            
            let mut params = Vec::new();
            if !self.check(&Token::RParen) {
                // Parse first parameter
                params.push(self.consume_identifier("Expected parameter name")?);
                
                // Parse any additional parameters
                while self.match_token(&[Token::Comma]) {
                    params.push(self.consume_identifier("Expected parameter name")?);
                }
            }
            
            self.consume(&Token::RParen, "Expected ')' after closure parameters")?;
            self.consume(&Token::LBrace, "Expected '{' before closure body")?;
            
            let body = self.block()?;
            
            return Ok(Expression::Closure {
                params,
                body,
            });
        }
        
        Err(format!("Expected expression, got {:?}", self.peek()))
    }
    
    fn array_literal(&mut self) -> Result<Expression, String> {
        let mut elements = Vec::new();
        
        if !self.check(&Token::RBracket) {
            // Parse first element
            elements.push(self.expression()?);
            
            // Parse any additional elements
            while self.match_token(&[Token::Comma]) {
                elements.push(self.expression()?);
            }
        }
        
        self.consume(&Token::RBracket, "Expected ']' after array elements")?;
        
        Ok(Expression::Array {
            elements,
        })
    }
    
    // Helper methods for token handling
    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        
        false
    }
    
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            match (token, &self.tokens[self.current]) {
                (Token::Number(_), Token::Number(_)) => true,
                (Token::Float(_), Token::Float(_)) => true,
                (Token::Bool(_), Token::Bool(_)) => true,
                (Token::String(_), Token::String(_)) => true,
                (Token::Identifier(_), Token::Identifier(_)) => true,
                _ => std::mem::discriminant(token) == std::mem::discriminant(&self.tokens[self.current]),
            }
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current] == Token::EOF
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token: &Token, message: &str) -> Result<&Token, String> {
        if self.check(token) {
            Ok(self.advance())
        } else {
            let (line, column) = Lexer::get_position_for_error(&self.tokens, self.current, self.source_code);
            Err(format!("{} at line {}, column {}", message, line, column))
        }
    }
    
    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if let Token::Identifier(_) = self.peek() {
            if let Token::Identifier(name) = self.advance() {
                return Ok(name.clone());
            }
        }
        
        let (line, column) = Lexer::get_position_for_error(&self.tokens, self.current, self.source_code);
        Err(format!("{} at line {}, column {}", message, line, column))
    }
}

// Helper method added to Lexer for error reporting
impl<'a> Lexer<'a> {
    pub fn get_position_for_error(tokens: &[Token], current: usize, source_code: &str) -> (usize, usize) {
        if current >= tokens.len() {
            // If at the end, count lines and columns manually
            let lines: Vec<&str> = source_code.lines().collect();
            return (lines.len(), lines.last().map_or(0, |s| s.len() + 1));
        }
        
        // Return a default position if not available
        (1, 1)
    }
}