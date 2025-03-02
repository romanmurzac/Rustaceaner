## Chapter 8: Slices - Extracting Transaction Details

**Goal:** Learn how to use slices to work with parts of strings.

### Task
1. Create a new Rust project:
```shell
cargo new finance_slices
```
2. Modify `main.rs` to:
   - Define a function `get_category_slice(category: &String) -> &str` that:
   - Takes a reference to a `String`
   - Returns the first 3 characters as a slice
   - In `main()`:
      - Create a `Transaction` struct with a `category` field
      - Call `get_category_slice() `on category
      - Print the original category and its slice

### Output
```
Category: Entertainment
Category Slice: Ent
```