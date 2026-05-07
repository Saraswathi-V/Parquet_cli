# Parquet Compactor CLI

A beginner-friendly Python command-line tool that solves the small file problem by compacting many small Parquet files into fewer larger Parquet files.

## Problem

In real data platforms, it is common to generate thousands of small Parquet files. This creates performance issues because query engines spend too much time opening files and reading metadata.

This is called the small file problem.

## Solution

This CLI reads small Parquet files from an input directory and writes fewer compacted Parquet files to an output directory based on a target size such as 128MB or 256MB.

## Features

- Read Parquet files from a directory
- Compact small files into larger files
- Preserve schema
- Configurable target file size
- Dry-run mode
- Verbose logging
- Summary report
- Beginner-friendly Python code
- Pytest test coverage

## Tech Stack

- Python
- PyArrow
- Typer
- pathlib
- logging
- pytest

## Project Structure

```text
parquet-compactor-cli/
│
├── README.md
├── pyproject.toml
├── requirements.txt
│
├── scripts/
│   └── generate_sample_data.py
│
├── src/
│   └── pcompact/
│       ├── __init__.py
│       ├── cli.py
│       ├── compactor.py
│       ├── file_utils.py
│       ├── report.py
│       └── config.py
│
├── tests/
│   ├── test_compactor.py
│   └── test_file_utils.py
│
└── sample_data/
    ├── small_files/
    └── compacted/