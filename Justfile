#!/bin/env -S just --justfile

# run all test suites in the project
test:
    cd tree-sitter-peppermint && just test
    cargo test
    cargo run -- -f ./sample_program.ppr parse

# generate documentation for all crates
doc *args="":
    cargo doc \
        --document-private-items \
        --workspace \
        --exclude tree-sitter-peppermint \
        {{ args }}
