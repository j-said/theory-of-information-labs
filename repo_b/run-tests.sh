#!/bin/bash

TEST_DIR="test-images"
OUTPUT_LOG="results_summary_$(date).txt"

echo "==========================================" > "$OUTPUT_LOG"

if [ ! -d "$TEST_DIR" ]; then
    echo "Error: Directory $TEST_DIR not found."
    exit 1
fi

for img in "$TEST_DIR"/*.png; do
    [ -e "$img" ] || continue
    echo "Processing $img..."
    cargo run -q -- "$img" >> "$OUTPUT_LOG" 2>> "error_log.txt"
done

echo "Done. Results stored in $OUTPUT_LOG"