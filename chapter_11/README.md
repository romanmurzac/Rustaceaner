## Chapter 11: Generics - Creating a Flexible Report Function

**Goal:** Learn how to use generics to make functions more flexible.

### Task
1. Create a new Rust project:
```shell
cargo new finance_generics
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
      - `transaction_type: TransactionType`
   - Implement a generic function `print_summary<T: std::fmt::Display>(label: &str, value: T)`, which:
      - Takes a label and a value of any type that implements `Display`.
      - Prints them in a formatted way.
   - In `main()`:
      - Use `print_summary()` to display a balance and a transaction category.

### Output
```
Balance: $1200.50
Category: Groceries
```