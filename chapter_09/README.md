## Chapter 9: Structs - Defining Transactions

**Goal:** Learn how to use structs to model data.

### Task
1. Create a new Rust project:
```shell
cargo new finance_structs
```
2. Modify `main.rs` to:
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
      - `description: String`
   - Implement a function `print_transaction(transaction: &Transaction)` that:
      - Accepts a reference to a `Transaction`
      - Prints the transaction details
   - In `main()`:
      - Create an example transaction
      - Call `print_transaction()` to display the details

### Output
```
Transaction Details:
Amount: $45.99
Category: Groceries
Description: Bought fresh vegetables and fruits.
```