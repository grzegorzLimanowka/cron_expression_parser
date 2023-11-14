// TODO: REMOVE:

use std::env;

use crate::cron::parser::Parser;
use crate::parser::ArgParser;

mod cron;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(arg) = ArgParser::from_args(args) {
        let cron = Parser::parse(arg.cron(), arg.command());

        match cron {
            Ok(c) => {
                c.print_schedule();
            }
            Err(e) => {
                println!("Error occured: {e} !")
            }
        }
    }
}
