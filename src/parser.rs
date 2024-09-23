// External dependencies
use anyhow::Result;

// Internal dependencies
use crate::errors::ParseError;
use crate::obj::expression::Expression;
use crate::obj::statement::Statement;
use crate::obj::value::Value;
use crate::obj::token::Token;
use crate::obj::token_type::TokenType;

/// The only public function of the parser module that is the interface
/// between the main module (or some other higher level module) and the
/// whole parsing process. It takes in a collection of tokens and spits
/// out an Expression, that represents the AST formed by the tokens.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>> {
    let mut parser = Parser::new(tokens);
    let mut statements: Vec<Statement> = Vec::new();
    while !parser.is_at_end() {
        if let Ok(stmt) = parser.declaration() { // If the statement parsing fails, we synchronize and continue
            statements.push(stmt);
        } else {
            parser.synchronize()?;
        }
    }

    Ok(statements)
}

/// The Parser is a contraption that holds a collection of
/// Tokens, traverses through them one by one and returns an
/// AST of expressions.
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

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_token_types([TokenType::Var])? {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier, ParseError::ExpectedIdentifier)?;
        let mut initializer: Option<Expression> = None;
        if self.match_token_types([TokenType::Equal])? {
            initializer = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, ParseError::UnterminatedVarDeclaration)?;
        Ok(Statement::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.match_token_types([TokenType::Print])? {
            self.print_statement()
        } else if self.match_token_types([TokenType::LeftBrace])? {
            Ok(Statement::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, ParseError::UnterminatedPrintStatement)?;
        Ok(Statement::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, ParseError::UnterminatedExpressionStatement)?;
        Ok(Statement::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(TokenType::RightBrace)? && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, ParseError::UnterminatedBlock)?;

        Ok(statements)
    }

    // Since recursive descent is used, the next function
    // is the lowest level of precedenct and it goes up level by level.
    // each level represents a context-free grammar rule.

    // Lowest level of precedence
    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expr = self.equality()?;

        if self.match_token_types([TokenType::Equal])? {
            let value = self.assignment()?;

            if let Expression::Variable(name) = expr {
                return Ok(Expression::Assign(name, Box::new(value)));
            }

            return Err(ParseError::InvalidAssignmentTarget.into());
        }

        Ok(expr)
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

    // Highest level of precedence
    fn primary(&mut self) -> Result<Expression> {
        if self.match_token_types([TokenType::False])? {
            return Ok(Expression::Literal(Value::Bool(false))); // creating already existing literal, fuck it
        } else if self.match_token_types([TokenType::True])? {
            return Ok(Expression::Literal(Value::Bool(true)));
        } else if self.match_token_types([TokenType::Nil])? {
            return Ok(Expression::Literal(Value::Nil));
        } else if self.match_token_types([TokenType::String, TokenType::Number])? {
            return Ok(Expression::Literal(
                self.previous()?
                    .literal()
                    .ok_or(ParseError::NoLiteralOnToken(self.current))?,
            ));
        } else if self.match_token_types([TokenType::Identifier])? { // If we have an identifier, we return a variable expression
            return Ok(Expression::Variable(self.previous()?));
        } else if self.match_token_types([TokenType::LeftParen])? {
            let expr = self.expression()?; // If we encounter a '(', we start a new expression that is grouped
            self.consume(TokenType::RightParen, ParseError::UnterminatedGrouping)?; // We consume the ')'
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

    /// Advance until the next tokentype (given as parameter) and if not
    /// possible, return the passed ParseError type.
    fn consume(&mut self, token_type: TokenType, error: ParseError) -> Result<Token> {
        if self.check(token_type)? {
            self.advance()
        } else {
            Err(error.into())
        }
    }

    /// If check returns true, we advance and also return true.
    /// It takes in the constant generic `N`, which is the size of the
    /// array of `TokenType`'s.
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
        Ok(self.peek()?.token_type() == token_type)
    }
}
