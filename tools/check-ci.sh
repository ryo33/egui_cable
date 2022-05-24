#! /usr/bin/env bash

# fail fast
set -eo pipefail
shopt -s inherit_errexit

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::dbg_macro
cargo check --tests --all-features
cargo test --no-run --locked --all-features
cargo test --all-features
