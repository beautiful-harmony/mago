# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Mago is a comprehensive PHP toolchain written in Rust, inspired by the Rust ecosystem. It provides linting, formatting, static analysis, AST parsing, and reference finding capabilities for PHP projects. The project follows a modular workspace architecture with multiple crates.

## Development Commands

### Building
- `just build` - Build the library in release mode
- `cargo build` - Build in debug mode
- `just build-wasm` - Build the WebAssembly module (in crates/wasm)

### Testing
- `just test` - Run all tests in the workspace
- `cargo test --workspace --locked --all-targets` - Equivalent test command
- `cargo test -p <crate-name>` - Run tests for a specific crate

### Linting and Formatting
- `just lint` - Run comprehensive linting (rustfmt, clippy, cargo check)
- `just fix` - Automatically fix linting issues
- `cargo +nightly fmt --all -- --unstable-features` - Format code
- `cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings` - Run clippy

### Cleaning
- `just clean` - Clean all build artifacts

## Architecture

### Workspace Structure
The project uses a Cargo workspace with crates organized under `/crates/`:

**Core Infrastructure:**
- `syntax-core` - Core parsing utilities and macros
- `syntax` - PHP syntax parsing, lexing, and AST generation
- `interner` - String interning for memory efficiency
- `source` - Source code handling and management
- `span` - Source location tracking
- `reporting` - Error and diagnostic reporting

**Analysis Components:**
- `analyzer` - Static analysis engine with type checking and flow analysis
- `codex` - Symbol table and metadata management
- `linter` - Pluggable linting system with rule engine
- `formatter` - Code formatting with configurable style options
- `reference` - Symbol reference finding and resolution
- `names` - Name resolution and scope handling
- `semantics` - Semantic analysis utilities

**Specialized Modules:**
- `type-syntax` - Type annotation parsing and handling
- `docblock` - PHPDoc comment parsing
- `php-version` - PHP version compatibility handling
- `composer` - Composer.json schema and dependency handling
- `fixer` - Automated code fixes
- `casing` - String case conversion utilities
- `algebra` - Logic and constraint solving
- `wasm` - WebAssembly bindings

### Main Application
The main CLI application is in `/src/` with:
- `commands/` - CLI command implementations (lint, format, analyze, ast, find, init, self-update)
- `config/` - Configuration management for different components
- `utils/` - Shared utilities (logging, progress, version management)

### Key Design Patterns
- Modular crate architecture with clear separation of concerns
- Async/await with Tokio runtime for concurrent file processing
- Plugin system for linter rules and extensibility
- Configurable multi-threading support
- Comprehensive error handling with custom error types

## Configuration

### Project Configuration
- `mago.toml` - Main configuration file (see example in repository root)
- Supports PHP version specification, source paths, linter plugins, and rule customization
- Environment variables: `MAGO_PHP_VERSION`, `MAGO_THREADS`, `MAGO_ALLOW_UNSUPPORTED_PHP_VERSION`

### Development Requirements
- Rust 1.88.0+ (specified in Cargo.toml)
- Just task runner for development commands
- Nightly Rust toolchain for formatting and clippy

## Testing Strategy
- Unit tests within each crate
- Integration tests in `/tests/` directories
- Formatter has extensive test cases with before/after examples
- Property-based testing where applicable

## Performance Considerations
- Multi-threaded processing with configurable thread count
- String interning to reduce memory usage
- Optimized release builds with LTO and single codegen unit
- Custom allocator (mimalloc) on supported platforms

## PHP Compatibility
- Supports PHP 8.1+ (configurable minimum/maximum versions)
- Includes comprehensive PHP stubs in `/stubs/` directory
- Handles PHP-specific syntax and semantics accurately