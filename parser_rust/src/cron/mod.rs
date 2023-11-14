use std::{
    collections::{BTreeMap, VecDeque},
    ops::Range,
};

use self::{parser::ParseError, token::Token};

pub mod parser;
pub mod token;

/// Full info about single Cron Job
pub struct CronExpression {
    minutes: Value,
    hours: Value,
    day_of_month: Value,
    month: Value,
    day_of_week: Value,
    command: String,
}

impl CronExpression {
    // Prints schedule in required format
    pub fn print_schedule(&self) {
        println!("---CronExpr---");
        match self.minutes.ticks() {
            Ok(ok) => {
                println!("minutes \t{:?}", ok);
            }
            Err(err) => {
                println!("minutes: err while parsing: {:?}", err);
            }
        }

        match self.hours.ticks() {
            Ok(ok) => {
                println!("hours \t\t\t{:?}", ok);
            }
            Err(err) => {
                println!("err parsing hours: {:?}", err);
            }
        }

        match self.day_of_month.ticks() {
            Ok(ok) => {
                println!("day of month \t\t{:?}", ok);
            }
            Err(err) => {
                println!("err parsing day_of_month: {:?}", err);
            }
        }

        match self.month.ticks() {
            Ok(ok) => {
                println!("month \t\t\t{:?}", ok);
            }
            Err(err) => {
                println!("err parsing month: {:?}", err);
            }
        }

        match self.day_of_week.ticks() {
            Ok(ok) => {
                println!("day of week \t\t{:?}", ok);
            }
            Err(err) => {
                println!("err parsing day_of_week: {:?}", err);
            }
        }

        println!("command \t\t {:?}", self.command);
    }
}

pub struct Value {
    /// Name of a field, for example "Hour"
    kind: Kind,

    /// All single Tokens, that come with value
    value: Vec<Token>,
}

// TODO: Refactor ticks, its too nested
impl Value {
    /// returns vector of ticks for a value
    pub fn ticks(&self) -> Result<Vec<usize>, ParseError> {
        let mut ticks = BTreeMap::<usize, ()>::new();
        let mut stack = VecDeque::<Token>::new();

        // Here we are pushing all the valid expressions
        for token in &self.value {
            // println!("curr: {:?} | {:?}", token, stack);

            // TODO: Refactor in future, currently it's ugly:
            match token {
                Token::Value(curr_v) => {
                    if let Some(last) = stack.pop_back() {
                        match last {
                            Token::Comma => {
                                if let Some(before_last) = stack.pop_back() {
                                    // Case: 2, 5 -> insert [2, 5]
                                    if let Token::Value(bef_last_v) = before_last {
                                        ticks.insert(*curr_v, ());
                                        ticks.insert(bef_last_v, ());
                                        // continue; // to prevent pushing Token::Value(i)
                                    } else {
                                        return Err(ParseError::ExpectedToken(Token::Value(0)));
                                    }
                                } else {
                                    return Err(ParseError::ExpectedToken(Token::Value(0)));
                                }
                            }
                            Token::Dash => {
                                if let Some(before_last) = stack.pop_back() {
                                    // Case: 2 - 5 -> insert [2, 3, 4, 5]
                                    if let Token::Value(before_last_v) = before_last {
                                        // TODO: Add validation
                                        for i in before_last_v..=*curr_v {
                                            ticks.insert(i, ());
                                        }
                                        // continue; // to prevent pushing Token::Value(i)
                                    } else {
                                        return Err(ParseError::ExpectedToken(Token::Value(0)));
                                    }
                                } else {
                                    return Err(ParseError::ExpectedToken(Token::Value(0)));
                                }
                            }
                            _ => {
                                return Err(ParseError::WrongTokenOrder(token.clone(), last));
                            }
                        }
                    } else {
                        stack.push_back(token.clone())
                    }
                }
                Token::Comma => {
                    // TODO
                    stack.push_back(Token::Comma)
                }
                Token::Dash => {
                    // TODO

                    stack.push_back(Token::Dash)
                }
                Token::Asterisk => {
                    // TODO

                    stack.push_back(Token::Asterisk)
                }
                Token::Slash => {
                    // TODO

                    stack.push_back(Token::Slash)
                }
            }
        }

        let accepted_range = self.kind.valid_defaults();

        if let Some(token) = stack.pop_back() {
            match token {
                Token::Value(v) => {
                    ticks.insert(v, ());
                }
                Token::Asterisk => {
                    for i in accepted_range.range.start..=accepted_range.range.end {
                        ticks.insert(i, ());
                    }
                }
                Token::Comma | Token::Dash | Token::Slash => {
                    return Err(ParseError::RemainingTokenUnexpected(token));
                }
            }
        }

        // If there are Tokens left, something went wrong
        if stack.len() > 0 {
            return Err(ParseError::ExpectedTokenStackEmpty(stack.len()));
        }

        // println!("{:?}", ticks);

        // filtered: [1, 2, 3, 4] -> (range: (1, 3)) -> [1, 2, 3]
        Ok(ticks
            .into_keys()
            .collect::<Vec<usize>>()
            .into_iter()
            .filter(|v| v >= &accepted_range.range.start && v <= &accepted_range.range.end)
            .collect())
    }
}

// TODO: Impl operations on Tokens

pub enum Kind {
    Minutes,
    Hours,
    DayOfMonth,
    Month,
    DayOfWeek,
}

impl Kind {
    // returns default range of allowed values
    pub fn valid_defaults(&self) -> ValidValues {
        match &self {
            Kind::Minutes => ValidValues {
                range: Range { start: 0, end: 59 },
                text: false,
            },
            Kind::Hours => ValidValues {
                range: Range { start: 0, end: 23 },
                text: false,
            },
            Kind::DayOfMonth => ValidValues {
                range: Range { start: 0, end: 31 },
                text: false,
            },
            Kind::Month => ValidValues {
                range: Range { start: 1, end: 12 },
                text: true, // Unsupported
            },
            Kind::DayOfWeek => ValidValues {
                range: Range { start: 1, end: 7 },

                text: false, // Unsupported
            },
        }
    }
}

#[allow(unused)]
pub struct ValidValues {
    // range of numeric values
    range: Range<usize>,

    // text values allowed
    text: bool,
}

#[cfg(test)]
mod tests {
    use crate::cron::token::Token;
    use rstest::rstest;

    #[rstest]
    #[case[vec![Token::Value(55), Token::Dash, Token::Value(70)], Ok(vec![55, 56, 57, 58, 59])]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(8)], Ok(vec![1, 8])]]
    #[test]
    fn minutes(#[case] input: Vec<Token>, #[case] output: Result<Vec<usize>, super::ParseError>) {
        let val = super::Value {
            kind: super::Kind::Minutes,
            value: input,
        };

        assert_eq!(val.ticks(), output);
    }

    #[rstest]
    #[case[vec![Token::Value(19), Token::Dash, Token::Value(30)], Ok(vec![19, 20, 21, 22, 23])]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(8)], Ok(vec![1, 8])]]
    #[test]
    fn hours(#[case] input: Vec<Token>, #[case] output: Result<Vec<usize>, super::ParseError>) {
        let val = super::Value {
            kind: super::Kind::Hours,
            value: input,
        };

        assert_eq!(val.ticks(), output);
    }

    #[rstest]
    #[case[vec![Token::Value(1), Token::Dash, Token::Value(8)], Ok(vec![1, 2, 3, 4, 5, 6, 7, 8])]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(8)], Ok(vec![1, 8])]]
    #[test]
    fn day_of_month(
        #[case] input: Vec<Token>,
        #[case] output: Result<Vec<usize>, super::ParseError>,
    ) {
        let val = super::Value {
            kind: super::Kind::DayOfMonth,
            value: input,
        };

        assert_eq!(val.ticks(), output);
    }

    #[rstest]
    #[case[vec![Token::Value(9), Token::Dash, Token::Value(18)], Ok(vec![9, 10, 11, 12])]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(8)], Ok(vec![1, 8])]]
    #[test]
    fn month(#[case] input: Vec<Token>, #[case] output: Result<Vec<usize>, super::ParseError>) {
        let val = super::Value {
            kind: super::Kind::Month,
            value: input,
        };

        assert_eq!(val.ticks(), output);
    }

    #[rstest]
    #[case[vec![Token::Value(1), Token::Dash, Token::Value(8)], Ok(vec![1, 2, 3, 4, 5, 6, 7])]]
    #[case[vec![Token::Value(1), Token::Comma, Token::Value(8)], Ok(vec![1])]]
    #[test]
    fn day_of_week(
        #[case] input: Vec<Token>,
        #[case] output: Result<Vec<usize>, super::ParseError>,
    ) {
        let val = super::Value {
            kind: super::Kind::DayOfWeek,
            value: input,
        };

        assert_eq!(val.ticks(), output);
    }
}
