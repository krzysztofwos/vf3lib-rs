# vf3lib-rs — Rust Bindings for vf3lib

[![Crates.io](https://img.shields.io/crates/v/vf3lib-rs.svg)](https://crates.io/crates/vf3lib-rs)
[![Documentation](https://docs.rs/vf3lib-rs/badge.svg)](https://docs.rs/vf3lib-rs)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](https://github.com/krzysztofwos/vf3lib-rs#license)
[![Build Status](https://github.com/krzysztofwos/vf3lib-rs/workflows/CI/badge.svg)](https://github.com/krzysztofwos/vf3lib-rs/actions)

Rust FFI bindings to the VF3/VF3L/VF3P subgraph isomorphism algorithms from [MIVIA Lab](https://github.com/MiviaLab/vf3lib).

## Features

- **High Performance**: Direct bindings to the optimized C++ implementation
- **Multiple Algorithms**: VF3 (full heuristics), VF3L (lightweight), and VF3P (parallel)
- **Flexible Matching**: Both node-induced and edge-induced subgraph isomorphism
- **Graph Formats**: Supports VF legacy and edge list formats
- **Safe Rust API**: Type-safe wrapper around the C++ library

## Quick Start

```rust
use vf3lib_rs::{run_vf3, RunOptions};

// Run VF3 algorithm on graph files
let result = run_vf3("pattern.grf", "target.grf", RunOptions::default())?;
println!("Found {} matches in {:.3}s", result.solutions, result.time_all);
```

## Algorithm Variants

### VF3 — Full Heuristics

Best for medium to large dense graphs.

```rust
use vf3lib_rs::{run_vf3, RunOptions};
let result = run_vf3("pattern.grf", "target.grf", RunOptions::default())?;
```

### VF3L — Lightweight

Best for small or sparse graphs (no look-ahead).

```rust
use vf3lib_rs::{run_vf3l, RunOptions};
let result = run_vf3l("pattern.grf", "target.grf", RunOptions::default())?;
```

### VF3P — Parallel (Linux only)

For computationally hard instances. Requires Linux due to thread affinity APIs.

```rust
use vf3lib_rs::{run_vf3p, RunOptions, ParallelOptions};
let mut par_opts = ParallelOptions::default();
par_opts.num_threads = 4;
let result = run_vf3p("pattern.grf", "target.grf", RunOptions::default(), par_opts)?;
```

## Options

```rust
use vf3lib_rs::{RunOptions, GraphFormat};

let opts = RunOptions {
    format: GraphFormat::VFLegacy, // or GraphFormat::EdgeList
    undirected: false,             // Treat graphs as undirected
    edge_induced: false,           // Use edge-induced instead of node-induced
    first_only: false,             // Stop after first solution
    verbose: false,                // Enable verbose output
    store_solutions: false,        // Store all mappings (uses more memory)
    repetition_time_limit: 1.0,    // Minimum time for averaging multiple runs
};
```

## Builder API

For a more ergonomic interface:

```rust
use vf3lib_rs::VF3Query;

let result = VF3Query::new("pattern.grf", "target.grf")
    .edge_induced()
    .undirected()
    .run_light()?; // Uses VF3L variant
```

## Building

This crate requires a C++ compiler (GCC, Clang, or MSVC) to build the bundled vf3lib.

```bash
cargo build --release
```

## Testing

The crate includes comprehensive test coverage with 32 bundled graph files from the vf3lib repository, located in `tests/data/`. These range from small validation graphs to larger SI2 datasets, enabling thorough testing without any additional setup.

```bash
cargo test           # Run all tests
cargo test --release # Run in release mode (faster for larger graphs)
```

## License

The Rust bindings are dual-licensed under MIT OR Apache-2.0.

The bundled vf3lib C++ headers are licensed under LGPL v3. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for details and compliance information.

## References

- [vf3lib GitHub Repository](https://github.com/MiviaLab/vf3lib)
- [VF3: A New Algorithm for Subgraph Isomorphism](https://ieeexplore.ieee.org/document/8693808)
