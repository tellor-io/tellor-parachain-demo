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
- This requires the `Ethereum-XCM` pallet, which is currently only available on Moonbase Alpha and does not yet offer 
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

The launch process should conclude with a summary of the various network nodes, along with hyperlinks to launch network explorers for monitoring and interacting with the chains.

### Deploy Contracts & Initialise Chain State
In a new terminal shell, use the `deploy` script to deploy the Tellor contracts to Moonbeam as well as perform required chain state initialisation:
```
./scripts/deploy.sh
```

### Usage
You can then call the contracts using Foundry's `cast`, using the development addresses listed at https://github.com/PureStake/moonbeam#prefunded-development-addresses or submit extrinsics on the consumer parachain at https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9930#/explorer.

#### Approve Token
The following command approves the transfer of 100 TRB for the staking contract (as Baltathar/Bob):
```
cast send --private-key 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b \
  --rpc-url http://localhost:9921/ --legacy 0xFFFFFFFFC8BE577A279484431B9444687EC3D2AE \
  "approve(address,uint256)" 0x970951a12F975E6762482ACA81E57D5A2A4e73F4 100000000000000
```

#### Deposit Stake
The following command deposits a new stake of 100 TRB into the staking contract for a particular parachain (as Baltathar/Bob), which should then report the stake to the corresponding oracle consumer parachain so that the reporter can begin reporting:
```
cast send --private-key 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b \
  --rpc-url http://localhost:9921/ --legacy 0x970951a12F975E6762482ACA81E57D5A2A4e73F4 \
  "depositParachainStake(uint32,bytes,uint256)" 3000 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 100000000000000
```

#### Create Tip
A tip can be created on the consumer parachain by connecting to https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9930#/extrinsics/decode and then pasting in the following hex-encoded call:
```
0x28051c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac80010a5d4e800000000000000000000001468656c6c6f
```

Click **Submission**, ensure that the selected account is **Ferdie** and then click **Submit Transaction** and then **Sign and Submit**.

#### Submit Value
A value can now be submitted to the oracle on the consumer parachain by connecting to https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9930#/extrinsics/decode and then pasting in the following hex-encoded call:
```
0x28071c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac88081afeeaff0ed5cee7d05a21078399c2f56226b0cd5657062500cef4c4e736f85000000000000000000000000000000001468656c6c6f
```

Click **Submission**, ensure that the selected account is **Bob** (as stake deposited above) and then click **Submit Transaction** and then **Sign and Submit**.

#### Request Stake Withdraw
The following command deposits a new stake of 100 TRB into the staking contract for a particular parachain (as Baltathar/Bob), which should then report the stake to the corresponding oracle consumer parachain so that the reporter can begin reporting:
```
cast send --private-key 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b \
  --rpc-url http://localhost:9921/ --legacy 0x970951a12F975E6762482ACA81E57D5A2A4e73F4 \
  "requestParachainStakeWithdraw(uint32,uint256)" 3000 100000000000000
```

#### Begin Dispute
A submitted value can now be disputed on the consumer parachain by connecting to https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9930#/extrinsics/decode and then pasting in the following hex-encoded call:
```
0x28081c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac80000000000000000
```

**Note:** You will need to determine the timestamp for a previously submitted value and then enter it before submitting the call.

Click **Submission**, ensure that the selected account is **Bob** (as the only reporter) and then click **Submit Transaction** and then **Sign and Submit**.


### Pallet Usage
A new data dispute can be started from the network explorer of the oracle consumer chain by clicking **Developer**, **Extrinsics**,
selecting the `tellor` pallet from the drop-down, accepting the default extrinsic of `beginDispute()` and then clicking **Submit Transaction** and finally **Sign and Submit**.

You can then return to **Network**, **Explorer** to monitor the events to confirm that the XCM message was sent. 
A corresponding `xcmpQueue.Success` and `ethereum.Executed` event should appear within the events section of the network explorer of the destination parachain.