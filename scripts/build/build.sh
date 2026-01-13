#!/usr/bin/env bash
set -euo pipefail

# Source common functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Default values
BUILD_TYPE="debug"
TARGET=""
CLEAN=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
done

# Clean if requested
if [[ "$CLEAN" == true ]]; then
    info "Cleaning build directories..."
    cargo clean
    rm -rf "$DIST_DIR"/*
fi

# Build all workspace members
info "Building all workspace members ($BUILD_TYPE)..."
cargo build --workspace $([[ "$BUILD_TYPE" == "release" ]] && echo "--release") $([[ -n "$TARGET" ]] && echo "--target $TARGET")

# Create list of crates to process
CRATES=(
    "scripts/blockstream_info:blockstream_balance_loop"
    "scripts/blockstream_tx:blockstream_tx"
    "scripts/brain_wallet:brain_wallet"
    "scripts/create_tx:create_tx"
    "scripts/generate_addresses:generate_addresses"
    "scripts/generate_mnemonic:generate_mnemonic"
    "scripts/sign_tx:sign_tx"
    "api:bitcoin_tx_api"
)

# Process each crate
for crate_spec in "${CRATES[@]}"; do
    IFS=':' read -r crate_dir bin_name <<< "$crate_spec"
    
    # Determine source path based on target and build type
    local target_dir=""
    if [[ -n "$TARGET" ]]; then
        target_dir="$TARGET/$([[ "$BUILD_TYPE" == "release" ]] && echo "release" || echo "debug")"
    else
        target_dir="$([[ "$BUILD_TYPE" == "release" ]] && echo "release" || echo "debug")"
    fi
    
    src_path="$PROJECT_ROOT/target/$target_dir/$bin_name"
    dest_path="$DIST_DIR/$bin_name"
    
    # Copy binary
    if [[ -f "$src_path" ]]; then
        copy_binary "$src_path" "$dest_path"
        post_process_binary "$dest_path"
    else
        error "Binary not found: $src_path"
    fi
done

# Copy library files if they exist
for lib_ext in so dll dylib; do
    lib_path="$PROJECT_ROOT/target/$([[ "$BUILD_TYPE" == "release" ]] && echo "release" || echo "debug")/libbtcx_lib.$lib_ext"
    if [[ -f "$lib_path" ]]; then
        copy_binary "$lib_path" "$DIST_DIR/libbtcx_lib.$lib_ext"
        post_process_binary "$DIST_DIR/libbtcx_lib.$lib_ext"
    fi
done

success "\nBuild completed successfully!"
echo "Binaries are available in: $DIST_DIR"
ls -lh "$DIST_DIR"
