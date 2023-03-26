use soroban_sdk::{Address, Bytes, BytesN, contracttype, Env, Map, map, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    GoalDesc,
    CurState,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Uninitlzd,
    Initlzd,
    Completed,
}

#[contracttype]
pub struct GoalDesc {
    pub creator: Address,
    pub token: BytesN<32>,
    pub goals: Vec<i128>,
}

impl GoalDesc {
    pub fn amount_to_pay_out(&self, cur_state: &CurrentState) -> i128 {
        self.goals
            .slice(cur_state.unpaid_idx..cur_state.cur_goal).iter()
            .fold(0i128, |acc, goal| { acc.saturating_add(goal.unwrap()) })
    }

    pub fn goals_met(&self, cur_state: &CurrentState, new_state: &CurrentState) -> Vec<i128> {
        self.goals.slice(cur_state.cur_goal..new_state.cur_goal)
    }
}

#[contracttype]
#[derive(Clone)]
pub struct Tipper {
    pub nickname: Bytes,
    pub address: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct CurrentState {
    pub phase: Phase,
    pub goal_money: i128,
    pub unpaid_idx: u32,
    pub cur_goal: u32,
    pub transfers: Map<Bytes, i128>,
}


pub fn empty_state(env: &Env, phase: Phase) -> CurrentState {
    CurrentState {
        phase,
        goal_money: 0,
        unpaid_idx: 0,
        cur_goal: 0,
        transfers: map![&env],
    }
}
