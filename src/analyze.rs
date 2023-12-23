use crate::{error::Result, models::AscentDB};

pub fn analyze_ascent_db(database: &String) -> Result<String> {
    let db = AscentDB::new(database)?;

    let total_count = db.total_count()?;

    let analysis = format!("Total count: {total_count}");

    Ok(analysis)
}
