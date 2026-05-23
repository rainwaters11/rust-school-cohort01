# Block Builder

A beginner Rust homework project demonstrating a simple blockchain-style block model.

## Project Overview

This project models a `Block` containing multiple `Transaction` entries. It demonstrates:

- `Transaction` struct with sender, receiver, and amount fields
- `Block` struct with block ID and a vector of transactions
- `Vec<Transaction>` to store multiple transactions in a block
- `impl` blocks for constructors and behavior methods
- Ownership of data inside structs and safe Rust borrowing

## Features

- Create new transactions with clear sender/receiver semantics
- Build a block and add transactions to it
- Display block details and total transaction value
- Keep the design beginner-friendly and easy to follow

## Run

```bash
cargo run
```

## Expected output

```text
=== Block #1 ===
Transactions: 3
  Alice -> Bob : 50.00
  Bob -> Carol : 25.00
  Carol -> Alice : 10.00
Total Value : 85.00
```
