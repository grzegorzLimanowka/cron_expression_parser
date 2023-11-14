use std::fmt::Display;

use super::parser::ParseError;

/// Single Token found in cron value
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Value(usize), // 0-9
    Comma,        // ,
    Dash,         // -
    Asterisk,     // *
    Slash,        // /
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    // compare 2 tokens and decide whether next token is valid against self
    pub fn check(&self, next: &Token) -> bool {
        match (&self, next) {
            (Token::Value(_), Token::Comma | Token::Dash | Token::Slash) => true,
            (Token::Value(_), Token::Value(_) | Token::Asterisk) => false,
            (Token::Comma, Token::Value(_)) => true,
            (Token::Comma, _) => false,
            (Token::Dash, Token::Value(_)) => true,
            (Token::Dash, _) => false,
            (Token::Asterisk, Token::Slash) => true,
            (Token::Asterisk, _) => false,
            (Token::Slash, Token::Value(_)) => true,
            (Token::Slash, _) => false,
        }
    }
}

impl TryFrom<char> for Token {
    type Error = ParseError;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            ',' => Ok(Token::Comma),
            '-' => Ok(Token::Dash),
            '*' => Ok(Token::Asterisk),
            '/' => Ok(Token::Slash),
            '0'..='9' => Ok(Token::Value(v.to_digit(10).unwrap() as usize)),
            _ => Err(ParseError::InvalidToken(v)),
        }
    }
}
