# Tellor
A high-level proof-of-concept to assess XCM message sending between Tellor staking/governance smart contracts hosted on an 
EVM-enabled parachain like Moonbeam, and a `tellor` oracle consumer pallet hosted on some other parachain.

## Overview
The following sections summarise the options available when sending XCM messages from each direction (smart contracts <> pallet).

### Remote Execution
As described at https://docs.moonbeam.network/builders/xcm/xcm-transactor, remote execution via the `XCM-Transactor` 
pallet allows smart contracts to send XCM messages via Solidity precompiles (which effectively wrap the pallet extrinsics).

The following options for remote execution are available:
- `transactThroughDerivative`: transact through an address derived from the source chain's sovereign account in the destination chain, using fees based on an asset (`CurrencyId`) registered with the local `asset_manager` pallet
  - Transactions are dispatched from a derivative account of the sovereign account on the destination chain
  - Requires pre-registration by `DerivativeAddressRegistrationOrigin` (Root), which maps an 'owner' address on the source chain to a derivative 'index' (or key), which in turn maps to a sovereign derivative account of the parachain sovereign account on destination chain
  - Fees may be set as `SelfReserve`, `ForeignAsset` or `LocalAssetReserve`, where caller must hold tokens _locally_ of the type specified
  - Deducts and burns amount of fees for remote execution from caller _before_ sending
  - Resulting XCM message includes instruction to withdraw corresponding execution fees from parachain sovereign account on destination chain for execution payment
  - requires `pallet-utility` and paid XCM message execution (`WithdrawAsset`,`BuyExecution`,`Transact`) to be allowed on the destination chain
  - Requires Moonbeam to have native token of oracle consumer parachain registered locally and tokens bridged across to pay for fees
- `transactThroughDerivativeMultilocation`: as above, but using fees based on a multilocation rather than `CurrencyId`
- `transactThroughSignedMultilocation`: transact through XCM via signed origins, using fees based on a multilocation
  - Transact through an account derived from the multilocation representing the signed user making the call 
  - `DescendOrigin` is the first instruction of resulting XCM message, ensuring fee payment and dispatch from some derivative account on the destination chain
  - No token is burnt before sending the message. The caller must ensure the destination is able to understand the `DescendOrigin` message, and create a unique account from which to dispatch the call
  - No pre-registration required, but assumption that derivative account on remote chain has sufficient balance to pay for execution
  - Requires paid XCM message execution (`DescendOrigin`,`WithdrawAsset`,`BuyExecution`,`Transact`) to be allowed on the destination chain
  - Requires the destination chain to derivate a new corresponding account
- `transactThroughSigned`: as above, but using fee based on an erc20 address (e.g. precompile address of XC-20 asset, which is a ERC-20 wrapper over a Substrate asset)

`transactThroughSignedMultilocation` was selected to start as it did not require the bridging of remote assets and allowed 
an interior multilocation, denoting the deployed smart contract address on the source parachain, to simply be converted to 
the account of the `tellor` pallet on the oracle consumer parachain (via `xcm_config`). The `tellor` pallet is then funded 
with native tokens for fee payment. This implementation was simply to get calls working and is not assumed to be secure.

### Remote EVM Calls
Based on https://docs.moonbeam.network/builders/xcm/remote-evm-calls, remote evm calls are dispatched from a keyless 
derivative account which is derivated from a hash of the source multilocation (example below, see [calculate-multilocation-derivative](https://docs.moonbeam.network/builders/xcm/remote-evm-calls/#calculate-multilocation-derivative) for more info). This derivative account pays for fees and is set as
the dispatcher of the call.

An example of a multilocation:
```
{"parents":1,"interior":{"x1":{"accountId32":{"network":{"named":"0x57657374656e64"},"id":"0x78914a4d7a946a0e4ed641f336b498736336e05096e342c799cc33c0f868d62f"}}}}
```

**Note:** 
- This requires the `Ethereum-XCM` pallet, which is currently only available on Moonbase Alpha and does not offer 
as many options as the remote execution section above.
- This will require that the corresponding multilocation derivative account of the `tellor` pallet account on Moonbeam is funded to pay for ongoing Xcm/Transact fees.

---

## Setup

### Prerequisites:
- Install required packages and Rust as per https://docs.substrate.io/install/
- Install Foundry as per https://getfoundry.sh/
- Install `yarn`
- Clone this repository, making sure to initialise the submodules: `git clone --recursive https://github.com/evilrobot-01/tellor`

### Build
Build the `polkadot` (relay chain), `polkadot-parachain` (asset reserve), `moonbeam` and `parachain-template-node` (oracle consumer parachain) binaries using the `build` script:
  ```
  ./scripts/build.sh
  ```
### Launch Network
Launch a local network (`rococo-local`, `statemine-local`, `moonbase-local` and consumer parachain (with Tellor pallet) using the `launch` script:
```
./scripts/launch.sh
``` 
**NOTE:** this currently requires a custom build of the `parachains-integration-tests` tool which adds support for Ethereum signing required by Moonbeam. 
See https://github.com/paritytech/parachains-integration-tests/pull/85 for more details. The custom build can be installed globally by cloning the branch used for the PR and then using `yarn global add file:$PWD` to install.

### Deploy Contracts & Initialise Chain State
In a new terminal shell, use the `deploy` script to deploy the Tellor contracts to Moonbeam as well as perform required chainstate initialisation:
```
./scripts/deploy.sh
```

### Contract Usage
You can then call the contracts using Foundry's `cast`, using the development addresses listed at https://github.com/PureStake/moonbeam#prefunded-development-addresses.

#### Register Parachain
The following command simply registers a parachain into the staking contract:
```
cast send --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --rpc-url http://localhost:9921/ --legacy 0xc01Ee7f10EA4aF4673cFff62710E1D7792aBa8f3 "register(uint32,uint8,uint256)" 3000 40 100
```

#### Deposit Stake
The following command deposits a new stake into the staking contract for a particular parachain, which should then report the stake to the corresponding oracle consumer parachain:
```
cast send --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --rpc-url http://localhost:9921/ --legacy 0xc01Ee7f10EA4aF4673cFff62710E1D7792aBa8f3 "depositStake(uint32,uint256)" 3000 100
```

### Pallet Usage
todo