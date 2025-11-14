# ROM Patcher RS Examples

This directory contains practical examples of using rompatcherrs as a library.

## Examples

### Basic Usage
- **`basic_apply.rs`** - Simple patch application
- **`with_validation.rs`** - Apply patch with pre-validation and checksums
- **`batch_patching.rs`** - Apply patches to multiple ROMs

### Advanced Usage
- **`format_detection.rs`** - Auto-detect patch format
- **`metadata_extraction.rs`** - Extract and display patch metadata
- **`custom_error_handling.rs`** - Custom error handling patterns

## Running Examples

```bash
# Run a specific example
cargo run --example basic_apply

# Run with release optimizations
cargo run --release --example with_validation

# List all examples
cargo run --example
```

## Requirements

Each example is self-contained and demonstrates a specific use case. You'll need:
- Rust 1.91 or newer
- A ROM file and corresponding patch file for testing

**Note**: Examples use placeholder paths. Update them to point to your actual ROM and patch files.
