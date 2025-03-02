## Chapter 27: Final Project - Personal Finance Tracker CLI

**Goal:** Build a complete command-line application that manages personal finances.

### Task
1. Create a new Rust project:
```shell
cargo new finance_tracker
```
2. Modify `main.rs` to:
   - Implement a *command-line interface (CLI)* using `std::io`.
   - Allow users to:
      - *Add* a transaction (income/expense).
      - *View* all transactions.
      - *View* the total balance.
      - *Filter* transactions by category.
   - Use previous concepts:
      - *Modules* for organizing logic.
      - *Enums & Structs* for transaction modeling.
      - *HashMaps* for categorizing transactions.
      - *Iterators & Closures* for processing data.
      - *Error Handling* for user input.

### Features
✅ Add income and expense transactions\
✅ View all transactions\
✅ View balance\
✅ Filter transactions by category\
✅ Save and load transactions (File I/O)\
✅ Use concurrency for parallel transaction processing\
✅ Implement logging using custom macros\
✅ Organize code with modules\
✅ Handle errors gracefully\
✅ Provide an interactive CLI interface

### Project Structure
```
finance_tracker/
│── src/
│   ├── main.rs
│   ├── transactions/
│   │   ├── mod.rs
│   │   ├── transaction.rs
│   │   ├── file_io.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── logger.rs
│   │   ├── concurrency.rs
│── transactions.json  # Persistent transaction storage
│── Cargo.toml
```

### Output
```yaml
Welcome to the Finance Tracker!
1. Add Transaction
2. View Transactions
3. View Balance
4. Filter by Category
5. Exit
Enter your choice: 1

Enter amount: 1500
Enter category: Salary
Enter type (income/expense): income
Transaction added!

Enter your choice: 2
Transactions:
- $1500.00 (Income) - Salary

Enter your choice: 3
Total Balance: $1500.00

Enter your choice: 5
Goodbye!
```