#![cfg(test)]

extern crate std;

use soroban_sdk::{Address, BytesN, Env, IntoVal, symbol, vec};
use soroban_sdk::testutils::{Address as _, Events};

use super::{Contract, ContractClient};

#[test]
fn test_hello() {
    let (env, _, client) = setup_env();

    let words = client.hello(&symbol!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol!("Hello"), symbol!("Dev")]
    )
}

#[test]
fn test_counter() {
    let (_, _, client) = setup_env();

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

#[test]
fn test_event() {
    let (env, contract_id, client) = setup_env();

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol!("COUNTER"), symbol!("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
        ]
    )
}

#[test]
fn test_auth() {
    let (env, contract_id, client) = setup_env();

    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    assert_eq!(client.auth_incr(&user_1, &5), 5);

    // Verify that the user indeed needed to authorize the call
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            // address
            user_1.clone(),
            contract_id.clone(),
            // name of the function
            symbol!("auth_incr"),
            // arguments
            (user_1.clone(), 5_u32).into_val(&env)
        )]
    );

    assert_eq!(client.auth_incr(&user_2, &1), 1);
    assert_eq!(client.auth_incr(&user_1, &1), 6);
}

pub fn setup_env() -> (Env, BytesN<32>, ContractClient) {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}