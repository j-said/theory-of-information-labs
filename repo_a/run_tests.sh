#!/bin/bash

RESULTS_FILE="results.txt"
echo "--- Compression Tests Results $(date) ---" > $RESULTS_FILE

TEST_DIR="tests"

if [ ! -d "$TEST_DIR" ]; then
    echo "Error: Directory $TEST_DIR not found."
    exit 1
fi

for img in "$TEST_DIR"/*.{png,jpg,jpeg}; do
    [ -e "$img" ] || continue

    echo -e "\nFILE: $img" >> $RESULTS_FILE
    
    for bits in 1 2 4 8 16 32 64 128; do
        cargo run -q -- "$img" "$bits" >> $RESULTS_FILE
    done
    
    echo "--------------------------------------" >> $RESULTS_FILE
done

echo "Done. Check $RESULTS_FILE"