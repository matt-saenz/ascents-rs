use std::io;

fn main() {
    println!("Enter the name of the route:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Congrats on sending {name}!");
}
