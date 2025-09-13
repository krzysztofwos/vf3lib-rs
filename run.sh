#!/bin/bash -e

cargo test -p vf3-native --features compare-cpp --release
cargo run --release --example bench-harness --features compare-cpp,profile -- --reps 30 --compare-cpp
