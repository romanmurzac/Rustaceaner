## Chapter 18: File I/O - Saving and Loading Transactions

**Goal:** Learn how to use file reading and writing to persist transaction data.

### Task
1.Create a new Rust project:
```shell
cargo new finance_file_io
```
2. Modify main.rs to:
   - Define a Transaction struct with:
      - amount: f64
      - category: String
      - transaction_type: String (either "income" or "expense")
   - Implement two functions:
      - save_transaction(transaction: &Transaction) -> std::io::Result<()>
      - load_transactions() -> std::io::Result<Vec<Transaction>>
   - Use serde & serde_json for serialization.
3. Add dependencies in Cargo.toml:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Output
First Run - Adding a transaction:
```
Transactions Loaded:
Transaction { amount: 100.0, category: "Groceries", transaction_type: "expense" }
```

Second Run - After adding more transactions:
```
Transactions Loaded:
Transaction { amount: 100.0, category: "Groceries", transaction_type: "expense" }
Transaction { amount: 50.0, category: "Transport", transaction_type: "expense" }
Transaction { amount: 500.0, category: "Salary", transaction_type: "income" }
```