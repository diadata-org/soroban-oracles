use soroban_sdk::{Address, Env, String, Symbol};

use crate::storage_types::OracleValue;

pub fn admin_changed(e: &Env, id: Address) {
    let topics = (Symbol::new(&e, "admin_change"),);
    e.events().publish(topics, id)
}

pub fn oracle_updated(e: &Env, key: String, value: OracleValue) {
    let topics = (Symbol::new(&e, "oracle_updated"),);
    e.events().publish(topics, (key, value))
}
