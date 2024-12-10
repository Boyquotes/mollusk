#!/bin/bash

agave-install init 1.18.26
rm -rf target
cargo build
./scripts/build-test-programs.sh
cargo +nightly-2023-10-05 fmt --all -- --check
cargo +nightly-2023-10-05 clippy --all --all-features -- -D warnings
cargo test --all-features
