use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::admin::{has_admin, read_admin, write_admin};
use crate::events;
use crate::last_round::{read_last_round, write_last_round};
use crate::oracle::{read_oracle_value, write_oracle_value};
use crate::storage_types::{RandomValue, ZERO_ADDRESS};

#[contract]
pub struct RandomOracleContract;

#[contractimpl]
impl RandomOracleContract {
    pub fn initialize(e: Env, admin: Address) {
        assert!(!has_admin(&e), "already initialized");
        write_admin(&e, &admin)
    }

    pub fn admin(e: Env) -> Address {
        read_admin(&e)
    }

    pub fn last_round(e: Env) -> u128 {
        read_last_round(&e)
    }

    pub fn get_random_value(e: Env, round: u128) -> RandomValue {
        read_oracle_value(&e, round).expect("invalid round")
    }

    pub fn set_random_value(e: Env, round: u128, value: RandomValue) {
        assert!(read_last_round(&e) < round, "old round");

        let admin = read_admin(&e);
        admin.require_auth();

        write_last_round(&e, round);
        write_oracle_value(&e, round, &value);
        events::oracle_updated(&e, round, value)
    }

    pub fn change_admin(e: Env, new_admin: Address) {
        assert_ne!(
            new_admin,
            Address::from_string(&String::from_bytes(&e, ZERO_ADDRESS)),
            "invalid address"
        );

        let admin = read_admin(&e);
        admin.require_auth();

        write_admin(&e, &new_admin);
        events::admin_changed(&e, new_admin)
    }
}
