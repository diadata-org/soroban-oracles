use dia_soroban_utils::ttl::extend_instance_ttl;
use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::admin::{has_admin, read_admin, write_admin};
use crate::events;
use crate::last_round::{read_last_round, write_last_round};
use crate::oracle::{read_oracle_value, write_oracle_value};
use crate::storage_types::RandomValue;

#[contract]
pub struct RandomOracleContract;

#[contractimpl]
impl RandomOracleContract {
    pub fn initialize(e: Env, admin: Address) {
        assert!(!has_admin(&e), "already initialized");
        write_admin(&e, &admin)
    }

    pub fn admin(e: Env) -> Address {
        extend_instance_ttl(&e);
        read_admin(&e)
    }

    pub fn last_round(e: Env) -> u128 {
        extend_instance_ttl(&e);
        read_last_round(&e)
    }

    pub fn get_random_value(e: Env, round: u128) -> RandomValue {
        extend_instance_ttl(&e);
        read_oracle_value(&e, round)
    }

    pub fn set_random_value(e: Env, round: u128, value: RandomValue) {
        let admin = read_admin(&e);
        admin.require_auth();

        extend_instance_ttl(&e);
        write_last_round(&e, round);
        write_oracle_value(&e, round, &value);
        events::oracle_updated(&e, round, value)
    }

    pub fn change_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        extend_instance_ttl(&e);
        write_admin(&e, &new_admin);
        events::admin_changed(&e, new_admin)
    }
}
