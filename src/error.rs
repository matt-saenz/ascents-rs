use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    User(User),
    Internal(Internal),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::User(e) => write!(f, "{e}"),
            Error::Internal(e) => write!(f, "{e}"),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Internal(Internal::SQLRelatedIssue(error))
    }
}

#[derive(Debug, PartialEq)]
pub enum User {
    InvalidGrade,
    InvalidDate,
    DatabaseNotFound,
    DatabaseAlreadyExists,
    MissingArg(&'static str),
    InvalidSubcommand,
    TooManyArgs,
    AscentAlreadyLogged(String),
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            User::InvalidGrade => write!(
                f,
                "grade must be in YDS with no pluses, minuses, or slashes",
            ),
            User::InvalidDate => write!(f, "date must be a valid date in YYYY-MM-DD format"),
            User::DatabaseNotFound => write!(
                f,
                "database not found, must be an already initialized ascent database",
            ),
            User::DatabaseAlreadyExists => write!(f, "Cannot initialize database, already exists"),
            User::MissingArg(arg) => write!(f, "Must provide {arg}"),
            User::InvalidSubcommand => write!(f, "Invalid subcommand"),
            User::TooManyArgs => write!(f, "Too many args provided"),
            User::AscentAlreadyLogged(date) => {
                write!(f, "That ascent was already logged with a date of {date}")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Internal {
    SQLRelatedIssue(rusqlite::Error),
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Internal::SQLRelatedIssue(e) => write!(f, "SQL-related issue: {e}"),
        }
    }
}
