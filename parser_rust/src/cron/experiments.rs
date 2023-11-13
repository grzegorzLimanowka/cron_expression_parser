trait ParseCronJob {
    // Hom many arguments will be passed:
    type ArgCount;
}

// pub struct

// pub struct CronJob {
//     // TODO
// }

#[cfg(test)]
mod tests {
    //

    use super::LineParser;

    #[test]
    pub fn testing() {
        let line_parser = LineParser::parse("1,2,3,4,5");
    }
}

// minute, hour, day_of_month, month, day_of_week, command

// impl FromStr for CronJob {
//     type Err = CronParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         //

//         todo!()
//     }
// }

// `Raw` args, that are parsed from input.
// struct CronArgsRaw {
//     minute: String,
//     hour: String,
//     day_of_month: String,
//     month: String,
//     day_of_week: String,
//     command: String,
// }

// struct CronExpr {}

// pub struct CronJob {
//     minute: Vec<u32>,
//     hour: Vec<u32>,
//     day_of_month: Vec<u32>,
//     day_of_week: Vec<u32>,
//     command: String,
// }
