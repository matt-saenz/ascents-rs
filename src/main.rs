use ascents::{self, Ascent, Route};
use std::{
    io::{self, Write},
    process,
};

fn input(prompt: &str) -> String {
    print!("{prompt}");

    io::stdout().flush().expect("Failed to flush");

    let mut resp = String::new();

    io::stdin()
        .read_line(&mut resp)
        .expect("Failed to read line");

    let resp: &str = resp.trim();

    resp.to_string()
}

fn get_route() -> Route {
    let name = input("Enter the name of the route: ");
    let grade = input("Enter the grade of the route: ");
    let crag = input("Enter the name of the crag where the route is located: ");

    match Route::new(name, grade, crag) {
        Ok(route) => route,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}

fn get_ascent() -> Ascent {
    let route = get_route();
    // TODO: Get/use date of ascent

    Ascent::new(route)
}

fn main() {
    let ascent = get_ascent();
    ascents::log_ascent(ascent);
}
