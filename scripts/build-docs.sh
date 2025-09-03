#!/bin/bash

# Documentation build script for owl2_rs

set -e  # Exit on any error

echo "Building owl2_rs documentation..."

# Create directories if they don't exist
mkdir -p target/doc
mkdir -p target/book

# Build Rust API documentation
echo "Building Rust API documentation..."
cargo doc --no-deps

# Build mdBook documentation
echo "Building mdBook documentation..."
mdbook build

# Copy mdBook output to target directory
cp -r book/* target/book/

echo "Documentation build complete!"
echo "API documentation available at: target/doc/owl2_rs/index.html"
echo "User guide available at: target/book/index.html"