#![no_std]

use soroban_sdk::{Bytes, BytesN, contractimpl, Env, RawVal, Symbol, Vec};

pub struct Deployer;

#[contractimpl]
impl Deployer {
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
}

#[cfg(test)]
mod test;
