## Chapter 16: Strings - Handling Transaction Descriptions

**Goal:** Learn how to work with String and &str efficiently.

### Task
1. Create a new Rust project:
```shell
cargo new finance_strings
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
      - `description: String`
   - Implement a function `summarize(transaction: &Transaction) -> String` that:
      - Returns a formatted string summarizing the transaction.
   - In `main()`:
      - Create a transaction with a long description.
      - Extract a short summary (first 20 characters) and print both.

### Output:
```
Full Description: Bought fresh vegetables and fruits for the week.
Short Summary: Bought fresh vegeta...
```