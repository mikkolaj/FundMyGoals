#![cfg(test)]

extern crate std;

use soroban_sdk::{Address, Bytes, BytesN, Env, IntoVal, symbol, vec};
use soroban_sdk::testutils::{Address as _, Events};

use crate::test::token::TokenClient;
use crate::Tipper;

use super::{Contract, ContractClient};

mod token {
    soroban_sdk::contractimport!(file = "soroban_token_spec.wasm");
    pub type TokenClient = Client;
}

const ONE_XLM: i128 = 10000000;

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
fn test() {
    let (env, contract_id, client) = setup_env();
    let contract_admin = Address::random(&env);
    let owner = Address::random(&env);
    let tipper = Address::random(&env);

    let contract = create_token_contract(&env, &contract_admin);
    contract.mint(&contract_admin, &tipper, &(ONE_XLM * 1000));

    client.init(&owner, &contract.contract_id, &vec![&env, ONE_XLM, 10 * ONE_XLM]);

    client.tip(&Tipper {
        nickname: Bytes::from_array(&env, &[123, 11]),
        address: tipper.clone(),
    }, &(1 * ONE_XLM));

    let all_events = env.events().all();

    assert_eq!(
        all_events.contains((
            contract_id.clone(),
            (symbol!("goals"), ).into_val(&env),
            ONE_XLM.into_val(&env)
        )),
        true
    );

    assert_eq!(
        all_events.contains((
            contract_id.clone(),
            (symbol!("goals"), ).into_val(&env),
            (10 * ONE_XLM).into_val(&env)
        )),
        false
    );

    client.withdraw();

    client.tip(&Tipper {
        nickname: Bytes::from_array(&env, &[123, 11]),
        address: tipper.clone(),
    }, &(10 * ONE_XLM));

    let all_events = env.events().all();

    assert_eq!(
        all_events.contains((
            contract_id.clone(),
            (symbol!("goals"), ).into_val(&env),
            (10 * ONE_XLM).into_val(&env)
        )),
        true
    );

    client.withdraw();

    assert_eq!(contract.balance(&owner), 11 * ONE_XLM);
}

fn setup_env() -> (Env, BytesN<32>, ContractClient) {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}