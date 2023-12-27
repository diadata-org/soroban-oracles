use soroban_sdk::{Address, Env, Symbol};

use crate::storage_types::RandomValue;

pub fn admin_changed(e: &Env, id: Address) {
    let topics = (Symbol::new(&e, "admin_change"),);
    e.events().publish(topics, id)
}

pub fn oracle_updated(e: &Env, round: u128, value: RandomValue) {
    let topics = (Symbol::new(&e, "oracle_updated"),);
    e.events().publish(topics, (round, value));
}
