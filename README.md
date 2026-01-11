# Bitcoin Tools (btcx_tools)

A comprehensive collection of Bitcoin utilities written in Rust, including an API server and various command-line tools for Bitcoin address generation, mnemonics, wallet operations, and blockchain queries.

## Project Structure

### `/api` - Bitcoin Transaction API

A REST API server for creating unsigned Bitcoin transactions. The API allows you to construct Bitcoin transactions by specifying inputs and outputs in JSON format.

**Features:**
- Create unsigned Bitcoin transactions
- Validate transaction inputs and outputs
- Support for Bitcoin mainnet (configurable)
- Hex-encoded transaction output

**Running the API:**

**Method 1: Using Cargo (Development)**
```bash
cd api
cargo build --release
./target/release/bitcoin_tx_api
```

**Method 2: Using Cargo run (Development with auto-rebuild)**
```bash
cd api
cargo run
```

**Method 3: Using Docker**
```bash
cd api
docker build -t bitcoin_tx_api .
docker run -p 8080:8080 bitcoin_tx_api
```

**Method 4: Using pre-built binary**
```bash
cd api
# Download the binary from releases (if available)
./bitcoin_tx_api
```

**API Usage:**

The API exposes a single endpoint:

**POST /create_tx** - Create an unsigned Bitcoin transaction

**Request Body:**
```json
{
    "inputs": [
        {
            "txid": "abc1234567890abcdef1234567890abcdef1234567890abcdef1234567890abc",
            "vout": 0
        }
    ],
    "outputs": [
        {
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "amount": 10000
        }
    ]
}
```

**Response:**
```json
{
    "tx_hex": "01000000..."
}
```

**Testing with curl:**
```bash
curl -X POST http://localhost:8080/create_tx \
     -H "Content-Type: application/json" \
     -d '{"inputs":[{"txid":"abc1234567890abcdef1234567890abcdef1234567890abcdef1234567890abc","vout":0}],"outputs":[{"address":"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa","amount":10000}]}'
```

**Deployment:**
- Docker support via `Dockerfile`
- Deploy to Fly.io using `fly.toml` configuration
- The server binds to `0.0.0.0:8080` by default

**Notes:**
- The API returns unsigned transactions that must be signed separately before broadcasting
- Currently configured for Bitcoin mainnet (can be modified in source code)
- Input validation includes checking txid format and address network compatibility

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
