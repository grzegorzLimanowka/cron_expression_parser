use super::parser::CronParseError;

/// Single Token found in cron value
#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Value(usize), // 4
    Comma,        // ,
    Dash,         // -
    Asterisk,     // *
    Slash,        // /
}

impl Token {
    // lists all Tokens, that are allowed after current one
    pub fn allowed(&self) -> Vec<Token> {
        match &self {
            Token::Value(_) => vec![Token::Comma, Token::Dash, Token::Slash],
            Token::Comma => vec![Token::Value(0)],
            Token::Dash => vec![Token::Value(0)],
            Token::Asterisk => vec![Token::Slash],
            Token::Slash => vec![Token::Value(0)],
        }
    }
}

impl TryFrom<char> for Token {
    type Error = CronParseError;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            ',' => Ok(Token::Comma),
            '-' => Ok(Token::Dash),
            '*' => Ok(Token::Asterisk),
            '/' => Ok(Token::Slash),
            '0'..='9' => Ok(Token::Value(v.to_digit(10).unwrap() as usize)),
            _ => Err(CronParseError::InvalidToken(v)),
        }
    }
}
