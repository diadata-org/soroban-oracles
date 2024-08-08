#![cfg(test)]
extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, Env, IntoVal, String, Symbol,
};

use super::*;

fn create_oracle_contract<'a>(e: &Env, admin: &Address) -> OracleContractClient<'a> {
    let contract_address = e.register_contract(None, OracleContract);
    let oracle = OracleContractClient::new(e, &contract_address);
    oracle.initialize(&admin);
    oracle
}

#[test]
fn test_initialize() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_oracle_contract(&e, &admin);
    assert_eq!(oracle.admin(), admin);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_already_initialized() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_oracle_contract(&e, &admin);
    oracle.initialize(&admin);
}

#[test]
fn test_set_value() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_oracle_contract(&e, &admin);

    let key = String::from_str(&e, "key");
    let value = OracleValue(1, 1);

    oracle.set_value(&key, &value);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    oracle.address.clone(),
                    symbol_short!("set_value"),
                    (key.clone(), value.clone()).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    assert_eq!(oracle.get_value(&key), value);
}

#[test]
fn test_set_multiple_values() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_oracle_contract(&e, &admin);

    let keys = vec![
        &e,
        String::from_str(&e, "1"),
        String::from_str(&e, "2"),
        String::from_str(&e, "3"),
    ];

    let values = vec![&e, OracleValue(1, 1), OracleValue(2, 2), OracleValue(3, 3)];

    oracle.set_multiple_values(&keys, &values);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    oracle.address.clone(),
                    Symbol::new(&e, "set_multiple_values"),
                    (keys.clone(), values.clone()).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    for (key, val) in keys.iter().zip(values.iter()) {
        assert_eq!(oracle.get_value(&key), val);
    }
}

#[test]
#[should_panic(expected = "input lengths should be equal")]
fn test_multiple_len() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let oracle = create_oracle_contract(&e, &admin);

    let keys = vec![&e, String::from_str(&e, "1")];
    let values = vec![&e, OracleValue(1, 1), OracleValue(2, 2)];
    oracle.set_multiple_values(&keys, &values);
}

#[test]
fn test_change_admin() {
    let e = Env::default();
    e.mock_all_auths();

    let old_admin = Address::generate(&e);
    let new_admin = Address::generate(&e);

    let oracle = create_oracle_contract(&e, &old_admin);

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

    let oracle = create_oracle_contract(&e, &old_admin);
    oracle.change_admin(&new_admin);
}
