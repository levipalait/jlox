// External dependencies
use anyhow::Result;

// Internal dependencies
use crate::literal::Literal;
use crate::token::{Token, TokenType};
use crate::errors::ScanError;

/// Only public function of the scanner module. It takes in a raw source code String
/// and spits out a Vector of freshly baked Tokens. It is the *blackbox interface* of the
/// scanner module.
pub fn scan_tokens(source: String) -> Result<Vec<Token>> {
    let scanner = Scanner::new(source);
    scanner.scan_tokens() // No propagation needed because it returns a Result
}

/// Contraption that holds the necessary data for the scanning process.
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // First char of lexeme being scanned
    current: usize, // Current considered char
    line: usize,    // What line 'current' is on
}

impl Scanner {
    /// Creates a new Scanner by passing in the source code as a `String`.
    /// It also sets counters to default values and initializes the tokens
    /// Vector.
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Scans every character of the source code for tokens. The while loop
    /// continues as long as the counter is not at the end of the source code.
    /// When there are any errors while a token gets scanned, the **had_error**
    /// is set to true and after scanning, the program will exit with an error.
    /// # Move occurence
    /// When `scan_tokens` is called, the scanner gets consumed and only the Vector
    /// of Tokens remains. Scanner cannot be used again (it probably doesn't need to)
    fn scan_tokens(mut self) -> Result<Vec<Token>> {
        let mut had_error: bool = false;

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                eprintln!("{}", e);
                had_error = true;
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line)); // push an EOF token

        // If there was an error while scanning, a ScanError gets returned as the Result
        if had_error {
            Err(ScanError::HadError.into())
        } else {
            Ok(self.tokens) // Return the reference to the tokens, not the cloned tokens itself
        }
    }

    /// Function that scans one Token at a time and adds it to the Token Vector of the Scanner struct
    fn scan_token(&mut self) -> Result<()> {
        // Get the character that is in advance and return an Error if it fails
        let c = self.advance()?;

        match c {
            // Useless characters
            ' ' | '\r' | '\t' => Ok(()), // do nothing, just advance forward
            '\n' => {
                self.line += 1; // Increment line counter
                Ok(())
            }

            // Single-character tokens
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // One or two character tokens
            '!' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            // Slash
            '/' => {
                if self.match_advance('/')? {
                    while !self.is_at_end() && self.peek()? != '\n' {
                        self.advance()?;
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }

            // Strings
            '"' => self.handle_string(), // Passes further handling to handle_string()

            _ => {
                if c.is_numeric() {
                    self.handle_number() // We don't want to match every digit, so we just handle this in the default case
                } else if c.is_alphabetic() {
                    self.handle_identifier() // Same here with a random alphabetic character
                } else {
                    return Err(ScanError::UnexpectedCharacter(c, self.line).into());
                }
            }
        }
    }

    /// Gets the current char and steps one ahead
    fn advance(&mut self) -> Result<char> {
        let res = self.peek();
        self.current += 1;
        res
    }

    /// Returns a bool on whether the current char matches a specific char.
    /// If so, the 'current' counter continues. If not, it stays where it is.
    fn match_advance(&mut self, expected: char) -> Result<bool> {
        if self.is_at_end() {
            return Ok(false);
        }

        if self.peek()? != expected {
            return Ok(false);
        }

        self.current += 1;
        Ok(true)
    }

    /// Gets the current char without stepping
    fn peek(&self) -> Result<char> {
        self.source
            .chars()
            .nth(self.current)
            .ok_or(ScanError::CharacterAccessError(self.line).into())
    }

    /// Gets the next char without stepping
    fn peek_next(&self) -> Result<char> {
        self.source
            .chars()
            .nth(self.current + 1)
            .ok_or(ScanError::CharacterAccessError(self.line).into())
    }

    /// Adds a `Token` to the token vector without any literal
    fn add_token(&mut self, token_type: TokenType) -> Result<()> {
        let lexeme_text = self.get_lexeme_text()?;
        let token = Token::new(token_type, lexeme_text, None, self.line);
        self.tokens.push(token);
        Ok(())
    }

    /// Adds a `Token` to the token vector with a literal
    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) -> Result<()> {
        let lexeme_text = self.get_lexeme_text()?;
        let token = Token::new(token_type, lexeme_text, Some(literal), self.line);
        self.tokens.push(token);
        Ok(())
    }

    /// Gets the lexeme text from the `start` to the `current` counter
    fn get_lexeme_text(&self) -> Result<String> {
        let text = self
            .source
            .get(self.start..self.current)
            .ok_or(ScanError::CharacterAccessError(self.line))?;
        Ok(text.to_string())
    }

    /// Checks if the `current` pointer is at the end or above of the source String
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Checks if the current pointer could advance one and then peek
    fn can_peek_next(&self) -> bool {
        self.current < (self.source.len() - 1)
    }

    /// Gets called when scan_token encounters a " character, so the
    /// String can be correctly saved as a literal token.
    fn handle_string(&mut self) -> Result<()> {
        while !self.is_at_end() && self.peek()? != '"' {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(ScanError::UnterminatedString(self.line).into());
        }

        self.advance()?; // The closing "

        let value = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .ok_or(ScanError::CharacterAccessError(self.line))?
            .to_string(); // Text between ""
        self.add_token_literal(TokenType::String, Literal::String(value))
    }

    /// Gets called when scan_tokens encounters a digit character, so the
    /// Number that the characters represent can be parsed and correctly
    /// saved as a literal token.
    fn handle_number(&mut self) -> Result<()> {
        while !self.is_at_end() && self.peek()?.is_numeric() {
            self.advance()?;
        }

        if !self.is_at_end()
            && self.peek()? == '.'
            && self.can_peek_next()
            && self.peek_next()?.is_numeric()
        {
            self.advance()?; // Consume the .
            while !self.is_at_end() && self.peek()?.is_numeric() {
                self.advance()?;
            }
        }

        let lexeme = self.get_lexeme_text()?;
        let value = lexeme.parse::<f64>()?;

        self.add_token_literal(TokenType::Number, Literal::Number(value))
    }

    fn handle_identifier(&mut self) -> Result<()> {
        while !self.is_at_end() && self.peek()?.is_alphanumeric() {
            self.advance()?;
        }

        let text = self.get_lexeme_text()?;

        // First matches if the lexeme is a keyword, then if it's a literal keyword.
        // If it's neither, it's just an identifier.
        match match_keyword(&text) {
            Some(token_type) => match token_type {
                TokenType::True => self.add_token_literal(token_type, Literal::True),
                TokenType::False => self.add_token_literal(token_type, Literal::False),
                TokenType::Nil => self.add_token_literal(token_type, Literal::Nil),
                _ => self.add_token(token_type),
            },
            None => self.add_token(TokenType::Identifier),
        }
    }
}

/// Matches a keyword to a TokenType. If the keyword is not found, it returns None.
fn match_keyword(lexeme: &str) -> Option<TokenType> {
    match lexeme {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

/// ---------- Tests for the Scanner module ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_scan() {
        let source = "print \"Hello, World!\";".to_string();
        let tokens = scan_tokens(source).expect("Token Scanning failed!");

        let cmp_token = Token::new(TokenType::Print, "print".to_string(), None, 1);
        assert_eq!(*tokens.get(0).unwrap(), cmp_token);

        let cmp_token = Token::new(
            TokenType::String,
            "\"Hello, World!\"".to_string(),
            Some(Literal::String("Hello, World!".to_string())),
            1,
        );
        assert_eq!(*tokens.get(1).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Semicolon, ";".to_string(), None, 1);
        assert_eq!(*tokens.get(2).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Eof, String::new(), None, 1);
        assert_eq!(*tokens.get(3).unwrap(), cmp_token);
    }

    #[test]
    fn keyword_scan() {
        let source = "var x = true;\r\nclass TestClass {\r\n    testMethod(s) {\r\n        print s;\r\n    }\r\n}".to_string();
        let tokens = scan_tokens(source).expect("Token Scanning failed!");

        let cmp_token = Token::new(TokenType::Var, "var".to_string(), None, 1);
        assert_eq!(*tokens.get(0).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        assert_eq!(*tokens.get(1).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Equal, "=".to_string(), None, 1);
        assert_eq!(*tokens.get(2).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::True, "true".to_string(), None, 1);
        assert_eq!(*tokens.get(3).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Semicolon, ";".to_string(), None, 1);
        assert_eq!(*tokens.get(4).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Class, "class".to_string(), None, 2);
        assert_eq!(*tokens.get(5).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "TestClass".to_string(), None, 2);
        assert_eq!(*tokens.get(6).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::LeftBrace, "{".to_string(), None, 2);
        assert_eq!(*tokens.get(7).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "testMethod".to_string(), None, 3);
        assert_eq!(*tokens.get(8).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::LeftParen, "(".to_string(), None, 3);
        assert_eq!(*tokens.get(9).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "s".to_string(), None, 3);
        assert_eq!(*tokens.get(10).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::RightParen, ")".to_string(), None, 3);
        assert_eq!(*tokens.get(11).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::LeftBrace, "{".to_string(), None, 3);
        assert_eq!(*tokens.get(12).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Print, "print".to_string(), None, 4);
        assert_eq!(*tokens.get(13).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "s".to_string(), None, 4);
        assert_eq!(*tokens.get(14).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Semicolon, ";".to_string(), None, 4);
        assert_eq!(*tokens.get(15).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::RightBrace, "}".to_string(), None, 5);
        assert_eq!(*tokens.get(16).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::RightBrace, "}".to_string(), None, 6);
        assert_eq!(*tokens.get(17).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Eof, String::new(), None, 6);
        assert_eq!(*tokens.get(18).unwrap(), cmp_token);
    }

    #[test]
    fn number_scan() {
        let source = "123 45.67".to_string();
        let tokens = scan_tokens(source).expect("Token Scanning failed!");

        let cmp_token = Token::new(
            TokenType::Number,
            "123".to_string(),
            Some(Literal::Number(123.0)),
            1,
        );
        assert_eq!(*tokens.get(0).unwrap(), cmp_token);

        let cmp_token = Token::new(
            TokenType::Number,
            "45.67".to_string(),
            Some(Literal::Number(45.67)),
            1,
        );
        assert_eq!(*tokens.get(1).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Eof, String::new(), None, 1);
        assert_eq!(*tokens.get(2).unwrap(), cmp_token);
    }

    #[test]
    fn string_scan() {
        let source = "\"Hello, World!\"".to_string();
        let tokens = scan_tokens(source).expect("Token Scanning failed!");

        let cmp_token = Token::new(
            TokenType::String,
            "\"Hello, World!\"".to_string(),
            Some(Literal::String("Hello, World!".to_string())),
            1,
        );
        assert_eq!(*tokens.get(0).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Eof, String::new(), None, 1);
        assert_eq!(*tokens.get(1).unwrap(), cmp_token);
    }

    #[test]
    fn comment_scan() {
        let source = "// This is a comment\nvar x = 42;".to_string();
        let tokens = scan_tokens(source).expect("Token Scanning failed!");

        let cmp_token = Token::new(TokenType::Var, "var".to_string(), None, 2);
        assert_eq!(*tokens.get(0).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Identifier, "x".to_string(), None, 2);
        assert_eq!(*tokens.get(1).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Equal, "=".to_string(), None, 2);
        assert_eq!(*tokens.get(2).unwrap(), cmp_token);

        let cmp_token = Token::new(
            TokenType::Number,
            "42".to_string(),
            Some(Literal::Number(42.0)),
            2,
        );
        assert_eq!(*tokens.get(3).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Semicolon, ";".to_string(), None, 2);
        assert_eq!(*tokens.get(4).unwrap(), cmp_token);

        let cmp_token = Token::new(TokenType::Eof, String::new(), None, 2);
        assert_eq!(*tokens.get(5).unwrap(), cmp_token);
    }
}
