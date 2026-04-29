#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

OUTPUT_FILE="results.txt"
ORIGINAL_TEXT="foo bar"
ECC_SYMBOLS=6

cargo build --release

echo "Running Reed-Solomon Code Experiments..." > $OUTPUT_FILE
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

ENC_OUTPUT=$(./target/release/reed-solomon-codes --enc "$ORIGINAL_TEXT" --ecc $ECC_SYMBOLS)
echo "$ENC_OUTPUT" >> $OUTPUT_FILE

HEX_CODEWORD=$(echo "$ENC_OUTPUT" | grep "Encoded Codeword:" | sed 's/Encoded Codeword: //')

echo -e "\n3. CLI Decoding Test (No Errors)" >> $OUTPUT_FILE
./target/release/reed-solomon-codes --dec "$HEX_CODEWORD" --ecc $ECC_SYMBOLS --verbose >> $OUTPUT_FILE

echo -e "\n4. Error Correction Demonstration (Recoverable Errors)" >> $OUTPUT_FILE
CORRUPTED_RECOVERABLE="FF FF FF ${HEX_CODEWORD:8}"
echo "Corrupted Hex input: $CORRUPTED_RECOVERABLE" >> $OUTPUT_FILE
./target/release/reed-solomon-codes --dec "$CORRUPTED_RECOVERABLE" --ecc $ECC_SYMBOLS --verbose >> $OUTPUT_FILE

echo -e "\n5. Error Correction Demonstration (Unrecoverable Errors)" >> $OUTPUT_FILE
CORRUPTED_UNRECOVERABLE="FF FF FF FF ${HEX_CODEWORD:11}"
echo "Corrupted Hex input: $CORRUPTED_UNRECOVERABLE" >> $OUTPUT_FILE
set +e
./target/release/reed-solomon-codes --dec "$CORRUPTED_UNRECOVERABLE" --ecc $ECC_SYMBOLS --verbose >> $OUTPUT_FILE
set -e

echo "Experiments finished. Results saved to $OUTPUT_FILE."
cat $OUTPUT_FILE
