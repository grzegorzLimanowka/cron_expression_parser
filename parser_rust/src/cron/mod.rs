use std::{ops::Range, str::FromStr};
use thiserror::Error;

use self::{parser::ParseError, token::Token};

pub mod parser;
pub mod token;

/// Full info about single Cron Job
pub struct Expression {
    minute: Value,
    hour: Value,
    day_of_month: Value,
    month: Value,
    day_of_week: Value,
}

pub struct Value {
    /// Name of a field, for example "Hour"
    kind: Kind,

    /// All single Tokens, that come with value
    value: Vec<Token>,
}

// impl TryFrom<Vec<Token>> for Value {
//     type Error = ParseError;

//     fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {

//     }
// }

pub enum Kind {
    Minutes,
    Hours,
    DayOfMonth,
    Month,
    DayOfWeek,
}

impl Kind {
    // returns default range of allowed values
    pub fn default_allowed(&self) -> AllowedValues {
        match &self {
            Kind::Minutes => AllowedValues {
                range: Range { start: 0, end: 59 },
                text: false,
            },
            Kind::Hours => AllowedValues {
                range: Range { start: 0, end: 59 },
                text: false,
            },
            Kind::DayOfMonth => AllowedValues {
                range: Range { start: 0, end: 23 },
                text: false,
            },
            Kind::Month => AllowedValues {
                range: Range { start: 1, end: 31 },
                text: false,
            },
            Kind::DayOfWeek => AllowedValues {
                range: Range { start: 0, end: 11 },

                text: false,
            },
        }
    }
}

pub struct AllowedValues {
    // range of numeric values
    range: Range<usize>,

    // text values allowed
    text: bool,
}
