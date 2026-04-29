# Reed-Solomon Codes (Lab 5)

Console utility in Rust for demonstrating Reed-Solomon encoding and error correction over GF(2^8).

## Features

* Encode plain text into a Reed-Solomon protected codeword.
* Decode hexadecimal codewords back into the original message.
* Correct recoverable symbol errors using syndrome computation, Berlekamp-Massey, Chien search, and Forney-style correction.
* Print verbose intermediate values for polynomial division and error correction steps.

## Quick start

Example encoding command:

```bash
cargo run -- --enc "foo bar" --ecc 6
```

Example decoding command:

```bash
cargo run -- --dec "66 6F 6F 20 62 61 72 1A 2B 3C 4D 5E 6F" --ecc 6
```

## Testing

Run the sample experiments and save the output to `results.txt`:

```bash
chmod +x run_tests.sh
./run_tests.sh
```

---

*Completed as part of the Information Theory and Coding course.*
