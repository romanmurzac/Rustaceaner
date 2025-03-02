## Chapter 14: Modules and Visibility - Organizing the Finance Tracker

**Goal:** Learn how to use modules and visibility to structure a Rust project properly.

### Task
1. Create a new Rust project:
```shell
cargo new finance_modules
```
2. Modify the project structure:
   - Create a `transactions` module for handling transaction logic.
   - Use `pub` *visibility* to control access to functions and structs.

### Project Structure
```
finance_modules/
│── src/
│   ├── main.rs
│   ├── transactions/
│   │   ├── mod.rs
│   │   ├── transaction.rs
│   │   ├── file_io.rs
```

### Output
First Run - Adding a transaction:
```
Transactions Loaded:
Transaction { amount: 200.0, category: "Dining", transaction_type: "expense" }
```

Second Run - After adding more transactions:
```
Transactions Loaded:
Transaction { amount: 200.0, category: "Dining", transaction_type: "expense" }
Transaction { amount: 100.0, category: "Groceries", transaction_type: "expense" }
Transaction { amount: 500.0, category: "Salary", transaction_type: "income" }
```