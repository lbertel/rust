# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Build & Development Commands

### Core Cargo Commands
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the main binary
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo test --lib` - Run only library tests
- `cargo test --bin <binary_name>` - Run tests for a specific binary
- `cargo check` - Fast syntax/type checking without full compilation
- `cargo clippy` - Run the Rust linter for code quality suggestions
- `cargo fmt` - Format code using rustfmt

### Development Workflow
- `cargo watch -x check` - Continuously check code on file changes (requires cargo-watch)
- `cargo watch -x test` - Continuously run tests on file changes
- `cargo doc --open` - Generate and open documentation
- `cargo clean` - Clean build artifacts from target directory

### Package Management
- `cargo add <crate_name>` - Add a dependency
- `cargo update` - Update dependencies to latest compatible versions
- `cargo tree` - Show dependency tree
- `cargo outdated` - Check for outdated dependencies (requires cargo-outdated)

## Project Structure

This is a Rust workspace that follows standard Cargo conventions:

### Standard Rust Project Layout
- `Cargo.toml` - Project manifest with metadata and dependencies
- `Cargo.lock` - Lock file with exact dependency versions (should be committed)
- `src/` - Source code directory
  - `src/main.rs` - Main binary entry point
  - `src/lib.rs` - Library root (if this is a library crate)
  - `src/bin/` - Additional binary targets
  - `src/examples/` - Example code
- `tests/` - Integration tests
- `benches/` - Benchmark tests
- `target/` - Build artifacts (ignored by git)

### Key Configuration Files
- `.gitignore` - Contains standard Rust exclusions for build artifacts and IDE files
- `rust-toolchain.toml` - Rust version specification (if present)
- `.cargo/config.toml` - Cargo configuration (if present)

## Rust-Specific Development Notes

### Testing Strategy
- Unit tests: Place `#[cfg(test)]` modules in source files
- Integration tests: Place in `tests/` directory as separate files
- Documentation tests: Include examples in doc comments with `/// # Examples`
- Use `cargo test -- --nocapture` to see println! output during tests

### Code Organization Patterns
- Use `mod` declarations to organize code into modules
- `pub use` statements in `lib.rs` to re-export important items
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Prefer composition over inheritance
- Use `Result<T, E>` for error handling rather than exceptions

### Performance Considerations
- Use `cargo build --release` for performance testing
- Profile with `cargo flamegraph` or similar tools
- Consider `cargo bench` for micro-benchmarks
- Use `cargo bloat` to analyze binary size

### Common Development Tools
- `rustup` for managing Rust toolchains
- `cargo-watch` for continuous compilation
- `cargo-edit` for managing dependencies from command line
- `cargo-audit` for security vulnerability scanning
- `cargo-outdated` for dependency updates

## Environment Setup

### Required Tools
- Rust toolchain (managed via rustup)
- Cargo (comes with Rust)

### Recommended Tools
```bash
cargo install cargo-watch cargo-edit cargo-audit cargo-outdated cargo-bloat
```

### IDE Integration
- Language Server Protocol support via rust-analyzer
- Formatting on save with rustfmt
- Linting integration with clippy
