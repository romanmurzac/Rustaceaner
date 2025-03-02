## Chapter 25: Macros - Creating a Custom Logging Macro

**Goal:** Learn how to define and use macros for logging transactions dynamically.

### Task
1. Create a new Rust project:
```shell
cargo new finance_macros
```
2. Modify `main.rs` to:
   - Define a custom macro `log_transaction!()` that:
      - Accepts a transaction message and logs it with a timestamp.
      - Uses `std::time::SystemTime` to get the current time.
   - Use the macro to log:
      - When a transaction is added.
      - When a balance is checked.

### Output
```
[1709382345] Transaction added: $500.00 - Salary
[1709382345] Balance checked: $1500.00
```