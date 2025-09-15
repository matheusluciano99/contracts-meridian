#![cfg(test)]

use insurance::{RiskPoolContract, risk_pool::RiskPoolContractClient};
use soroban_sdk::{testutils::Address as _, Env, Address};

#[test]
fn test_collect_premium_increases_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RiskPoolContract);
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Saldo inicial deve ser 0
    assert_eq!(client.get_balance(), 0);

    // Coletar premium de 100
    client.collect_premium(&user, &100);

    // Saldo atualizado
    assert_eq!(client.get_balance(), 100);

    // Coletar mais 50
    client.collect_premium(&user, &50);

    // Saldo deve ser 150
    assert_eq!(client.get_balance(), 150);
}

#[test]
fn test_payout_reduces_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RiskPoolContract);
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Carregar o pool com 200
    client.collect_premium(&user, &200);
    assert_eq!(client.get_balance(), 200);

    // Executar payout de 80
    client.payout(&user, &80);

    // Saldo deve ser 120
    assert_eq!(client.get_balance(), 120);
}

#[test]
#[should_panic(expected = "Pool sem fundos")]
fn test_payout_fails_if_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RiskPoolContract);
    let client = RiskPoolContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Pool vazio, tentar payout deve falhar
    client.payout(&user, &50);
}
