#![no_std]

use soroban_sdk::{Address, Bytes, BytesN, contractimpl, contracttype, Env, log, RawVal, symbol, Symbol, vec, Vec};

const COUNTER: Symbol = symbol!("COUNTER");
const DEPLOYER_KEY: Symbol = symbol!("depl_val");

pub struct Contract;

#[contracttype]
pub enum DataKey {
    // Address may represent a Stellar account, a contract or an 'account contract'
    Counter(Address),
    // Address can be used any time some network identity needs to be represented,
    // like to distinguish between counters for different users in this example.
}

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }

    pub fn increment(env: Env) -> u32 {
        env.current_contract_address();
        let current_count: u32 = env.storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0))
            .unwrap();
        log!(&env, "Current count: {}", current_count);
        let incremented_count = current_count + 1;
        env.storage().set(&COUNTER, &incremented_count);
        env.events().publish((COUNTER, symbol!("increment")), incremented_count);
        incremented_count
    }

    pub fn auth_incr(env: Env, user: Address, value: u32) -> u32 {
        // Requires `user` to have authorized call of the `increment` of this
        // contract with all the arguments passed to `increment`, i.e. `user`
        // and `value`. This will panic if auth fails for any reason.
        // When this is called, Soroban host performs the necessary
        // authentication, manages replay prevention and enforces the user's
        // authorization policies.
        // The contracts normally shouldn't worry about these details and just
        // write code in generic fashion using `Address` and `require_auth` (or
        // `require_auth_for_args`).
        user.require_auth();
        // This call is equilvalent to the above:
        // user.require_auth_for_args((&user, value).into_val(&env));

        // The following has less arguments but is equivalent in authorization
        // scope to the above calls (the user address doesn't have to be
        // included in args as it's guaranteed to be authenticated).
        // user.require_auth_for_args((value,).into_val(&env));

        // Construct a key for the data being stored. Use an enum to set the
        // contract up well for adding other types of data to be stored.
        let key = DataKey::Counter(user.clone());

        // Get the current count for the invoker.
        let mut count: u32 = env
            .storage()
            .get(&key)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.

        // Increment the count.
        count += value;

        // Save the count.
        env.storage().set(&key, &count);

        // Return the count to the caller.
        count
    }

    // Deployer test, normally this deploy function would be placed in a separate package,
    // but it is here for simplicity
    pub fn deploy(
        env: Env,
        salt: Bytes,
        wasm_hash: BytesN<32>, // wasm_hash specifies an on-chain location of the contract we want to deploy
        init_fn: Symbol,
        init_args: Vec<RawVal>,
    ) -> (BytesN<32>, RawVal) {
        // new contract id is deterministic and derived from provided salt and wasm_hash
        let id = env.deployer().with_current_contract(&salt).deploy(&wasm_hash);
        // deployer calls the contract's initialization function and passes through the arguments
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);

        // deployer returns the new contract ID and the result of the initialization function
        (id, res)
    }

    pub fn init(env: Env, value: u32) {
        env.storage().set(&DEPLOYER_KEY, &value);
    }

    pub fn value(env: Env) -> u32 {
        env.storage().get_unchecked(&DEPLOYER_KEY).unwrap()
    }
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod deployer_test;
