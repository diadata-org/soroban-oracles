use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::admin::{has_admin, read_admin, write_admin};
use crate::events;
use crate::oracle::{read_oracle_value, write_oracle_value};
use crate::storage_types::OracleValue;

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    pub fn initialize(e: Env, admin: Address) {
        assert!(!has_admin(&e), "already initialized");
        write_admin(&e, &admin)
    }

    pub fn admin(e: Env) -> Address {
        read_admin(&e)
    }

    pub fn get_value(e: Env, key: String) -> OracleValue {
        read_oracle_value(&e, key)
    }

    pub fn set_value(e: Env, key: String, value: OracleValue) {
        let admin = read_admin(&e);
        admin.require_auth();

        write_oracle_value(&e, key.clone(), &value);
        events::oracle_updated(&e, key, value)
    }

    pub fn set_multiple_values(e: Env, keys: Vec<String>, values: Vec<OracleValue>) {
        assert!(keys.len() == values.len(), "input lengths should be equal");

        let admin = read_admin(&e);
        admin.require_auth();

        for (key, value) in keys.iter().zip(values.iter()) {
            write_oracle_value(&e, key.clone(), &value);
            events::oracle_updated(&e, key, value);
        }
    }

    pub fn change_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        write_admin(&e, &new_admin);
        events::admin_changed(&e, new_admin)
    }
}
