use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    InvalidGrade,
    InvalidDate,
    DatabaseNotFound,
    DatabaseAlreadyExists,
    MissingArg(&'static str),
    InvalidSubcommand,
    TooManyArgs,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidGrade => write!(
                f,
                "grade must be in YDS with no pluses, minuses, or slashes",
            ),
            Error::InvalidDate => write!(f, "date must be a valid date in YYYY-MM-DD format"),
            Error::DatabaseNotFound => write!(
                f,
                "database not found, must be an already initialized ascent database",
            ),
            Error::DatabaseAlreadyExists => write!(f, "Cannot initialize database, already exists"),
            Error::MissingArg(arg) => write!(f, "Must provide {arg}"),
            Error::InvalidSubcommand => write!(f, "Invalid subcommand"),
            Error::TooManyArgs => write!(f, "Too many args provided"),
        }
    }
}
