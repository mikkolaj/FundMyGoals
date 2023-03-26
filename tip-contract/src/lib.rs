#![no_std]

use soroban_sdk::{Bytes, contractimpl, Env, Map, Symbol, symbol, Vec};

use crate::datatypes::{CurrentState, DataKey, empty_state, GoalDesc, Phase, Tipper};

pub struct Contract;

mod token {
    soroban_sdk::contractimport!(file = "soroban_token_spec.wasm");
}

const GOALS_TOPIC: Symbol = symbol!("goals");

#[contractimpl]
impl Contract {
    pub fn init(env: Env, goal_desc: GoalDesc) {
        verify_init(&env, &goal_desc.goals);

        env.storage().set(&DataKey::CurState, &empty_state(&env, Phase::Initlzd));
        env.storage().set(&DataKey::GoalDesc, &goal_desc);
    }

    pub fn tip(env: Env, tipper: Tipper, max_transfer: i128) -> i128 {
        verify_tip(&env);

        let (goal_desc, cur_state) = full_state(&env);
        let (new_state, tipped) = state_after_tip(&goal_desc, &cur_state, &tipper, max_transfer);
        let contract = env.current_contract_address();
        let client = token::Client::new(&env, &goal_desc.token);

        emit_events(&env, goal_desc.goals_met(&cur_state, &new_state));
        client.xfer(&tipper.address, &contract, &tipped);
        env.storage().set(&DataKey::CurState, &new_state);
        tipped
    }

    pub fn withdraw(env: Env) -> i128 {
        verify_withdraw(&env);

        let (goal_desc, mut cur_state) = full_state(&env);
        let client = token::Client::new(&env, &goal_desc.token);
        let contract = env.current_contract_address();
        let to_pay_out = goal_desc.amount_to_pay_out(&cur_state);

        client.xfer(&contract, &goal_desc.creator, &to_pay_out);
        cur_state.goal_money = 0;
        cur_state.unpaid_idx = cur_state.cur_goal;

        env.storage().set(&DataKey::CurState, &cur_state);
        to_pay_out
    }


    pub fn scoreboard(env: Env) -> Map<Bytes, i128> {
        env.storage().get(&DataKey::CurState).unwrap_or(Ok(empty_state(&env, Phase::Uninitlzd))).unwrap().transfers
    }
}

fn emit_events(env: &Env, goals_met: Vec<i128>) {
    for goal in goals_met {
        env.events().publish((GOALS_TOPIC, ), goal.unwrap());
    }
}

fn verify_init(env: &Env, goals: &Vec<i128>) {
    if current_phase(&env) != Phase::Uninitlzd {
        panic!("Contract already initialized!")
    } else if goals.is_empty() {
        panic!("Can't set up a contract with no goals!")
    } else {
        goals.iter().fold(0i128, |acc, goal| {
            acc.checked_add(goal.unwrap())
                .unwrap_or_else(|| { panic!("Goals overflow max i128 number") })
        });
    }
}

fn verify_withdraw(env: &Env) {
    if current_phase(&env) == Phase::Uninitlzd {
        panic!("Contract not yet initialized!")
    }
}

fn verify_tip(env: &Env) {
    if current_phase(&env) != Phase::Initlzd {
        panic!("Contract not yet initialized!")
    }
}

fn current_phase(env: &Env) -> Phase {
    env.storage().get(&DataKey::CurState).unwrap_or(Ok(empty_state(env, Phase::Uninitlzd))).unwrap().phase
}

fn full_state(env: &Env) -> (GoalDesc, CurrentState) {
    let desc = env.storage().get(&DataKey::GoalDesc).unwrap().unwrap();
    let state = env.storage().get(&DataKey::CurState).unwrap().unwrap();
    (desc, state)
}

fn state_after_tip(goal_desc: &GoalDesc, cur_state: &CurrentState, tipper: &Tipper, max_transfer: i128) -> (CurrentState, i128) {
    let mut goals_met = 0;
    let mut amount_left_to_transfer = max_transfer;
    let mut cur_goal_money = cur_state.goal_money;

    for goal in goal_desc.goals.slice(cur_state.cur_goal..goal_desc.goals.len()).iter_unchecked() {
        let amount_to_meet_goal = goal - cur_goal_money;
        if amount_to_meet_goal <= amount_left_to_transfer {
            amount_left_to_transfer -= amount_to_meet_goal;
            goals_met += 1;
            cur_goal_money = 0;
        } else {
            cur_goal_money += amount_left_to_transfer;
            amount_left_to_transfer = 0;
            break;
        }
    }

    let cur_tips = cur_state.transfers.get(tipper.nickname.clone()).unwrap_or(Ok(0)).unwrap();
    let tipped_now = max_transfer - amount_left_to_transfer;
    let mut new_map = cur_state.transfers.clone();
    new_map.set(tipper.nickname.clone(), cur_tips + tipped_now);

    let new_state = CurrentState {
        phase: get_next_contract_phase(goal_desc, &cur_state, &goals_met),
        goal_money: cur_goal_money,
        unpaid_idx: cur_state.unpaid_idx,
        cur_goal: cur_state.cur_goal + goals_met,
        transfers: new_map,
    };

    (new_state, tipped_now)
}

fn get_next_contract_phase(goal_desc: &GoalDesc, cur_state: &CurrentState, goals_met: &u32) -> Phase {
    if cur_state.cur_goal + goals_met >= goal_desc.goals.len() {
        Phase::Completed
    } else {
        cur_state.phase
    }
}

#[cfg(test)]
mod test;
mod datatypes;