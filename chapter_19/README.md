## Chapter 19: Error Handling - Safely Parsing Transactions

**Goal:** Learn how to handle errors using Result and ? for safer input parsing.

### Task
1. Create a new Rust project:
```shell
cargo new finance_error_handling
```
2. Modify `main.rs` to:
   - Import `std::io` for user input.
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct.
   - Implement a function `parse_amount(input: &str) -> Result<f64, String>` that:
      - Tries to convert input into an `f64`.
      - Returns `Err("Invalid amount")` if parsing fails.
   - In `main()`:
      - Prompt the user for a transaction amount.
      - Use `parse_amount()` and handle errors with `match`.

### Output
✅ Valid Input:
```
Enter transaction amount: 100.50
Transaction recorded: $100.50
```

❌ Invalid Input:
```
Enter transaction amount: abc
Error: Invalid amount
```