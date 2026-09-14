#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, String, Symbol};

const COUNT: Symbol = symbol_short!("COUNT");
const LAST: Symbol = symbol_short!("LAST");

/// Emitted every time someone signs the guestbook.
#[contractevent]
#[derive(Clone)]
pub struct Signed {
    pub name: String,
    pub count: u32,
}

#[contract]
pub struct Guestbook;

#[contractimpl]
impl Guestbook {
    /// Sign the guestbook. Returns your signature number.
    pub fn sign(env: Env, name: String) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0) + 1;
        env.storage().instance().set(&COUNT, &count);
        env.storage().instance().set(&LAST, &name);
        // Contract state has rent on Stellar: keep this entry alive ~1 month
        // (ledgers close every ~5 seconds).
        env.storage().instance().extend_ttl(100_000, 500_000);
        Signed { name, count }.publish(&env);
        count
    }

    /// How many signatures the guestbook holds.
    pub fn total(env: Env) -> u32 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }

    /// The most recent signer, if anyone signed yet.
    pub fn last(env: Env) -> Option<String> {
        env.storage().instance().get(&LAST)
    }
}

mod test;
