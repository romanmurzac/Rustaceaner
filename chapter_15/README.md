## Chapter 15: Project Structure - Organizing Transactions in Modules

**Goal:** Learn how to structure a Rust project using modules.

### Task
1. Create a new Rust project:
```shell
cargo new finance_project_structure
```
2. Modify the project structure:
   - Create a `src/transactions.rs` file to define transaction-related logic.
   - In `transactions.rs`:
      - Define an enum `TransactionType` (`Income` or `Expense`).
      - Define a `Transaction` struct.
      - Implement a function `print_transaction(transaction: &Transaction)`.
   - In `main.rs`:
      - Declare the `transactions` module.
      - Import and use the `Transaction` struct to create and print transactions.

### Project Structure
```
finance_project_structure/
│── src/
│   ├── main.rs
│   ├── transactions.rs
│── Cargo.toml
```

### Output
```
Transaction Details:
Type: Expense | Amount: $45.99 | Category: Groceries
```