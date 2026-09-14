# Milestone escrow, a Stellar workshop contract

You lock money for someone. A third party decides whether they get paid. If
nobody decides in time, you take your money back. That is the whole contract,
and it is about 120 lines of Rust.

By the end of this session it is deployed on Stellar testnet under your own
account, and you have the explorer link to prove it.

## Get a machine in two minutes

Click **Code, Codespaces, Create codespace on main** on this repo. Wait for the
setup to finish. Rust, the `wasm32v1-none` target and the Stellar CLI are
already there.

Prefer your own laptop? Do this before the session, not during it:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none
brew install stellar-cli            # macOS and Linuxbrew
# or: curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh
stellar network use testnet
```

Linux from source also needs `build-essential pkg-config libdbus-1-dev`.

Check it worked:

```sh
rustc --version      # 1.84 or higher
stellar --version
```

## Run it

```sh
cargo test                        # 10 tests, no network, no account, no waiting
bash scripts/deploy-testnet.sh    # accounts, build, deploy, lock, release
```

The script prints a contract address and an explorer link. That link is the
thing you keep.

If your build is slow, `prebuilt/milestone_escrow.wasm` is the same contract,
already compiled. Point `stellar contract deploy --wasm` at it and carry on.

## What it does

| Function | Who signs | What happens |
| --- | --- | --- |
| `init` | payer | Pulls `amount` from the payer into the contract |
| `release` | arbiter | Sends the full amount to the payee |
| `refund` | payer | Sends it back, only from `deadline_ledger` onwards |
| `state` | nobody | `Funded`, `Released` or `Refunded` |

The token is XLM through its Stellar Asset Contract, so no asset issuance and
no trustlines. Swap `--asset native` in the deploy script for your own asset
once you want one.

## Your turn

Pick one. Each is a small edit plus a test that fails before you make it.

1. **Partial release.** `release(amount)` pays part of the escrow and leaves the
   rest funded. Decide what `state()` returns in between.
2. **Fee.** The arbiter keeps 1% on release. Watch out for integer division.
3. **Cancel by agreement.** Payer and payee together can unwind before the
   deadline, without the arbiter. Two `require_auth()` calls, not one.
4. **Deposit in tranches.** `init` sets the target, `fund()` can be called more
   than once until the target is reached.

Write the test first. `cargo test` takes a second and needs no network, which
is the whole reason the Soroban test host exists.

## Point your AI agent at Stellar

Generic models guess Soroban APIs and they guess wrong, usually with an EVM
accent. Give yours real context before you ask it anything:

```sh
# Claude Code, official Stellar skills
/plugin marketplace add stellar/stellar-dev-skill

# Raven, Stellar's MCP server, for any MCP-capable agent
claude mcp add --transport http stellar-raven "https://raven.stellar.buzz/mcp"
```

No agent installed? Paste <https://developers.stellar.org/llms.txt> into a
ChatGPT project or a custom instruction field. It is a machine-readable index
of the docs.

Then read `AGENTS.md` in this repo. It is the context file the agent reads, and
it is written for a human too.

## Links

- Stellar docs: <https://developers.stellar.org>
- Stellar Lab, accounts and transactions in a browser: <https://lab.stellar.org>
- Testnet explorer: <https://stellar.expert/explorer/testnet>
- Skills directory: <https://skills.stellar.org>
- Developer Discord: <https://discord.com/invite/DxzSCZYxZG>
- HackMeridian, Lisbon, 25 and 26 October 2026: <https://www.hackmeridian.com>
