#![cfg(test)]

use insurance::{RiskPoolContract, risk_pool::RiskPoolContractClient};
use soroban_sdk::{testutils::Address as _, Env, Address, String as SorobanString};

#[test]
fn test_collect_premium_increases_balance() {
    let env = Env::default();
    let contract_id = env.register(RiskPoolContract, ());
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.init(&admin);

    // Saldo inicial deve ser 0
    assert_eq!(client.get_balance(), 0);

    // Coletar premium de 100
    client.collect_premium_with_ref(&user, &100, &SorobanString::from_str(&env, "ref1"));

    // Saldo atualizado
    assert_eq!(client.get_balance(), 100);

    // Coletar mais 50
    client.collect_premium_with_ref(&user, &50, &SorobanString::from_str(&env, "ref2"));

    // Saldo deve ser 150
    assert_eq!(client.get_balance(), 150);
}

#[test]
fn test_payout_reduces_balance() {
    let env = Env::default();
    let contract_id = env.register(RiskPoolContract, ());
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.init(&admin);
    // Carregar o pool com 200 (duas refs diferentes)
    client.collect_premium_with_ref(&user, &120, &SorobanString::from_str(&env, "a"));
    client.collect_premium_with_ref(&user, &80, &SorobanString::from_str(&env, "b"));
    assert_eq!(client.get_balance(), 200);

    // Executar payout de 80
    client.payout(&admin, &user, &80);

    // Saldo deve ser 120
    assert_eq!(client.get_balance(), 120);
}

#[test]
#[should_panic(expected = "insufficient pool")]
fn test_payout_fails_if_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register(RiskPoolContract, ());
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.init(&admin);
    // Pool vazio, tentar payout deve falhar (admin autorizado porém sem fundos)
    client.payout(&admin, &user, &50);
}

#[test]
fn test_collect_premium_with_ref_idempotent() {
    let env = Env::default();
    let contract_id = env.register(RiskPoolContract, ());
    let client = RiskPoolContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    client.init(&admin);
    client.collect_premium_with_ref(&user, &100, &SorobanString::from_str(&env, "refX"));
    // segunda vez mesma ref - não deve aumentar
    client.collect_premium_with_ref(&user, &100, &SorobanString::from_str(&env, "refX"));
    assert_eq!(client.get_balance(), 100);
}
