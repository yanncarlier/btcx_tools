# Build System

This directory contains the build system for the btcx_tools project.

## Directory Structure

- `build/` - Build-related scripts
  - `common.sh` - Common functions and variables
  - `build.sh` - Main build script
- `dev/` - Development scripts
  - `setup.sh` - Development environment setup

## Usage

### Building the Project

```bash
# Build in debug mode (default)
./scripts/build/build.sh

# Build in release mode
./scripts/build/build.sh --release

# Build for a specific target
./scripts/build/build.sh --target x86_64-unknown-linux-gnu --release
