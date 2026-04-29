#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

OUTPUT_FILE="results.txt"
ORIGINAL_TEXT="hello world"



echo "Running Hamming Code Experiments..." > $OUTPUT_FILE
echo "===================================" >> $OUTPUT_FILE

echo -e "\n1. Running Unit Tests (Verifying average and edge cases)..." >> $OUTPUT_FILE
if cargo test > /dev/null 2>&1; then
    echo "Unit tests passed successfully." >> $OUTPUT_FILE
else
    echo "Unit tests FAILED. Check console for details." >> $OUTPUT_FILE
    exit 1
fi

echo -e "\n2. CLI Encoding Test" >> $OUTPUT_FILE
echo "Original Text: '$ORIGINAL_TEXT'" >> $OUTPUT_FILE

ENCODED=$(cargo run --quiet -- -enc="$ORIGINAL_TEXT")
echo "Encoded (Binary): $ENCODED" >> $OUTPUT_FILE

echo -e "\n3. CLI Decoding Test (No Errors)" >> $OUTPUT_FILE

DECODED=$(cargo run --quiet -- -dec="$ENCODED")
echo "Decoded Text: '$DECODED'" >> $OUTPUT_FILE

echo -e "\n4. Error Correction Demonstration" >> $OUTPUT_FILE
CORRUPTED_BIN="011001001110 010000101100 011001000000 011001100000 011001110110 001010000010 011110110100 011001111110 011110011001 011001100000 011000101011"


echo "Corrupted Binary input: $CORRUPTED_BIN" >> $OUTPUT_FILE

CORRECTED=$(cargo run --quiet -- -dec="$CORRUPTED_BIN")
echo "Decoded Text from Corrupted: '$CORRECTED'" >> $OUTPUT_FILE

echo "Experiments finished. Results saved to $OUTPUT_FILE."
cat $OUTPUT_FILE