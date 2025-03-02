## Chapter 6: Ownership - Managing Transactions

**Goal:** Understand Rust’s ownership system by handling transaction data.

### Task
1. Create a new Rust project:
```shell
cargo new finance_ownership
```
2. Modify `main.rs` to:
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
   - Implement a function `print_transaction(transaction: Transaction)` that:
      - Takes ownership of a `Transaction`
      - Prints the transaction details
   - In `main()`:
      - Create a `Transaction` instance
      - Pass it to `print_transaction()`
      - Try printing it again in `main()` (expect a compile-time error due to ownership transfer)

### Output
```
Transaction: $50.0 - Food
(error occurs when trying to access transaction after ownership transfer)
```