# Rust Guessing Game

My very first project built while diving into Rust. I built this using the official Rust documentation to understand how the language handles memory safety, strong typing, and basic control flow.

## What I Learned
* **Immutability by Default:** Understanding how Rust forces variables to be safe unless explicitly marked with `mut`.
* **Pattern Matching:** Using `match` structures instead of messy `if/else` chains.
* **Type Shadowing:** Reusing variable names when converting types (e.g., parsing a `String` into a `u32`).

## My Custom Additions
Instead of just copying the basic textbook example, I customized the backend logic:
1. **Bulletproof Input Handling:** Used an `Ok`/`Err` match block on string parsing so the game doesn't crash if a user accidentally types letters or symbols.
2. **User Feedback:** Added a clean `Numbers Only!!` terminal alert to explicitly tell the player when they made an invalid input before recycling the input loop.
3. **Clean Exits:** Implemented proper loop breaking so the program releases control back to the terminal shell immediately upon winning.

## How to Run
Make sure you have Rust and Cargo installed, then run:
```bash
cargo run
