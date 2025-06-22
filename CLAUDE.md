# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build and Test
- `just build` - Build the library in release mode
- `just build-wasm` - Build the WebAssembly module
- `just test` - Run all tests in the workspace
- `cargo test -p <crate-name>` - Run tests for a specific crate
- `cargo test --workspace --locked --all-targets` - Run all tests with locked dependencies

### Code Quality
- `just lint` - Check for linting problems using rustfmt, clippy, and cargo check
- `just fix` - Automatically fix linting problems using clippy, cargo fix, and rustfmt
- `cargo +nightly fmt --all -- --check --unstable-features` - Check formatting
- `cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings` - Run clippy

### Mago CLI Usage
- `cargo run -- init` - Initialize Mago configuration
- `cargo run -- lint <path>` - Lint PHP code
- `cargo run -- format <path>` - Format PHP code
- `cargo run -- ast <path>` - Analyze AST of PHP code
- `cargo run -- find <symbol>` - Find references to symbols

## Architecture Overview

### Workspace Structure
Mago is structured as a Rust workspace with multiple crates organized by functionality:

**Core Foundation:**
- `mago-syntax-core` - Core syntax abstractions and utilities
- `mago-syntax` - PHP lexer, parser, and AST implementation
- `mago-interner` - String interning for memory efficiency
- `mago-source` - Source code management and representation
- `mago-span` - Source code location tracking

**Analysis & Processing:**
- `mago-linter` - PHP linting engine with pluggable rules
- `mago-formatter` - Code formatting engine
- `mago-reflection` - Type and symbol reflection system
- `mago-names` - Name resolution and scoping
- `mago-reference` - Symbol reference tracking
- `mago-typing` - Type system and analysis
- `mago-project` - Project structure and configuration management

**Specialized Components:**
- `mago-type-syntax` - Type annotation parsing and representation
- `mago-docblock` - PHPDoc comment parsing
- `mago-composer` - Composer.json integration
- `mago-php-version` - PHP version handling and feature detection
- `mago-fixer` - Automated code fixes
- `mago-wasm` - WebAssembly bindings

**Utilities:**
- `mago-reporting` - Error reporting and diagnostics
- `mago-casing` - String case conversion utilities
- `mago-trinary` - Three-state logic utilities

### Data Flow
1. **Parsing Phase**: `mago-syntax` lexes and parses PHP source into AST
2. **Analysis Phase**: Various analyzers (linter, reflection, typing) process the AST
3. **Transformation Phase**: Formatter or fixer applies changes
4. **Reporting Phase**: Results are formatted and presented via `mago-reporting`

### Configuration System
- Configuration files are named `mago.toml`
- Configuration structure includes `[source]`, `[linter]`, `[formatter]` sections
- Rule-specific configuration uses `[[linter.rules]]` array syntax
- PHP version can be specified globally or overridden per command

### Linter Plugin Architecture
The linter uses a plugin-based architecture where:
- Plugins are organized by category (best-practices, safety, strictness, doctrine-strict, etc.)
- Each plugin contains multiple rules that share common functionality or domain focus
- Rules can be individually configured with thresholds and severity levels
- Framework-specific plugins available (symfony, laravel, phpunit, doctrine-strict)
- Plugin naming follows kebab-case for compound names (e.g., `doctrine-strict`, `best-practices`)

### Threading Model
- Configurable thread count via `threads` setting or `--threads` CLI option
- Uses Tokio async runtime with either current_thread or multi_thread executor
- Stack size is configurable for deep AST traversal

## Important Implementation Details

### PHP Version Support
- Minimum supported version defined in `MINIMUM_PHP_VERSION`
- Maximum supported version defined in `MAXIMUM_PHP_VERSION`
- Version validation can be bypassed with `--allow-unsupported-php-version`
- PHP 8.5 pipe operator (`|>`) is implemented

### Memory Management
- String interning via `mago-interner` for memory efficiency
- Spans track source locations without storing duplicate text
- AST nodes use interned identifiers

### Error Handling
- Comprehensive error types in each crate
- Structured reporting via `mago-reporting`
- Integration with `codespan-reporting` for pretty error messages

### Testing Strategy
- Unit tests in each crate under `tests/` directories
- Integration tests for CLI commands
- Linter rules use the `rule_test!` macro with example-based testing
- Rule examples include both valid and invalid code snippets with expected behavior
- Formatter has extensive test cases in `crates/formatter/tests/cases/`
- Test cases follow before/after pattern with settings files

## Development Notes

### Rust Edition and Requirements
- Uses Rust edition 2024
- Minimum Rust version: 1.87.0
- Requires nightly Rust for some linting features

### Code Style
- Clippy lints enforced with warnings as errors
- `print_stdout`, `print_stderr`, `dbg_macro` are forbidden
- Uses workspace-level lint configuration
- Trailing whitespace is not allowed
- Files must end with a single newline

### Linter Rule Development
When creating new linter rules:
- Implement the `Rule` trait with `get_definition()` and `lint_node()` methods
- Use `RuleDefinition::enabled()` with appropriate severity level
- Provide comprehensive examples using `RuleUsageExample::valid()` and `RuleUsageExample::invalid()`
- Include clear descriptions and helpful error messages
- Prefer interface/inheritance-based detection over naming conventions
- Use `context.scope` to access current scope information
- Leverage `mago-reflection` for type hierarchy analysis

### Release Process
- Publishing order is critical due to crate dependencies
- `just publish` handles correct dependency order
- Version synchronization across workspace members

## Design Principles

### Type and Interface Detection
When implementing linter rules that need to determine class types or architectural layers (such as identifying Repository classes), follow this priority order:

1. **Interface Implementation (Primary)**: Prioritize checking if a class implements specific interfaces rather than relying on naming conventions
2. **Inheritance Hierarchy (Secondary)**: Check if a class extends known base classes or abstract classes
3. **Naming Conventions (Fallback)**: Use naming patterns as a last resort when interface/inheritance information is unavailable

**Rationale**: Interface-based detection is more reliable and follows SOLID principles, as it relies on actual contracts rather than naming conventions which can be inconsistent or misleading.

**Implementation Notes**:
- Use `mago-reflection` to access type hierarchy information
- Leverage the codebase reflection system to determine implemented interfaces
- Fall back to naming patterns only when reflection data is insufficient
- Document any naming-based heuristics clearly as fallback mechanisms