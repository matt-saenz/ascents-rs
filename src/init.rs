use crate::{
    error::{Error, Result},
    utils,
};
use rusqlite::Connection;

pub fn init_ascent_db(database: &String) -> Result<()> {
    if utils::exists(database) {
        return Err(Error::DatabaseAlreadyExists);
    }

    let conn = Connection::open(database)?;

    conn.execute(
        "
        CREATE TABLE ascents(
            route TEXT NOT NULL,
            grade TEXT NOT NULL,
            crag TEXT NOT NULL,
            date TEXT NOT NULL,
            PRIMARY KEY(route, grade, crag)
        )
        ",
        (),
    )?;

    Ok(())
}
