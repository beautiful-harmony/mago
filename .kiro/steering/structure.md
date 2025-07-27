# Project Structure

## Root Directory Organization

```
mago/
├── src/                     # Main CLI binary source code
├── crates/                  # Workspace crates (libraries)
├── docs/                    # Documentation website
├── examples/                # Example configurations and PHP code  
├── stubs/                   # PHP extension stubs for analysis
├── scripts/                 # Installation and utility scripts
├── composer/                # Composer plugin integration
├── target/                  # Build artifacts (generated)
├── Cargo.toml              # Root workspace configuration
├── mago.toml               # Mago tool configuration
└── Justfile                # Task runner configuration
```

## Main Binary Structure (`src/`)

```
src/
├── main.rs                 # Application entry point and runtime setup
├── commands/               # CLI command implementations
│   ├── mod.rs              # Command module exports
│   ├── analyze.rs          # Semantic analysis command
│   ├── ast.rs              # AST parsing and visualization
│   ├── format.rs           # Code formatting command
│   ├── lint.rs             # Linting command
│   ├── find.rs             # Code search command
│   ├── init.rs             # Project initialization
│   ├── self_update.rs      # Tool self-update functionality
│   └── args/               # Command-line argument definitions
├── config/                 # Configuration management
│   ├── mod.rs              # Configuration module
│   ├── analyzer.rs         # Analyzer-specific configuration
│   ├── formatter.rs        # Formatter settings
│   ├── linter.rs           # Linter configuration
│   └── source.rs           # Source file discovery settings
├── utils/                  # Utility functions
│   ├── logger.rs           # Logging initialization
│   ├── progress.rs         # Progress reporting
│   └── version.rs          # Version management
├── baseline/               # Baseline management for gradual adoption
├── consts.rs               # Application constants
├── error.rs                # Error type definitions
├── metadata.rs             # Project metadata handling
└── source.rs               # Source file discovery and filtering
```

## Workspace Crates (`crates/`)

### Core Language Processing
- **`syntax-core/`** - Fundamental syntax processing utilities
- **`syntax/`** - PHP lexer, parser, and AST definitions
- **`type-syntax/`** - Type annotation parsing and representation
- **`docblock/`** - PHPDoc comment parsing and analysis

### Analysis Engine
- **`analyzer/`** - Semantic analysis and type inference
- **`codex/`** - Symbol table and metadata management
- **`semantics/`** - High-level semantic operations
- **`reference/`** - Symbol reference tracking and resolution
- **`names/`** - Name resolution and scope management

### Tooling Components
- **`linter/`** - Linting rules and plugin system
- **`formatter/`** - Code formatting engine
- **`fixer/`** - Automated code fixing capabilities
- **`reporting/`** - Error reporting and output formatting

### Utility Crates
- **`interner/`** - String interning for memory efficiency
- **`source/`** - Source file handling and representation
- **`span/`** - Source location tracking
- **`casing/`** - String case conversion utilities
- **`php-version/`** - PHP version handling and feature detection

### Integration Crates
- **`composer/`** - Composer.json parsing and integration
- **`wasm/`** - WebAssembly bindings for browser integration
- **`algebra/`** - Mathematical utilities for analysis algorithms

## Code Organization Patterns

### Module Structure Convention
Each crate follows a consistent internal organization:
```
crate/
├── src/
│   ├── lib.rs              # Public API and re-exports
│   ├── internal/           # Internal implementation details
│   │   ├── mod.rs          # Internal module organization
│   │   └── *.rs            # Implementation files
│   ├── error.rs            # Crate-specific error types
│   └── *.rs                # Public modules
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks (optional)
└── Cargo.toml             # Crate-specific dependencies
```

### AST Organization Pattern
The syntax crate organizes AST nodes by PHP language construct:
```
syntax/src/ast/ast/
├── expression.rs           # Expression nodes
├── statement.rs            # Statement nodes
├── class_like/             # Classes, interfaces, traits, enums
├── function_like/          # Functions, methods, closures
├── control_flow/           # If, switch, match statements  
├── loop/                   # For, foreach, while loops
└── ...                     # Other language constructs
```

### Linter Plugin Organization
```
linter/src/plugin/
├── best_practices/         # General best practice rules
├── consistency/            # Code style consistency rules
├── maintainability/        # Code maintainability metrics
├── security/               # Security-focused analysis
├── strictness/             # Type strictness enforcement
├── symfony/                # Symfony framework rules
├── laravel/                # Laravel framework rules
└── phpunit/                # PHPUnit testing rules
```

## File Naming Conventions

### Rust Conventions
- **Module files**: `snake_case.rs` 
- **Test files**: `mod.rs` in `tests/` directories
- **Benchmark files**: `*.rs` in `benches/` directories
- **Library entry points**: `lib.rs`
- **Binary entry points**: `main.rs`

### Configuration Files
- **Tool configuration**: `mago.toml`
- **Workspace settings**: `Cargo.toml` (root and per-crate)
- **Task definitions**: `Justfile`
- **Development settings**: `.gitignore`, `.github/`

### Documentation
- **Crate documentation**: `README.md` per crate
- **Project documentation**: `docs/` directory with docsify
- **Code examples**: `examples/` with working sample code

## Import Organization

### Standard Import Order
1. **Standard library** imports (`std::*`)
2. **External crates** (workspace and third-party)
3. **Internal crate modules** (relative imports)
4. **Super/self imports** when needed

### Example Import Pattern
```rust
use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::error::Error;
use crate::internal::parser::Parser;

use super::utils::normalize_path;
```

## Key Architectural Principles

### Separation of Concerns
- **Parsing** (syntax crates) is separate from **analysis** (analyzer, semantics)
- **Configuration** management is centralized but modular
- **CLI commands** are thin wrappers around library functionality
- **Error handling** is explicit and typed per domain

### Performance-First Design
- **Zero-copy parsing** where possible with string interning
- **Incremental analysis** capabilities for large codebases
- **Multi-threading** at appropriate abstraction levels
- **Memory efficiency** through careful data structure choices

### Extensibility
- **Plugin architecture** for linting rules
- **Visitor pattern** for AST traversal
- **Configuration layering** for flexible customization
- **Stable APIs** with semantic versioning

### Testing Strategy
- **Unit tests** within each crate for isolated functionality
- **Integration tests** in `tests/` directories for cross-crate behavior
- **Benchmark tests** for performance regression detection
- **Example-driven testing** with real-world PHP code samples