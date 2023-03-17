#![cfg(test)]

extern crate std;

use soroban_sdk::{Address, BytesN, Env, IntoVal, symbol, vec};
use soroban_sdk::testutils::{Address as _, Events, Logger};

use super::{Contract, ContractClient};

#[test]
fn test() {
    // let (env, _, client) = setup_env();
    //
    // client.test();
    // client.test2(&4i32, &5i32);
    // let result = client.test3(&4i32);
    //
    // let logs = env.logger().all();
    // std::println!("{}", logs.join("\n"));
    // assert_eq!(
    //     result,
    //     5i32
    // )
}

fn setup_env() -> (Env, BytesN<32>, ContractClient) {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}