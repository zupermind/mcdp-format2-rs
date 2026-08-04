# mcdp-format2-rs

A Rust library for loading and saving datasets in the MCDP V2 format.

## Overview

`mcdp-format2-rs` is a Rust implementation for parsing and handling MCDP format version 2 files. 
It supports multiple serialization formats including JSON, YAML, and CBOR, with optional gzip compression.

## Features

- **Multiple Format Support**: JSON, YAML, CBOR with optional gzip compression
- **Pattern-based File Discovery**: Find MCDP files using glob patterns
- **Command-line Interface**: Built-in CLI tool for parsing and validation
- **Comprehensive Type System**: Strongly-typed data structures for MCDP components
- **Error Handling**: Robust error handling with detailed context

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mcdp-format2-rs = "2"
```

## Usage

### Library Usage

```rust
use mcdp_format2_rs::{read_mcdp_root, Root};
use std::path::Path;

// Load an MCDP file
let path = Path::new("example.mcdp2.json");
let root: Root = read_mcdp_root(path)?;
```

### Command-line Usage

The package includes a CLI tool `mcdp-format2-rs-load` for parsing and validating MCDP files:

```bash
# Parse all MCDP files in a directory
mcdp-format2-rs-load /path/to/mcdp/files

# Use a custom pattern
mcdp-format2-rs-load -p "*.mcdp2.json" /path/to/files

# Verbose output
mcdp-format2-rs-load -v /path/to/files

# Require a corpus of at least the expected size
mcdp-format2-rs-load --min-files 100 /path/to/files
```

Matching no file is a failure, not a success: a path that does not exist, an
unfetched Git-LFS payload, or a pattern that no longer matches the corpus
naming would otherwise report a clean exit having read nothing. `--min-files`
defaults to 1 for that reason, and callers who know how large the corpus should
be pass its expected size so that a corpus which shrank is caught too. Values
below 1 are rejected.

## Supported File Formats

The library automatically detects file format based on extension:

- `.json` - JSON format
- `.yaml`, `.yml` - YAML format  
- `.cbor` - CBOR format
- `.json.gz` - Gzipped JSON
- `.yaml.gz`, `.yml.gz` - Gzipped YAML
- `.cbor.gz` - Gzipped CBOR

