# Product Overview

## What is Mago?

Mago is a comprehensive toolchain for PHP that brings the modern development experience of Rust's ecosystem to PHP projects. Inspired by tools like Clippy, OXC, and other Rust-based development tools, Mago provides a unified CLI and library interface for analyzing, linting, formatting, and understanding PHP code.

## Core Features

### Analysis & Linting
- **Advanced static analysis** with semantic understanding of PHP code
- **Customizable rule sets** with plugin architecture
- **Multi-plugin support** for frameworks like Symfony, Laravel, PHPUnit
- **Configurable severity levels** and rule customization
- **Performance metrics** including cyclomatic complexity and Halstead analysis

### Code Formatting
- **Consistent code style** enforcement across projects
- **Customizable formatting rules** with detailed configuration options
- **Fast formatting** with Rust-powered performance
- **Integration-ready** for CI/CD pipelines

### AST & Semantic Analysis  
- **Abstract Syntax Tree parsing** for deep code understanding
- **Semantic analysis** for type inference and correctness checking
- **Code structure visualization** and exploration tools
- **Reference tracking** and symbol resolution

### Developer Experience
- **Unified CLI interface** for all tools (lint, format, analyze, ast, find)
- **Detailed reporting** with helpful error messages and suggestions
- **Configuration flexibility** with workspace and project-level settings
- **Performance optimized** with multi-threading support

## Target Use Cases

### Individual Developers
- **Code quality improvement** through comprehensive linting
- **Consistent formatting** across personal projects  
- **Learning tool** for understanding PHP code structure through AST visualization
- **Migration assistance** for upgrading PHP versions with deprecation warnings

### Development Teams
- **Standardized code style** enforcement across team projects
- **Code review automation** with pre-commit hooks and CI integration
- **Technical debt analysis** through maintainability metrics
- **Framework-specific best practices** with plugin-based rules

### Large Codebases
- **Scalable analysis** with efficient multi-threaded processing
- **Workspace configuration** for mono-repo and multi-project setups
- **Selective analysis** with include/exclude patterns
- **Baseline support** for gradual adoption in legacy projects

### CI/CD Integration
- **Fast execution** suitable for continuous integration
- **Configurable exit codes** for pipeline integration  
- **Multiple output formats** for different reporting needs
- **Self-update capability** for automated tool management

## Key Value Propositions

### Performance & Reliability
- **Rust-powered speed** - significantly faster than traditional PHP tools
- **Memory efficient** processing of large codebases
- **Reliable parsing** with comprehensive PHP syntax support
- **Stable API** with semantic versioning

### Modern Development Experience
- **Unified tooling** - one tool instead of multiple separate tools
- **Rich configuration** with TOML-based settings
- **Excellent error reporting** with helpful suggestions and context
- **Active development** with regular updates and improvements

### Framework Integration
- **Plugin architecture** supporting popular PHP frameworks
- **Best practice enforcement** for Symfony, Laravel, PHPUnit workflows
- **Migration assistance** for PHP version upgrades
- **Extensible rule system** for custom organizational standards

### Community & Ecosystem
- **Open source** with dual MIT/Apache licensing
- **Community-driven** development with Discord support
- **Well-documented** with comprehensive guides and examples
- **Editor integration** support for modern development environments

## Replacement Strategy

Mago is designed to eventually replace or supplement traditional PHP tools:

- **PHP CS Fixer** → Mago Format (faster, more reliable formatting)
- **Psalm/PHPStan** → Mago Analyze (comprehensive semantic analysis)  
- **PHP_CodeSniffer** → Mago Lint (modern rule engine with plugins)
- **Custom scripts** → Mago Find (powerful code search and analysis)

While respecting and acknowledging the foundation these tools have built for the PHP community, Mago provides a unified, performance-optimized alternative with modern development experience.