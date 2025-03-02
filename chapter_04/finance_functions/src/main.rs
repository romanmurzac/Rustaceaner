fn main() {
    let amount: f64 = 58.57;
    let category: &str = "Food";
    
    add_transaction(amount, category);
}

fn add_transaction(amount: f64, category: &str) {
    println!("Added transaction: ${} on {}.", amount, category);
}