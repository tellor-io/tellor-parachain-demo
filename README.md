# Tellor

## Prerequisites:
- Install required packages and Rust as per https://docs.substrate.io/install/
- Install Foundry as per https://getfoundry.sh/
- Install `yarn`
- Clone this repository, making sure to initialise the submodules: `git clone --recursive https://github.com/evilrobot-01/tellor`
- Build Polkadot relay chain and Statemine, Moonbeam and oracle consumer parachain binaries using build script:
  ```
  ./scripts/build.sh
  ```
## Launch Network
  - Launch and initialise a local network (`rococo-local`, `statemine-local`, `moonbase-local` and consumer parachain with Tellor pallet) using launch script:
    ```
    ./scripts/launch.sh
    ``` 
    **NOTE:** this currently requires a custom build of the `parachains-integration-tests` tool which adds support for Ethereum signing required by Moonbeam. 
    See https://github.com/paritytech/parachains-integration-tests/pull/85 for more details. The custom build can be installed globally by cloning the branch used for the PR and then using `yarn global add file:$PWD` to install.


## Deploy Contracts
- In a new terminal shell, deploy the Tellor contracts to Moonbeam, noting the resulting contract addresses:
  ```
  ./scripts/deploy.sh
  ```
  

## Calling Contracts
- You can then call the contracts using Foundry's `cast`, using the development addresses listed at https://github.com/PureStake/moonbeam#prefunded-development-addresses:
  ```
  cast send --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --rpc-url http://localhost:9921/ --legacy 0xContractAddress "depositStake(bytes)" "0x0000000BB8"`
  ```