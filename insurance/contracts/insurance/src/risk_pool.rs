use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contract]
pub struct RiskPoolContract;

#[contracttype]
pub enum DataKey {
    PoolBalance,
}

#[contractimpl]
impl RiskPoolContract {
    pub fn collect_premium(env: Env, from: Address, amount: i128) {
        // Aqui chamaríamos o contrato USDC (SAC) para transferir do user → pool
        let balance: i128 = env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0);
        let new_balance = balance + amount;
        env.storage().instance().set(&DataKey::PoolBalance, &new_balance);
        env.events().publish(("PremiumCollected",), (from, amount));
    }

    pub fn payout(env: Env, to: Address, amount: i128) {
        let balance: i128 = env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0);
        assert!(balance >= amount, "Pool sem fundos");
        let new_balance = balance - amount;
        env.storage().instance().set(&DataKey::PoolBalance, &new_balance);
        env.events().publish(("PayoutExecuted",), (to, amount));
    }

    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0)
    }
}
