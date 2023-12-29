use ascents::{
    cli::{self, Args},
    error::Error,
};
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
        match e {
            Error::User(e) => eprintln!("Error: {e}"),
            Error::Internal(e) => eprintln!("Unexpected internal error: {e}"),
        }

        process::exit(1);
    }
}
