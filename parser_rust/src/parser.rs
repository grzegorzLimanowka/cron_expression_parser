use thiserror::Error;

const ARG_COUNT: usize = 6;

#[derive(Error, Debug)]
pub enum ArgParserError {
    #[error("Zero arguments provided")]
    ZeroArgumentsProvided,
    #[error("Two or more arguments provided")]
    TwoOrMoreArgumentProvided,
    #[error("Invalid arg count, Expected {ARG_COUNT}, expected {0}")]
    InvalidArgCount(usize),
}

pub struct ArgParser {
    cron: Vec<String>,
    task: String,
}

impl ArgParser {
    // Takes args
    pub fn from_args(args: Vec<String>) -> Result<Self, ArgParserError> {
        match args.len() {
            1 => Err(ArgParserError::ZeroArgumentsProvided),
            2 => {
                let parts: Vec<String> = args[1].split(" ").map(|v| v.to_string()).collect();

                if parts.len() != ARG_COUNT {
                    return Err(ArgParserError::InvalidArgCount(parts.len()));
                }

                let (cron, task) = parts.split_at(parts.len() - 1);

                println!("{:?} {:?}", cron, task);

                Ok(Self {
                    cron: cron.to_owned(),
                    task: task.concat(),
                })
            }
            _ => Err(ArgParserError::TwoOrMoreArgumentProvided),
        }
    }

    pub fn cron(&self) -> Vec<String> {
        self.cron.clone()
    }

    pub fn task(&self) -> String {
        self.task.clone()
    }
}
