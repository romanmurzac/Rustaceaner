## Chapter 2: Variables and Mutability - Storing User Balance

**Goal:** Introduce variables and mutability in Rust.

### Task
1. Create a new Rust project:
```sh
cargo new finance_variables
```

2. Modify `main.rs` to:
* Declare a **mutable** variable balance initialized to `0.0`
* Ask the user to input an initial balance
* Update `balance` with the user input
* Print `"Your starting balance is: $<balance>"`

### Output
```
Enter your initial balance: 250.50
Your starting balance is: $250.50
```