# Bitcoin Tools (btcx_tools)

**Use at Your Own Risk This is experimental/beta software. It may contain bugs or cause unexpected behavior. No warranties are provided. Use entirely at your own discretion and risk.**

A comprehensive collection of Bitcoin utilities written in Rust, including an API server and various command-line tools for Bitcoin address generation, mnemonics, wallet operations, and blockchain queries.

## Prerequisites

### Rust Installation

If you don't have Rust installed, install it using rustup:

bash

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



Follow the prompts to install. After installation, run:

bash

```
source $HOME/.cargo/env
```



On Windows: Download and run the installer from [rustup.rs](https://rustup.rs/). Follow the installation instructions.

Verify installation:

bash

```
rustc --version
cargo --version
```



## Building

### Compile the Code

To compile the Rust code, run the following command in the project directory:

bash

```
cargo build
```



This command downloads the dependencies specified in Cargo.toml, compiles the code, and generates an executable.

To compile in release mode (optimized, smaller binary):

bash

```
cargo build --release
```



### Build All Projects

bash

```
# Build individual projects
cd api && cargo build --release
cd ../scripts/generate_mnemonic && cargo build --release
cd ../brain_wallet && cargo build --release
cd ../generate_addresses && cargo build --release
cd ../blockstream_info && cargo build --release
```



## Project Structure

### `/api` - Bitcoin Transaction API

A REST API server for creating unsigned Bitcoin transactions. The API allows you to construct Bitcoin transactions by specifying inputs and outputs in JSON format.

**Features:**

- Create unsigned Bitcoin transactions
- Validate transaction inputs and outputs
- Support for Bitcoin mainnet (configurable)
- Hex-encoded transaction output

#### Running the API

**Method 1: Using Cargo (Development)**

bash

```
cd api
cargo build --release
./target/release/bitcoin_tx_api
```



**Method 2: Using Cargo run (Development with auto-rebuild)**

bash

```
cd api
cargo run
```



**Method 3: Using Docker**

bash

```
cd api
docker build -t bitcoin_tx_api .
docker run -p 8080:8080 bitcoin_tx_api
```



Note: For modern Docker with BuildKit:

bash

```
docker buildx build -t bitcoin_tx_api .
```



**Method 4: Using pre-built binary**

bash

```
cd api
# Download the binary from releases (if available)
./bitcoin_tx_api
```



#### API Usage

**POST /create_tx** - Create an unsigned Bitcoin transaction

**Request Body:**

json

```
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

json

```
{
    "tx_hex": "01000000..."
}
```



**Testing with curl:**

bash

```
curl -X POST http://localhost:8080/create_tx \
     -H "Content-Type: application/json" \
     -d '{"inputs":[{"txid":"abc1234567890abcdef1234567890abcdef1234567890abcdef1234567890abc","vout":0}],"outputs":[{"address":"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa","amount":10000}]}'
```



#### Additional Notes

- **Network**: The API is currently configured to use the Bitcoin mainnet. To use it with testnet or another network, modify the network field in the AppState struct in [main.rs](https://main.rs/).
- **Unsigned Transactions**: The transaction returned by the API is unsigned and must be signed using a separate tool or library before it can be broadcast to the Bitcoin network.
- **Error Handling**: The API performs basic validation on inputs and outputs, such as checking for valid txid formats and ensuring addresses match the network. Invalid requests will result in a 400 Bad Request response with a descriptive message.

### `/scripts`

#### `generate_mnemonic`

Generate BIP-39 mnemonic phrases for wallet creation.

**Supports:** English, Spanish, French, Italian, Portuguese, Chinese (Simplified & Traditional), Japanese, Korean

**Compile:**

```
cd scripts/generate_mnemonic
cargo run --release
```

Usage: ./target/release/generate_mnemonic <wordlist_path> <number_of_words>

```
./target/release/generate_mnemonic english.txt 12
```



#### `generate_addresses`

Generate Bitcoin addresses from seeds or mnemonics.

**Compile:**

```
cd scripts/generate_addresses
cargo run --release
```

Usage: ./target/release/generate_addresses <mnemonic_phrase> <derivation_path> [passphrase]

```
./target/release/generate_addresses "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" "m/44'/0'/0'/0" ""
```



#### `brain_wallet`

Create deterministic Bitcoin wallets from passphrases.

**Compile:**

```
cd scripts/brain_wallet
cargo run --release
```

Usage: ./target/release/brain_wallet <passphrase words...>

```
./target/release/brain_wallet example
WIF Private Key: 5JRtfJfcY84BnjEHpSMkiemQHoSWgF5gqPzm4175zmPLYNMRbYM
Bitcoin Address: 17PTKw8b4pvajpt3UhviwPJE4REr9XUm7X
```



#### `blockstream_info` / `blockstream_balance_loop`

Query blockchain information via Blockstream API.

**Compile:**

```
cd scripts/blockstream_info
cargo run --release
```

Usage: target/release/blockstream_balance_loop <file_name>

```
./target/release/blockstream_balance_loop addresses.txt 
Address: 17PTKw8b4pvajpt3UhviwPJE4REr9XUm7X, Balance: 0 satoshis
```



## Development

### Advanced Usage Examples

**Generate mnemonic and addresses in one command:**

bash

```
./generate_mnemonic/target/release/generate_mnemonic ./generate_mnemonic/english.txt 12 | tee mnemonic.txt | xargs -I {} generate_addresses/target/release/generate_addresses "{}" "m/44'/0'/0'/0" "" | grep address | cut -d ':' -f 2 > addresses.txt && blockstream_info/target/release/blockstream_balance_loop addresses.txt
```



**Continuous address generation with balance checking:**

bash

```
while true; do
    if ./generate_mnemonic/target/release/generate_mnemonic ./generate_mnemonic/english.txt 12 | tee mnemonic.txt | xargs -I {} generate_addresses/target/release/generate_addresses "{}" "m/44'/0'/0'/0" "" | grep address | cut -d ':' -f 2 > addresses.txt && blockstream_info/target/release/blockstream_balance_loop addresses.txt | tee /dev/tty | grep -q -v "Balance: 0 satoshis"; then
        break
    fi
done
```



**Process multiple passphrases from a file:**

bash

```
while IFS= read -r line || [ -n "$line" ]; do ./brain_wallet/target/release/brain_wallet "$line"; done < quotes.txt | grep Address| cut -d ':' -f 2 > addresses.txt && ./blockstream_info/target/release/blockstream_balance_loop addresses.txt
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