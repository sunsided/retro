[private]
help:
    @just --list --unsorted

# Format the code
fmt:
    cargo fmt --all

# Lint the code and apply fixes
lint *ARGS:
    cargo clippy --fix --allow-staged {{ ARGS }}
    cargo fmt --all

# Build the workspace
build:
    cargo build --workspace

# Test all targets.
test *ARGS:
    cargo test --workspace {{ARGS}}

# Run benchmarks
bench *ARGS:
    cargo bench {{ARGS}}

# Run in release mode
run:
    RUST_LOG=retro=info cargo run --release --bin retrorun

# Run with debug logging enabled
debug:
    RUST_LOG=retro=debug cargo run --bin retrorun

# Run with tracing enabled
trace:
    RUST_LOG=retro=trace cargo run --bin retrorun

# Just build the documentation.
build-docs:
    export DOCSRS=1
    cargo +nightly doc --no-deps --workspace --all-features --document-private-items

[private]
build-doc: build-docs

# Build and open the documentation.
docs:
    export DOCSRS=1
    cargo +nightly doc --no-deps --workspace --all-features --document-private-items --open

[private]
doc: docs

[private]
open-docs: docs