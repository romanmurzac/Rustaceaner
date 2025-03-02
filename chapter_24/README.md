## Chapter 24: Concurrency - Processing Transactions in Parallel

**Goal:** Learn how to use threads and async programming to process transactions efficiently.

### Task 1
#### Using Threads for Parallel Processing
1. Create a new Rust project:
```shell
cargo new finance_concurrency
```
2. Modify `main.rs` to:
   - Use *threads* `(std::thread)` to process incomes and expenses *in parallel*.
   - Use `join` to ensure both threads complete before printing the final balance.

### Output 1
```
Processed incomes.
Processed expenses.
Final Balance: $1250.00
```

### Task 2
#### Using Async Rust for Non-Blocking Execution
1. Modify Cargo.toml to include async-std:
```
[dependencies]
async-std = "1.10"
```
2. Modify `main.rs` to:
   - Use *async/await* to fetch incomes and expenses concurrently.
   - Use `async-std::task` instead of threads.

### Output 2
```
Processed incomes.
Processed expenses.
Final Balance: $1250.00
```