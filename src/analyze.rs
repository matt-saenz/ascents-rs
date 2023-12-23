use crate::{
    error::Result,
    models::{AscentDB, Count},
};

fn make_counts_table(counts: Vec<Count>) -> String {
    let mut table: Vec<String> = Vec::new();

    for count in counts {
        table.push(format!("{:>4}  {}", count.value(), count.category()));
    }

    table.join("\n")
}

pub fn analyze_ascent_db(database: &String) -> Result<String> {
    let db = AscentDB::new(database)?;

    let total_count = db.total_count()?;
    let crag_counts = db.crag_counts()?;

    let analysis = format!(
        "Analysis of ascents in {database}\n\n\
        Total count: {total_count}\n\n\
        Count of ascents by crag:\n{}",
        make_counts_table(crag_counts),
    );

    Ok(analysis)
}
