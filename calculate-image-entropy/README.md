# Entropy & Compression (TI Lab 1)

Console utility in Rust for analyzing luma image entropy and data compression using Huffman encoding + demo of arithmetic coding for small data slices.

## Features

* Reading images (PNG, JPEG, BMP) via CLI arguments.
* Building histograms of pixel luminance distribution.
* Computing Shannon information entropy.
* Generating Huffman codes and calculating encoding efficiency.

Run the program by passing the path to an image:

```bash
cargo run -- relative/path/to/image.png [block_size]
```

---

*Completed as part of the Information Theory and Coding course.*
