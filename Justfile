#!/bin/env -S just --justfile

# run all test suites in the project
test:
    cd tree-sitter-peppermint && just test
    cd cli && cargo check
    cargo test

# generate documentation for all crates
doc *args="":
    cargo doc \
        --document-private-items \
        --workspace \
        {{ args }}
