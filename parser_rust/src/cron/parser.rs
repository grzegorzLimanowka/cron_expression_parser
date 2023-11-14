use thiserror::Error;

use super::{token::Token, CronExpression, Kind, ValidValues, Value};

// https://docs.oracle.com/cd/E12058_01/doc/doc.1014/e12030/cron_expressions.htm

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Invalid Token while parsing cron value line: {0}")]
    InvalidToken(char),
    #[error("This sequence of tokens is invalid: {0} -> {1}")]
    WrongTokenOrder(Token, Token),
    #[error("Validation was no correct")]
    ValidationError,
    #[error("Expected token {0}, but not found")]
    ExpectedToken(Token),
    #[error("Token stack not empty, nr of elems in stack: {0}")]
    ExpectedTokenStackEmpty(usize),
    #[error("Token stack not empty, nr of elems in stack: {0}")]
    RemainingTokenUnexpected(Token),
}

pub struct Parser {}

impl Parser {
    pub fn parse(input: Vec<String>, command: String) -> Result<CronExpression, ParseError> {
        let mut expression_tokens = vec![];

        for expr in input {
            let tokens: Vec<Token> = SubExpressionParser::parse(&expr)?;
            let _ =
                SubExpressionValidator::validate(&tokens, Some(&Kind::Minutes.valid_defaults()))?;

            expression_tokens.push(tokens);
        }

        Ok(CronExpression {
            minutes: Value {
                kind: Kind::Minutes,
                value: expression_tokens[0].to_owned(),
            },
            hours: Value {
                kind: Kind::Hours,
                value: expression_tokens[1].to_owned(),
            },
            day_of_month: Value {
                kind: Kind::DayOfMonth,
                value: expression_tokens[2].to_owned(),
            },
            month: Value {
                kind: Kind::Month,
                value: expression_tokens[3].to_owned(),
            },
            day_of_week: Value {
                kind: Kind::DayOfWeek,
                value: expression_tokens[4].to_owned(),
            },
            command,
        })
    }
}

/// SubExpression parser. Used for parsing single sub expressions
/// into vec of tokens
///
/// I: "1-9" -> O: [Token::Value(1), Token::Dash, Token::Value(9)]
pub struct SubExpressionParser {}

impl SubExpressionParser {
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

impl SubExpressionValidator {
    pub fn validate(input: &Vec<Token>, allowed: Option<&ValidValues>) -> Result<(), ParseError> {
        for i in 0..input.len() - 1 {
            let (curr, next) = (input.get(i).unwrap(), input.get(i + 1).unwrap());

            match allowed {
                Some(validate) => {
                    if curr.validate(validate) == false {
                        return Err(ParseError::ValidationError);
                    }
                }
                None => {}
            };

            if !curr.check(next) {
                return Err(ParseError::WrongTokenOrder(curr.clone(), next.clone()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cron::token::Token;
    use rstest::rstest;

    use super::SubExpressionParser;
    use super::SubExpressionValidator;

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

    #[rstest]
    #[case[vec![Token::Value(1), Token::Dash, Token::Value(12)], Ok(())]]
    #[case[vec![Token::Asterisk, Token::Comma, Token::Value(15)], Err(crate::cron::parser::ParseError::WrongTokenOrder(Token::Asterisk, Token::Comma))]]
    #[case[vec![Token::Value(0)], Ok(())]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(19)], Ok(())]]
    #[case[vec![Token::Asterisk], Ok(())]]
    #[test]
    fn validate_sub_expression(
        #[case] input: Vec<Token>,
        #[case] output: Result<(), super::ParseError>,
    ) {
        assert_eq!(SubExpressionValidator::validate(&input, None), output);
    }
}

// TODO: Finish these traits in future:

// Trait for parsing tokens
trait ITokenParser {
    type Error;

    fn parse(&self) -> Result<Vec<Token>, Self::Error>;
}

// Trait for valdating tokens
trait ITokenValidator {
    type Error;

    fn validate(&self, tokens: Vec<Token>, allowed: ValidValues) -> Result<Value, Self::Error>;
}
