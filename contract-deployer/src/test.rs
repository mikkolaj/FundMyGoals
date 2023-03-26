#![cfg(test)]

extern crate std;

use soroban_sdk::{Bytes, Env, IntoVal, RawVal, symbol, Vec};

use super::Deployer;
use crate::DeployerClient;

mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/tip_contract.wasm"
    );
}

#[test]
fn test_deploy() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    let wasm_hash = env.install_contract_wasm(contract::WASM);

    // deploy contract using deployer and include an init function to call
    let salt = Bytes::from_array(&env, &[0; 32]);
    let init_fn = symbol!("init");
    let init_fn_args: Vec<RawVal> = (5u32, ).into_val(&env);
    let (contract_id, init_result) = client.deploy(&salt, &wasm_hash, &init_fn, &init_fn_args);
    assert!(init_result.is_void());

    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5)
}
