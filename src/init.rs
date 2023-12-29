use crate::{
    error::{Error, Result, User},
    utils,
};
use rusqlite::Connection;

struct Grade {
    value: String,
    number: u32,
    letter: Option<String>,
}

fn generate_grade_info_data() -> Vec<Grade> {
    let mut grades = Vec::new();

    for number in 0..=15 {
        if number < 10 {
            grades.push(Grade {
                value: format!("5.{number}"),
                number,
                letter: None,
            });
        } else {
            for letter in 'a'..='d' {
                grades.push(Grade {
                    value: format!("5.{number}{letter}"),
                    number,
                    letter: Some(letter.to_string()),
                });
            }
        }
    }

    grades
}

pub fn init_ascent_db(database: &String) -> Result<()> {
    if utils::exists(database) {
        return Err(Error::User(User::DatabaseAlreadyExists));
    }

    let grade_info_data = generate_grade_info_data();

    let conn = Connection::open(database)?;

    conn.execute_batch(
        "
        CREATE TABLE ascents(
            route TEXT NOT NULL,
            grade TEXT NOT NULL,
            crag TEXT NOT NULL,
            date TEXT NOT NULL,
            PRIMARY KEY(route, grade, crag)
        );

        CREATE TABLE grade_info(
            grade TEXT PRIMARY KEY,
            grade_number INTEGER NOT NULL,
            grade_letter TEXT
        );
        ",
    )?;

    for grade in grade_info_data {
        conn.execute(
            "
            INSERT INTO grade_info
            VALUES(?, ?, ?)
            ",
            (grade.value, grade.number, grade.letter),
        )?;
    }

    Ok(())
}
