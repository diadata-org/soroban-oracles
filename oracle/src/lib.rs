#![no_std]
mod admin;
mod contract;
mod events;
mod oracle;
mod storage_types;
mod test;

pub use contract::{OracleContract, OracleContractClient};
pub use storage_types::OracleValue;
