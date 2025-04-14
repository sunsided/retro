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
    cargo test --workspace {{ ARGS }}

# Run benchmarks
bench *ARGS:
    cargo bench {{ ARGS }}

# Run in release mode
run:
    RUST_LOG=ecs=info cargo run --release --bin ecs

# Run with debug logging enabled
debug:
    RUST_LOG=ecs=debug cargo run --bin ecs

# Run with tracing enabled
trace:
    RUST_LOG=ecs=trace cargo run --bin ecs

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

# Emits the MIRI code of an application into the `target/debug/deps` directory.
emit-mir *ARGS="--release":
    RUSTFLAGS="--emit mir" cargo build {{ ARGS }}

# Dump the code into text files
dump-code:
    bash -c "shopt -s globstar && tail -n +1 **/*.wgsl > dump-shader.wgsl.txt"
    bash -c "shopt -s globstar && tail -n +1 **/src/***.rs > dump-game.rs.txt"
    bash -c "shopt -s globstar && tail -n +1 **/generated/***.rs > dump-engine.rs.txt"
