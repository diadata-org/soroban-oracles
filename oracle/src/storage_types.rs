use dia_soroban_utils::ttl::DAY_IN_LEDGERS;
use soroban_sdk::{contracttype, String};

pub(crate) const VALUE_TTL_BUMP: u32 = 3 * DAY_IN_LEDGERS;
pub(crate) const VALUE_TTL_THRESHOLD: u32 = VALUE_TTL_BUMP - DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Value(String),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleValue(pub u128, pub u128);

impl Default for OracleValue {
    fn default() -> Self {
        OracleValue(0, 0)
    }
}
