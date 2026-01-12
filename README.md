# Bitcoin Tools (btcx_tools)

**Use at Your Own Risk This is experimental/beta software. It may contain bugs or cause unexpected behavior. No warranties are provided. Use entirely at your own discretion and risk.**

A comprehensive collection of Bitcoin utilities written in Rust, including an API server and various command-line tools for Bitcoin address generation, mnemonics, wallet operations, and blockchain queries.

## Prerequisites

- Rust Installation

## Quick Start

### Building & Installation

#### One-command build (recommended)

The project includes a convenient build script that compiles **all** tools in release mode and places the binaries in the `dist/` directory:

```bash
./build-release.sh
```

What it does:

- Builds all crates in release mode
- Copies binaries to ./dist/
- Strips debug symbols (if strip is available)
- Applies UPX compression (if upx is installed – optional)

After running the script you will have:

```
dist/
├── bitcoin_tx_api
├── blockstream_balance_loop
├── blockstream_tx
├── brain_wallet
├── create_tx
├── generate_addresses
├── generate_mnemonic
├── sign_tx
```

#### Alternative: Build manually (individual crates)

```bash
# API server
cd api && cargo build --release

# Individual tools
cd ../scripts/generate_mnemonic     && cargo build --release
cd ../scripts/generate_addresses    && cargo build --release
cd ../scripts/brain_wallet          && cargo build --release
cd ../scripts/create_tx             && cargo build --release
cd ../scripts/sign_tx               && cargo build --release
cd ../scripts/blockstream_info      && cargo build --release
cd ../scripts/blockstream_tx        && cargo build --release
```

**Recommendation:** Use `./build-release.sh` — it's faster and keeps everything organized in one place.

### Basic Usage

```bash
# Generate 12-word mnemonic
./dist/generate_mnemonic wordlists/english.txt 12

# Generate addresses from mnemonic (no passphrase)
./dist/generate_addresses "abandon abandon ... about" "m/44'/0'/0'/0" ""

# Brain wallet from passphrase
./dist/brain_wallet "correct horse battery staple"

# Check balances of many addresses
./dist/blockstream_balance_loop addresses.txt

# Look up transaction details by txid
./dist/blockstream_tx abc123def456...

# Create unsigned transaction (JSON from command-line)
./dist/create_tx '{"inputs":[{"txid":"abc123...","vout":0}],"outputs":[{"address":"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa","amount":1000}]}'

# Sign transaction (JSON from command-line)
./dist/sign_tx '{"unsigned_tx_hex":"...","inputs":[{"private_key_wif":"5K...","address":"1A1z..."}]}'

# Start the transaction builder API
./dist/bitcoin_tx_api
```

## Tools Overview

This project contains 8 main components:

### 1. Generate Mnemonic (scripts/generate_mnemonic/src/main.rs)

- Generates BIP39 mnemonic phrases
- Supports wordlists (2048 words)
- Supports 12, 15, 18, 21, or 24 word mnemonics
- Includes checksum verification

### 2. Generate Addresses (scripts/generate_addresses/src/main.rs)

- Generates Bitcoin addresses from BIP39 mnemonic
- Supports BIP32 derivation paths
- Optional passphrase support
- Generates 10 addresses from a parent derivation path
- Outputs: derivation path, address, public key, private key (hex), and WIF

### 3. Brain Wallet (scripts/brain_wallet/src/main.rs)

- Generates Bitcoin wallet from a passphrase
- Uses SHA-256 to derive private key from passphrase
- Outputs WIF private key and P2PKH Bitcoin address
- Supports uncompressed public keys

### 4. Create Transaction (scripts/create_tx/src/main.rs)

- CLI tool to create unsigned Bitcoin transactions
- Accepts JSON input via command-line argument or stdin
- Accepts inputs (txid, vout) and outputs (address, amount)
- Returns hex-encoded transaction
- Same functionality as the API server endpoint, but as a command-line tool

### 5. Sign Transaction (scripts/sign_tx/src/main.rs)

- CLI tool to sign unsigned Bitcoin transactions
- Accepts JSON input via command-line argument or stdin
- Takes unsigned transaction hex and private keys (WIF format) with corresponding addresses
- Signs all inputs using ECDSA signatures
- Returns hex-encoded signed transaction ready for broadcast
- Supports P2PKH (Pay-to-Public-Key-Hash) transactions

### 6. Blockstream Balance Loop (scripts/blockstream_info/src/main.rs)

- CLI tool to check Bitcoin address balances
- Reads addresses from a file
- Queries Blockstream API for balance info
- Stops when it finds a non-zero balance

### 7. Blockstream Transaction Lookup (scripts/blockstream_tx/src/main.rs)

- CLI tool to look up Bitcoin transaction details by txid
- Queries Blockstream API for transaction information
- Displays transaction details: inputs, outputs, fees, confirmation status
- Outputs both human-readable format and JSON

### 8. API Server (api/src/main.rs)

- HTTP server using Actix-web
- Endpoint: POST /create_tx to create unsigned Bitcoin transactions
- Accepts inputs (txid, vout) and outputs (address, amount)
- Returns hex-encoded transaction
- Binds to 0.0.0.0:8080

### Supporting Files

- Build script (build-release.sh): Builds all tools in release mode
- Dockerfile: Containerizes the API server
- fly.toml: Fly.io deployment configuration
- Cargo.toml files: Dependency specifications for each crate

## Advanced Usage Examples

**Generate mnemonic -> save it -> generate addresses -> save addresses -> check balances**

```bash
./dist/generate_mnemonic ./wordlists/english.txt 12 \
  | tee mnemonic.txt \
  | xargs -I {} ./dist/generate_addresses "{}" "m/44'/0'/0'/0" "" \
  | grep address \
  | cut -d ':' -f 2 \
  > ./addresses.txt \
  && ./dist/blockstream_balance_loop ./addresses.txt
```

**Generate mnemonic -> addresses -> check balances in one pipeline**

```bash
while true; do
    ./dist/generate_mnemonic ./wordlists/english.txt 12 \
        | tee ./mnemonic.txt \
        | xargs -I {} ./dist/generate_addresses "{}" "m/44'/0'/0'/0" "" \
        | grep address \
        | cut -d ':' -f 2 \
        > ./addresses.txt \
        && ./dist/blockstream_balance_loop ./addresses.txt \
        | tee /dev/tty \
        | grep -q -v "Balance: 0 satoshis" \
        && break
done
```

**Generate addresses from quotes -> extract addresses -> check balances**

```bash
cat ./quotes.txt \
    | while read -r quote; do ./dist/brain_wallet "$quote"; done \
    | grep Address \
    | cut -d ':' -f 2 \
    > ./addresses.txt \
    && ./dist/blockstream_balance_loop ./addresses.txt
```

**Process multiple passphrases from a file**

```bash
while IFS= read -r line || [ -n "$line" ]; do \
./dist/brain_wallet "$line"; done \
< ./quotes.txt \
| grep Address \
| cut -d ':' -f 2 > ./addresses.txt \
&& ./dist/blockstream_balance_loop ./addresses.txt
```

## Dependencies

### Key Dependencies

- `secp256k1` - Elliptic curve cryptography for Bitcoin
- `bitcoin` - Bitcoin library
- `serde` & `serde_json` - Serialization
- `tokio` - Async runtime (for API server)
- `actix-web` - Web framework (for API server)

### Development

- **Dependency Management:** Automated via Dependabot for Cargo packages
- **Dependencies:** All Rust crates defined in respective `Cargo.toml` files
- **Language:** Rust (systems programming language, memory safe)

## Deployment

- Docker support via `Dockerfile`
- Deploy to [Fly.io](https://fly.io/) using `fly.toml` configuration
- The server binds to `0.0.0.0:8080` by default

## Probability Note

This project is primarily for educational purposes. The odds of finding an address with some bitcoins in it per cycle are approximately 2.94×10⁻³¹ to 1, assuming there are about 100 million seeds in use. This probability is extremely low, reflecting the vastness of the seed space and the security of Bitcoin's design. Alternatively, as a ratio, the odds are 1 to 3.4×10³⁰.

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]
