use soroban_sdk::Env;

use crate::storage_types::{DataKey, RandomValue, VALUE_BUMP_AMOUNT, VALUE_LIFETIME_THRESHOLD};

pub fn read_oracle_value(e: &Env, round: u128) -> Option<RandomValue> {
    let key = DataKey::Value(round);

    if let Some(value) = e.storage().temporary().get(&key) {
        e.storage()
            .temporary()
            .extend_ttl(&key, VALUE_LIFETIME_THRESHOLD, VALUE_BUMP_AMOUNT);
        Some(value)
    } else {
        None
    }
}

pub fn write_oracle_value(e: &Env, round: u128, value: &RandomValue) {
    let key = DataKey::Value(round);
    e.storage().temporary().set(&key, value);
    e.storage()
        .temporary()
        .extend_ttl(&key, VALUE_LIFETIME_THRESHOLD, VALUE_BUMP_AMOUNT)
}
