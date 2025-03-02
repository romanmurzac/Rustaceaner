## Chapter 26: Testing - Verifying Transaction Calculations

**Goal:** Learn how to write unit tests to ensure our finance logic is correct.

### Task
1. Create a new Rust project:
```shell
cargo new finance_testing
```
2. Modify `main.rs` to:
   - Define an `enum TransactionType` (`Income` or `Expense`).
   - Define a `Transaction` struct with:
      - `amount: f64`
      - `transaction_type: TransactionType`
   - Implement a function `calculate_balance(transactions: &Vec<Transaction>) -> f64` that:
      - Iterates over transactions.
      - Sums incomes and subtracts expenses to compute the final balance.
   - Add unit tests in a `#[cfg(test)]` module:
      - Test with only incomes.
      - Test with only expenses.
      - Test with mixed transactions.

### Tests
```shell
cargo test
```

### Output
```
running 3 tests
test tests::test_only_incomes ... ok
test tests::test_only_expenses ... ok
test tests::test_mixed_transactions ... ok

test result: ok. 3 passed; 0 failed;
```