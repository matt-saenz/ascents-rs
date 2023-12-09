use std::io::{self, Write};

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

struct Route {
    name: String,
    grade: String,
    crag: String,
}

fn get_route() -> Route {
    let name = input("Enter the name of the route: ");
    let grade = input("Enter the grade of the route: ");
    let crag = input("Enter the name of the crag where the route is located: ");

    Route { name, grade, crag }
}

fn main() {
    let route = get_route();

    println!(
        "Congrats on sending {} {} at {}!",
        route.name, route.grade, route.crag,
    );
}
