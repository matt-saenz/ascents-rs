use std::io::{self, Write};

pub type Result<T> = std::result::Result<T, &'static str>;

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
