#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Ledger as _},
    token, Address, Env,
};

struct Fixture {
    env: Env,
    client: MilestoneEscrowClient<'static>,
    token: token::Client<'static>,
    payer: Address,
    payee: Address,
    arbiter: Address,
}

const AMOUNT: i128 = 1_000;
const DEADLINE: u32 = 100;

fn setup() -> Fixture {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_sequence_number(10);

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let payee = Address::generate(&env);
    let arbiter = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    token::StellarAssetClient::new(&env, &sac.address()).mint(&payer, &AMOUNT);
    let token = token::Client::new(&env, &sac.address());

    let client = MilestoneEscrowClient::new(&env, &env.register(MilestoneEscrow, ()));

    Fixture {
        env,
        client,
        token,
        payer,
        payee,
        arbiter,
    }
}

fn init(f: &Fixture) {
    f.client.init(
        &f.payer,
        &f.payee,
        &f.arbiter,
        &f.token.address,
        &AMOUNT,
        &DEADLINE,
    );
}

#[test]
fn init_moves_funds_into_the_contract() {
    let f = setup();
    init(&f);

    assert_eq!(f.token.balance(&f.payer), 0);
    assert_eq!(f.token.balance(&f.client.address), AMOUNT);
    assert_eq!(f.client.state(), symbol_short!("Funded"));
}

#[test]
fn release_pays_the_payee() {
    let f = setup();
    init(&f);

    f.client.release();

    assert_eq!(f.token.balance(&f.payee), AMOUNT);
    assert_eq!(f.token.balance(&f.client.address), 0);
    assert_eq!(f.client.state(), symbol_short!("Released"));
}

#[test]
fn refund_before_the_deadline_is_rejected() {
    let f = setup();
    init(&f);

    assert_eq!(f.client.try_refund(), Err(Ok(Error::DeadlineNotReached)));
    assert_eq!(f.token.balance(&f.client.address), AMOUNT);
}

#[test]
fn refund_after_the_deadline_returns_the_money() {
    let f = setup();
    init(&f);
    f.env.ledger().set_sequence_number(DEADLINE);

    f.client.refund();

    assert_eq!(f.token.balance(&f.payer), AMOUNT);
    assert_eq!(f.client.state(), symbol_short!("Refunded"));
}

#[test]
fn cannot_release_twice() {
    let f = setup();
    init(&f);
    f.client.release();

    assert_eq!(f.client.try_release(), Err(Ok(Error::NotFunded)));
    assert_eq!(f.token.balance(&f.payee), AMOUNT);
}

#[test]
fn cannot_refund_after_release() {
    let f = setup();
    init(&f);
    f.client.release();
    f.env.ledger().set_sequence_number(DEADLINE);

    assert_eq!(f.client.try_refund(), Err(Ok(Error::NotFunded)));
}

#[test]
fn cannot_init_twice() {
    let f = setup();
    init(&f);

    assert_eq!(
        f.client.try_init(
            &f.payer,
            &f.payee,
            &f.arbiter,
            &f.token.address,
            &AMOUNT,
            &DEADLINE
        ),
        Err(Ok(Error::AlreadyInitialized))
    );
}

#[test]
fn amount_must_be_positive() {
    let f = setup();

    assert_eq!(
        f.client.try_init(
            &f.payer,
            &f.payee,
            &f.arbiter,
            &f.token.address,
            &0,
            &DEADLINE
        ),
        Err(Ok(Error::AmountNotPositive))
    );
}

#[test]
fn payer_and_payee_must_differ() {
    let f = setup();

    assert_eq!(
        f.client.try_init(
            &f.payer,
            &f.payer,
            &f.arbiter,
            &f.token.address,
            &AMOUNT,
            &DEADLINE
        ),
        Err(Ok(Error::SamePayerAndPayee))
    );
}

#[test]
fn release_requires_the_arbiter_signature() {
    let env = Env::default();
    env.ledger().set_sequence_number(10);

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let payee = Address::generate(&env);
    let arbiter = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    env.mock_all_auths();
    token::StellarAssetClient::new(&env, &sac.address()).mint(&payer, &AMOUNT);
    let token = token::Client::new(&env, &sac.address());
    let client = MilestoneEscrowClient::new(&env, &env.register(MilestoneEscrow, ()));
    client.init(&payer, &payee, &arbiter, &token.address, &AMOUNT, &DEADLINE);

    // Drop the mocked auths: nobody has signed anything from here on.
    env.set_auths(&[]);
    assert!(client.try_release().is_err());
    assert_eq!(token.balance(&payee), 0);
}
