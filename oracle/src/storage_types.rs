use soroban_sdk::{contracttype, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const VALUE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const VALUE_LIFETIME_THRESHOLD: u32 = VALUE_BUMP_AMOUNT - DAY_IN_LEDGERS;

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
