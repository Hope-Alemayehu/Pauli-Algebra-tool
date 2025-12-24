# Pauli Algebra Tool

A small Rust project that performs Pauli algebra simplification on single-qubit sequences.
Instead of multiplying matrices, it uses Pauli group rules to efficiently compute the resulting Pauli operator and global phase.

## Features

- Simplify sequences of single-qubit Pauli gates (I, X, Y, Z)
- Tracks the global phase (1, -1, i, -i)
- Outputs the final simplified Pauli operator and phase
- Fully written in Rust, no external dependencies

## Usage

Run the program:

```bash
cargo run
```

It will read a hardcoded Pauli string in main.rs (e.g., "XYZXXYY") and print the simplified result:

```text
Final result: iI
```

Testing
Run all tests with:

```bash
cargo test
```

Tests cover common sequences to ensure correct algebra simplification.

## Project Structure

```text
pauli-algebra-tool/
├── src/
│   ├── main.rs       # Entry point with CLI logic
│   └── lib.rs        # Core library code (Pauli enums, phase tracking, simplification logic)
├── tests/
│   └── pauli_tests.rs  # Integration tests
├── Cargo.toml        # Project metadata and dependencies
└── README.md         # Project documentation

```