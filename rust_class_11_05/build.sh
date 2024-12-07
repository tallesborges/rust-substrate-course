#!/bin/bash
set -e

####################################################################
# STEP 1: Build the project with the wasm32-unknown-unknown target #
####################################################################

# Set the stack size to 64KB
STACK_SIZE=65536

# List of custom flags to pass to all compiler invocations that Cargo performs.
RUST_WASM_FLAGS=(
    # Max wasm stack size
    "-Clink-arg=-zstack-size=$STACK_SIZE"
    # Configure the wasm target to import instead of export memory
    '-Clink-arg=--import-memory'
    # Doesn't optimize this build for any specific CPU
    '-Ctarget-cpu=mvp'
)

# Separated flags by 0x1f (ASCII Unit Separator)
# Reference: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads
RUST_WASM_FLAGS="$(printf '%s\x1f%s\x1f%s' ${RUST_WASM_FLAGS[@]})"
echo "$RUST_WASM_FLAGS"

# Build the project with the wasm32-unknown-unknown target
CARGO_ENCODED_RUSTFLAGS="$RUST_WASM_FLAGS" cargo build --release --target=wasm32-unknown-unknown

######################################################
# Remove unnecessary code and optimize the wasm file #
######################################################
# Run `wasm-opt --help` to see all available options
WASM_OPT_OPTIONS=(
    -O3
    --dce
    --precompute
    --precompute-propagate
    --optimize-instructions
    --optimize-casts
    --low-memory-unused
    --optimize-added-constants
    --optimize-added-constants-propagate
    --simplify-globals-optimizing
    --inlining-optimizing
    --once-reduction
    --merge-locals
    --merge-similar-functions
    --strip
    --strip-debug
    # --remove-memory
    --remove-unused-names
    --remove-unused-types
    --remove-unused-module-elements
    --duplicate-function-elimination
    --duplicate-import-elimination
    --reorder-functions
    --abstract-type-refining
    --alignment-lowering
    --avoid-reinterprets
    # --zero-filled-memory
    --disable-simd
    --disable-threads
    --disable-gc
    --disable-multivalue
    --disable-reference-types
    --disable-exception-handling
    --optimize-stack-ir
    --vacuum
    # --unsubtyping
)

# Create the `optimized.wasm` file (binary format)
wasm-opt \
    "${WASM_OPT_OPTIONS[@]}" \
    --output ./optimized.wasm \
    ./target/wasm32-unknown-unknown/release/rust_wasm_minimal.wasm

# Create the `optimized.wat` file (text format)
wasm-opt \
    "${WASM_OPT_OPTIONS[@]}" \
    --emit-text \
    --output ./optimized.wat \
    ./target/wasm32-unknown-unknown/release/rust_wasm_minimal.wasm

# Print the `optimized.wat` file
cat ./optimized.wat
