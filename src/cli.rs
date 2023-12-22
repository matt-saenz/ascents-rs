use crate::{
    error::{Error, Result, User},
    init,
    models::{Ascent, AscentDB, Route},
    utils,
};
use time::Date;

pub const USAGE: &str = "Usage: ascents [-h] {init,log,drop,analyze} database";

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
            None => return Err(Error::User(User::MissingArg("subcommand"))),
        };

        let subcommand = match subcommand.as_str() {
            // Immediate successful exit with usage if first arg
            // was actually a request for help
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            "init" => Subcommand::Init,
            "log" => Subcommand::Log,
            "drop" => Subcommand::Drop,
            "analyze" => Subcommand::Analyze,
            _ => return Err(Error::User(User::InvalidSubcommand)),
        };

        let database = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::User(User::MissingArg("database"))),
        };

        if args.next().is_some() {
            return Err(Error::User(User::TooManyArgs));
        }

        Ok(Self {
            subcommand,
            database,
        })
    }
}

fn get_route() -> Result<Route> {
    let name = utils::input("Enter the name of the route: ");
    let grade = utils::input("Enter the grade of the route: ");
    let crag = utils::input("Enter the name of the crag where the route is located: ");

    Route::new(name, grade, crag)
}

fn parse_date(date: String) -> Result<Date> {
    Date::parse(&date, utils::DATE_FORMAT).map_err(|_| Error::User(User::InvalidDate))
}

fn get_ascent() -> Result<Ascent> {
    let route = get_route()?;

    let date = utils::input("Enter the date of the ascent in YYYY-MM-DD format: ");
    let date = parse_date(date)?;

    Ok(Ascent::new(route, date))
}

fn init(args: Args) -> Result<()> {
    println!("Initializing ascent database: {}", args.database);
    init::init_ascent_db(&args.database)?;
    println!("Successfully initialized database");
    Ok(())
}

fn log(args: Args) -> Result<()> {
    let db = AscentDB::new(&args.database)?;

    let ascent = get_ascent()?;
    let crag = ascent.route().crag();
    let known_crags = db.crags()?;

    if !known_crags.is_empty() && !known_crags.contains(crag) {
        println!("Warning: '{crag}' is not a known crag");
        println!("Known crags currently include:\n{}", known_crags.join("\n"));
        utils::confirm("Continue logging");
    }

    println!("Ascent to be logged: {ascent}");
    utils::confirm("Log the above ascent");

    db.log_ascent(&ascent)?;
    println!("Successfully logged the above ascent");

    Ok(())
}

fn drop(args: Args) -> Result<()> {
    let db = AscentDB::new(&args.database)?;

    let route = get_route()?;
    let ascent = db.find_ascent(route)?;

    println!("Ascent to be dropped: {ascent}");
    utils::confirm("Drop the above ascent");

    db.drop_ascent(ascent.route())?;
    println!("Successfully dropped the above ascent");

    Ok(())
}

pub fn run(args: Args) -> Result<()> {
    match args.subcommand {
        Subcommand::Init => init(args),
        Subcommand::Log => log(args),
        Subcommand::Drop => drop(args),
        _ => {
            println!("That subcommand has not been implemented yet!");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
