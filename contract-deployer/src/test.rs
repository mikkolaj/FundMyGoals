#![cfg(test)]

extern crate std;

use soroban_sdk::{Address, Bytes, BytesN, Env, vec};
use soroban_sdk::testutils::{Address as _, BytesN as _};

use crate::DeployerClient;

use super::Deployer;

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
    let salt = Bytes::from_array(&env, &[0; 32]);

    client.deploy(&salt, &wasm_hash, &Address::random(&env), &BytesN::<32>::random(&env), &vec![&env, 1i128, 2i128]);
}
