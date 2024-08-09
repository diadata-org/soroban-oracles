# DIA Soroban oracles

This repository contains DIA oracle smart contracts for Soroban network on Stellar. At the moment, the following oracles are implemented:

- Key/value oracle with support for multiple updates in one transaction
- Randomness oracle, designed to be used for values from [drand](https://drand.love/)

## Requirements

- [Rust 1.74+](https://www.rust-lang.org/tools/install)
- [GNU Make](https://www.gnu.org/software/make)

## Installing dependencies

First, you'll need to install the `wasm32-unknown-unknown` build target for Rust:

```sh
rustup target add wasm32-unknown-unknown
```

Then install [stellar-cli](https://github.com/stellar/stellar-cli) with the `opt` feature:

```sh
cargo install --locked --version 21.3.0 stellar-cli --features opt
```

> Additionaly, you can configure shell autocompletion for stellar-cli (replace bash if you're using another shell):
>
> ```sh
> echo "source <(soroban completion --shell bash)" >> ~/.bashrc
> ```

After that, you'll need to configure stellar-cli to deploy and interact with oracle smart contracts. This example uses testnet configuration:

```sh
# Add testnet network
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Configure oracle admin identitiy
soroban config identity generate --global dia_oracle_admin

# Request test XLM
curl "https://friendbot.stellar.org/?addr=$(soroban config identity address dia_oracle_admin)"
```

## Building the project

This repository uses GNU Make to simplify the build process. All commands shown below need to be run from the project root directory. To compile all oracle smart contracts:

```sh
make build
```

It will output optimized wasm binaries needed for contract deployment. You can find them in the corresponding oracle's `target/wasm32-unknown-unknown/release` directory.
<br>
<br>
To run all unit tests:

```sh
make test
```

## Deploying smart contracts

To deploy the key/value oracle smart contract:

```sh
# Uploads compiled wasm binary to the network and creates a new contract instance. This command
# will output a contract id, which is needed to interact with the contract later.
soroban contract deploy \
    --wasm oracle/target/wasm32-unknown-unknown/release/dia_soroban_oracle.optimized.wasm
    --network testnet \
    --source dia_oracle_admin

# Sets the oracle admin account (can only be run once)
soroban contract invoke \
    --id <CONTRACT_ID> \
    --network testnet \
    --source dia_oracle_admin \
    -- \
    initialize \
    --admin $(soroban config identity address dia_oracle_admin)
```

And to deploy the randomness oracle contract:

```sh
soroban contract deploy \
    --wasm random/target/wasm32-unknown-unknown/release/dia_soroban_random_oracle.optimized.wasm \
    --network testnet \
    --source dia_oracle_admin

# (initializing the contract is done the same way as shown above)
```

If you are deploying the same contract multiple times, instead of uploading the wasm binary every time you can reference it by hash (as it is already saved in the network):

```sh
# Create a new instance of the key/value oracle
soroban contract deploy \
    --wasm-hash 73d38dbc531c8c116f1bed05d6848bcf67a3943c0b4be5c338772c73153af1e0 \
    --network testnet \
    --source dia_oracle_admin
```

> You can find the wasm hash of a deployed contract in Stellar explorer under Wasm id.

After deploying and initializing the oracle instance you can start using it. Invoking contract functions can be done similarly to `initialize` call shown above. To inspect all available functions of a deployed contract:

```sh
soroban contract invoke --id <CONTRACT_ID> --network testnet -- help
```

## Important notes

- We highly recommend to use different admin accounts for every oracle smart contract you deploy. Stellar accounts have a sequence number (similar to nonce in Ethereum), therefore it's not possible to submit multiple transctions in parallell. Doing so might cause issues at runtime.
- All oracle values are saved in `Temporary` storage, meaning that after a certain amount of ledgers pass the value is deleted permanently. The TTL used for temporary entires in oracle smart contracts is approximately 7 days. It is extended when an entry is accessed if the remaining amount of ledgers is below a threshold (~6 days).
- Unlike in EVM, a separate ABI is not needed to interact with a contract deployed on Soroban. Compiled wasm binary contains all information needed to encode function calls and decode return data. Therefore, there is no need to verify deployed contracts on Stellar explorer because the contract interface is decompiled from its bytecode.
