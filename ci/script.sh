#!/usr/bin/env bash

# Make the entire script fail if one of the commands fails
set -ex

# Formatting
cargo fmt -- --check

# Clippy
touch ./*/*/*.rs
cargo clippy -- -W warnings

# Tests
cargo test --workspace -- --test-threads=1
