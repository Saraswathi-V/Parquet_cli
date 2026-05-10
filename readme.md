# Parquet Compactor CLI - Rust
---

## Project Introduction

The **Parquet Compactor CLI** is a Rust data engineering project designed to solve a common real-world problem called the **small file problem**.

In modern data platforms, data is often stored in **Parquet format** because Parquet is efficient for analytics, supports compression, and stores schema information. However, data pipelines can sometimes generate hundreds or thousands of very small Parquet files.

When query engines such as Spark, Athena, Trino, or Presto read these files, they need to open each file, read its metadata, check its schema, and then scan the data. If there are too many small files, query performance becomes poor.

This project builds a Rust CLI tool called `pcompact` that reads many small Parquet files from an input directory and writes fewer compacted Parquet files into an output directory.

In simple words:

```text
Many small Parquet files go in.
Fewer optimized Parquet files come out.
```

---

## Problem Statement

In real data engineering systems, the **small file problem** happens when a pipeline creates too many tiny files instead of fewer optimized files.

This can happen because of:

- Streaming data ingestion
- Frequent batch jobs
- Partitioned data writes
- Multiple pipeline runs
- Continuous appends to a data lake

Having many small files causes:

- Slow query performance
- High metadata overhead
- More cloud storage requests
- Poor Spark performance
- Inefficient scans
- Increased operational cost

Example input folder:

```text
small_files/
├── small_file_001.parquet
├── small_file_002.parquet
├── small_file_003.parquet
├── small_file_004.parquet
├── ...
└── small_file_100.parquet
```

Even though each file is valid, the large number of files makes querying inefficient.

---

## Project Solution

The solution is to compact many small Parquet files into fewer larger Parquet files.

The CLI command looks like this:

```bash
pcompact -i ./small_files -o ./compacted --target-size 128MB
```

Another example:

```bash
pcompact -i ./small_files -o ./compacted --target-size 256MB --verbose
```

The tool reads files from the input folder, groups them based on the target size, and writes compacted files into the output folder.

The final result is fewer Parquet files with the same rows and same schema.

---

## What is Parquet?

Parquet is a **columnar file format** widely used in data engineering and analytics.

It is useful because:

- It stores data column by column
- It supports compression
- It stores schema information
- It works well with big data tools
- It is efficient for analytical queries

Example columns in a Parquet dataset:

```text
transaction_id
customer_id
city
status
product_category
amount
transaction_notes
```

Parquet is commonly used in data lakes because it is optimized for reading only the columns needed by a query.

For example, if a query only needs the `amount` column, the query engine does not need to read the full dataset. It can read only the required column. This makes Parquet very useful for analytics workloads.

---

## What is File Compaction?

File compaction is the process of combining many small files into fewer larger files.

Example:

```text
Before compaction:
100 small Parquet files

After compaction:
1 or a few compacted Parquet files
```

The main goal of compaction is to reduce the **number of files**.

Compaction does not always reduce total storage size. If the input files are already compressed, the total input size and total output size may look similar.

However, compaction is still successful if the file count decreases.

---

## Why Rust?

This project is implemented in **Rust** because Rust provides:

- Strong performance
- Memory safety
- Fast file processing
- Reliable command-line tooling
- Cargo-based project management
- Strong type checking
- Good support for production-style systems programming

Rust is a good choice for building command-line data engineering tools because it can handle file processing efficiently while avoiding many common memory-related errors.

---

## Features

This project supports:

- Directory-based input
- Directory-based output
- Target output file size
- Schema preservation
- Parquet read/write using Rust crates
- Dry-run mode
- Verbose logging
- Summary report
- Rust tests using `cargo test`
- Professional Rust project structure

---

## Technology Stack

This project uses:

- Rust
- Cargo
- clap
- Apache Arrow Rust
- Parquet Rust
- walkdir
- tracing
- anyhow
- cargo test

### Rust

Rust is used as the main programming language.

### Cargo

Cargo is used to build, run, test, and manage the Rust project.

### clap

`clap` is used to build the command-line interface.

### Apache Arrow Rust

Apache Arrow Rust is used for Arrow record batch handling.

### Parquet Rust

The Rust Parquet crate is used to read and write Parquet files.

### walkdir

`walkdir` is used to recursively find Parquet files inside the input directory.

### tracing

`tracing` is used for normal and verbose logging.

### anyhow

`anyhow` is used for error handling.

---

## Project Structure

```text
parquet-compactor-rs/
│
├── Cargo.toml
├── README.md
├── .gitignore
│
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── file_utils.rs
│   ├── compactor.rs
│   ├── report.rs
│   │
│   └── bin/
│       └── generate_sample_data.rs
│
├── tests/
│   └── file_utils_test.rs
│
└── sample_data/
    ├── small_files/
    └── compacted/
```

---

## Explanation of Important Files

### `Cargo.toml`

This file defines the Rust package, binaries, and dependencies.

It includes dependencies such as:

- `clap`
- `parquet`
- `arrow-array`
- `arrow-schema`
- `walkdir`
- `tracing`
- `anyhow`
- `rand`

It also defines two binaries:

```text
pcompact
generate_sample_data
```

---

### `src/main.rs`

This is the main entry point of the CLI application.

It:

- Parses CLI arguments
- Sets up logging
- Creates the configuration object
- Calls the compaction function
- Prints the final report

---

### `src/lib.rs`

This file exposes project modules for use by the main binary and tests.

It includes:

```rust
pub mod cli;
pub mod compactor;
pub mod config;
pub mod file_utils;
pub mod report;
```

---

### `src/cli.rs`

This file handles command-line arguments using `clap`.

It supports:

```text
-i
-o
--target-size
--dry-run
--verbose
```

Example command:

```bash
pcompact -i ./small_files -o ./compacted --target-size 128MB
```

---

### `src/config.rs`

This file stores configuration values such as:

- Input directory
- Output directory
- Target size in bytes
- Dry-run option
- Verbose option

It converts CLI arguments into a configuration object used by the rest of the program.

---

### `src/file_utils.rs`

This file contains helper functions for:

- Finding Parquet files
- Calculating file sizes
- Parsing target sizes like `128MB`
- Creating output folders
- Cleaning old Parquet files
- Showing file sizes in readable format

---

### `src/compactor.rs`

This is the main logic file.

It handles:

- Finding input files
- Grouping files by target size
- Reading Parquet files
- Checking schema consistency
- Writing compacted Parquet files
- Counting processed rows
- Returning the final report

---

### `src/report.rs`

This file prints the final summary report.

The report includes:

- Input files
- Output files
- Total input size
- Total output size
- Target output file size
- Total rows processed
- File compaction ratio

---

### `src/bin/generate_sample_data.rs`

This file generates sample Parquet files for testing and demonstration.

It creates many small Parquet files inside:

```text
sample_data/small_files
```

These files simulate small files created by a real data pipeline.

---

### `tests/file_utils_test.rs`

This file contains Rust tests.

The tests verify:

- Target size parsing
- Human-readable file size conversion

Tests can be run using:

```bash
cargo test
```

---

## How the Tool Works

The tool works in this flow:

```text
User runs CLI command
        ↓
Tool reads input directory
        ↓
Tool finds all Parquet files
        ↓
Tool calculates file sizes
        ↓
Tool groups files based on target size
        ↓
Tool reads Parquet data using Rust Parquet APIs
        ↓
Tool validates schema
        ↓
Tool writes compacted Parquet output files
        ↓
Tool prints summary report
```

The user runs the command from the terminal. The tool reads the input directory and finds all Parquet files. After that, it calculates the size of each file and groups files together based on the target size.

For example, if the target size is `128MB`, the tool groups files until the group is close to that size. Then it reads the Parquet data, validates the schema, writes compacted output files, and prints a final summary report.

---

## Installation and Setup

Make sure Rust and Cargo are installed.

Check Rust installation:

```bash
rustc --version
```

Check Cargo installation:

```bash
cargo --version
```

Clone or open the project folder:

```bash
cd parquet-compactor-rs
```

Build the project:

```bash
cargo build
```

---

## Generate Sample Data

Run:

```bash
cargo run --bin generate_sample_data
```

This creates sample Parquet files inside:

```text
sample_data/small_files
```

These files are used as input for the compactor.

---

## Demo Workflow

Generate sample Parquet files:

```bash
cargo run --bin generate_sample_data
```

Run dry-run mode:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB --dry-run
```

Run actual compaction:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB
```

Run with verbose mode:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 256MB --verbose
```

---

## Example Usage

Basic usage:

```bash
pcompact -i ./small_files -o ./compacted --target-size 128MB
```

Verbose usage:

```bash
pcompact -i ./small_files -o ./compacted --target-size 256MB --verbose
```

Project demo usage with Cargo:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB
```

Dry-run usage with Cargo:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB --dry-run
```

---

## Build Release Executable

To create a release build:

```bash
cargo build --release
```

After building, the executable will be available inside:

```text
target/release/
```

On Windows, run:

```bash
target\release\pcompact.exe -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB --verbose
```

On macOS/Linux, run:

```bash
./target/release/pcompact -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB --verbose
```

---

## Sample Output

Example output after running compaction:

```text
INFO Found 100 Parquet files
INFO Total input size: 223097170 bytes
INFO Planned output files: 2
INFO Writing output file sample_data\compacted\part-00001.parquet using 60 input files
INFO Writing output file sample_data\compacted\part-00002.parquet using 40 input files

============================================================
Parquet Compactor Summary - COMPACTION COMPLETED
============================================================
Input files              : 100
Output files             : 2
Total input size         : 212.76 MB
Total output size        : 55.28 MB
Target output file size  : 128.00 MB
Total rows processed     : 500000
File compaction ratio    : 50.00x
============================================================
Output files:
  - sample_data\compacted\part-00001.parquet
  - sample_data\compacted\part-00002.parquet
```

---

## Output Explanation

The summary report explains what happened during compaction.

### Input files

```text
Input files : 100
```

This means the tool found **100 small Parquet files** in the input folder.

### Output files

```text
Output files : 2
```

This means the tool compacted those 100 small files into **2 optimized Parquet output files**.

### Total input size

```text
Total input size : 212.76 MB
```

This shows the total size of all input Parquet files before compaction.

### Total output size

```text
Total output size : 55.28 MB
```

This shows the total size of the compacted output files after compaction.

In this demo, the output size is smaller because the input files were generated in a less optimized form, and the compactor wrote optimized compressed Parquet output files.

### Target output file size

```text
Target output file size : 128.00 MB
```

This is the target file size given by the user.

The target size is used to decide how files should be grouped during compaction.

### Total rows processed

```text
Total rows processed : 500000
```

This means the tool processed **500,000 rows**.

This confirms that the data was preserved during compaction.

### File compaction ratio

```text
File compaction ratio : 50.00x
```

This means:

```text
100 input files / 2 output files = 50x compaction ratio
```

So, the tool successfully reduced 100 small Parquet files into 2 compacted Parquet files.

---

## Main Project Outcome

The main outcome of this project is **file count reduction** and **optimized Parquet output generation**.

Example from the final demo:

```text
Before compaction:
100 small Parquet files

After compaction:
2 compacted Parquet files
```

The project also reduced the total storage size in this demo:

```text
Before compaction:
212.76 MB

After compaction:
55.28 MB
```

The output still preserves:

- Same rows
- Same schema
- Same columns
- Same data types

This proves that the tool successfully compacted the files without losing data.

---

## Dry-Run Mode

Dry-run mode shows the compaction plan without creating output files.

Command:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 128MB --dry-run
```

Dry-run mode is useful because it allows users to preview the output before writing files.

Example dry-run output:

```text
============================================================
Parquet Compactor Summary - DRY RUN
============================================================
Input files              : 100
Output files             : 2
Total input size         : 212.76 MB
Total output size        : 0 B
Target output file size  : 128.00 MB
Total rows processed     : 0
File compaction ratio    : 50.00x
============================================================
```

---

## Verbose Mode

Verbose mode shows detailed logs during execution.

Command:

```bash
cargo run --bin pcompact -- -i ./sample_data/small_files -o ./sample_data/compacted --target-size 256MB --verbose
```

Verbose mode helps users understand what the tool is doing internally.

Example logs:

```text
INFO Found 100 Parquet files
INFO Total input size: 223097170 bytes
INFO Planned output files: 2
INFO Writing output file sample_data\compacted\part-00001.parquet using 60 input files
INFO Writing output file sample_data\compacted\part-00002.parquet using 40 input files
```

---

## Schema Preservation

The tool validates schema before writing compacted output files.

When the first Parquet file is read, its schema is stored.

Every other input file is compared against that schema.

If the schema matches, compaction continues.

If the schema does not match, the tool raises an error.

This prevents files with different structures from being incorrectly merged together.

---

## Testing

Run tests using:

```bash
cargo test
```

The tests check:

- Target size parsing
- Human-readable file size conversion

Example expected test output:

```text
running 4 tests
test test_parse_size_mb ... ok
test test_parse_size_gb ... ok
test test_parse_size_invalid ... ok
test test_human_readable_size_mb ... ok

test result: ok. 4 passed; 0 failed
```

Testing confirms that the project works correctly and that important helper functions behave as expected.

---

## Project Outcome

The final outcome of this project is a working Rust CLI tool that solves the small file problem.

The tool successfully:

- Reads many small Parquet files
- Groups files based on target size
- Preserves schema
- Writes compacted output files
- Reduces file count
- Supports dry-run mode
- Supports verbose logging
- Prints a useful summary report
- Includes Rust tests

In the final demo, the project converted this:

```text
100 small Parquet files
```

into this:

```text
2 compacted Parquet files
```

while processing **500,000 rows** and preserving the same schema.

---

## Real-World Use Case

This tool can be useful in real data lake environments.

For example, a company may store sales transactions, log data, or customer activity data in cloud storage such as Amazon S3.

If the pipeline writes small files frequently, query performance becomes slower.

A compactor can be scheduled to run periodically and merge those small files into larger optimized files.

This can improve performance for:

- Spark
- Athena
- Trino
- Presto
- Other analytics engines

It can also reduce metadata overhead and cloud storage request costs.

---

## Challenges Faced

During the project, a few challenges were identified.

One challenge was understanding the difference between reducing file count and reducing file size.

At first, it may look like compaction should always reduce total storage size. However, Parquet files are already compressed and optimized, so compaction does not always reduce storage size.

The main goal of compaction is to reduce file count.

Another challenge was choosing the correct target size.

If the target size is too small, files may not combine properly.

If the target size is larger than the total input size, all input files may become one output file.

Therefore, target size should be selected based on the total input size and average file size.

Another challenge was implementing Parquet reading and writing in Rust, because Rust requires stronger type handling and more explicit file processing compared to scripting languages.

---

## What We Learned

Through this project, we learned:

- Rust project structure
- Cargo build and run workflow
- Small file problem
- Parquet file handling in Rust
- File compaction
- Apache Arrow Rust usage
- Parquet Rust crate usage
- CLI development using clap
- Schema preservation
- Logging using tracing
- Dry-run mode
- Rust testing using cargo test
- Professional Rust project structure

This project helped us understand how real data engineering systems optimize files for better query performance.

---

## Future Improvements

Future improvements include:

- Partition-aware compaction
- Amazon S3 support
- Compression options such as Snappy and ZSTD
- Row group tuning
- Metadata-aware planning
- Parallel compaction using Rust threads
- Schema evolution support
- Output validation report

Partition-aware compaction would be especially useful for real data lakes.

Example partitioned folder structure:

```text
year=2026/month=05/day=06
```

In a future version, the tool can compact files separately inside each partition.

---

## Final Conclusion

The **Parquet Compactor CLI** successfully solves the small file problem by compacting many small Parquet files into fewer optimized files.

The project supports:

- Rust-based CLI implementation
- Input and output directories
- Target file size
- Dry-run mode
- Verbose logging
- Schema validation
- Summary reporting
- Rust testing

The final outcome is a working Rust data engineering CLI tool that reduced **100 small Parquet files into 2 compacted Parquet files**, processed **500,000 rows**, and reduced the demo dataset size from **212.76 MB to 55.28 MB** while preserving the same data and schema.

---

## Short Summary

This project is called **Parquet Compactor CLI - Rust**.

It solves the small file problem in data engineering by compacting many small Parquet files into fewer larger files.

The tool reads Parquet files from an input directory, groups them based on a target size such as `128MB` or `256MB`, and writes compacted output files to an output directory.

The project uses Rust, Cargo, clap, Apache Arrow Rust, Parquet Rust, walkdir, tracing, and cargo test.

The tool supports dry-run mode, verbose logging, schema validation, and summary reporting.

