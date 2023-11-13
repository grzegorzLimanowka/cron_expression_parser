use std::{f32::DIGITS, str::FromStr};
use thiserror::Error;

use super::token::Token;

// use token::Token;

// Based on :
// https://docs.oracle.com/cd/E12058_01/doc/doc.1014/e12030/cron_expressions.htm

#[derive(Error, Debug)]
pub enum CronParseError {
    #[error("Invalid Token while parsing cron value line: {0}")]
    InvalidToken(char),
    // #[error("Invalid arg count, Expected {ARG_COUNT}, expected {0}")]
    // InvalidArgCount(usize),
}

pub struct CronJobParser {}

impl CronJobParser {
    // Sample input: ["*/15", "0", "1,15", "*", "1-5"]

    pub fn parse(input: Vec<String>) -> Result<(), CronParseError> {
        println!("{:?}", input);

        Ok(())
    }
}

pub struct CronLineParser {}

impl CronLineParser {
    // 1-12 -> vec![Token::Value(1), Token::Dash, Token::Value(12)]
    pub fn parse(input: &str) -> Result<Vec<Token>, CronParseError> {
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

#[cfg(test)]
mod tests {
    use crate::cron::token::Token;
    use rstest::rstest;

    use super::CronLineParser;

    #[rstest]
    #[case["1-12", vec![Token::Value(1), Token::Dash, Token::Value(12)]]]
    #[case["*/15", vec![Token::Asterisk, Token::Slash, Token::Value(15)]]]
    #[case["0", vec![Token::Value(0)]]]
    #[case["1,19", vec![Token::Value(1), Token::Comma, Token::Value(19)]]]
    #[case["*", vec![Token::Asterisk]]]
    #[test]
    fn parse_cron_line(#[case] input: &str, #[case] output: Vec<Token>) {
        assert_eq!(CronLineParser::parse(input).unwrap(), output);
    }
}
