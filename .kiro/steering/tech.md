# Technology Stack

## Architecture Overview

Mago is built as a Rust workspace with a modular architecture, featuring a main CLI binary and multiple specialized crates that provide different functionality. The architecture emphasizes performance, reliability, and extensibility.

## Core Technologies

### Language & Runtime
- **Rust 2024 Edition** (minimum version 1.88.0)
- **Tokio async runtime** for concurrent processing
- **Multi-threading support** with configurable worker threads
- **Memory optimization** with mimalloc allocator on macOS/Windows/musl

### CLI Framework
- **Clap 4.5+** for command-line interface and argument parsing
- **Structured commands** with subcommands (lint, format, analyze, ast, find, init, self-update)
- **Rich help text** with Unicode support and auto-wrapping
- **Self-update capability** with archive and compression support

### Configuration & Serialization
- **TOML configuration** files (mago.toml) with hierarchical settings
- **Serde** for serialization/deserialization
- **Config crate** for configuration management with environment variable support
- **JSON output** support for programmatic integration

### Performance & Concurrency
- **Multi-threaded execution** with configurable thread pools
- **Async file I/O** with tokio and async-walkdir
- **String interning** with lasso for memory efficiency
- **Optimized collections** with ahash and indexmap

### Parsing & Analysis
- **Custom PHP lexer and parser** built in Rust
- **AST representation** with visitor pattern support
- **Type system** with comprehensive PHP type modeling
- **Symbol table** management and reference resolution

## Development Environment

### Build System
- **Cargo workspace** with 20+ specialized crates
- **Just** task runner for common development workflows
- **Release optimization** with LTO, strip symbols, and single codegen unit
- **Cross-platform builds** for Linux, macOS, Windows

### Dependencies Management
- **Workspace-level dependency management** for version consistency
- **Feature flags** for optional functionality
- **Vendored OpenSSL** for static linking on Linux
- **WASM support** through wasm-bindgen for web integration

### Testing & Quality
- **Comprehensive test coverage** with workspace-wide testing
- **Nightly Rust toolchain** for linting (rustfmt, clippy)
- **Criterion benchmarking** for performance regression testing
- **Pretty assertions** for readable test failures

## Common Development Commands

### Building
```bash
# Development build
cargo build

# Release build (optimized)
just build
cargo build --release

# WebAssembly build
just build-wasm
```

### Testing & Quality Assurance
```bash
# Run all tests
just test
cargo test --workspace --locked --all-targets

# Linting (requires nightly)
just lint
cargo +nightly fmt --all -- --check --unstable-features
cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings

# Auto-fix issues
just fix
```

### Development Workflow
```bash
# List all available commands
just list

# Clean build artifacts
just clean
cargo clean --workspace

# Check code without building
cargo +nightly check --workspace --locked
```

## Environment Variables

### Build Configuration
- **`RUST_LOG`** - Controls Rust logging levels for debugging
- **`MAGO_LOG`** - Controls Mago-specific logging output
- **`CARGO_TARGET_DIR`** - Override default target directory for builds

### Runtime Configuration
- **PHP version validation** can be disabled via CLI flags
- **Thread count** configurable via CLI or configuration file
- **Stack size** adjustable for complex parsing scenarios

## Port Configuration

Mago is primarily a CLI tool and does not bind to network ports. However, for development:

- **Documentation server** - typically served on localhost:3000 when running docsify locally
- **WASM module** - can be integrated into web applications on any port
- **CI/CD integration** - runs in containerized environments without port requirements

## Performance Characteristics

### Optimization Settings
- **Release mode**: Full optimization with LTO enabled
- **Debug symbols**: Stripped in release builds for smaller binaries
- **Panic handling**: Abort on panic in release for better performance
- **Memory allocator**: Platform-specific optimized allocators

### Scalability Features
- **Multi-threading**: Scales with available CPU cores
- **Memory efficiency**: String interning and optimized data structures
- **Incremental processing**: Supports partial analysis of large codebases
- **Caching**: Intelligent caching of parsed ASTs and analysis results

## Integration Points

### External Tools
- **Composer integration** via composer.json parsing
- **Git integration** for file discovery and workspace detection
- **Editor support** through Language Server Protocol compatibility
- **CI/CD systems** via exit codes and structured output formats

### Extension Mechanisms
- **Plugin architecture** for custom linting rules
- **Configuration layering** for team and project-specific settings
- **Output formatters** for different reporting requirements
- **WASM bindings** for browser-based tooling integration