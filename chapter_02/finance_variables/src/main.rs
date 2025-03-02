use std::io::{self, Write};

fn main() {
    println!("Welcome to the Personal Finance Tracker!");

    // Asking for the user's name.
    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush output");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // Remove any trailing newline or spaces.
    let name = name.trim();

    println!("Hello, {}! Let's start tracking your finances.", name);

    // Asking for the initial balance.
    let mut balance = 0.0;
    print!("Enter your initial balance: ");
    io::stdout().flush().expect("Failed to flush output");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");

    // Convert user input to f64.
    balance = match user_input.trim().parse::<f64>() {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid input! Setting balance to $0.00");
            0.0
        }
    };

    println!("Your starting balance is: ${:.2}", balance);
}
