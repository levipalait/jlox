use anyhow::Result;

use crate::token::Token;
use crate::token_type::TokenType;
use crate::ScanError;

pub struct Scanner {
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
    pub fn new(source: String) -> Self {
        Self {
            source: source,
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
    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        let mut had_error: bool = false;

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    had_error = true; // It continues till the end and then ends the program with an error
                }
            };
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

    /// Function that scans one Token at a time and spits it out to be collected
    fn scan_token(&mut self) -> Result<()> {
        // Get the character that is in advance and return an Error if it fails
        let c = self.advance()?;

        match c {
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
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_advance('=')? {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            // / (slash)
            '/' => {
                if self.match_advance('/')? {
                    while !self.is_at_end() && self.peek()? != '\n' {
                        self.advance()?;
                    }
                    // If the last char was \n, the program does not advance.
                    // So, we have to call it one more time.
                    self.advance()?;
                } else {
                    self.add_token(TokenType::Slash);
                }
            },

            // Useless characters
            ' ' | '\r' | '\t' => {}, // do nothing, just advance forward
            '\n' => self.line += 1,


            _ => return Err(ScanError::UnexpectedCharacter { line: self.line }.into()),
        }
        Ok(())
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
            .ok_or(ScanError::CharacterAccessError {
                current: self.current,
                line: self.line,
            }.into())
    }

    /// Adds a token without any literal
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme_text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            String::from(lexeme_text),
            None,
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
