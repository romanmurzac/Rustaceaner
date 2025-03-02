## Chapter 7: References and Borrowing - Fixing Ownership Issues

**Goal:** Learn how to use references to avoid ownership transfer issues.

### Task
1. Create a new Rust project:
```shell
cargo new finance_borrowing
```
2. Modify `main.rs` to:
   - Reuse the `Transaction` struct from the previous exercise.
   - Change `print_transaction(transaction: Transaction)` to `print_transaction(transaction: &Transaction)`, making it accept a reference instead of taking ownership.
   - In `main()`:
      - Create a `Transaction` instance.
      - Pass a reference of it to `print_transaction()`.
      - Print it again in `main()` to confirm the data is still accessible.

### Output
```
Transaction: $50.0 - Food
Printing again in main: $50.0 - Food
```