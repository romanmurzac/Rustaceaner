## Chapter 23: Iterators - Summing Transactions Efficiently

**Goal:** Learn how to use iterators to process transactions efficiently.

### Task
1. Create a new Rust project:
```shell
cargo new finance_iterators
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `transaction_type: TransactionType`
   - Create a `Vec<Transaction>` with multiple transactions.
   - Use *iterators* to:
      - Calculate the total income.
      - Calculate the total expenses.
      - Compute the final balance (income - expenses).

### Output
```
Total Income: $2000.00
Total Expenses: $650.75
Final Balance: $1349.25
```