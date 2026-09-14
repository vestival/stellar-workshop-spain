#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn sign_increments_and_records_last() {
    let env = Env::default();
    let contract_id = env.register(Guestbook, ());
    let client = GuestbookClient::new(&env, &contract_id);

    assert_eq!(client.total(), 0);
    assert_eq!(client.last(), None);

    let ada = String::from_str(&env, "Ada");
    assert_eq!(client.sign(&ada), 1);

    let linus = String::from_str(&env, "Linus");
    assert_eq!(client.sign(&linus), 2);

    assert_eq!(client.total(), 2);
    assert_eq!(client.last(), Some(linus));
}
