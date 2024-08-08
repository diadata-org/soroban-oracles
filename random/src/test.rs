#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, String, Symbol,
};

use crate::storage_types::RandomValue;

use super::*;

fn create_random_oracle_contract<'a>(e: &Env, admin: &Address) -> RandomOracleContractClient<'a> {
    let contract_address = e.register_contract(None, RandomOracleContract);
    let oracle = RandomOracleContractClient::new(e, &contract_address);
    oracle.initialize(&admin);
    oracle
}

#[test]
fn test_initialize() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_random_oracle_contract(&e, &admin);
    assert_eq!(oracle.admin(), admin);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_already_initialized() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_random_oracle_contract(&e, &admin);
    oracle.initialize(&admin);
}

#[test]
#[should_panic(expected = "invalid round")]
fn test_invalid_round() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_random_oracle_contract(&e, &admin);

    oracle.get_random_value(&0);
}

#[test]
fn test_set_random_value() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_random_oracle_contract(&e, &admin);

    let round = 100_u128;
    let value = RandomValue {
        randomness: "random bytes".into_val(&e),
        signature: "signature".into_val(&e),
        prev_signature: "prev signature".into_val(&e),
    };

    oracle.set_random_value(&round, &value);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    oracle.address.clone(),
                    Symbol::new(&e, "set_random_value"),
                    (round, value.clone()).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    assert_eq!(oracle.last_round(), round);
    assert_eq!(oracle.get_random_value(&round), value);
}

#[test]
#[should_panic(expected = "old round")]
fn test_old_round() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_random_oracle_contract(&e, &admin);

    let value = RandomValue {
        randomness: "randomness".into_val(&e),
        signature: "signature".into_val(&e),
        prev_signature: "prev_signature".into_val(&e),
    };

    oracle.set_random_value(&2_u128, &value);
    oracle.set_random_value(&1_u128, &value);
}

#[test]
fn test_change_admin() {
    let e = Env::default();
    e.mock_all_auths();

    let old_admin = Address::generate(&e);
    let new_admin = Address::generate(&e);

    let oracle = create_random_oracle_contract(&e, &old_admin);

    oracle.change_admin(&new_admin);
    assert_eq!(
        e.auths(),
        std::vec![(
            old_admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    oracle.address.clone(),
                    Symbol::new(&e, "change_admin"),
                    (&new_admin,).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    assert_eq!(oracle.admin(), new_admin);
}

#[test]
#[should_panic(expected = "invalid address")]
fn test_zero_address() {
    let e = Env::default();
    e.mock_all_auths();

    let old_admin = Address::generate(&e);
    let new_admin = Address::from_string(&String::from_bytes(&e, storage_types::ZERO_ADDRESS));

    let oracle = create_random_oracle_contract(&e, &old_admin);
    oracle.change_admin(&new_admin);
}
