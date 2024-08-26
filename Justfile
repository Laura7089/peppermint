#!/bin/env -S just --justfile

# run all test suites in the project
test:
    cd tree-sitter-peppermint && just test
    cargo test
