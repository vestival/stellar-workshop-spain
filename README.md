# Stellar Workshop Spain

Barcelona, Sep 18 2026 & Madrid, Sep 19 2026.

By the end of this session you will have a smart contract deployed on Stellar
testnet under your own account, an explorer link to prove it, and an AI agent
that actually knows Soroban. This repo is everything you need. It costs
nothing: testnet XLM is free and there is no gas token.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/vestival/stellar-workshop-spain)

## 1. Get a machine (5 minutes)

Step by step, on GitHub:

1. You need a GitHub account (free). No account? Create one at
   https://github.com/signup first, then come back here.
2. Sign in and click the badge above (or the green **Code** button >
   **Codespaces** tab > **Create codespace on master**).
3. GitHub shows a "Create a new codespace" page. Change nothing: branch
   `master`, configuration "Stellar Workshop (Madrid)", 2-core. Press
   **Create codespace**. It runs on YOUR free monthly quota (120 core hours),
   not on anyone's card.
4. Wait. First boot takes a few minutes: the container installs Rust, the
   `wasm32v1-none` target and the Stellar CLI, and pre-builds both contracts.
   It is ready when the terminal at the bottom shows a normal prompt.

Then check it worked:

```sh
stellar --version
```

No GitHub account? Use the browser IDE at https://stellaride.dev as a
fallback, or install locally (do this before the session, not during):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32v1-none
cargo install --locked stellar-cli
```

## 2. Your first deploy: the guestbook (10 minutes)

The simplest possible contract: sign your name on-chain. Run these five
commands and you are live on testnet.

```sh
cd guestbook

# tests run locally against the Soroban host, no network needed
cargo test

# a funded testnet account: Friendbot gives it 10,000 test XLM, free
stellar keys generate --fund alice --network testnet

# compile Rust to WASM and deploy it
stellar contract build
stellar contract deploy \
  --wasm target/wasm32v1-none/release/guestbook.wasm \
  --source alice --network testnet --alias guestbook

# sign it, then count the signatures
stellar contract invoke --id guestbook --source alice --network testnet \
  -- sign --name '"YOUR_NAME"'
stellar contract invoke --id guestbook --source alice --network testnet \
  -- total
```

The deploy prints a contract ID starting with `C`. See yours live:

```
https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>
```

Total cost of all of the above: fractions of a cent, paid in XLM. Fees are
measured in stroops (1 stroop = 0.0000001 XLM, minimum 100 per operation).

## 3. The main build: milestone escrow

A real contract in ~120 lines of Rust: a payer locks money, an arbiter
releases it to the payee, and after a deadline the payer can take it back.
Ten tests and a one-command happy path:

```sh
cd ../escrow    # coming from guestbook/; from the repo root it is: cd escrow
cargo test
bash scripts/deploy-testnet.sh   # creates 3 accounts, deploys, locks, releases
```

Details, a function table and four extension challenges: [escrow/README.md](escrow/README.md).

The script IS the demo: it builds, deploys, locks 1 XLM in the escrow and has
the arbiter release it, then prints the contract's explorer link. A bare
`stellar contract deploy --wasm ...` only does the deploy step: you get an
address, but no money ever moves, so the explorer shows an empty contract.

Build too slow? Same full demo, no compilation:

```sh
USE_PREBUILT=1 bash scripts/deploy-testnet.sh
```

## 4. Build with AI

Generic models guess Soroban APIs and guess wrong, usually with an EVM accent.
Give your agent real Stellar context first:

```sh
# Claude Code
/plugin install stellar-dev@stellar-dev
claude mcp add --transport http stellar-raven "https://raven.stellar.buzz/mcp"
```

Cursor, Codex and others: grab the skills from https://skills.stellar.org.
The docs also ship an `llms.txt` at https://developers.stellar.org/llms.txt.

Then pick ONE exercise and do it agent-driven: prompt, review the diff,
`cargo test`, redeploy, invoke.

1. **Guestbook:** add a `signers` function returning every name, not just the
   last one (persistent storage plus `Vec<String>`).
2. **Guestbook:** reject empty names with a proper contract error
   (`#[contracterror]`).
3. **Guestbook:** let each account sign only once (`require_auth` plus
   `Address`).

Harder variant: add a second milestone to the escrow. Test first, then deploy.

## If something breaks

| Symptom | Fix |
| --- | --- |
| Friendbot rate-limits you | Fund the account at https://lab.stellar.org/account/fund |
| Codespace still installing | Work in https://stellaride.dev while it finishes |
| Build too slow | Deploy the prebuilt WASM (see section 3) |
| `stellar: command not found` | `source "$HOME/.cargo/env"`, then retry |
| Weird CLI errors | You are on testnet, right? `stellar network use testnet` |

Testnet resets quarterly (next: Dec 16 2026). Contracts and accounts vanish;
your code and this repo do not. Redeploying takes a minute.

## Links

* Docs: https://developers.stellar.org
* AI tooling: https://skills.stellar.org
* Explorer: https://stellar.expert/explorer/testnet
* Fund a testnet account by hand: https://lab.stellar.org/account/fund
* Stellar Lab (build and inspect transactions in the browser): https://lab.stellar.org
* Browser IDE, no installs: https://stellaride.dev
* Wallet (browser extension): https://freighter.app
* Stellar Developer Discord: https://discord.gg/st7Mxd58BV
* Funding after the hackathon (SCF): https://communityfund.stellar.org
* **HackMeridian, Lisbon, Oct 25-26:** https://www.hackmeridian.com. Travel
  support available, applications close Oct 19. What you deployed today is
  your starting point.
