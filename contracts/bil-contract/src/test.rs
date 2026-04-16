#![cfg(test)]

use crate::{ BouwnceReputationContract, contract::BouwnceReputationContractClient };
use soroban_sdk::{ testutils::{ Address as _ }, Address, BytesN, Env, Symbol };

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BouwnceReputationContract {}, ());
    let client = BouwnceReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let operator = Address::generate(&env);

    client.initialize(&admin, &operator);

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_operator(), operator);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_add_interaction_unauthorized_operator() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BouwnceReputationContract {}, ());
    let client = BouwnceReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let operator = Address::generate(&env);
    let attacker = Address::generate(&env);

    client.initialize(&admin, &operator);

    let hash = BytesN::from_array(&env, &[0; 32]);
    let nonce = BytesN::from_array(&env, &[1; 32]);

    client.add_interaction(
        &attacker,
        &Symbol::new(&env, "user1"),
        &Symbol::new(&env, "vendor1"),
        &5,
        &hash,
        &nonce
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_duplicate_review_prevention() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BouwnceReputationContract {}, ());
    let client = BouwnceReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let operator = Address::generate(&env);

    client.initialize(&admin, &operator);

    let hash = BytesN::from_array(&env, &[0; 32]);
    let nonce1 = BytesN::from_array(&env, &[1; 32]);
    let nonce2 = BytesN::from_array(&env, &[2; 32]);

    client.add_interaction(
        &operator,
        &Symbol::new(&env, "user1"),
        &Symbol::new(&env, "vendor1"),
        &5,
        &hash,
        &nonce1
    );

    client.add_interaction(
        &operator,
        &Symbol::new(&env, "user1"),
        &Symbol::new(&env, "vendor1"),
        &5,
        &hash,
        &nonce2
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_nonce_replay_protection() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BouwnceReputationContract {}, ());
    let client = BouwnceReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let operator = Address::generate(&env);

    client.initialize(&admin, &operator);

    let hash1 = BytesN::from_array(&env, &[1; 32]);
    let hash2 = BytesN::from_array(&env, &[2; 32]);
    let nonce = BytesN::from_array(&env, &[9; 32]);

    client.add_interaction(
        &operator,
        &Symbol::new(&env, "user1"),
        &Symbol::new(&env, "vendor1"),
        &5,
        &hash1,
        &nonce
    );

    client.add_interaction(
        &operator,
        &Symbol::new(&env, "user2"),
        &Symbol::new(&env, "vendor1"),
        &5,
        &hash2,
        &nonce
    );
}
