## Chapter 4. Functions - Modularizing Logic

**Goal:** Introduce functions for reusable logic.

### Task
1. Create a Rust project:
   ```shell
   cargo new finance_functions
   ```
2. Modify `main.rs` to:
   - Define a function `add_transaction(amount: f64, category: &str)`
   - Inside the function, print `"Added transaction: $<amount> on <category>."`
   - Call this function in `main` with sample values

### Output
```
Added transaction: $100.00 on Rent.
Added transaction: $20.50 on Entertainment.
```