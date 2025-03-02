## Chapter 21: Lifetimes - Ensuring Safe References in Transactions

**Goal:** Learn how to use lifetimes to manage references safely in Rust.

### Task
1. Create a new Rust project:
```shell
cargo new finance_lifetimes
```
2. Modify `main.rs` to:
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: &str` (a string reference instead of String).
   - Implement a function `largest_transaction<'a>(t1: &'a Transaction, t2: &'a Transaction) -> &'a Transaction` that:
      - Compares two transactions and returns the one with the larger amount.
   - In `main()`:
      - Create two transactions with string *slices* as categories.
      - Call `largest_transaction()` and print the result.

### Output
```
Larger Transaction: $150.00 - Rent
```