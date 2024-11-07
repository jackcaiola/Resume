#!/usr/bin/env bash
cargo test --all-features
cargo fmt -- --check
cargo clippy --all-targets --all-features
cargo doc --no-deps