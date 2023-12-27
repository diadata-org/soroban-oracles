use soroban_sdk::Env;

use crate::storage_types::DataKey;

pub fn read_last_round(e: &Env) -> u128 {
    e.storage()
        .instance()
        .get(&DataKey::LastRound)
        .unwrap_or_default()
}

pub fn write_last_round(e: &Env, value: u128) {
    e.storage().instance().set(&DataKey::LastRound, &value)
}
