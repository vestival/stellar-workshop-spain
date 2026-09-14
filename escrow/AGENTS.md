# Context for AI coding agents

Read this before changing anything. It is the same file for Claude Code, Codex,
Cursor and anything else that reads repo context. `CLAUDE.md` is a symlink to it.

## What this repo is

A single Soroban smart contract used in a 3-hour Stellar workshop. One payer
locks tokens against one milestone. An arbiter releases them to the payee. If
nobody releases before `deadline_ledger`, the payer refunds themselves.

Keep it one contract and one file. This is teaching material, not a product.

## Stack

- Rust, `no_std`, compiled to `wasm32v1-none`
- `soroban-sdk` 27 (pinned in `Cargo.toml`)
- `stellar` CLI 28
- Network: Stellar testnet, RPC `https://soroban-testnet.stellar.org`,
  passphrase `Test SDF Network ; September 2015`

## Rules that matter on Soroban

1. Every state-changing entry point authorises exactly one address with
   `require_auth()`. Never authorise "whoever called".
2. Money moves through `token::Client::transfer`. The contract itself is the
   `from` address when it pays out, which works because the contract is the
   authoriser of its own balance.
3. Errors are a `#[contracterror]` enum returned as `Result`, never `panic!`.
   Tests assert on `try_*` returning `Err(Ok(Error::X))`.
4. Instance storage holds the whole escrow. It is small and it lives and dies
   with the contract, so it is the right storage type here. Persistent and
   temporary storage are for per-user entries.
5. Instance storage has a TTL. A production version would extend it inside
   every entry point. This one does not, on purpose, so the workshop can talk
   about rent without hiding it behind a helper.

## Commands

```sh
cargo test                  # 10 unit tests, no network
stellar contract build      # wasm to target/wasm32v1-none/release/
bash scripts/deploy-testnet.sh
```

## Do not

- Add a frontend, a second contract, a proxy, or an upgrade path.
- Replace the tests with mocks. The tests run the real Soroban host.
- Swap `env.register_stellar_asset_contract_v2` for a hand-written mock token.
