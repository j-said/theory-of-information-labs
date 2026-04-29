# Optimal Coding (Lab 3)

## Overview
 Console utility in Rust for analyzing the effects Shannon-Fano and Huffman encoding.
 
## Features
* Calculate the unconditional entropy of a message source using Shannon's formula.
* Encode the data using the optimal Huffman code and Shannon-Fano code.
* Build and visualize the Huffman coding tree.
* Compute the average code length (L) and evaluate coding efficiency against the theoretical lower bound, ensuring the inequality L >= H is satisfied.

## Quick Start

Pass the path to a text you wanna test via CLI:
```bash
cargo run -- {path-to-the-text}
```

Or put it in the test/{file-name}.txt and run the provided shell script => generate a summary report:
```bash
chmod +x run_test.sh
./run_test.sh
```

---

*Completed as part of the Information Theory and Coding course.*
