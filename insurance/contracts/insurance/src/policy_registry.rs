use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address};

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub id: u64,
    pub user: Address,
    pub product: String,
    pub amount: i128,
    pub active: bool,
}

#[contract]
pub struct PolicyRegistryContract;

#[contracttype]
pub enum DataKey {
    Admin,
    NextId,
}

#[contractimpl]
impl PolicyRegistryContract {
    pub fn init_policy_registry(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) { panic!("already initialized"); }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextId, &1u64);
        env.events().publish((Symbol::new(&env, "policy_registry_initialized"),), admin);
    }

    fn read_admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("not initialized")
    }

    fn assert_admin(env: &Env, caller: &Address) {
        let admin = Self::read_admin(env);
        caller.require_auth();
        assert!(admin == *caller, "not admin");
    }

    fn next_id(env: &Env) -> u64 {
        env.storage().instance().get(&DataKey::NextId).unwrap_or(1)
    }

    fn bump_id(env: &Env, id: u64) {
        env.storage().instance().set(&DataKey::NextId, &(id + 1));
    }

    pub fn activate_policy(env: Env, user: Address, product: String, amount: i128, payment_ref: String) -> u64 {
        let id = Self::next_id(&env);
        let policy = Policy { id, user: user.clone(), product, amount, active: true };
        env.storage().persistent().set(&id, &policy);
        Self::bump_id(&env, id);
        env.events().publish((Symbol::new(&env, "policy_activated"),), (id, user, amount, payment_ref));
        id
    }

    pub fn pause_policy(env: Env, caller: Address, policy_id: u64) {
        Self::assert_admin(&env, &caller);
        if let Some(mut policy) = env.storage().persistent().get::<u64, Policy>(&policy_id) {
            if policy.active {
                policy.active = false;
                env.storage().persistent().set(&policy_id, &policy);
                env.events().publish((Symbol::new(&env, "policy_paused"),), policy_id);
            }
        }
    }

    pub fn get_policy(env: Env, policy_id: u64) -> Option<Policy> { env.storage().persistent().get(&policy_id) }
    pub fn policy_admin(env: Env) -> Address { Self::read_admin(&env) }
}
