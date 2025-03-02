## Chapter 10: Enums - Categorizing Transactions

**Goal:** Learn how to use enums to represent different transaction types.

### Task
1. Create a new Rust project:
```shell
cargo new finance_enums
```
2. Modify `main.rs` to:
   - Define an enum TransactionType with:
      - `Income`
      - `Expense`
   - Update the `Transaction` struct to include:
      - `transaction_type: TransactionType`
   - Implement a function `print_transaction(transaction: &Transaction)` that:
      - Prints the transaction details, including whether it's an *income* or *expense*.
   - In `main()`:
      - Create two transactions: one income and one expense
      - Print both transactions

### Output
```
Transaction Details:
Type: Income
Amount: $1500.00
Category: Salary
Description: Monthly paycheck
```
```
Transaction Details:
Type: Expense
Amount: $45.99
Category: Groceries
Description: Bought fresh vegetables and fruits.
```