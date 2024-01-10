use soroban_sdk::{Env, String};

use crate::storage_types::{DataKey, RandomValue, VALUE_TTL_BUMP};

pub fn read_oracle_value(e: &Env, round: u128) -> RandomValue {
    let key = DataKey::Value(round);

    if let Some(value) = e.storage().temporary().get(&key) {
        e.storage()
            .temporary()
            .extend_ttl(&key, VALUE_TTL_BUMP, VALUE_TTL_BUMP);
        value
    } else {
        let empty = String::from_str(&e, "");
        RandomValue {
            randomness: empty.clone(),
            signature: empty.clone(),
            prev_signature: empty,
        }
    }
}

pub fn write_oracle_value(e: &Env, round: u128, value: &RandomValue) {
    let key = DataKey::Value(round);
    e.storage().temporary().set(&key, value);
    e.storage()
        .temporary()
        .extend_ttl(&key, VALUE_TTL_BUMP, VALUE_TTL_BUMP)
}
