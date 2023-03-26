#![no_std]

use soroban_sdk::{Address, Bytes, BytesN, contractimpl, Env, IntoVal, symbol, Vec};

mod tip_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/tip_contract.wasm"
    );
}

pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        creator: Address,
        token: BytesN<32>,
        goals: Vec<i128>,
    ) -> BytesN<32> {
        let id = env.deployer().with_current_contract(&salt).deploy(&wasm_hash);
        let goal_desc = tip_contract::GoalDesc { creator: creator, token, goals };
        let _: () = env.invoke_contract(&id, &symbol!("init"), (goal_desc, ).into_val(&env));
        id
    }
}

#[cfg(test)]
mod test;
