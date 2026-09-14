#![no_std]

//! Milestone escrow: one payer locks tokens for one payee against one milestone.
//! An arbiter releases the funds. If nobody releases before the deadline ledger,
//! the payer can take the money back.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env,
    Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Config {
    pub payer: Address,
    pub payee: Address,
    pub arbiter: Address,
    pub token: Address,
    pub amount: i128,
    /// Ledger sequence from which refund() becomes available.
    pub deadline_ledger: u32,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Funded = 0,
    Released = 1,
    Refunded = 2,
}

#[contracttype]
pub enum DataKey {
    Config,
    State,
}

#[contracterror]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    AmountNotPositive = 3,
    NotFunded = 4,
    DeadlineNotReached = 5,
    SamePayerAndPayee = 6,
}

#[contract]
pub struct MilestoneEscrow;

#[contractimpl]
impl MilestoneEscrow {
    /// Creates the escrow and pulls `amount` from `payer` into this contract.
    pub fn init(
        env: Env,
        payer: Address,
        payee: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
        deadline_ledger: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }
        if amount <= 0 {
            return Err(Error::AmountNotPositive);
        }
        if payer == payee {
            return Err(Error::SamePayerAndPayee);
        }
        payer.require_auth();

        token::Client::new(&env, &token).transfer(
            &payer,
            &env.current_contract_address(),
            &amount,
        );

        let cfg = Config {
            payer,
            payee,
            arbiter,
            token,
            amount,
            deadline_ledger,
        };
        env.storage().instance().set(&DataKey::Config, &cfg);
        env.storage().instance().set(&DataKey::State, &State::Funded);
        Ok(())
    }

    /// Arbiter pays the payee. Callable at any time while the escrow is funded.
    pub fn release(env: Env) -> Result<(), Error> {
        let cfg = Self::config(env.clone())?;
        Self::require_funded(&env)?;
        cfg.arbiter.require_auth();

        token::Client::new(&env, &cfg.token).transfer(
            &env.current_contract_address(),
            &cfg.payee,
            &cfg.amount,
        );
        env.storage()
            .instance()
            .set(&DataKey::State, &State::Released);
        Ok(())
    }

    /// Payer takes the money back, only from `deadline_ledger` onwards.
    pub fn refund(env: Env) -> Result<(), Error> {
        let cfg = Self::config(env.clone())?;
        Self::require_funded(&env)?;
        if env.ledger().sequence() < cfg.deadline_ledger {
            return Err(Error::DeadlineNotReached);
        }
        cfg.payer.require_auth();

        token::Client::new(&env, &cfg.token).transfer(
            &env.current_contract_address(),
            &cfg.payer,
            &cfg.amount,
        );
        env.storage()
            .instance()
            .set(&DataKey::State, &State::Refunded);
        Ok(())
    }

    /// Returns `Funded`, `Released` or `Refunded` as a symbol, so the CLI prints
    /// a word instead of an integer.
    pub fn state(env: Env) -> Result<Symbol, Error> {
        Ok(match Self::raw_state(&env)? {
            State::Funded => symbol_short!("Funded"),
            State::Released => symbol_short!("Released"),
            State::Refunded => symbol_short!("Refunded"),
        })
    }

    pub fn config(env: Env) -> Result<Config, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(Error::NotInitialized)
    }

    fn raw_state(env: &Env) -> Result<State, Error> {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .ok_or(Error::NotInitialized)
    }

    fn require_funded(env: &Env) -> Result<(), Error> {
        if Self::raw_state(env)? != State::Funded {
            return Err(Error::NotFunded);
        }
        Ok(())
    }
}

mod test;
