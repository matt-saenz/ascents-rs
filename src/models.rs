use crate::{
    error::{Error, Result, User},
    utils,
};
use regex::Regex;
use rusqlite::Connection;
use std::fmt;
use time::Date;

#[derive(Debug, PartialEq)]
pub struct Route {
    name: String,
    grade: String,
    crag: String,
}

impl Route {
    pub fn new(name: String, grade: String, crag: String) -> Result<Self> {
        let valid_yds = Regex::new(r"^5\.([0-9]|1[0-5][a-d])$").expect("Regex should compile");

        if !valid_yds.is_match(&grade) {
            return Err(Error::User(User::InvalidGrade));
        }

        Ok(Self { name, grade, crag })
    }

    pub fn crag(&self) -> &String {
        &self.crag
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} at {}", self.name, self.grade, self.crag)
    }
}

#[derive(Debug, PartialEq)]
pub struct Ascent {
    route: Route,
    date: Date,
}

impl Ascent {
    pub fn new(route: Route, date: Date) -> Self {
        Self { route, date }
    }

    pub fn route(&self) -> &Route {
        &self.route
    }
}

impl fmt::Display for Ascent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.route, self.date)
    }
}

#[derive(Debug, PartialEq)]
pub struct Count {
    category: String,
    value: u32,
}

impl Count {
    pub fn category(&self) -> &String {
        &self.category
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub struct AscentDB {
    connection: Connection,
}

impl AscentDB {
    pub fn new(database: &String) -> Result<Self> {
        if !utils::exists(database) {
            return Err(Error::User(User::DatabaseNotFound));
        }

        let connection = Connection::open(database)?;

        Ok(Self { connection })
    }

    pub fn crags(&self) -> Result<Vec<String>> {
        let mut crags = Vec::new();

        let mut statement = self.connection.prepare(
            "
            SELECT DISTINCT crag
            FROM ascents
            ORDER BY crag
            ",
        )?;

        let rows = statement.query_map((), |row| row.get(0))?;

        for crag in rows {
            crags.push(crag?);
        }

        Ok(crags)
    }

    pub fn log_ascent(&self, ascent: &Ascent) -> Result<()> {
        let mut statement = self.connection.prepare(
            "
            SELECT date
            FROM ascents
            WHERE route = ? AND grade = ? AND crag = ?
            ",
        )?;

        let mut rows = statement.query_map(
            (&ascent.route.name, &ascent.route.grade, &ascent.route.crag),
            |row| row.get::<usize, String>(0),
        )?;

        if let Some(date) = rows.next() {
            return Err(Error::User(User::AscentAlreadyLogged(date?)));
        }

        self.connection.execute(
            "
            INSERT INTO ascents(route, grade, crag, date)
            VALUES(?, ?, ?, ?)
            ",
            (
                &ascent.route.name,
                &ascent.route.grade,
                &ascent.route.crag,
                format_date(ascent.date),
            ),
        )?;

        Ok(())
    }

    pub fn find_ascent(&self, route: Route) -> Result<Ascent> {
        let mut statement = self.connection.prepare(
            "
            SELECT date
            FROM ascents
            WHERE route = ? AND grade = ? AND crag = ?
            ",
        )?;

        let mut rows = statement.query_map((&route.name, &route.grade, &route.crag), |row| {
            row.get::<usize, String>(0)
        })?;

        match rows.next() {
            None => Err(Error::User(User::AscentNotFound)),
            Some(date) => Ok(Ascent::new(
                route,
                Date::parse(&date?, utils::DATE_FORMAT).expect("Should be able to parse date"),
            )),
        }
    }

    pub fn drop_ascent(&self, route: &Route) -> Result<()> {
        let mut statement = self.connection.prepare(
            "
            SELECT 1
            FROM ascents
            WHERE route = ? AND grade = ? AND crag = ?
            ",
        )?;

        let exists = statement.exists((&route.name, &route.grade, &route.crag))?;

        if !exists {
            return Err(Error::User(User::AscentNotFound));
        }

        self.connection.execute(
            "
            DELETE FROM ascents
            WHERE route = ? AND grade = ? AND crag = ?
            ",
            (&route.name, &route.grade, &route.crag),
        )?;

        Ok(())
    }

    pub fn total_count(&self) -> Result<u32> {
        let total_count = self.connection.query_row(
            "
            SELECT count(*)
            FROM ascents
            ",
            (),
            |row| row.get(0),
        )?;

        Ok(total_count)
    }

    pub fn year_counts(&self) -> Result<Vec<Count>> {
        let statement = self.connection.prepare(
            "
            SELECT strftime('%Y', date) AS year, count(*)
            FROM ascents
            GROUP BY year
            ORDER BY year
            ",
        )?;

        gather_counts(statement)
    }

    pub fn crag_counts(&self) -> Result<Vec<Count>> {
        let statement = self.connection.prepare(
            "
            SELECT crag, count(*)
            FROM ascents
            GROUP BY crag
            ORDER BY crag
            ",
        )?;

        gather_counts(statement)
    }
}

fn format_date(date: Date) -> String {
    date.format(utils::DATE_FORMAT)
        .expect("Should be able to format date")
}

fn gather_counts(mut statement: rusqlite::Statement) -> Result<Vec<Count>> {
    let mut counts = Vec::new();

    let rows = statement.query_map((), |row| {
        Ok(Count {
            category: row.get(0)?,
            value: row.get(1)?,
        })
    })?;

    for count in rows {
        counts.push(count?);
    }

    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

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

    fn ascents() -> [Ascent; 8] {
        [
            Ascent::new(
                Route::new(
                    "Classic Route".to_string(),
                    "5.12a".to_string(),
                    "Some Crag".to_string(),
                )
                .unwrap(),
                date!(2023 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Some Other Route".to_string(),
                    "5.9".to_string(),
                    "Some Crag".to_string(),
                )
                .unwrap(),
                date!(2022 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "New Route".to_string(),
                    "5.10d".to_string(),
                    "New Crag".to_string(),
                )
                .unwrap(),
                date!(2022 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Another Route".to_string(),
                    "5.10a".to_string(),
                    "Another Crag".to_string(),
                )
                .unwrap(),
                date!(2023 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Some Route".to_string(),
                    "5.7".to_string(),
                    "Some Crag".to_string(),
                )
                .unwrap(),
                date!(2023 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Old Route".to_string(),
                    "5.11a".to_string(),
                    "Old Crag".to_string(),
                )
                .unwrap(),
                date!(2022 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Cool Route".to_string(),
                    "5.10a".to_string(),
                    "Some Crag".to_string(),
                )
                .unwrap(),
                date!(2022 - 01 - 01),
            ),
            Ascent::new(
                Route::new(
                    "Last Route".to_string(),
                    "5.7".to_string(),
                    "Old Crag".to_string(),
                )
                .unwrap(),
                date!(2023 - 01 - 01),
            ),
        ]
    }

    fn set_up_test_db() -> AscentDB {
        let test_db = "test.db".to_string();

        if !utils::exists(&test_db) {
            panic!("{test_db} must be initialized to test");
        }

        let conn = Connection::open(&test_db).unwrap();
        conn.execute("DELETE FROM ascents", ()).unwrap();

        let db = AscentDB::new(&test_db).unwrap();

        for ascent in &ascents() {
            db.log_ascent(ascent).unwrap();
        }

        db
    }

    #[test]
    fn crags() {
        let db = set_up_test_db();

        let expected = vec![
            "Another Crag".to_string(),
            "New Crag".to_string(),
            "Old Crag".to_string(),
            "Some Crag".to_string(),
        ];

        assert_eq!(db.crags().unwrap(), expected);
    }

    #[test]
    fn log_ascent() {
        let db = set_up_test_db();

        for ascent in &ascents() {
            assert_eq!(
                db.log_ascent(ascent).unwrap_err(),
                Error::User(User::AscentAlreadyLogged(format_date(ascent.date))),
            );
        }
    }

    #[test]
    fn find_ascent() {
        let db = set_up_test_db();

        for ascent in ascents() {
            let route = Route::new(
                ascent.route.name.clone(),
                ascent.route.grade.clone(),
                ascent.route.crag.clone(),
            )
            .unwrap();

            assert_eq!(ascent, db.find_ascent(route).unwrap());
        }

        let route = Route::new(
            "Non-existent route".to_string(),
            "5.7".to_string(),
            "Non-existent crag".to_string(),
        )
        .unwrap();

        assert_eq!(
            db.find_ascent(route).unwrap_err(),
            Error::User(User::AscentNotFound),
        );
    }

    #[test]
    fn drop_ascent() {
        let db = set_up_test_db();

        for ascent in &ascents() {
            let route = &ascent.route;
            assert!(db.drop_ascent(route).is_ok());
            assert_eq!(
                db.drop_ascent(route).unwrap_err(),
                Error::User(User::AscentNotFound),
            );
        }
    }

    #[test]
    fn total_count() {
        let db = set_up_test_db();
        assert_eq!(db.total_count().unwrap(), 8);
    }

    #[test]
    fn year_counts() {
        let db = set_up_test_db();

        let expected = vec![
            Count {
                category: "2022".to_string(),
                value: 4,
            },
            Count {
                category: "2023".to_string(),
                value: 4,
            },
        ];

        assert_eq!(db.year_counts().unwrap(), expected);
    }

    #[test]
    fn crag_counts() {
        let db = set_up_test_db();

        let expected = vec![
            Count {
                category: "Another Crag".to_string(),
                value: 1,
            },
            Count {
                category: "New Crag".to_string(),
                value: 1,
            },
            Count {
                category: "Old Crag".to_string(),
                value: 2,
            },
            Count {
                category: "Some Crag".to_string(),
                value: 4,
            },
        ];

        assert_eq!(db.crag_counts().unwrap(), expected);
    }
}
