use crate::utils::{self, Result};

pub fn init_ascent_db(database: &String) -> Result<()> {
    if utils::exists(database) {
        return Err("Cannot initialize database, already exists");
    }

    Ok(())
}
