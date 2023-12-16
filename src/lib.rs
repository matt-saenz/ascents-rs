type Result<T> = std::result::Result<T, &'static str>;

mod models {
    use crate::Result;
    use regex::Regex;
    use std::fmt;
    use time::Date;

    pub struct Route {
        name: String,
        grade: String,
        crag: String,
    }

    impl Route {
        pub fn new(name: String, grade: String, crag: String) -> Result<Self> {
            let valid_yds = Regex::new(r"^5\.([0-9]|1[0-5][a-d])$").expect("Regex should compile");

            if !valid_yds.is_match(&grade) {
                return Err("grade must be in YDS with no pluses, minuses, or slashes");
            }

            Ok(Self { name, grade, crag })
        }
    }

    impl fmt::Display for Route {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} at {}", self.name, self.grade, self.crag)
        }
    }

    pub struct Ascent {
        route: Route,
        date: Date,
    }

    impl Ascent {
        pub fn new(route: Route, date: Date) -> Self {
            Self { route, date }
        }
    }

    impl fmt::Display for Ascent {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} on {}", self.route, self.date)
        }
    }

    pub struct AscentDB {
        database: String,
    }

    impl AscentDB {
        pub fn new(database: String) -> Self {
            Self { database }
        }

        pub fn log_ascent(&self, ascent: Ascent) {
            println!("Logged ascent in {}: {}", self.database, ascent);
        }
    }
}

pub mod cli {
    use crate::{
        models::{Ascent, AscentDB, Route},
        Result,
    };
    use std::io::{self, Write};
    use time::{macros::format_description, Date};

    pub const USAGE: &str = "Usage: ascents {init,log,drop,analyze} database";

    enum Subcommand {
        Init,
        Log,
        Drop,
        Analyze,
    }

    pub struct Args {
        subcommand: Subcommand,
        database: String,
    }

    impl Args {
        pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self> {
            args.next();

            let subcommand = match args.next() {
                Some(arg) => arg,
                None => return Err("Must provide subcommand"),
            };

            let subcommand = match subcommand.as_str() {
                "init" => Subcommand::Init,
                "log" => Subcommand::Log,
                "drop" => Subcommand::Drop,
                "analyze" => Subcommand::Analyze,
                _ => return Err("Invalid subcommand"),
            };

            let database = match args.next() {
                Some(arg) => arg,
                None => return Err("Must provide database"),
            };

            if args.next().is_some() {
                return Err("Invalid extra arg");
            }

            Ok(Self {
                subcommand,
                database,
            })
        }
    }

    fn input(prompt: &str) -> String {
        print!("{prompt}");

        io::stdout().flush().expect("Failed to flush");

        let mut resp = String::new();

        io::stdin()
            .read_line(&mut resp)
            .expect("Failed to read line");

        let resp: &str = resp.trim();

        resp.to_string()
    }

    fn get_route() -> Result<Route> {
        let name = input("Enter the name of the route: ");
        let grade = input("Enter the grade of the route: ");
        let crag = input("Enter the name of the crag where the route is located: ");

        Route::new(name, grade, crag)
    }

    fn parse_date(date: String) -> Result<Date> {
        let format = format_description!("[year]-[month]-[day]");

        Date::parse(&date, &format).map_err(|_| "date must be a valid date in YYYY-MM-DD format")
    }

    fn get_ascent() -> Result<Ascent> {
        let route = get_route()?;

        let date = input("Enter the date of the ascent in YYYY-MM-DD format: ");
        let date = parse_date(date)?;

        Ok(Ascent::new(route, date))
    }

    pub fn run(args: Args) -> Result<()> {
        let db = AscentDB::new(args.database);

        match args.subcommand {
            Subcommand::Log => {
                let ascent = get_ascent()?;
                db.log_ascent(ascent);
                Ok(())
            }
            _ => {
                println!("That subcommand has not been implemented yet!");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{cli::*, models::*};

    #[test]
    fn invalid_grade() {
        let invalid_grades = [
            "5.9+".to_string(),
            "5.10".to_string(),
            "5.11a/b".to_string(),
            "5.12-".to_string(),
        ];

        for invalid_grade in invalid_grades {
            let result = Route::new(
                "Some Route".to_string(),
                invalid_grade,
                "Some Crag".to_string(),
            );

            assert!(result.is_err());
        }
    }

    #[test]
    fn valid_grade() {
        let valid_grades = [
            "5.0".to_string(),
            "5.9".to_string(),
            "5.10a".to_string(),
            "5.11d".to_string(),
        ];

        for valid_grade in valid_grades {
            let result = Route::new(
                "Some Route".to_string(),
                valid_grade,
                "Some Crag".to_string(),
            );

            assert!(result.is_ok());
        }
    }

    #[test]
    fn valid_args() {
        let valid_subcommands = [
            "init".to_string(),
            "log".to_string(),
            "drop".to_string(),
            "analyze".to_string(),
        ];

        for valid_subcommand in valid_subcommands {
            let result = Args::new(
                [
                    "program".to_string(),
                    valid_subcommand,
                    "database".to_string(),
                ]
                .into_iter(),
            );

            assert!(result.is_ok());
        }
    }

    #[test]
    fn invalid_args() {
        let invalid_arg_sets = [
            vec!["program".to_string()],
            vec!["program".to_string(), "invalid-subcommand".to_string()],
            vec!["program".to_string(), "log".to_string()],
            vec![
                "program".to_string(),
                "log".to_string(),
                "database".to_string(),
                "extra".to_string(),
            ],
        ];

        for invalid_arg_set in invalid_arg_sets {
            let result = Args::new(invalid_arg_set.into_iter());
            assert!(result.is_err());
        }
    }
}
