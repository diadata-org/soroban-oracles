use soroban_sdk::{Env, String};

use crate::storage_types::{DataKey, OracleValue, VALUE_BUMP_AMOUNT, VALUE_LIFETIME_THRESHOLD};

pub fn read_oracle_value(e: &Env, key: String) -> OracleValue {
    let store_key = DataKey::Value(key);

    if let Some(value) = e
        .storage()
        .temporary()
        .get::<DataKey, OracleValue>(&store_key)
    {
        e.storage()
            .temporary()
            .extend_ttl(&store_key, VALUE_LIFETIME_THRESHOLD, VALUE_BUMP_AMOUNT);
        value
    } else {
        OracleValue::default()
    }
}

pub fn write_oracle_value(e: &Env, key: String, value: &OracleValue) {
    let store_key = DataKey::Value(key);
    e.storage().temporary().set(&store_key, value);
    e.storage()
        .temporary()
        .extend_ttl(&store_key, VALUE_LIFETIME_THRESHOLD, VALUE_BUMP_AMOUNT);
}
