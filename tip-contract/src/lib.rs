#![no_std]

use core::iter::{Enumerate, VecIter};
use core::ptr::null;
use core::vec::VecIter;

use soroban_sdk::{Address, Bytes, BytesN, contractimpl, contracttype, Env, log, map, Map, RawVal, symbol, Symbol, vec, Vec};

const DEPLOYER_KEY: Symbol = symbol!("depl_val");
const COUNTER: Symbol = symbol!("COUNTER");

pub struct Contract;

mod token {
    soroban_sdk::contractimport!(file = "soroban_token_spec.wasm");
}

#[contracttype]
pub enum DataKey {
    GoalDesc,
    CurState,
}

#[contracttype]
pub struct GoalDesc {
    pub owner: Address,
    pub token: BytesN<32>,
    pub goals: Vec<i128>,
}

#[contracttype]
pub struct CurrentState {
    pub balance: i128,
    pub cur_goal: u32,
    pub transfers: Map<Address, i128>,
}

#[contractimpl]
impl Contract {
    pub fn init(env: Env, owner: Address, token: BytesN<32>, goals: Vec<i128>) {
        verify_args(&owner, &token, &goals);

        env.storage().set(&DataKey::CurState, &initial_state(&env));
        env.storage().set(&DataKey::CurState, &GoalDesc { owner, token, goals });
    }

    pub fn tip(env: Env, tipper: Address, amount: i128) {
        verify_goal(&env);

        let (goal_desc, cur_state) = full_state(&env);
        let (new_state, tipped) = state_after_tip(&goal_desc, cur_state, tipper, max_transfer);

        token::Client::new()
    }

    pub fn verify(env: Env) {}
}

fn verify_args(payout_address: &Address, token: &BytesN<32>, goals: &Vec<i128>) {
    payout_address.require_auth();
    if goals.is_empty() {
        panic!("Can't set up a contract with no goals!")
    }
}

fn initial_state(env: &Env) -> CurrentState {
    CurrentState {
        balance: 0,
        cur_goal: 0,
        transfers: map![&env],
    }
}

fn full_state(env: &Env) -> (GoalDesc, CurrentState) {
    let desc = env.storage().get(&DataKey::GoalDesc).unwrap().unwrap();
    let state = env.storage().get(&DataKey::CurState).unwrap().unwrap();
    (desc, state)
}

fn state_after_tip(goal_desc: &GoalDesc, mut cur_state: CurrentState, tipper: Address, max_transfer: i128) -> (CurrentState, i128) {
    let mut goals_met = 0;
    let mut amount_left = max_transfer;
    let mut cur_balance = cur_state.balance;

    for goal in active_goals(goal_desc, &cur_state) {
        let amount_to_meet_goal = (goal - cur_balance);
        if amount_to_meet_goal <= amount_left {
            amount_left -= amount_to_meet_goal;
            goals_met += 1;
            cur_balance = 0;
        } else {
            cur_balance += amount_left;
            amount_left = 0;
            break;
        }
    }

    let cur_tips = cur_state.transfers.get(tipper.clone()).unwrap_or(Ok(0)).unwrap();
    let tipped_now = cur_tips + max_transfer - amount_left;

    cur_state.transfers.set(tipper, tipped_now);

    let new_state = CurrentState {
        balance: cur_balance,
        cur_goal: cur_state.cur_goal + goals_met,
        transfers: cur_state.transfers,
    };

    (new_state, tipped_now)
}

fn active_goals(goal_desc: &GoalDesc, cur_state: &CurrentState) -> VecIter<i128> {
    goal_desc.goals.slice(cur_state.cur_goal..goal_desc.goals.len()).iter()
}

#[cfg(test)]
mod test;