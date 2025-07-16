# AGENTS.md - Development Guidelines for daqhats-rs

## Build Commands
- `cargo build` - Build the project
- `cargo run` - Run the main binary
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo check` - Check code without building
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

## Project Structure
This is a Rust FFI project that provides bindings for the MCC DAQ HAT Library (C library in `lib/` and `include/`). The build system uses bindgen to generate Rust bindings from C headers.

## Code Style Guidelines
- Use Rust 2024 edition conventions
- Follow standard Rust naming: snake_case for functions/variables, PascalCase for types
- Use `cargo fmt` for consistent formatting
- Prefer explicit types for FFI boundaries
- Handle C library errors appropriately with Result types
- Use safe Rust wrappers around unsafe FFI calls
- Document public APIs with rustdoc comments

## Error Handling
- Wrap C library errors in Rust Result types
- Use appropriate error types (consider thiserror crate for custom errors)
- Handle null pointers and invalid values from C code safely