

# Bitcoin Tools (btcx_tools)

**Use at Your Own Risk This is experimental/beta software. It may contain bugs or cause unexpected behavior. No warranties are provided. Use entirely at your own discretion and risk.**

A comprehensive collection of Bitcoin utilities written in Rust, including an API server and various command-line tools for Bitcoin address generation, mnemonics, wallet operations, and blockchain queries.

Prerequisites: Rust Installation

## Building & Installation

### One-command build (recommended)

The project includes a convenient build script that compiles **all** tools in release mode and places the binaries in the `dist/` directory:  
```
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
├── brain_wallet
├── generate_addresses
├── generate_mnemonic
```

### Usage (after building)  
```
# Generate 12-word mnemonic
./dist/generate_mnemonic wordlists/english.txt 12

# Generate addresses from mnemonic (no passphrase)
./dist/generate_addresses "abandon abandon ... about" "m/44'/0'/0'/0" ""

# Brain wallet from passphrase
./dist/brain_wallet "correct horse battery staple"

# Check balances of many addresses
./dist/blockstream_balance_loop addresses.txt

# Start the transaction builder API
./dist/bitcoin_tx_api
```

### Alternative: Build manually (individual crates)  
```
# API server
cd api && cargo build --release

# Individual tools
cd ../scripts/generate_mnemonic     && cargo build --release
cd ../scripts/generate_addresses    && cargo build --release
cd ../scripts/brain_wallet          && cargo build --release
cd ../scripts/blockstream_info      && cargo build --release
```

**Recommendation:** Use ./build-release.sh — it's faster and keeps everything organized in one place.

## Development

### Advanced Usage Examples

**Generate mnemonic -> save it -> generate addresses -> save addresses -> check balances**

```
./dist/generate_mnemonic ./wordlists/english.txt 12 \
  | tee mnemonic.txt \
  | xargs -I {} ./dist/generate_addresses "{}" "m/44'/0'/0'/0" "" \
  | grep address \
  | cut -d ':' -f 2 \
  > ./addresses.txt \
  && ./dist/blockstream_balance_loop ./addresses.txt
```

**Generate mnemonic -> addresses -> check balances in one pipeline**

```
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

```
cat ./quotes.txt \
    | while read -r quote; do ./dist/brain_wallet "$quote"; done \
    | grep Address \
    | cut -d ':' -f 2 \
    > ./addresses.txt \
    && ./dist/blockstream_balance_loop ./addresses.txt 
```

**Process multiple passphrases from a file:**  

```
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

### Development Dependencies

- **Dependency Management:** Automated via Dependabot for Cargo packages
- **Dependencies:** All Rust crates defined in respective `Cargo.toml` files
- **Language:** Rust (systems programming language, memory safe)

## Probability Note

This project is primarily for educational purposes. The odds of finding an address with some bitcoins in it per cycle are approximately 2.94×10⁻³¹ to 1, assuming there are about 100 million seeds in use. This probability is extremely low, reflecting the vastness of the seed space and the security of Bitcoin's design. Alternatively, as a ratio, the odds are 1 to 3.4×10³⁰.

## Deployment

- Docker support via `Dockerfile`
- Deploy to [Fly.io](https://fly.io/) using `fly.toml` configuration
- The server binds to `0.0.0.0:8080` by default

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]
