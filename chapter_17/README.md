## Chapter 17: HashMaps - Categorizing Transactions

**Goal:** Learn how to use HashMap to group transactions by category.

### Task
1. Create a new Rust project:
```shell
cargo new finance_hashmaps
```
2. Modify `main.rs` to:
   - Import `std::collections::HashMap`.
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `category: String`
   - Create a `HashMap<String, Vec<Transaction>>` to store transactions by category.
   - Add at least three transactions, grouping them by category.
   - Iterate over the HashMap and print transactions under each category.

### Output:
```
Transactions by Category:
Category: Groceries
  - Amount: $45.99 (Expense)
  - Amount: $12.50 (Expense)

Category: Salary
  - Amount: $1500.00 (Income)
```