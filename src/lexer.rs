use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    Fn,
    Return,
    If,
    Else,
    While,
    For,
    Print,
    Main,   // Added for the 'main' keyword
    Import, // Added for import system
    Try,    // Added for error handling
    Catch,  // Added for error handling

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent, // Modulo operator
    Equal,       // '='
    EqualEqual,  // '=='
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,    // '<='
    GreaterThanOrEqual, // '>='
    And,         // &&
    Or,          // ||
    Not,         // !
    
    // Symbols
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Comma,
    Semicolon,
    Dot,         // .

    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { 
            input, 
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            match ch {
                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                        self.column = 1;
                    } else {
                        self.column += 1;
                    }
                    self.consume_char();
                }

                '/' if self.peek_char_at(1) == Some('/') => {
                    // Line comment
                    while let Some(c) = self.peek_char() {
                        if c == '\n' {
                            break;
                        }
                        self.consume_char();
                    }
                }

                '/' if self.peek_char_at(1) == Some('*') => {
                    // Block comment
                    self.consume_char(); // consume /
                    self.consume_char(); // consume *
                    
                    let mut nesting = 1;
                    while nesting > 0 {
                        if self.peek_char() == Some('*') && self.peek_char_at(1) == Some('/') {
                            self.consume_char(); // consume *
                            self.consume_char(); // consume /
                            nesting -= 1;
                        } else if self.peek_char() == Some('/') && self.peek_char_at(1) == Some('*') {
                            self.consume_char(); // consume /
                            self.consume_char(); // consume *
                            nesting += 1;
                        } else if self.peek_char() == None {
                            return Err("Unterminated block comment".into());
                        } else {
                            if self.peek_char() == Some('\n') {
                                self.line += 1;
                                self.column = 1;
                            } else {
                                self.column += 1;
                            }
                            self.consume_char();
                        }
                    }
                }

                '+' => { tokens.push(Token::Plus); self.consume_char(); self.column += 1; }
                '-' => { tokens.push(Token::Minus); self.consume_char(); self.column += 1; }
                '*' => { tokens.push(Token::Star); self.consume_char(); self.column += 1; }
                '/' => { tokens.push(Token::Slash); self.consume_char(); self.column += 1; }
                '%' => { tokens.push(Token::Percent); self.consume_char(); self.column += 1; }
                '.' => { tokens.push(Token::Dot); self.consume_char(); self.column += 1; }

                '=' => {
                    if self.peek_char_at(1) == Some('=') {
                        tokens.push(Token::EqualEqual);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        tokens.push(Token::Equal);
                        self.consume_char();
                        self.column += 1;
                    }
                }

                '!' => {
                    if self.peek_char_at(1) == Some('=') {
                        tokens.push(Token::NotEqual);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        tokens.push(Token::Not);
                        self.consume_char();
                        self.column += 1;
                    }
                }

                '<' => {
                    if self.peek_char_at(1) == Some('=') {
                        tokens.push(Token::LessThanOrEqual);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        tokens.push(Token::LessThan);
                        self.consume_char();
                        self.column += 1;
                    }
                }
                
                '>' => {
                    if self.peek_char_at(1) == Some('=') {
                        tokens.push(Token::GreaterThanOrEqual);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        tokens.push(Token::GreaterThan);
                        self.consume_char();
                        self.column += 1;
                    }
                }

                '&' => {
                    if self.peek_char_at(1) == Some('&') {
                        tokens.push(Token::And);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        return Err(format!("Unexpected character '&' at line {}, column {}", self.line, self.column));
                    }
                }

                '|' => {
                    if self.peek_char_at(1) == Some('|') {
                        tokens.push(Token::Or);
                        self.consume_char(); self.consume_char();
                        self.column += 2;
                    } else {
                        return Err(format!("Unexpected character '|' at line {}, column {}", self.line, self.column));
                    }
                }

                '(' => { tokens.push(Token::LParen); self.consume_char(); self.column += 1; }
                ')' => { tokens.push(Token::RParen); self.consume_char(); self.column += 1; }
                '{' => { tokens.push(Token::LBrace); self.consume_char(); self.column += 1; }
                '}' => { tokens.push(Token::RBrace); self.consume_char(); self.column += 1; }
                '[' => { tokens.push(Token::LBracket); self.consume_char(); self.column += 1; }
                ']' => { tokens.push(Token::RBracket); self.consume_char(); self.column += 1; }
                ',' => { tokens.push(Token::Comma); self.consume_char(); self.column += 1; }
                ';' => { tokens.push(Token::Semicolon); self.consume_char(); self.column += 1; }

                '"' => {
                    let token = self.read_string()?;
                    tokens.push(token);
                }

                c if c.is_ascii_digit() => {
                    let token = self.read_number();
                    tokens.push(token);
                }

                c if c.is_ascii_alphabetic() || c == '_' => {
                    let token = self.read_identifier_or_keyword();
                    tokens.push(token);
                }

                _ => {
                    return Err(format!("Unexpected character '{}' at line {}, column {}", 
                                     ch, self.line, self.column));
                }
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let start_col = self.column;
        let mut is_float = false;
        
        // Read integer part
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.consume_char();
                self.column += 1;
            } else if c == '.' && !is_float {
                // Check if followed by a digit to confirm it's a float
                if let Some(next_c) = self.peek_char_at(1) {
                    if next_c.is_ascii_digit() {
                        is_float = true;
                        self.consume_char(); // consume dot
                        self.column += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        let number_str = &self.input[start..self.pos];
        
        if is_float {
            let float_val = f64::from_str(number_str).unwrap_or(0.0);
            Token::Float(float_val)
        } else {
            let int_val = i64::from_str(number_str).unwrap_or(0);
            Token::Number(int_val)
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.consume_char(); // consume opening "
        self.column += 1;

        let start = self.pos;
        let mut string_content = String::new();
        let mut is_escaped = false;

        while let Some(c) = self.peek_char() {
            self.consume_char();
            self.column += 1;
            
            if is_escaped {
                // Handle escape sequences
                let escaped_char = match c {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    _ => return Err(format!("Invalid escape sequence '\\{}' at line {}, column {}", 
                                          c, self.line, self.column)),
                };
                string_content.push(escaped_char);
                is_escaped = false;
            } else if c == '\\' {
                is_escaped = true;
            } else if c == '"' {
                // End of string
                return Ok(Token::String(string_content));
            } else {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                string_content.push(c);
            }
        }

        Err(format!("Unterminated string literal starting at line {}", self.line))
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.pos;
        let start_col = self.column;
        
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.consume_char();
                self.column += 1;
            } else {
                break;
            }
        }

        let ident = &self.input[start..self.pos];

        match ident {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "print" => Token::Print,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "main" => Token::Main,
            "import" => Token::Import,
            "try" => Token::Try,
            "catch" => Token::Catch,
            _ => Token::Identifier(ident.to_string()),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn peek_char_at(&self, offset: usize) -> Option<char> {
        self.input[self.pos..].chars().nth(offset)
    }

    fn consume_char(&mut self) {
        if let Some(c) = self.peek_char() {
            self.pos += c.len_utf8();
        }
    }
}