use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Map};

#[contract]
pub struct RiskPoolContract;

#[contracttype]
pub enum DataKey {
    PoolBalance,
    Admin,
    PremiumRefs, // Map<String,bool> para idempotência de cobranças
}

#[contractimpl]
impl RiskPoolContract {
    // Inicialização: define admin e zera pool. Só pode ser chamada uma vez.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) { panic!("already initialized"); }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::PoolBalance, &0i128);
        env.events().publish((Symbol::new(&env, "pool_initialized"),), admin);
    }

    fn read_balance(env: &Env) -> i128 {
        env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0)
    }

    fn write_balance(env: &Env, value: i128) {
        env.storage().instance().set(&DataKey::PoolBalance, &value);
    }

    fn read_admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("not initialized")
    }

    fn assert_admin(env: &Env, caller: &Address) {
        let admin = Self::read_admin(env);
        caller.require_auth();
        assert!(admin == *caller, "not admin");
    }

    // Coleta de prêmio: apenas acumula XLM lógico (stroops externos convertidos previamente no backend).
    // Versão legada (sem ref) mantida por compatibilidade durante migração.
    pub fn collect_premium(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let balance = Self::read_balance(&env);
        let new_balance = balance + amount;
        Self::write_balance(&env, new_balance);
        env.events().publish((Symbol::new(&env, "premium_collected"),), (from, amount, new_balance));
    }

    // Nova versão idempotente com payment_ref.
    pub fn collect_premium_with_ref(env: Env, from: Address, amount: i128, payment_ref: soroban_sdk::String) {
        from.require_auth();
        let mut map: Map<soroban_sdk::String, bool> = env.storage().instance().get(&DataKey::PremiumRefs).unwrap_or(Map::new(&env));
        if map.get(payment_ref.clone()).unwrap_or(false) {
            // Já processado: emite evento de replay para transparência
            env.events().publish((Symbol::new(&env, "premium_ref_skipped"),), (from, amount, payment_ref));
            return;
        }
        let balance = Self::read_balance(&env);
        let new_balance = balance + amount;
        Self::write_balance(&env, new_balance);
        map.set(payment_ref.clone(), true);
        env.storage().instance().set(&DataKey::PremiumRefs, &map);
        env.events().publish((Symbol::new(&env, "premium_collected"),), (from, amount, payment_ref, new_balance));
    }

    // Payout: protegido por admin.
    pub fn payout(env: Env, caller: Address, to: Address, amount: i128) {
        Self::assert_admin(&env, &caller);
        let balance = Self::read_balance(&env);
        assert!(balance >= amount, "insufficient pool");
        let new_balance = balance - amount;
        Self::write_balance(&env, new_balance);
        env.events().publish((Symbol::new(&env, "payout_executed"),), (to, amount, new_balance));
    }

    pub fn get_balance(env: Env) -> i128 { Self::read_balance(&env) }
    pub fn get_admin(env: Env) -> Address { Self::read_admin(&env) }
}
