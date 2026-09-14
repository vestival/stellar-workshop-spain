# Stellar Workshop: Deploy Your First Contract

A guestbook contract on Stellar testnet. You will build it, deploy it, sign it on-chain, and then extend it with your AI assistant. Total time: about 45 minutes.

## 0. Environment

Open this repo in a GitHub Codespace (green Code button, Codespaces tab, Create). The container installs the Rust toolchain and the Stellar CLI for you. Wait for the post-create step to finish before running commands.

Working locally instead? You need Rust, the wasm target and the Stellar CLI:

```bash
rustup target add wasm32v1-none
cargo install --locked stellar-cli
```

## 1. Run the tests

Contracts on Stellar are Rust compiled to WASM. Tests run natively against a local copy of the on-chain environment, so no network is needed:

```bash
cargo test
```

## 2. Create a funded testnet account

Friendbot gives every new testnet account 10,000 test XLM. The CLI calls it for you:

```bash
stellar keys generate --fund alice --network testnet
stellar keys address alice
```

## 3. Build and deploy

```bash
stellar contract build
stellar contract deploy \
  --wasm target/wasm32v1-none/release/guestbook.wasm \
  --source alice \
  --network testnet \
  --alias guestbook
```

The command prints your contract ID (starts with C). That is your contract, live on testnet.

## 4. Sign the guestbook

```bash
stellar contract invoke --id guestbook --source alice --network testnet \
  -- sign --name '"YOUR_NAME"'

stellar contract invoke --id guestbook --source alice --network testnet \
  -- total
```

See it on the explorer: `https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>`

Fees for all of this: fractions of a cent, paid in XLM. There is no separate gas token.

## 5. Extend it with AI

Stellar ships official skills for AI coding assistants. Set them up:

```bash
# Claude Code
/plugin install stellar-dev@stellar-dev

# Optional: live ecosystem data via MCP
claude mcp add --transport http stellar-raven "https://raven.stellar.buzz/mcp"
```

Cursor and Codex users: install from skills.stellar.org.

Now ask your assistant for one of these, then rebuild, redeploy and invoke:

1. Add a `signers` function that returns every name, not just the last one (hint: persistent storage and a `Vec<String>`).
2. Reject empty names with a proper contract error.
3. Let each account sign only once (hint: `require_auth` and `Address`).

## Troubleshooting

- `stellar: command not found`: the post-create install has not finished, or `~/.cargo/bin` is not on PATH.
- Friendbot rate limits: wait a minute, or fund from https://lab.stellar.org/account/fund
- Testnet resets a few times a year (next: December 16, 2026). If your contract vanishes after a reset, redeploy.

## Where to go next

- HackMeridian, Lisbon, October 25 and 26: https://www.hackmeridian.com/
- Docs: https://developers.stellar.org/
- Skills for AI assistants: https://skills.stellar.org/
- Full-stack scaffolding: `stellar scaffold init` (Scaffold Stellar)
