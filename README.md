# ROM Patcher RS

A modern, modular ROM patcher written in Rust supporting multiple patch formats.

## Supported Formats

- **IPS** (International Patching System) - Implemented
- **BPS** (Beat Patching System) - Planned
- **UPS** (Universal Patching System) - Planned
- **APS** (Nintendo 64 APS Format) - Planned
- **RUP** (Rupture Patches) - Planned
- **PPF** (PlayStation Patch Format) - Planned
- **xdelta** (Generic binary diff) - Planned

## Features

- Apply patches to ROMs
- Create patches from ROM pairs
- Automatic format detection
- Patch validation
- CRC32/MD5/SHA checksums (planned)
- RetroAchievements hash checking (planned)

## Architecture

The project is organized as a Cargo workspace with 4 crates:

```
rom-patcher-rs/
├── crates/
│   ├── core/           # Core traits and types
│   ├── formats/        # Patch format implementations
│   ├── features/       # Extended features (validation, hash checking)
│   └── cli/            # Command-line interface
```

### Design Principles

1. **Modular** - Each format and feature is independently implemented
2. **Extensible** - Easy to add new formats via trait implementation
3. **Type-safe** - Leverages Rust's type system for safety
4. **Zero-copy** - Efficient memory usage with slice references

## Building

Requires Rust 1.91+ with 2024 edition support:

```bash
cargo build --release
```

The binary will be at `target/release/rompatch`.

## Usage

### Apply a patch

```bash
rompatch apply --rom game.smc --patch hack.ips --output game-patched.smc
```

### Create a patch

```bash
rompatch create --original game.smc --modified game-hacked.smc --output hack.ips
```

### Get patch info

```bash
rompatch info patch.ips
```

### Validate a patch

```bash
rompatch validate patch.ips
```

### Compute ROM hash

```bash
rompatch hash game.gba --algorithm crc32
```

## Development

### Prerequisites

Install development tools:

```bash
cargo install just cargo-watch cargo-audit cargo-outdated cargo-tarpaulin
```

### Common Tasks

```bash
just              # Show all available commands
just build        # Build in release mode
just test         # Run all tests
just clippy       # Run linter (warnings as errors)
just fmt          # Format code
just bench        # Run benchmarks
just ci           # Run all CI checks
just doc          # Generate and open documentation
```

### Adding a New Format

1. Create a new module in `crates/formats/src/`
2. Implement the `PatchFormat` trait
3. Add format detection in `detect_format()`
4. Add CLI support in `crates/cli/src/main.rs`
5. Add tests and benchmarks

### Testing

```bash
just test                 # Run all tests
just watch-test           # Watch and run tests on changes
cargo test --all-features # Direct cargo command
```

## Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history and release notes
- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed design documentation
- [docs/API.md](docs/API.md) - Complete API reference
- [docs/CLI_USAGE.md](docs/CLI_USAGE.md) - CLI usage guide
- [examples/](examples/) - Code examples

## License

MIT OR Apache-2.0
