# AGENTS.md

## Project Overview
This repository is a small Rust playground for exploring matrix multiplication performance
optimizations. The primary focus is on comparing different approaches (naive, cache-friendly
loop ordering, tiling, and parallelism with Rayon) and benchmarking their runtime.

## Tech Stack
- **Rust** (standard library)
- **rayon** for parallel iterators

## Project Structure
- `main.rs` contains the entire implementation:
  - `Matrix` struct and helpers
  - Multiple `dot` implementations (most are commented out)
  - A `main()` function that benchmarks multiplication

## Build & Run
There is **no Cargo.toml** in the repo yet, so the project will not build with `cargo`
until one is added. Recommended setup:

1. Add a `Cargo.toml` (example):
   ```toml
   [package]
   name = "matrix-mult"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   rayon = "1"

   [[bin]]
   name = "matrix-mult"
   path = "main.rs"
   ```
2. Build and run:
   ```bash
   cargo run --release
   ```

If you prefer standard Cargo layout, move `main.rs` to `src/main.rs` and omit the `[[bin]]`
section.

## Code Architecture
- **`Matrix` struct**: stores `rows`, `columns`, and `values` (row-major).
- **Indexing formula** (intended): `index = row * columns + col`.
- **Multiplication (`dot`)**:
  - Several alternative implementations are included as commented blocks.
  - The active version uses tiling and Rayon for parallel row blocks.

## Development Guidelines
- Follow Rust naming conventions (snake_case for functions/variables).
- Keep alternate optimization strategies (and their timings) commented for easy comparison.
- Prefer small, deterministic matrices for debugging; use large ones for benchmarking.

## Known Issues
- **Duplicate `dot` definition**: there are two `dot` methods with the same name, which
  will not compile as-is.
- **Indexing bug**: `get_val`/`set_val` currently compute indices as
  `row_ind * col_ind + col_ind` (should be `row_ind * columns + col_ind`).
- **Missing Cargo.toml**: required for easy builds and dependency management.
- **No .gitignore**: consider adding standard Rust ignores.

## Testing & Benchmarking
- The `main()` function measures runtime using `Instant` and prints results.
- For correctness checks, add small matrix unit tests (e.g., 2x2, 3x3) and verify dot
  product outputs.
