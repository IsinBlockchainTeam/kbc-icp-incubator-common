# ICP Common

This project creates a decentralized network based on [_ICP_](https://internetcomputer.org/) technology. It creates _canister_ for storing data in a decentralized way.

This package is written in Rust.

## Prerequisites

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [dfx](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-parent) (latest version strongly suggested)
- `wasm32-unknown-unknown` target, you can add it by running `rustup target add wasm32-unknown-unknown`

## Getting Started - Local replica

1. Make sure you have installed [dfx](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-parent) and updated the latest version
2. Clone this repository using command `git clone https://gitlab-core.supsi.ch/dti-isin/giuliano.gremlich/blockchain/one_lib_to_rule_them_all.git`
3. Enter the newly created folder using `cd one_lib_to_rule_them_all`
4. Change branch to `coffetrading/dev` using command `git checkout coffetrading/dev`
5. Enter the package folder using `cd icp/common`
6. Run `dfx start --clean` to start the local ICP replica
7. In another terminal, run `./scripts/deploy-local.sh` to create, build and deploy the canisters on the local replica


## Getting Started - Mainnet

1. Follow the instructions in the section [Getting Started - Local replica](#getting-started---local-replica) up to step 5
2. Check you have configured your wallet correctly and have enough cycles to deploy the canisters. You can check your balance using `dfx wallet balance --network ic`
3. If you have not created the canisters on the mainnet yet, run `dfx canister create --network ic permission --with-cycles <desired_cycles> <canister_name>` to create the first canister
   1. Create now the remaining canisters using the command `dfx canister create --network ic --next-to permission --with-cycles <desired_cycles> <canister_name>`
   > The `--next-to` flag is used to create canisters on the same subnet. While this is not mandatory, it is strongly recommended to speed up the canister interactions
4. Build canister using `dfx build --network ic`
5. Generate declarations running `dfx generate --network ic`
6. Deploy canister using `dfx canister install --network ic --mode <mode> --argument=<arguments> <canister_name>`. Mode "reinstall" is suggested

> If you have already created the canisters, you can also use the script `./scripts/deploy-mainnet.sh` to deploy the canisters on the mainnet.
> Note, however, that this operation involves an actual cost, therefore you might want to run every command manually to handle any error that might occur.
   

## Troubleshooting

- If you cannot contact the local replica from other services, try deleting file <home>/<user>/.config/dfx/networks.json and restart the local replica
