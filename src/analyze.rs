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
    let year_counts = db.year_counts()?;
    let crag_counts = db.crag_counts()?;
    let grade_counts = db.grade_counts()?;

    let analysis = format!(
        "Analysis of ascents in {database}\n\n\
        Total count: {total_count}\n\n\
        Count of ascents by year:\n{}\n\n\
        Count of ascents by crag:\n{}\n\n\
        Count of ascents by grade:\n{}",
        make_counts_table(year_counts),
        make_counts_table(crag_counts),
        make_counts_table(grade_counts),
    );

    Ok(analysis)
}
