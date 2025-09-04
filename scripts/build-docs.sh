#!/bin/bash

# Documentation build script for owl2_rs

set -e  # Exit on any error

echo "Building owl2_rs documentation..."

# Create directories if they don't exist
mkdir -p target/doc
mkdir -p target/book

# Build Rust API documentation
echo "Building Rust API documentation..."
cargo doc --no-deps --document-private-items

# Check if mdBook is installed
if ! command -v mdbook &> /dev/null
then
    echo "mdbook could not be found, installing..."
    cargo install mdbook
fi

# Build mdBook documentation
echo "Building mdBook documentation..."
mdbook build

# Copy mdBook output to target directory
if [ -d "book" ]; then
    cp -r book/* target/book/
    echo "Documentation copied to target/book/"
else
    echo "Warning: book directory not found"
fi

echo "Documentation build complete!"
echo "API documentation available at: target/doc/owl2_rs/index.html"
echo "User guide available at: target/book/index.html"