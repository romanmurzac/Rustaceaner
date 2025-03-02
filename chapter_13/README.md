## Chapter 13: Vectors - Storing Multiple Transactions

**Goal:** Learn how to use Vec<T> to store multiple transactions dynamically.

### Task
1. Create a new Rust project:
```shell
cargo new finance_vectors
```
2. Modify `main.rs` to:
   - Define an enum `TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
      - `transaction_type: TransactionType`
   - Create a `Vec<Transaction>` to store multiple transactions.
   - Add at least three sample transactions to the vector.
   - Iterate over the vector and print each transaction.

### Output
```
Transaction History:
1. Type: Income | Amount: $1500.00 | Category: Salary
2. Type: Expense | Amount: $45.99 | Category: Groceries
3. Type: Expense | Amount: $20.00 | Category: Transport
```