// TODO: REMOVE:
#![allow(unused)]

use std::{env, str::FromStr};

use thiserror::Error;

use crate::cron::parser::Parser;
use crate::parser::ArgParser;

mod cron;
mod parser;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if let Ok(arg) = ArgParser::from_args(args) {
        let cron = Parser::parse(arg.cron());
    }
}
