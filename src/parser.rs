// External dependencies
use anyhow::Result;

// Internal dependencies
use crate::errors::ParseError;
use crate::expression::Expression;
use crate::literal::Literal;
use crate::token::Token;
use crate::token::TokenType;

// WARNING: PARSER NOT DONE, DOES NOT REPORT SYNTAX ERRORS, BUT
// PROPAGATES AN ERROR WHEN THE FIRST ONE IS ENCOUNTERED!
// ONLY FOR TESTING!

pub fn parse(tokens: Vec<Token>) -> Result<Expression> {
    let mut parser = Parser::new(tokens);
    parser.expression() // No propagation needed, because parser returns a Result
}

// ----------CONTINUE HERE-----------
// Parser now works, but code is not very readable and redundant
// Apply match and check functions from the book

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
/// A recursive descent parser that parses lox tokens
/// into an AST that can then be walked.
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // Since recursive descent is used, the next function
    // is the top level and it goes down level by level.
    // each level represents a context-free grammar rule.

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?; // Expressions are unpacked on each layer (makes things easier)

        // Match those token types
        while self.match_token_types([TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while self.match_token_types([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        // Match those tokentypes
        while self.match_token_types([TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // Goddamn user input. Every function returns a Result

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        // Match those tokentypes
        while self.match_token_types([TokenType::Slash, TokenType::Star])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        // Match those tokentypes

        if self.match_token_types([TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            Ok(Expression::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.match_token_types([TokenType::False])? {
            return Ok(Expression::Literal(Literal::False));
        } else if self.match_token_types([TokenType::True])? {
            return Ok(Expression::Literal(Literal::True));
        } else if self.match_token_types([TokenType::Nil])? {
            return Ok(Expression::Literal(Literal::Nil));
        } else if self.match_token_types([TokenType::String, TokenType::Number])? {
            return Ok(Expression::Literal(
                self.previous()?
                    .literal()
                    .ok_or(ParseError::NoLiteralOnToken(self.current))?,
            ));
        } else if self.match_token_types([TokenType::LeftParen])? {
            let expr = self.expression()?; // If we encounter a '(', we start a new expression that is grouped
            self.consume(TokenType::RightParen)?; // We consume the ')'
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        // If we're at the end or don't match, we error. Otherwise, we return before this line
        Err(ParseError::ExpectedExpression(format!("{}", self.peek()?)).into())
    }

    /// When an error is encountered, it ignores any tokens until
    /// a statement is closed with a `;` or a keyword is encountered
    fn synchronize(&mut self) -> Result<()> {
        let mut token_type = self.advance()?.token_type();
        while !self.is_at_end() {
            if token_type == TokenType::Semicolon {
                return Ok(());
            }

            match token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return Ok(()),
                _ => {}
            }

            token_type = self.advance()?.token_type();
        }
        Ok(())
    }

    // Small helper functions

    /// Checks if the current pointer is already at the end
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1 // - 1 because EOF is already the end
    }

    /// Tries to get the current Token
    fn peek(&self) -> Result<Token> {
        self.tokens
            .get(self.current)
            .ok_or(ParseError::TokenAccessError(self.current).into())
            .cloned()
    }

    fn previous(&self) -> Result<Token> {
        self.tokens
            .get(self.current - 1)
            .ok_or(ParseError::TokenAccessError(self.current).into())
            .cloned()
    }

    /// Tries to get the current token and increments the current pointer by 1
    fn advance(&mut self) -> Result<Token> {
        let res = self.peek();
        if !self.is_at_end() {
            self.current += 1;
        }
        res
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        if self.check(token_type)? {
            self.advance()
        } else {
            Err(ParseError::UnterminatedGrouping.into())
        }
    }

    // If check returns true, we advance and also return true
    fn match_token_types<const N: usize>(&mut self, token_types: [TokenType; N]) -> Result<bool> {
        for token_type in token_types {
            if self.check(token_type)? {
                self.advance()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Checks if we're at the end and if not, we check if the current
    /// tokentype is the desired tokentype
    fn check(&self, token_type: TokenType) -> Result<bool> {
        if self.is_at_end() {
            return Ok(false);
        }
        return Ok(self.peek()?.token_type() == token_type);
    }
}
