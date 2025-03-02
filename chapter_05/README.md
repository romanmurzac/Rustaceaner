## Chapter 5. Control Flow - Validating Transactions

**Goal:** Use conditionals to validate user input.

### Task
1. Create a Rust project:
   ```shell
   cargo new finance_control_flow
   ```
2. Modify `main.rs` to:
   - Ask the user for a transaction amount
   - If the amount is negative, print `"Invalid amount!"`
   - Otherwise, print `"Transaction accepted: $<amount>"`

### Output
✅ Valid Input:
```
Enter transaction amount: 30.00
Transaction accepted: $30.00
```

❌ Invalid Input:
```
Enter transaction amount: -5.00
Invalid amount!
```