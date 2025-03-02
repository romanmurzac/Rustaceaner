## Chapter 22: Closures - Filtering Transactions Dynamically

**Goal:** Learn how to use closures to filter transactions based on custom conditions.

### Task
1. Create a new Rust project:
```shell
cargo new finance_closures
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
      - `transaction_type: TransactionType`
   - Implement a function `filter_transactions<F>(transactions: &Vec<Transaction>, filter: F) -> Vec<&Transaction>` that:
      - Accepts a vector of transactions and a *closure* as a filter condition.
      - Returns a vector of transactions that match the filter.
   - In `main()`:
      - Create multiple transactions.
      - Use closures to filter:
          - Only expenses
         - Transactions above `$50.00`

### Output
```
Expenses:
- $45.99 (Groceries)
- $20.00 (Transport)

Transactions above $50.00:
- $1500.00 (Salary)
```