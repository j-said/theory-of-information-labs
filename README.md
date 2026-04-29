# Theory of Information Labs

This repository contains a set of Rust labs for the Information Theory and Coding course. Each subdirectory is a separate Rust crate focused on one topic from the course, with a small CLI or demo program plus a helper script for repeatable experiments.

## Purpose

The goal of the repository is to collect lab-sized implementations of classic information theory algorithms in one place. Each crate can be built and run independently, and most folders include a shell script that reproduces the sample results stored in the corresponding `results.txt` file.

## Working With A Lab

Go into the crate you want to explore and use Cargo as usual:

```bash
cd reed-solomon-codes
cargo run -- --help
```

If the folder provides a test script, make it executable and run it from inside that crate:

```bash
chmod +x run_tests.sh
./run_tests.sh
```

---

*Completed as part of the Information Theory and Coding course.*
