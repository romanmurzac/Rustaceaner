use std::io;

fn main() {
    println!("Welcome to the Personal Finance Tracker!");

    println!("Enter your name: ");
    let mut name: String = String::new();

    io::stdin().read_line(&mut name).unwrap();

    println!(
        "Hello, {}! Let's start tracking your finances.",
        name.trim()
    )
}
