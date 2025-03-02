use std::io::{self, Write};

fn main() {
    println!("Welcome to the Personal Finance Tracker!");
    //Use macro to print messages to the terminal.
    println!("Enter your name: ");
    // Ensures that any pending output in the terminal is immediately displayed before taking user input.
    let _ = io::stdout().flush();
    // Creates a mutable empty string that will store the user input.
    let mut input = String::new();
    // Reads a full line of input from the user and stores the it into the input string.
    let _ = io::stdin().read_line(&mut input);
    println!(
        "Hello, {}! Let's start tracking your finances.",
        input.trim().to_string()
    )
}
