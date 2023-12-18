use crate::{
    error::{Error, Result},
    utils,
};

pub fn init_ascent_db(database: &String) -> Result<()> {
    if utils::exists(database) {
        return Err(Error::DatabaseAlreadyExists);
    }

    Ok(())
}
