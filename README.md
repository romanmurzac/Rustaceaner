# Rustaceaner

This GitHub repository will provide hands-on exercises aligned with a Udemy Rust course, progressively building a Personal Finance Tracker. The exercises are designed to reinforce Rust concepts while developing a practical, real-world application.

## Features
✅ Reinforces Rust concepts with **practical** exercises\
✅ Provides a **real-world** use case for each chapter\
✅ Encourages **progressive learning** — each exercise builds on previous ones\
✅ Ends with a complete, functional CLI **application**

## Chapters
0. Cheat Sheet
* Provide Rust cheat sheet

1. Getting Started
* Setup Rust environment
* Create a basic "Hello, world!" CLI

2. Variables and Mutability
* Store balance, income, and expenses
* Update mutable variables based on transactions

3. Data Types
* Use integers, floats, and booleans for financial calculations
* Define an enum for transaction type (Income/Expense)

4. Functions
* Modularize logic (e.g., add_transaction, calculate_balance)

5. Control Flow
* Implement decision-making (e.g., validating user input)

6. Ownership
* Pass transaction data between functions while managing ownership

7. References and Borrowing
* Optimize function signatures using references

8. Slices
* Extract parts of strings for transaction descriptions

9. Structs
* Define Transaction struct with fields for date, amount, and category

10. Enums
* Use enum for transaction categories

11. Generics
* Implement a generic function for formatting reports

12. Option & Result Enums
* Handle errors when parsing user input

13. Vectors
* Store multiple transactions in a Vec<Transaction>

14. Modules and Visibility
* Organize code into modules (transactions.rs, reports.rs)

15. Project Structure
* Create a structured CLI with organized files

16. Strings
* Format output messages and store transaction descriptions

17. HashMaps
* Track total spending per category using a HashMap<String, f64>

18. File I/O
* Read/write transactions to a JSON or CSV file

19. Error Handling
* Handle file read/write errors gracefully

20. Traits
* Implement Display and FromStr for Transaction

21. Lifetimes
* Manage borrowing in complex functions

22. Closures
* Use closures for filtering transactions

23. Iterators
* Process transactions efficiently with iterators

24. Concurrency (Threads & Async Rust)
* Run report generation in parallel using threads

25. Macros
* Define a macro for logging/debugging

26. Testing
* Write unit tests for core functionality

27. Final Project
* Combine all concepts into a fully functional finance tracker