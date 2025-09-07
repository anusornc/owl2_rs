#!/bin/bash

# Build script for WebAssembly compilation

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null
then
    echo "wasm-pack could not be found. Please install it with:"
    echo "cargo install wasm-pack"
    exit 1
fi

# Build for WebAssembly
wasm-pack build --target web --out-dir pkg

echo "WebAssembly build complete. Files are in the pkg/ directory."