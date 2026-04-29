#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building project..."
cargo build --release

mkdir -p ./test

echo "Generating a 32kB test file (random_text.txt)..."
head -c 24000 /dev/urandom | base64 > ./test/random_text.txt

echo "Running Analysis on all test files..."

rm -f results.txt

for test_file in ./test/*.txt; do
    if [ -f "$test_file" ]; then
        ./target/release/compression-shannon-fano "$test_file"
    fi
done >> results.txt

echo "Tests completed. Results saved to results.txt."

 