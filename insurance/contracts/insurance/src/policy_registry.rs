#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub id: u64,
    pub user: Address,
    pub product: String,
    pub amount: i128,
    pub status: String,
}

#[contract]
pub struct PolicyRegistryContract;

#[contractimpl]
impl PolicyRegistryContract {
    pub fn activate_policy(env: Env, user: Address, product: String, amount: i128, payment_ref: String) -> u64 {
        let id = env.storage().instance().get::<Symbol, u64>(&Symbol::short("next_id")).unwrap_or(1);
        let policy = Policy { id, user: user.clone(), product, amount, status: String::from_str(&env, "ACTIVE") };
        env.storage().persistent().set(&id, &policy);
        env.storage().instance().set(&Symbol::short("next_id"), &(id+1));
        env.events().publish(("PolicyActivated",), (id, user, amount, payment_ref));
        id
    }

    pub fn pause_policy(env: Env, policy_id: u64) {
        if let Some(mut policy) = env.storage().persistent().get::<u64, Policy>(&policy_id) {
            policy.status = String::from_str(&env, "PAUSED");
            env.storage().persistent().set(&policy_id, &policy);
            env.events().publish(("PolicyPaused",), policy_id);
        }
    }

    pub fn get_policy(env: Env, policy_id: u64) -> Option<Policy> {
        env.storage().persistent().get(&policy_id)
    }
}
