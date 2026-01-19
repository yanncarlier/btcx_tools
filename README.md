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
scripts/build/build.sh --release
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
├── broadcast_tx
├── create_tx
├── estimate_fee
├── fetch_utxos
├── generate_addresses
├── generate_mnemonic
├── sign_tx
```

#### Alternative: Build manually (individual crates)

```bash
# API server
cd api && cargo build --release

# Individual tools (in order of appearance in the Tools Overview)
cd ../scripts/generate_mnemonic     && cargo build --release
cd ../scripts/generate_addresses    && cargo build --release
cd ../scripts/brain_wallet          && cargo build --release
cd ../scripts/create_tx             && cargo build --release
cd ../scripts/sign_tx               && cargo build --release
cd ../scripts/blockstream_info      && cargo build --release
cd ../scripts/blockstream_tx        && cargo build --release
cd ../scripts/fetch_utxos           && cargo build --release
cd ../scripts/broadcast_tx          && cargo build --release
cd ../scripts/estimate_fee          && cargo build --release
```

**Recommendation:** Use `scripts/build/build.sh --release` — it's faster and keeps everything organized in one place.

The build script supports several options:
```bash
# Show help and usage information
scripts/build/build.sh --help

# Build debug binaries (default)
scripts/build/build.sh

# Build release binaries (recommended for production)
scripts/build/build.sh --release

# Clean build directories and rebuild
scripts/build/build.sh --clean --release
```

### Basic Usage

```bash
# Brain wallet from passphrase
./dist/brain_wallet "correct horse battery staple"

```
```bash
# Generate 12-word mnemonic
./dist/generate_mnemonic wordlists/english.txt 12

```
```bash
# Generate addresses from mnemonic (no passphrase)
./dist/generate_addresses "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" "m/44'/0'/0'/0" ""

```
```bash
# Check balances of many addresses
./dist/blockstream_balance_loop addresses.txt  

# or
./dist/blockstream_balance_loop <<< 12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S

# or
echo 12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S | ./dist/blockstream_balance_loop

```
```bash
# To get a transaction ID (txid) (address used to send BTC to Hal Finney)
./dist/fetch_utxos 12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S
# Example output includes txid field for each UTXO:
[
  {
    "txid": "3832f861eb0fd967fd079da2ee90e415d295dbc81bfb895b73a220aa689c89eb",
    "vout": 0,
    "status": {
      "confirmed": true,
      "block_height": 878308,
      "block_hash": "00000000000000000000287f37f0ddfc5756dddd8eecd2c146d36eafc744fc15",
      "block_time": 1736310913
    },
    "value": 1000,
    "address": null,
    "script_pubkey": null
  }, ...
  
```
```bash
# Look up transaction details by txid
./dist/blockstream_tx 3832f861eb0fd967fd079da2ee90e415d295dbc81bfb895b73a220aa689c89eb

```
```bash
# Create unsigned transaction (JSON from command-line)
./dist/create_tx  '{
  "inputs": [
    {
      "txid": "3832f861eb0fd967fd079da2ee90e415d295dbc81bfb895b73a220aa689c89eb",
      "vout": 0
    }
  ],
  "outputs": [
    {
      "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
      "amount": 100
    }
  ]
}'

# unsigned transaction output example
0100000001eb899c68aa20a2735b89fb1bc8db95d215e490eea29d07fd67d90feb61f832380000000000ffffffff0164000000000000001976a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1888ac00000000

```
```bash
# Sign transaction (JSON from command-line) (WIP don't use)
./dist/sign_tx '{"unsigned_tx_hex":"...","inputs":[{"private_key_wif":"5K...","address":"1A1z..."}]}'

```
```bash
# Start the transaction builder API (WIP don't use)
./dist/bitcoin_tx_api

```



## Advanced Usage Examples

```
./dist/generate_mnemonic wordlists/english.txt 12 \
|xargs -I {} ./dist/generate_addresses {} "m/44'/0'/0'/0" ""

```

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

## Tools Overview

This project contains 11 main components:

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
- Reads addresses from a file or stdin (use `-` as filename or pipe input)
- Queries Blockstream API for balance info
- Stops when it finds a non-zero balance
- Supports `--help` flag for usage information

### 7. Blockstream Transaction Lookup (scripts/blockstream_tx/src/main.rs)

- CLI tool to look up Bitcoin transaction details by txid
- Queries Blockstream API for transaction information
- Displays transaction details: inputs, outputs, fees, confirmation status
- Outputs both human-readable format and JSON

### 8. Fetch UTXOs (scripts/fetch_utxos/src/main.rs)

- CLI tool to fetch Unspent Transaction Outputs for a Bitcoin address
- Queries Blockstream API for UTXO information
- Outputs JSON formatted UTXO data including txid, vout, value, and confirmation status

### 9. Broadcast Transaction (scripts/broadcast_tx/src/main.rs)

- CLI tool to broadcast signed Bitcoin transactions to the network
- Submits transaction hex to Blockstream API
- Returns transaction ID (txid) upon successful broadcast

### 10. Estimate Fee (scripts/estimate_fee/src/main.rs)

- CLI tool to estimate Bitcoin transaction fees
- Queries fee estimation APIs for current network conditions
- Provides fee estimates for different confirmation targets

### 11. API Server (api/src/main.rs)

- HTTP server using Actix-web
- Endpoint: POST /create_tx to create unsigned Bitcoin transactions
- Accepts inputs (txid, vout) and outputs (address, amount)
- Returns hex-encoded transaction
- Binds to 0.0.0.0:8080

### Supporting Files

- Build script (`scripts/build/build.sh`): Builds all tools in release or debug mode
- Dockerfile: Containerizes the API server
- fly.toml: Fly.io deployment configuration
- Cargo.toml files: Dependency specifications for each crate

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
