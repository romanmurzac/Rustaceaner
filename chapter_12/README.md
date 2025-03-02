## Chapter 12: Option and Result Enums - Handling Missing or Invalid Data

**Goal:** Learn how to use Option and Result enums for safer data handling.

### Task
1. Create a new Rust project:
```shell
cargo new finance_option_result
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: Option<f64>` (to handle cases where the amount is missing).
      - `category: String`
      - `transaction_type: TransactionType`
   - Implement a function `parse_amount(input: &str) -> Result<f64, String>` that:
      - Tries to parse a string into an `f64` amount.
      - Returns `Ok(amount)` if successful, or `Err("Invalid amount")` if parsing fails.
   - In `main()`:
      - Prompt the user to enter a transaction amount.
      - Use `parse_amount()` to convert it.
      - If valid, store it in a transaction. Otherwise, print an error message.

### Output
✅ Valid Input:
```
Enter transaction amount: 45.99
Transaction recorded: $45.99 - Groceries
```

❌ Invalid Input:
```
Enter transaction amount: abc
Error: Invalid amount
```