#![cfg(test)]

use insurance::{PolicyRegistryContract, policy_registry::PolicyRegistryContractClient};
use soroban_sdk::{testutils::Address as _, Env, Address, String};

#[test]
fn test_activate_policy() {
    let env = Env::default();
    let contract_id = env.register(PolicyRegistryContract, ());
    let client = PolicyRegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    // Mock auth for admin (test environment) so init can require_auth successfully
    env.mock_all_auths();
    client.init_policy_registry(&admin);

    let user = Address::generate(&env);
    let product = String::from_str(&env, "Insurance");
    let amount = 1000;
    let payment_ref = String::from_str(&env, "ref123");

    let id = client.activate_policy(&user, &product, &amount, &payment_ref);
    assert_eq!(id, 1);

    let policy = client.get_policy(&id).unwrap();
    assert_eq!(policy.id, 1);
    assert_eq!(policy.user, user);
    assert_eq!(policy.product, product);
    assert_eq!(policy.amount, amount);
    assert!(policy.active, "Policy should be active after activation");
}

#[test]
fn test_pause_policy() {
    let env = Env::default();
    let contract_id = env.register(PolicyRegistryContract, ());
    let client = PolicyRegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.init_policy_registry(&admin);

    let user = Address::generate(&env);
    let product = String::from_str(&env, "Insurance");
    let amount = 1000;
    let payment_ref = String::from_str(&env, "ref123");

    let id = client.activate_policy(&user, &product, &amount, &payment_ref);
    client.pause_policy(&admin, &id);

    let policy = client.get_policy(&id).unwrap();
    assert!(!policy.active, "Policy should be inactive after pause");
}

#[test]
fn test_get_policy_non_existing() {
    let env = Env::default();
    let contract_id = env.register(PolicyRegistryContract, ());
    let client = PolicyRegistryContractClient::new(&env, &contract_id);

    let policy = client.get_policy(&999);
    assert!(policy.is_none());
}
