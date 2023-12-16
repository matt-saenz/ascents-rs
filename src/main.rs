use ascents::cli::{self, Args};
use std::{env, process};

fn main() {
    let args = match Args::new(env::args()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}\nError: {e}", cli::USAGE);
            process::exit(1);
        }
    };

    if let Err(e) = cli::run(args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
