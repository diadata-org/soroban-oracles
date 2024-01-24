use soroban_sdk::{contracttype, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const VALUE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const VALUE_LIFETIME_THRESHOLD: u32 = VALUE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    LastRound,
    Value(u128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomValue {
    pub randomness: String,
    pub signature: String,
    pub prev_signature: String,
}
