#!/usr/bin/env bash
set -e
cargo test --all-features
cargo fmt -- --check
cargo clippy --all-targets --all-features -- --deny warnings
cargo doc --no-deps
