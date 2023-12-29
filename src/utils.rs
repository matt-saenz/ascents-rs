use std::{
    io::{self, Write},
    path::Path,
    process,
};
use time::{format_description::FormatItem, macros::format_description};

pub const DATE_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

pub fn input(prompt: &str) -> String {
    print!("{prompt}");

    io::stdout().flush().expect("Failed to flush");

    let mut resp = String::new();

    io::stdin()
        .read_line(&mut resp)
        .expect("Failed to read line");

    let resp: &str = resp.trim();

    resp.to_string()
}

pub fn confirm(prompt: &str) {
    let prompt = format!("{prompt} (y/n)? ");
    let mut resp = input(&prompt);

    loop {
        if resp == "y" {
            break;
        }

        if resp == "n" {
            process::exit(0);
        }

        resp = input("Oops! Valid inputs are 'y' or 'n'. Please try again: ");
    }
}

pub fn exists(path: &String) -> bool {
    Path::new(path)
        .try_exists()
        .expect("Should be able to determine if path exists")
}
