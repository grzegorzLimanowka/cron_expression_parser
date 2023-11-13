use std::ops::Range;

use thiserror::Error;

use super::{token::Token, AllowedValues, Value};

// https://docs.oracle.com/cd/E12058_01/doc/doc.1014/e12030/cron_expressions.htm

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Token while parsing cron value line: {0}")]
    InvalidToken(char),
}

pub struct Parser {}

// Sample input: ["*/15", "0", "1,15", "*", "1-5"]
impl Parser {
    pub fn parse(input: Vec<String>) -> Result<(), ParseError> {
        let minutes = input.get(0).unwrap();

        // "*/15"
        let tokens = SubExpressionParser::parse(&minutes)?;

        Ok(())
    }
}

// Parses single line into vec of tokens
pub struct SubExpressionParser {} // -> TokenParser?

impl SubExpressionParser {
    // 1-12 -> vec![Token::Value(1), Token::Dash, Token::Value(12)]
    pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
        let mut tokens = vec![];
        let mut last_digits: Vec<char> = vec![];

        for curr in input.chars() {
            let token: Token = Token::try_from(curr)?;

            if let Token::Value(v) = token {
                last_digits.push(char::from_digit(v as u32, 10).unwrap()); // FIXME: unwrap
            } else {
                if last_digits.len() > 0 {
                    // FIXME: unwrap
                    let joined: usize = last_digits.iter().collect::<String>().parse().unwrap();
                    tokens.push(Token::Value(joined));

                    last_digits = vec![];
                }

                tokens.push(token);
            }
        }

        // FIXME: unwrap
        if last_digits.len() > 0 {
            let joined: usize = last_digits.iter().collect::<String>().parse().unwrap();
            tokens.push(Token::Value(joined));
        }

        Ok(tokens)
    }
}

pub struct SubExpressionValidator {}

// impl SubExpressionValidator {
//     pub fn validate(input: Vec<Token>, )
// }

// pub struct Val

#[cfg(test)]
mod tests {
    use crate::cron::token::Token;
    use rstest::rstest;

    use super::SubExpressionParser;

    #[rstest]
    #[case["1-12", vec![Token::Value(1), Token::Dash, Token::Value(12)]]]
    #[case["*/15", vec![Token::Asterisk, Token::Slash, Token::Value(15)]]]
    #[case["0", vec![Token::Value(0)]]]
    #[case["1,19", vec![Token::Value(1), Token::Comma, Token::Value(19)]]]
    #[case["*", vec![Token::Asterisk]]]
    #[test]
    fn parse_sub_expression(#[case] input: &str, #[case] output: Vec<Token>) {
        assert_eq!(SubExpressionParser::parse(input).unwrap(), output);
    }
}

// TODO: Finish these traits:

// Trait for parsing tokens
trait ITokenParser {
    type Error;

    fn parse(&self) -> Result<Vec<Token>, Self::Error>;
}

// Trait for valdating tokens
trait ITokenValidator {
    type Error;

    fn validate(&self, tokens: Vec<Token>, allowed: AllowedValues) -> Result<Value, Self::Error>;
}
