#!/usr/bin/env bash
# Runs once when the Codespace is created. Keep it idempotent.
set -euo pipefail

sudo apt-get update -qq
sudo apt-get install -y -qq build-essential pkg-config libdbus-1-dev libudev-dev jq

rustup target add wasm32v1-none

# Prebuilt binary first. The source build is a 20+ minute fallback, so say so.
if ! curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh; then
  echo "Binary install failed, building the CLI from source. This takes 20+ minutes."
  cargo install --locked stellar-cli@28.0.0
fi

stellar network use testnet || true

# Warm the dependency cache so the first build in the room is fast.
cargo fetch
cargo build --target wasm32v1-none --release || true

echo
echo "Ready. Next: bash scripts/deploy-testnet.sh"
