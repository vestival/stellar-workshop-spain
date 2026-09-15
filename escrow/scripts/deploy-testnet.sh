#!/usr/bin/env bash
# Deploys the escrow to Stellar testnet and runs one full happy path:
# payer locks 1 XLM, arbiter releases it to the payee.
set -euo pipefail

NETWORK=testnet
RPC=https://soroban-testnet.stellar.org
AMOUNT=10000000 # 1 XLM, the token has 7 decimals

say() { printf '\n\033[1;33m==> %s\033[0m\n' "$1"; }

say "1/6 Creating three funded testnet accounts"
for who in payer payee arbiter; do
  stellar keys generate "$who" --network "$NETWORK" --fund --overwrite
  printf '%-8s %s\n' "$who" "$(stellar keys address "$who")"
done

if [ "${USE_PREBUILT:-0}" = 1 ]; then
  say "2/6 Using the prebuilt WASM (skipping the build)"
  WASM=prebuilt/milestone_escrow.wasm
else
  say "2/6 Building the contract"
  stellar contract build
  WASM=target/wasm32v1-none/release/milestone_escrow.wasm
fi

say "3/6 Deploying the contract"
CONTRACT=$(stellar contract deploy \
  --wasm "$WASM" \
  --source-account payer \
  --network "$NETWORK" \
  --alias escrow)
echo "contract: $CONTRACT"

TOKEN=$(stellar contract id asset --asset native --network "$NETWORK")
LEDGER=$(curl -s -X POST "$RPC" -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"getLatestLedger"}' |
  sed -n 's/.*"sequence":\([0-9]*\).*/\1/p')
DEADLINE=$((LEDGER + 100))
echo "token (XLM SAC): $TOKEN"
echo "deadline ledger: $DEADLINE (now $LEDGER)"

say "4/6 Locking $AMOUNT stroops in escrow"
stellar contract invoke --id "$CONTRACT" --source-account payer --network "$NETWORK" -- \
  init \
  --payer "$(stellar keys address payer)" \
  --payee "$(stellar keys address payee)" \
  --arbiter "$(stellar keys address arbiter)" \
  --token "$TOKEN" \
  --amount "$AMOUNT" \
  --deadline_ledger "$DEADLINE"

say "5/6 State after funding"
stellar contract invoke --id "$CONTRACT" --source-account payer --network "$NETWORK" -- state

say "6/6 Arbiter releases the funds"
stellar contract invoke --id "$CONTRACT" --source-account arbiter --network "$NETWORK" -- release
stellar contract invoke --id "$CONTRACT" --source-account payer --network "$NETWORK" -- state

echo
echo "Done. Open the contract on the explorer:"
echo "https://stellar.expert/explorer/testnet/contract/$CONTRACT"
