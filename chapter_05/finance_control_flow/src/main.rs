use std::io::{self, Write};

fn main() {
    println!("Enter the amount: ");
    let mut amount: String = String::new();

    io::stdout().flush().expect("Failed to flush output");
    std::io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    let amount: f64 = amount.trim().parse().unwrap();
    
    if amount >= 0.0 {
        println!("Transaction accepted: ${:.2}", amount);
    } else {
        println!("Invalid amount!");
    }
}
