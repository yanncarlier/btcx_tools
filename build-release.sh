#!/usr/bin/env bash
set -euo pipefail

BUILD_ROOT="$PWD"
DIST="$BUILD_ROOT/dist"
mkdir -p "$DIST"

build_and_copy() {
    local bin="$1"
    local crate_dir="$2"
    local bin_name="${3:-$bin}"

    echo "→ Building $bin ..."
    (
        cd "$crate_dir" || exit 1
        cargo build --release --bin "$bin_name" || exit 1
        cp -v "target/release/$bin_name" "$DIST/$bin"
    ) || exit 1
}

build_and_copy "blockstream_balance_loop"  "scripts/blockstream_info"
build_and_copy "brain_wallet"              "scripts/brain_wallet"
build_and_copy "generate_addresses"        "scripts/generate_addresses"
build_and_copy "generate_mnemonic"         "scripts/generate_mnemonic"
build_and_copy "bitcoin_tx_api"            "api"                "bitcoin_tx_api"

echo -e "\n→ Post-processing"

echo "Stripping binaries..."
strip "$DIST"/* 2>/dev/null || true

echo "UPX compression (optional)..."
upx --best "$DIST"/* 2>/dev/null || true

# echo "Creating versioned copies..."
# VERSION=$(cd api && cargo pkgid | awk -F '[#@]' '{print $3}' || echo "dev")

# for f in "$DIST"/*; do
#     [[ -f $f && ! $f =~ -v ]] || continue
#     name=$(basename "$f")
#     cp -v "$f" "$DIST/${name}-v${VERSION}-linux-x64"
# done

echo -e "\nBuild completed:"
ls -lh "$DIST"/