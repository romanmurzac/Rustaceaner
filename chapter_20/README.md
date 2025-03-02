## Chapter 20: Traits - Defining a Common Transaction Interface

**Goal:** Learn how to use traits to define shared behavior for different types of transactions.

### Task
1. Create a new Rust project:
```shell
cargo new finance_traits
```
2. Modify `main.rs` to:
   - Define a `Transaction` trait with a method `summary(&self) -> String`.
   - Implement the `Transaction` trait for:
      - A `Payment` struct with `amount: f64` and `recipient: String`.
      - An `Investment` struct with `amount: f64` and `asset: String`.
   - In `main()`:
      - Create both types of transactions.
      - Print their summaries using the trait method.

### Output
```
Payment of $200.00 to John Doe.
Investment of $500.00 in Bitcoin.
```