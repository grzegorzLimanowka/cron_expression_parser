use std::str::FromStr;
use thiserror::Error;

use self::token::Token;

pub mod parser;
pub mod token;

/// Full info about single Cron Job
pub struct CronJob {
    minute: CronValue,
    hour: CronValue,
    day_of_month: CronValue,
    month: CronValue,
    day_of_week: CronValue,
}

pub struct CronValue {
    /// Name of a field, for example "Hour"
    kind: CronValueKind,

    /// All single Tokens, that come with value
    value: Vec<Token>,
}

pub enum CronValueKind {
    Minutes,
    Hours,
    DayOfMonth,
    Month,
    DayOfWeek,
}
