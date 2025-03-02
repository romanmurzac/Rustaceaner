use std::io::{self, Write};

fn main() {
    println!("Welcome to the Personal Finance Tracker!");

    println!("Enter your name: ");
    let mut name: String = String::new();

    io::stdin().read_line(&mut name).unwrap();

    println!(
        "Hello, {}! Let's start tracking your finances.",
        name.trim()
    );

    let mut balance: f64 = 0.0;

    println!("Enter your initial balance: ");
    let mut user_balance: String = String::new();

    io::stdout().flush().expect("Failed to flush output");
    std::io::stdin()
        .read_line(&mut user_balance)
        .expect("Failed to read line");

    let user_balance: f64 = user_balance.trim().parse().unwrap();
    balance += user_balance;

    println!("Your starting balance is: ${:.2}", balance);
}
