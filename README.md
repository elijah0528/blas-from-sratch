# Rust Matrix Multiplication Optimizations

This repository is a small Rust playground for experimenting with matrix
multiplication performance. It implements a `Matrix` type and several
dot-product strategies that illustrate how different optimization techniques
affect runtime.

## Features

- `Matrix` struct with helpers for setting, reading, and printing values
- Multiple dot-product implementations that show performance progression
- Uses Rayon for parallel iteration in the multithreaded variant

## Optimization techniques

The following approaches are included in `main.rs` (with 1024×1024 matrices):

1. **Naive triple loop** (~1.42s)
2. **Cache-friendly loop reordering** (~93.28ms)
3. **Column tiling** (~80.87ms)
4. **Parallel tiling (Rayon)** — current active implementation

## Getting started

1. Install Rust: <https://rustup.rs/>
2. Build and run:

```bash
cargo run --release
```

The program creates two 1024×1024 matrices, multiplies them, prints the result,
and reports the elapsed time.

## Dependencies

- [`rayon`](https://crates.io/crates/rayon)

## Notes

The multithreaded tiling implementation is experimental and may require
additional correctness validation.
