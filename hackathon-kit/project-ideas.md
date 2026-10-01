# Project Ideas for Stellar Hackathons

Thirty starting points for a team that knows it wants to build on Stellar but
not yet what. Each idea names a user, a problem, a scope you can ship in two
days and a demo a judge can verify on the explorer. Made for HackMeridian
Lisbon (October 25 and 26, 2026), useful for any Stellar hackathon.

These are starting points, not assignments. The best project in the room is
usually one of these bent towards a user you actually know. Take one, change
the user, cut the scope, then run it through page 1 of the
[Project & Evidence Canvas](01-project-evidence-canvas.pdf).

## Before you pick

1. **Choose where Stellar is strong.** Payments, stablecoins, fiat ramps
   through anchors, tokenized assets, agent payments. If your idea works the
   same on any chain, the "why Stellar" line on the Canvas will be weak.
2. **Check it is not already built.** Search past Stellar hackathons on
   DoraHacks, the [Stellar Community Fund awards](https://communityfund.stellar.org/awards)
   and [stellar.org/ecosystem](https://stellar.org/ecosystem). From inside
   your agent, the Stellar Scout skill on
   [skills.stellar.org](https://skills.stellar.org) and the Raven MCP server
   check an idea against ecosystem projects and past hackathon builds.
   Already built? Narrow the user or change the angle. Every idea below names
   the closest prior art we found on October 1, 2026.
3. **Reuse what is live.** Blend, Soroswap, DeFindex, Reflector, Trustless
   Work and OpenZeppelin's Stellar contracts are already deployed and
   documented. Gluing two of them into a product for one user often beats
   writing a protocol from scratch.
4. **Finish with proof.** Every idea below ends in something a judge can click:
   a contract ID, a transaction, a state change. Plan the demo first.

## How to read an idea

- **For:** the user, narrow enough to decide anything.
- **Problem:** what goes wrong today.
- **Build:** what ships in two days. Everything else is roadmap.
- **Demo:** the journey on stage and what the judge can check.
- **Stellar pieces:** what you use and why it matters.
- **Make it yours:** a stretch goal or a twist.
- **Prior art:** the closest existing project or hackathon build, when there
  is one. Read it before you start.
- **Track** (Genesis or Scale) · **Difficulty** (1 easy to 3 hard) · **Starts
  from**, when a contract in this repo gives you a head start.

## All ideas at a glance

| # | Idea | Category | Track | Difficulty | Starts from |
|---|---|---|---|---|---|
| 1 | Rental deposits that come back | Escrow | Genesis | 1 | `escrow/` |
| 2 | Renovation milestones | Escrow | Genesis | 2 | `escrow/` |
| 3 | No-show deposits for events | Escrow | Genesis | 1 | `escrow/` |
| 4 | Grants in tranches | Escrow | Genesis or Scale | 2 | `escrow/` |
| 5 | Bounties that pay on merge | Escrow | Genesis | 2 | `escrow/` |
| 6 | Cooperative payouts | Payments | Genesis | 1 | |
| 7 | Market stall QR payments | Payments | Genesis | 1 | |
| 8 | Remittance with cash-out | Payments | Genesis | 3 | |
| 9 | Small team payroll in stablecoins | Payments | Genesis or Scale | 2 | |
| 10 | Shared flat settle-up | Payments | Genesis | 1 | |
| 11 | Pay-per-call data API | Agent payments | Genesis | 2 | |
| 12 | Agent wallet with a budget | Agent payments | Genesis | 2 | |
| 13 | Metered service on payment channels | Agent payments | Scale | 3 | |
| 14 | Agent hires a human | Agent payments | Genesis | 2 | `escrow/` |
| 15 | Passkey wallet for first-time users | Smart accounts | Genesis | 2 | |
| 16 | Shared account for a residents' association | Smart accounts | Genesis | 2 | |
| 17 | Session keys for an app or game | Smart accounts | Genesis | 2 | |
| 18 | Social recovery | Smart accounts | Scale | 3 | |
| 19 | Pay from any chain, get paid on Stellar | Cross-chain | Scale | 3 | |
| 20 | Cross-chain checkout for merchants | Cross-chain | Scale | 3 | |
| 21 | Savings goal that earns yield | Composability | Genesis | 2 | |
| 22 | Treasury autopilot for an association | Composability | Scale | 2 | |
| 23 | Invoice financing for small suppliers | Tokenized assets | Scale | 3 | |
| 24 | Pay a euro price in USDC | Composability | Genesis | 2 | `escrow/` |
| 25 | A wizard for a live protocol | Composability | Genesis | 2 | |
| 26 | Rent and TTL watchdog | Dev tooling | Genesis | 2 | |
| 27 | Contract cost report in CI | Dev tooling | Genesis | 2 | |
| 28 | Testnet reset survival kit | Dev tooling | Genesis | 1 | `guestbook/` |
| 29 | Aid vouchers for approved shops | Public goods | Genesis | 2 | |
| 30 | Prove a fact without revealing it | ZK | Scale | 3 | |

## Escrow and milestone payments

The [`escrow/`](../escrow/) contract in this repo already locks funds, lets an
arbiter release them and refunds after a deadline. These five ideas start from
it. Trustless Work runs escrow infrastructure in production on Stellar, with a
skill and a React library: decide early whether you extend this repo's
contract or build on theirs, and say which in your README.

### 1. Rental deposits that come back

- **For:** tenants and small landlords renting flats in Spain.
- **Problem:** the deposit sits with the landlord for weeks after move-out,
  deductions are argued over WhatsApp and nothing records what was agreed.
- **Build:** a contract that holds the deposit in a stablecoin. At move-out
  the landlord proposes a deduction, the tenant accepts, and the contract
  splits the funds in one call. If nobody acts by a deadline, the tenant gets
  the full deposit back.
- **Demo:** lock the deposit, propose a 50 EUR deduction, accept, show two
  payments on the explorer. Then show the deadline path returning everything.
- **Stellar pieces:** Soroban contract, `require_auth` for both parties, a
  stablecoin through its Stellar Asset Contract.
- **Make it yours:** add an arbiter for disputes, or a photo checklist whose
  hash is stored at move-in.
- **Prior art:** SafeTrust, a Stellar hackathon build, escrows deposits for
  hotels and holiday rentals. Long-term flat rentals, with their deduction
  negotiation, are the gap.
- Genesis · 1 · starts from `escrow/` (add a partial split to `release`)

### 2. Renovation milestones

- **For:** homeowners paying a contractor for a renovation in stages.
- **Problem:** payments in advance leave the owner exposed; payments after
  completion leave the contractor exposed. Both sides distrust the other.
- **Build:** a multi-milestone escrow. The owner funds the whole budget, the
  contractor marks a stage done, the owner approves and that stage pays out.
- **Demo:** three milestones, approve the first, show the payout. Try to
  approve it twice and show the rejected call.
- **Stellar pieces:** contract state per milestone, contract errors as
  `#[contracterror]`, stablecoin payouts in seconds.
- **Make it yours:** a technical inspector as a second signer above a set
  amount.
- Genesis · 2 · starts from `escrow/` (the "second milestone" challenge in its
  README)

### 3. No-show deposits for events

- **For:** organizers of free meetups, workshops or restaurant group bookings.
- **Problem:** free events see a large share of sign-ups not turn up, and the
  organizer cannot plan food or space.
- **Build:** attendees lock a small refundable deposit at sign-up. Check-in at
  the door releases it back. No-shows fund the next event.
- **Demo:** three sign-ups, two check in through a QR scan, the third
  deposit moves to the organizer after the event ends.
- **Stellar pieces:** fees of fractions of a cent make a 2 EUR deposit
  viable, which is the whole point.
- **Make it yours:** donate no-show deposits to a cause the attendees chose.
- Genesis · 1 · starts from `escrow/`

### 4. Grants in tranches

- **For:** small foundations, city councils or communities that fund local
  projects.
- **Problem:** grant money goes out in one payment and reporting comes later,
  if ever.
- **Build:** a grant contract with tranches tied to reported deliverables. A
  reviewer approves each report, the tranche is released, and a public page
  shows what was paid for what.
- **Demo:** approve tranche one, show the payment and the public record.
- **Stellar pieces:** this is the shape the Stellar Community Fund itself
  uses (tranches tied to MVP, testnet and mainnet), so you can cite it.
- **Make it yours:** quadratic matching for community-voted projects.
- **Prior art:** grant platforms such as GrantFox and DeGrant appear among
  past Stellar hackathon builds. Differ on the funder: a city council or a
  neighbourhood fund, not crypto grants.
- Genesis or Scale · 2 · starts from `escrow/`

### 5. Bounties that pay on merge

- **For:** maintainers of small open-source projects.
- **Problem:** bounties are promised in an issue comment and paid by hand,
  late.
- **Build:** a maintainer funds an issue. A small backend listens for the
  GitHub merge event and acts as the arbiter that calls `release` to the
  contributor's address.
- **Demo:** fund a bounty, merge a pull request on a test repo, watch the
  payment land.
- **Stellar pieces:** escrow contract, a backend signer as arbiter, testnet
  stablecoin.
- **Make it yours:** split one bounty between several contributors.
- **Prior art:** DevAsign automates open-source bounties on Stellar, and
  Drips runs the Drips Wave bounty program around HackMeridian. Check both
  and differ from them.
- Genesis · 2 · starts from `escrow/`

## Payments and stablecoins

Moving money across borders cheaply and settling it in seconds is what Stellar
was built for. USDC and EURC run natively on Stellar, and anchors turn
stablecoins into local currency through the SEP standards.

### 6. Cooperative payouts

- **For:** agricultural cooperatives (olive oil, wine, fruit) paying members
  for what they delivered.
- **Problem:** settlements are monthly spreadsheets and bank transfers, and
  members cannot see how their payout was calculated.
- **Build:** the cooperative uploads a delivery sheet, the app computes each
  member's share and pays everyone in a single transaction.
- **Demo:** upload ten rows, send one transaction with ten payment
  operations, show it on the explorer next to each member's statement.
- **Stellar pieces:** up to 100 operations in one classic transaction, no
  contract needed for the payout. Add a contract only if you need rules.
- **Make it yours:** hold back a retention percentage in a contract until the
  harvest closes.
- **Prior art:** SDF's [Stellar Disbursement Platform](https://developers.stellar.org/docs/platforms/stellar-disbursement-platform)
  already makes bulk payments for organizations. Build the cooperative part
  (deliveries to shares to statements), and consider the platform underneath.
- Genesis · 1

### 7. Market stall QR payments

- **For:** stalls at street markets and fairs.
- **Problem:** card terminals cost a monthly fee, cash needs change, and
  nobody reconciles at the end of the day.
- **Build:** the vendor shows a QR with the amount, the buyer pays from a
  phone wallet, and the vendor's screen confirms and tallies the day.
- **Demo:** two purchases from a phone, the vendor dashboard updating, a
  daily summary with transaction links.
- **Stellar pieces:** SEP-7 payment URIs in the QR, a stablecoin, five
  second finality.
- **Make it yours:** a cash-out button through an anchor at the end of the
  day (simulated on testnet, labeled as such).
- Genesis · 1

### 8. Remittance with cash-out

- **For:** people in Spain sending money to family in Latin America.
- **Problem:** fees and exchange margins take a visible cut of every
  transfer.
- **Build:** send a stablecoin to a recipient who cashes out to local
  currency through an anchor. On testnet, use SDF's reference anchor at
  testanchor.stellar.org for the interactive deposit and withdrawal flow.
- **Demo:** send, then walk the recipient through the anchor's withdrawal
  screens. Show what is real and what is the test anchor.
- **Stellar pieces:** SEP-10 authentication, SEP-24 interactive
  deposit and withdrawal, the [Anchor Platform](https://developers.stellar.org/docs/platforms/anchor-platform).
- **Make it yours:** pick one real corridor and research which anchors serve
  it; that research is evidence for the judges.
- **Prior art:** remittances are one of the most attempted ideas at Stellar
  hackathons, with several placed builds. One corridor, one real user and
  real anchor research are what set yours apart. SEP-31 covers
  business-to-business corridors; SEP-12 covers the KYC step.
- Genesis · 3

### 9. Small team payroll in stablecoins

- **For:** small remote teams paying contractors in several countries.
- **Problem:** each international payment has its own fee, delay and
  paperwork.
- **Build:** a payroll contract funded monthly. Each contractor claims what
  has accrued so far, so pay flows by the day rather than once a month.
- **Demo:** fund the contract, fast-forward ledgers in the demo, claim, show
  the accrued amount moving.
- **Stellar pieces:** contract math on ledger sequence, per-user persistent
  storage, a stablecoin.
- **Make it yours:** payslips as signed PDFs with the transaction hash.
- **Prior art:** payroll products already on Stellar include PayZoll and
  Bloccpay, and Drips does continuous payment streams. Pick a team type they
  do not serve.
- Genesis or Scale · 2

### 10. Shared flat settle-up

- **For:** students and young workers sharing a flat.
- **Problem:** expense apps track who owes whom, then everyone settles by
  hand and forgets.
- **Build:** log expenses, compute the minimum set of payments, settle them
  in one transaction that every debtor signs.
- **Demo:** four flatmates, six expenses, one settlement transaction on the
  explorer.
- **Stellar pieces:** a multi-signer classic transaction, near-zero fees.
- **Make it yours:** this idea is common at hackathons. Win on the user:
  Erasmus students in one city, with euros on one side and home currencies
  on the other.
- Genesis · 1

## Agent payments

AI agents need to pay for things without accounts or API keys. On Stellar two
protocols do this: x402, where a request returns HTTP 402 with a price and
the agent pays to get the resource, and MPP, with one-off charges or payment
channels for high-frequency use. The official Agent Payments skill covers
both. Start at [developers.stellar.org/docs/build/agentic-payments](https://developers.stellar.org/docs/build/agentic-payments).

One warning from past events: SDF saw four or five near-identical agent
registries in a single hackathon. Do not build a directory of agents. Build
something an agent pays for to finish a real task.

### 11. Pay-per-call data API

- **For:** anyone sitting on a dataset that agents would pay for in small
  amounts: local transit delays, property listings, translated legal texts.
- **Problem:** selling data per request means accounts, keys and billing that
  cost more than the request.
- **Build:** wrap one useful endpoint behind x402. Without payment it answers
  402 with a price; an agent pays and gets the data.
- **Demo:** the agent hits the endpoint, receives 402, pays, gets the answer.
  Show the payment on the explorer.
- **Stellar pieces:** x402 with a facilitator (the x402.org one needs no
  key), USDC through its Stellar Asset Contract. The paying client needs no
  XLM because the facilitator sponsors fees.
- **Make it yours:** the data matters more than the paywall. Pick a dataset
  you can explain in one sentence.
- **Prior art:** x402 paywalls and paid MCP templates are among the most
  common Stellar hackathon builds, so the paywall alone will not stand out.
  An open SCF request for proposals asks for an x402 facilitator with
  discovery, if you want the infrastructure angle instead.
- Genesis · 2

### 12. Agent wallet with a budget

- **For:** a team that lets an agent buy things on its behalf.
- **Problem:** handing an agent a funded key means trusting it with all of
  the balance.
- **Build:** a contract account that enforces limits: per merchant, per day,
  per transaction. The agent can spend inside the limits and nothing else.
- **Demo:** one valid payment goes through; an overspend is rejected by the
  contract on stage.
- **Stellar pieces:** smart account policies, spending-limit policy from the
  Smart Account Kit.
- **Prior art:** crowded. Eunomia's Bounded Agent Treasury skill, Policywright
  and REAPP all work on agent spending rules, and SpendGuard and AgentCard
  were hackathon builds. Win on one concrete buyer, not on the policy engine.
- **Make it yours:** a human approval step above a threshold, sent to the
  owner's phone.
- Genesis · 2

### 13. Metered service on payment channels

- **For:** providers of a service billed by the second or by the token: GPU
  time, inference, scraping.
- **Problem:** paying on-chain per call is too frequent; prepaid credits lock
  the buyer in.
- **Build:** an MPP session. The client opens a channel, pays off-chain per
  unit consumed, and the channel settles on-chain once.
- **Demo:** run a job that consumes 200 units, show the running count, close
  the session, show one settlement transaction.
- **Stellar pieces:** MPP session mode, which needs its channel contract
  deployed first and a client with XLM for fees, plus USDC.
- **Make it yours:** let the client cap the session in advance.
- Scale · 3

### 14. Agent hires a human

- **For:** agents that hit a step only a person can do: a phone call, a photo
  of a shop front, a translation check.
- **Problem:** there is no simple way for an agent to commission and pay a
  person safely.
- **Build:** the agent posts a task and locks payment in escrow; a person
  accepts, submits proof, and the agent (or a reviewer) releases payment.
- **Demo:** agent posts, human submits, payment released on the explorer.
- **Stellar pieces:** escrow contract, an agent wallet, stablecoin payout.
- **Make it yours:** reputation from completed tasks stored on-chain.
- Genesis · 2 · starts from `escrow/`

## Smart accounts and onboarding

Smart accounts are one of HackMeridian 2026's stated priorities. A smart
account is a contract that acts as a wallet, with its own rules: passkeys
instead of seed phrases, several signers, spending policies, sponsored fees.
The [Smart Account Kit](https://github.com/stellar/smart-account-kit)
(TypeScript, built on OpenZeppelin's audited Stellar accounts) gives you
passkeys, multi-signer setups, context rules and policies, with a demo app.
It is pre-1.0, which is fine on testnet. Fees are sponsored through the
OpenZeppelin Relayer, which replaced the deprecated Launchtube. Passkey Kit is
a sibling SDK with a simpler signer model; the two are not interchangeable,
so pick one before you start.

### 15. Passkey wallet for first-time users

- **For:** people who have never used a crypto wallet and will not write down
  twelve words.
- **Problem:** seed phrases and buying XLM for fees lose most new users before
  their first payment.
- **Build:** sign up with Face ID or a fingerprint, receive a first payment,
  send it on. Fees sponsored by the app.
- **Demo:** a new user, a phone, no extension, a payment received in under a
  minute.
- **Stellar pieces:** passkeys (secp256r1) verified on-chain, the
  OpenZeppelin Relayer for fee sponsorship.
- **Prior art:** passkey wallets are a frequent hackathon build, with a few
  placed ones. The first use you pick is the differentiator, not the wallet.
- **Make it yours:** pick a real first use: a freelancer's first invoice, a
  student's first scholarship payment.
- Genesis · 2

### 16. Shared account for a residents' association

- **For:** a homeowners' association, a club or a parents' association.
- **Problem:** one treasurer holds the bank access and everyone else has to
  trust the spreadsheet.
- **Build:** a shared smart account. Small payments need one signer; anything
  above a threshold needs two of three. Every movement is visible to members.
- **Demo:** a 30 EUR payment with one signature goes through; a 900 EUR
  payment waits for the second signer.
- **Stellar pieces:** multi-signer smart account, threshold policy, read-only
  member view through RPC.
- **Make it yours:** attach the receipt hash to each payment.
- Genesis · 2

### 17. Session keys for an app or game

- **For:** apps where users take many small actions: a game, a tipping
  feature, a voting app.
- **Problem:** a wallet pop-up for every action kills the experience.
- **Build:** the user approves once, which creates a short-lived key limited
  to one contract and one amount. Actions inside the limit need no pop-up.
- **Demo:** approve once, take ten actions, then show an action outside the
  limit being rejected.
- **Stellar pieces:** an OpenZeppelin context rule scoped to one contract
  (`CallContract`) with a `Valid Until` ledger, so the permission expires on
  its own.
- **Make it yours:** show the session's remaining budget live on screen.
- Genesis · 2

### 18. Social recovery

- **For:** anyone who loses a phone with their only passkey on it.
- **Problem:** passkeys fix seed phrases, but a lost device still means a
  lost wallet unless recovery is designed in.
- **Build:** the account names guardians. If the owner loses access, enough
  guardians approve a new key after a time delay, during which the owner can
  cancel.
- **Demo:** lose the device, two of three guardians approve, wait out a
  shortened delay, the new passkey takes control.
- **Stellar pieces:** custom smart account policy, `require_auth` on
  guardians, ledger-based delays.
- **Make it yours:** recovery through a trusted institution as one guardian.
- Scale · 3

## Cross-chain

Cross-chain transfers are the third HackMeridian 2026 priority. The official
Cross Chain skill covers four rails: Circle's CCTP V2 for native USDC by burn
and mint (live on Stellar since May 2026), Axelar for messages and
interchain tokens, LayerZero for USDT0, and NEAR Intents for swaps from any
chain. Allbridge has its own SDK skill. Three traps catch most teams: USDC has
7 decimals on Stellar and 6 everywhere else, a `G...` recipient needs a USDC
trustline before funds can land, and NEAR Intents and USDT0 have no testnet.
CCTP does run on testnet, with test USDC from faucet.circle.com.

### 19. Pay from any chain, get paid on Stellar

- **For:** freelancers whose clients hold USDC on another chain.
- **Problem:** the client pays where they are; the freelancer wants funds on
  Stellar, next to an anchor that cashes out.
- **Build:** a payment link. The client pays USDC on their chain; it arrives
  on Stellar through CCTP and lands in the freelancer's account.
- **Demo:** pay on a testnet of another CCTP chain, show the mint on Stellar.
- **Stellar pieces:** CCTP with the `CctpForwarder` contract, which every
  Stellar recipient needs, native USDC, an anchor for the next step. Start
  from [ElliotFriend/stellar-cctp-demo](https://github.com/ElliotFriend/stellar-cctp-demo),
  which bridges testnet USDC between Stellar, Base Sepolia, Arc and Solana.
- **Make it yours:** route the arriving USDC straight into an escrow (idea 2).
- Scale · 3

### 20. Cross-chain checkout for merchants

- **For:** online shops that want stablecoin payments without caring which
  chain the buyer uses.
- **Problem:** each chain is another wallet, another balance, another
  reconciliation.
- **Build:** a checkout that accepts USDC from several chains and settles
  everything to the merchant on Stellar, with one daily report.
- **Demo:** two purchases from two chains, one merchant balance on Stellar.
- **Stellar pieces:** CCTP, a settlement contract, SEP-7 or a wallet kit on
  the buyer side.
- **Make it yours:** automatic refunds to the chain the buyer paid from.
- **Prior art:** Rozo Intent Pay already ships a pay-from-any-chain button
  for Stellar merchants. Differ on the merchant side: settlement, reporting,
  refunds.
- Scale · 3

## Composability and tokenized assets

Lessons from past winners: The Simple Fund won Rio 2025 with an on-chain
receivables fund, and the Composability track asked teams to build on Blend,
Soroswap, DeFindex, Kale and Reflector. The Blend hackathon was won by a
wizard that made creating lending pools easy. Making a live protocol usable
for one user is a product.

### 21. Savings goal that earns yield

- **For:** people saving for one thing: a trip, a deposit, a course.
- **Problem:** savings in a current account earn nothing and are easy to
  spend.
- **Build:** a goal vault. Deposits go into a yield vault, withdrawals are
  locked until the goal date or the target amount.
- **Demo:** set a goal, deposit, show the position in the underlying vault,
  try an early withdrawal and show it rejected.
- **Stellar pieces:** DeFindex or Blend underneath (both have skills or
  SDKs), a thin lock contract on top.
- **Make it yours:** group goals, where friends save towards one trip.
- Genesis · 2

### 22. Treasury autopilot for an association

- **For:** associations, clubs and small DAOs with idle funds.
- **Problem:** money sits idle because moving it in and out of yield is
  manual and nobody owns the job.
- **Build:** rules set by the board: keep three months of expenses liquid,
  put the rest in a lending pool, pull back when the balance runs low.
- **Demo:** trigger a rule and show funds moving into Blend and back.
- **Stellar pieces:** Blend, Reflector for prices, a rules contract or a
  scheduled backend signer.
- **Make it yours:** a monthly report the board can read without a wallet.
- Scale · 2

### 23. Invoice financing for small suppliers

- **For:** small suppliers waiting 60 or 90 days to be paid by large
  customers.
- **Problem:** cash is stuck in unpaid invoices and bank factoring is slow and
  expensive for small amounts.
- **Build:** tokenize one approved invoice, let an investor fund it at a
  discount, and pay the investor when the customer settles.
- **Demo:** issue, fund, settle, with all three steps on the explorer.
- **Stellar pieces:** a Stellar asset or contract token per invoice,
  stablecoin settlement, the Stellar Asset Contract.
- **Make it yours:** price the discount from the customer's on-chain payment
  history.
- **Prior art:** close to The Simple Fund, a past winner, and to
  InvoiceMate's DeFa on Stellar. Differ on the user (one sector, one country)
  and say so.
- Scale · 3

### 24. Pay a euro price in USDC

- **For:** sellers who price in euros but want to accept USDC.
- **Problem:** the exchange rate moves between quote and payment, and
  someone loses the difference.
- **Build:** an invoice contract that reads the EUR/USD rate from an oracle at
  payment time and checks the amount paid matches the euro price.
- **Demo:** issue a 100 EUR invoice, pay it in USDC, show the rate used and the
  contract accepting the payment. Underpay and show the rejection.
- **Stellar pieces:** Reflector, whose feeds include foreign exchange rates
  (Band, DIA and Pyth also list Stellar support), a stablecoin, contract
  errors.
- **Make it yours:** settle in EURC when the payer has it.
- Genesis · 2 · starts from `escrow/`

### 25. A wizard for a live protocol

- **For:** users of a Stellar protocol who give up halfway through its
  interface.
- **Problem:** powerful protocols are often hard to use for the first time.
- **Build:** pick one protocol and one task (open a Soroswap position,
  create a DeFindex vault, borrow against a Blend position) and make it a
  three-step guided flow. Blend pool creation already has its winner, Blend
  Pool Creator, so do not repeat it.
- **Demo:** a non-expert completes the task on stage in under a minute.
- **Stellar pieces:** the protocol's own SDK or skill.
- **Make it yours:** ask the protocol team on site what their users get
  stuck on. That conversation is evidence.
- Genesis · 2

## Developer tooling

Sharp tools that remove one known pain won the Dev Tooling track at the
São Paulo Builder Summit in 2026. Stellar has pains EVM developers have never
met, and those are the openings.

### 26. Rent and TTL watchdog

- **For:** teams with contracts on testnet or mainnet.
- **Problem:** Soroban state has a time to live. When it lapses, temporary
  entries are deleted and persistent ones are archived until someone restores
  them, and the app breaks in a way EVM developers do not expect.
- **Build:** a service that watches a list of contracts, reports each
  ledger entry's remaining TTL and extends it before it lapses.
- **Demo:** a contract with a short TTL, the alert, the automatic extension
  transaction.
- **Stellar pieces:** RPC `getLedgerEntries`, `extend_ttl`, the rent model.
- **Make it yours:** a cost forecast: what keeping this contract alive costs
  per year.
- **Prior art:** small scripts exist (luanlabs/stellar-extend-entry-ttl). A
  watching service with alerts is still open.
- Genesis · 2

### 27. Contract cost report in CI

- **For:** teams shipping Soroban contracts through pull requests.
- **Problem:** a change that doubles CPU or storage cost goes unnoticed until
  users pay for it.
- **Build:** a GitHub Action that simulates each contract function and posts
  a comment with resource and fee changes against the main branch.
- **Demo:** open a pull request with a costly change and show the comment.
- **Stellar pieces:** transaction simulation through RPC, resource fees.
- **Make it yours:** fail the build above a threshold the team sets.
- **Prior art:** soroban-cost-linter checks costs statically and SoroScope
  visualizes them. The per-pull-request diff is the gap.
- Genesis · 2

### 28. Testnet reset survival kit

- **For:** every hackathon team after the next testnet reset (the next one is
  scheduled for December 16, 2026).
- **Problem:** a reset wipes accounts and contracts, and demos break on the
  day someone wants to see them.
- **Build:** one command that recreates accounts, funds them, redeploys
  contracts, reseeds state and updates the frontend config with the new IDs.
- **Demo:** delete everything, run the command, the app works again.
- **Stellar pieces:** Stellar CLI, Friendbot, contract aliases.
- **Make it yours:** a GitHub Action that runs it after every reset.
- **Prior art:** we found no hackathon build that does this.
- Genesis · 1 · try it on `guestbook/`

## Public goods and ZK

### 29. Aid vouchers for approved shops

- **For:** NGOs and local councils distributing emergency aid.
- **Problem:** cash aid is hard to trace; paper vouchers are slow and easy to
  forge.
- **Build:** a voucher token that recipients can only spend at shops on an
  allowlist. Shops cash out; the funder sees spending by category.
- **Demo:** issue vouchers, a recipient pays an approved shop, a payment to a
  non-approved address is rejected.
- **Stellar pieces:** an asset with `AUTH_REQUIRED` (the issuer approves who
  may hold it) and clawback, or a contract with an allowlist, plus stablecoin
  backing.
- **Prior art:** SDF's Stellar Disbursement Platform already runs aid
  payouts, and AidShield and AidOS were hackathon builds. The shop-side
  restriction is the angle.
- **Make it yours:** an offline-friendly flow for recipients without
  smartphones.
- Genesis · 2

### 30. Prove a fact without revealing it

- **For:** services that need to know something about a user, not who they
  are: over 18, resident in a city, member of a group.
- **Problem:** proving one fact today means handing over a full identity
  document.
- **Build:** a user generates a zero-knowledge proof of one fact off-chain; a
  contract verifies it and grants access.
- **Demo:** a valid proof unlocks the action, a forged one is rejected.
- **Stellar pieces:** Groth16 verifies with a small contract on BLS12-381 or
  BN254; Noir's UltraHonk needs a dedicated verifier contract (Protocol 26
  and later). The official ZK Proofs skill walks through Circom, Noir and
  RISC Zero.
- **Make it yours:** keep the circuit small. One fact, done well.
- **Prior art:** crowded. The Stellar Hacks: Real-World ZK event produced
  dozens of privacy builds, ProofPass among them for age checks. Pick a fact
  nobody has proved yet.
- Scale · 3

## Ideas to think twice about

These are not banned, but each needs a strong answer to "why this, why
Stellar, why you":

- **Agent registries and directories.** SDF counted four or five near
  identical registries at a single hackathon.
- **Another x402 paywall, agent spending guard or ZK private payment.** All
  three are now common at Stellar hackathons; each needs a sharp user to
  stand out.
- **DEX or AMM clones.** Soroswap, Aquarius, Phoenix and the native DEX exist.
  Build on them instead.
- **NFT marketplaces and collectible drops.** Stellar's strengths are
  elsewhere.
- **A token with no user.** A launchpad, a meme coin or a governance token
  without someone who needs it.
- **"Blockchain for X" without a step that breaks without the chain.** If a
  database does the job, the judges will notice.

## Make any idea stronger

- Talk to three people who have the problem before the event, and write down
  what they said. That is your evidence row on the Canvas.
- Cut to one journey that ends in a transaction. Everything else goes on a
  roadmap slide.
- Set up your agent with the official Stellar skills and Raven first (see
  section 4 of the [main README](../README.md#4-build-with-ai)). Generic
  models guess Soroban APIs wrong.
- Say plainly what you reused, what you built at the event and what is
  simulated.
- Use the [Demo & Pitch Playbook](02-demo-pitch-playbook.pdf) and the
  [Submission Checklist](03-submission-checklist.pdf) before you submit.

## Sources

- HackMeridian Lisbon [FAQ](https://www.hackmeridian.com/faq) and [tracks](https://www.hackmeridian.com/build)
- [Meridian 2025 winners recap](https://stellar.org/blog/foundation-news/the-blueprint-at-meridian-2025)
- [Stellar Builder Summit São Paulo, Aug 13 2026](https://developers.stellar.org/meetings/2026/08/13)
- [SDF developer meeting, Apr 23 2026](https://developers.stellar.org/meetings/2026/04/23) (agent registries)
- [Agentic payments, Stellar docs](https://developers.stellar.org/docs/build/agentic-payments)
- [Stellar skills directory](https://skills.stellar.org/) and the official
  [stellar-dev-skill](https://github.com/stellar/stellar-dev-skill)
  (agentic payments, cross-chain, dApp, ZK)
- [OpenZeppelin context rules](https://docs.openzeppelin.com/stellar-contracts/accounts/context-rules)
- [Stellar Disbursement Platform, Stellar docs](https://developers.stellar.org/docs/platforms/stellar-disbursement-platform)
- [SEP-7, Stellar docs](https://developers.stellar.org/docs/build/apps/wallet/sep7)
- [State archival, Stellar docs](https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival)
- Prior art: Stellar Light Scout (hackathon builds, projects, RFPs) and
  LumenLoop, queried through the Raven MCP server on October 1, 2026
- [Smart Account Kit](https://github.com/stellar/smart-account-kit)
- [Circle CCTP is live on Stellar](https://stellar.org/blog/foundation-news/circle-cctp-is-live-on-stellar)
- [Oracle providers, Stellar docs](https://developers.stellar.org/docs/data/oracles/oracle-providers)
- [Anchor Platform, Stellar docs](https://developers.stellar.org/docs/platforms/anchor-platform)
- [Fees, resource limits and metering, Stellar docs](https://developers.stellar.org/docs/learn/fundamentals/fees-resource-limits-metering)
- [Networks and testnet resets, Stellar docs](https://developers.stellar.org/docs/networks)
- [Trustless Work](https://www.trustlesswork.com/)
- [Stellar Community Fund awards](https://communityfund.stellar.org/awards)

Facts checked on October 1, 2026. Protocol support and testnet deployments
change: confirm before you build your demo on them.

Stellar Workshop Spain · Your Way 2026 · Region 04
