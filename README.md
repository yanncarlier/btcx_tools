# Bitcoin Tools (btcx_tools)

A comprehensive collection of Bitcoin utilities written in Rust, including an API server and various command-line tools for Bitcoin address generation, mnemonics, wallet operations, and blockchain queries.

## Project Structure

### `/api`
A REST API server for Bitcoin transaction and blockchain data operations.

**Features:**
- Bitcoin transaction analysis
- Blockchain data queries
- RESTful endpoints for Bitcoin operations

**Running:**
```bash
cd api
cargo build --release
./target/release/bitcoin_tx_api
```

**Deployment:**
- Docker support via `Dockerfile`
- Deploy to Fly.io using `fly.toml`

### `/scripts`

#### `generate_mnemonic`
Generate BIP-39 mnemonic phrases for wallet creation.

**Supports:** English, Spanish, French, Italian, Portuguese, Chinese (Simplified & Traditional), Japanese, Korean

```bash
cd scripts/generate_mnemonic
cargo run --release
```

#### `generate_addresses`
Generate Bitcoin addresses from seeds or mnemonics.

```bash
cd scripts/generate_addresses
cargo run --release
```

#### `brain_wallet`
Create deterministic Bitcoin wallets from passphrases.

```bash
cd scripts/brain_wallet
cargo run --release
```

#### `blockstream_info`
Query blockchain information via Blockstream API.

```bash
cd scripts/blockstream_info
cargo run --release
```

## Prerequisites

- Rust 1.56+ (for building from source)
- Cargo (Rust package manager)

## Building

Build all projects:
```bash
# Build individual projects
cd api && cargo build --release
cd ../scripts/generate_mnemonic && cargo build --release
cd ../brain_wallet && cargo build --release
cd ../generate_addresses && cargo build --release
cd ../blockstream_info && cargo build --release
```

## Development

- **Dependency Management:** Automated via Dependabot for Cargo packages
- **Dependencies:** All Rust crates defined in respective `Cargo.toml` files
- **Language:** Rust (systems programming language, memory safe)

## Key Dependencies

- `secp256k1` - Elliptic curve cryptography for Bitcoin
- `bitcoin` - Bitcoin library
- `serde` & `serde_json` - Serialization
- `tokio` - Async runtime (for API server)
- `actix-web` - Web framework (for API server)

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]
