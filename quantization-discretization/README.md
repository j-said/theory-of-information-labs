# Image Discretization & Quantization (TI Lab 2)

Console utility in Rust for analyzing the effects of sampling and bit-depth reduction on image data, featuring entropy and Kullback-Leibler divergence metrics.

## Features

* **Discretization:** Resampling images with custom steps (e.g., 2, 4) to analyze spatial data loss.
* **Uniform Quantization:** Reducing luma dynamic range to 8, 16, or 64 levels to observe "false contouring" artifacts.
* **Information Metrics:** Computing Shannon Entropy for complexity and KL Divergence (Relative Entropy) for reconstruction quality.
* **Restoration:** Upscaling processed samples using Nearest Neighbor interpolation for visual comparison.

## Quick Start

Pass the path to a grayscale or color image (will be converted to luma) via CLI:

```bash
cargo run -- tests-images/linear-gradient.png
```

## Automation

Run the provided shell script to batch process all images in the test directory and generate a summary report:

```bash
chmod +x run_tests.sh
./run_tests.sh
```

---

*Completed as part of the Information Theory and Coding course.*
