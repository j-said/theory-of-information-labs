# Hamming Encoding

Console utility in Rust for demonstrating Hamming code encoding, decoding, and single-bit error correction.

## Features

* Encode text into Hamming-code binary blocks.
* Decode binary blocks back into text.
* Correct a single-bit error in each encoded block during decoding.

## Quick Start

Encode text from the command line:

```bash
cargo run -- -enc="hello world"
```

Decode binary blocks back to text:

```bash
cargo run -- -dec="011001001110 010000101100"
```

## Automation

Run the provided shell script to execute tests and sample CLI scenarios:

```bash
chmod +x run_tests.sh
./run_tests.sh
```

The script resolves its own location before running, so it keeps working even if the folder is moved.

---

*Completed as part of the Information Theory and Coding course*.
