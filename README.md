# Project Setup and Usage

This project is written in Rust, so the first step is to install Rust on your machine.

## Install Rust

Follow the official instructions for installing Rust and Cargo from:

[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

After installation, verify it works by running:

```bash
rustc --version
cargo --version
```

## Run the benchmarks

From the project root, build and run the benchmark suite with:

```bash
cargo bench
```

To run the benchmarks with a specific number of samples per benchmarked function (defaults to 100):

```bash
DIVAN_SAMPLE_COUNT=10 cargo bench
```

To run a specific benchmark, use its name in bench filter:

```bash
cargo bench --bench powersort_bench|closest_pair_bench|karatsuba_bench
```

To transform the `divan` output to csv, pipe the output to the script `bench2csv.sh` (only works when running for a single algorithm):

```bash
cargo bench --bench powersort_bench|closest_pair_bench|karatsuba_bench | ./bench2csv.sh
```


