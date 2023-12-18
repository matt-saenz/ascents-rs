use crate::{
    error::{Error, Result},
    utils,
};
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
            return Err(Error::InvalidGrade);
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
    pub fn new(database: String) -> Result<Self> {
        if !utils::exists(&database) {
            return Err(Error::DatabaseNotFound);
        }

        Ok(Self { database })
    }

    pub fn log_ascent(&self, ascent: Ascent) {
        println!("Logged ascent in {}: {}", self.database, ascent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
