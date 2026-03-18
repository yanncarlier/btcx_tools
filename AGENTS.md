# AGENTS.md - Agentic Coding Guidelines for btcx_tools

## Overview
This document provides guidelines for AI coding agents working on the btcx_tools codebase. btcx_tools is a Rust-based Bitcoin utilities collection with a workspace structure containing multiple crates for Bitcoin address generation, transaction handling, and blockchain queries.

## Build/Lint/Test Commands

### Building
```bash
# Build all workspace members in debug mode
make debug
# or
cargo build --workspace

# Build all workspace members in release mode
make release
# or
cargo build --workspace --release

# Use the custom build script (recommended) - builds all tools and places binaries in dist/
scripts/build/build.sh --release
```

### Testing
```bash
# Run all tests in workspace
make test
# or
cargo test --workspace

# Run a specific test
cargo test test_function_name

# Run tests for a specific package
cargo test --package package_name

# Run tests with verbose output
cargo test --workspace -- --nocapture
```

### Linting and Formatting
```bash
# Format code
make fmt
# or
cargo fmt --all

# Run clippy linter
make clippy
# or
cargo clippy --workspace -- -D warnings

# Check formatting without changing files
cargo fmt --all -- --check
```

### Cleaning
```bash
# Clean build artifacts
make clean
# or
cargo clean
```

## Code Style Guidelines

### Imports and Dependencies
- Group imports by standard library, external crates, then local crates
- Use explicit imports rather than glob imports (`use std::collections::HashMap` not `use std::collections::*`)
- Import commonly used types explicitly at the top
- Example:
```rust
use std::{collections::HashMap, str::FromStr};
use bitcoin::{Address, Network, Transaction};
use serde::{Deserialize, Serialize};
```

### Naming Conventions
- **Functions**: snake_case (e.g., `create_transaction`, `validate_address`)
- **Variables**: snake_case (e.g., `mnemonic_phrase`, `derivation_path`)
- **Structs/Enums**: PascalCase (e.g., `TransactionBuilder`, `NetworkType`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `DEFAULT_FEE_RATE`)
- **Modules**: snake_case (e.g., `transaction_builder`, `address_generator`)

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Prefer descriptive error messages over generic ones
- Use `?` operator for error propagation in functions that return `Result`
- Handle errors gracefully with proper user feedback
- Example:
```rust
let address = Address::from_str(&address_str)
    .map_err(|_| "Invalid Bitcoin address format")?;
```

### Code Structure and Patterns
- Use descriptive variable names (e.g., `mnemonic_phrase` instead of `m`)
- Add comments for complex logic, especially cryptographic operations
- Use early returns for error conditions to reduce nesting
- Structure functions to be testable with clear input/output
- Example function structure:
```rust
fn process_transaction(inputs: Vec<TransactionInput>) -> Result<Transaction, String> {
    if inputs.is_empty() {
        return Err("At least one input required".to_string());
    }

    // Process inputs...
    // Return result...
}
```

### Bitcoin-Specific Patterns
- Always validate network compatibility (mainnet vs testnet)
- Use proper amount handling with `Amount` types, not raw u64
- Validate addresses before use with network checking
- Handle both compressed and uncompressed public keys appropriately
- Use WIF format for private key export/import

### Serialization and Deserialization
- Use `serde` with `Serialize`/`Deserialize` derives for API structures
- Use descriptive field names in JSON structures
- Handle optional fields with `Option<T>`
- Example:
```rust
#[derive(Serialize, Deserialize)]
struct TransactionRequest {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub fee_rate: Option<u64>,
}
```

### Testing Guidelines
- Write unit tests for all public functions
- Use descriptive test names (e.g., `test_transaction_builder_with_valid_inputs`)
- Test error conditions and edge cases
- Use test fixtures for common data structures
- Mock external API calls in integration tests
- Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transaction_creation() {
        // Arrange
        let inputs = vec![create_test_input()];
        let outputs = vec![create_test_output()];

        // Act
        let result = create_transaction(inputs, outputs);

        // Assert
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.input.len(), 1);
    }

    #[test]
    fn test_invalid_empty_inputs() {
        let result = create_transaction(vec![], vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "At least one input required");
    }
}
```

### Security Best Practices
- Never log or print private keys or mnemonics
- Use secure random generation for cryptographic operations
- Validate all user inputs before processing
- Handle sensitive data with appropriate scoping
- Use constant-time operations where applicable

### Documentation
- Add doc comments for public APIs using `///`
- Include parameter descriptions and return value information
- Document error conditions and edge cases
- Example:
```rust
/// Creates a new Bitcoin transaction from the given inputs and outputs
///
/// # Arguments
/// * `inputs` - List of transaction inputs with UTXO references
/// * `outputs` - List of transaction outputs with addresses and amounts
///
/// # Returns
/// Returns the hex-encoded transaction on success
///
/// # Errors
/// Returns an error if inputs are invalid or amounts don't match
pub fn create_transaction(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Result<String, String> {
    // Implementation...
}
```

### File Organization
- Keep related functionality in separate modules
- Use `lib.rs` for shared library code
- Place binary-specific code in `main.rs` files
- Organize tests within the same files using `#[cfg(test)]` modules

### Performance Considerations
- Use efficient data structures (e.g., `HashMap` for lookups)
- Avoid unnecessary allocations in hot paths
- Use iterators and functional programming where appropriate
- Profile performance-critical code

### Version Compatibility
- Pin major versions for critical dependencies (bitcoin, secp256k1)
- Keep Rust edition consistent across workspace (currently 2021)
- Test compatibility when updating dependencies

## Common Patterns in This Codebase

### CLI Tools Structure
```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <arg1> <arg2>", args[0]);
        return;
    }

    // Parse and validate arguments
    let arg1 = &args[1];

    // Process with proper error handling
    match process_data(arg1) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

### API Handler Structure
```rust
async fn api_handler(data: web::Data<AppState>, req: web::Json<Request>) -> impl Responder {
    // Validate request
    if req.inputs.is_empty() {
        return HttpResponse::BadRequest().body("Inputs required");
    }

    // Process request
    match process_request(&data, &req).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
```

## Final Checklist Before Committing
1. Run `make test` to ensure all tests pass
2. Run `make clippy` to check for linting issues
3. Run `make fmt` to format code properly
4. Test build with `make release`
5. Verify no sensitive data is committed
6. Update documentation if public APIs changed

## Getting Help
- Use `make help` for available Makefile targets
- Check individual `Cargo.toml` files for dependency information
- Refer to `README.md` for project overview and usage examples</content>
<parameter name="filePath">/home/y/MY_PROJECTS/btcx_tools/AGENTS.md