use anyhow::Result;
use hex::FromHex;
use moonbeam::{AccountId20, EthereumSigner, MoonbeamConfig, Seed};
use std::{str::FromStr, sync::Arc};
use subxt::{
    config::substrate::U256,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

mod chains;

const PRIVATE_KEY: &str = "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"; // Alith
const XCTRB: &str = "FFFFFFFFC8BE577A279484431B9444687EC3D2AE";

#[tokio::main]
async fn main() -> Result<()> {
    let mut tasks = tokio::task::JoinSet::<Result<()>>::new();

    const ASSET_HUB_URL: &str = "ws://127.0.0.1:9910";
    const EVMP_URL: &str = "ws://127.0.0.1:9920";
    const OCP_URL: &str = "ws://127.0.0.1:9930";

    println!("Starting network initialization...");

    // Asset Reserve
    tasks.spawn(async move {
        let client = OnlineClient::<PolkadotConfig>::from_url(ASSET_HUB_URL).await?;
        let signer = subxt_signer::sr25519::dev::alice();
        // Create TRB token and set metadata
        asset_reserve::create_trb(&signer, &client).await?;
        asset_reserve::set_trb_metadata(&signer, &client).await?;
        // Mint total supply to owner
        asset_reserve::mint_trb(&signer, &client).await?;
        // Init evm parachain sovereign account (required for reserve transfer)
        asset_reserve::init_evm_parachain_sovereign_account(&signer, &client).await?;
        Ok(())
    });

    // EVM Parachain
    tasks.spawn(async move {
        use ethers::{core::utils::moonbeam::MoonbeamDev, prelude::*};
        use std::time::Duration;

        const MOONBEAM_DEV_ENDPOINT: &str = "http://localhost:9920";
        let key = MoonbeamDev::default().alith().clone();
        let wallet: LocalWallet = LocalWallet::from(key).with_chain_id(1280u64);
        let provider = Provider::<Http>::try_from(MOONBEAM_DEV_ENDPOINT)
            .expect("Unable to create Http provider from url")
            .interval(Duration::from_millis(10u64));
        let rpc_client = Arc::new(SignerMiddleware::new(provider, wallet));

        let client = OnlineClient::<MoonbeamConfig>::from_url(EVMP_URL).await?;
        let signer = EthereumSigner::from_seed(Seed::from_hex(PRIVATE_KEY)?)?;

        // Deploy parachain registry contract first, so resulting contract address consistent
        let registry_contract =
            evm::deploy_registry_contract((&client, rpc_client.clone())).await?;
        // Deploy staking contract
        let xctrb = <[u8; 20]>::from_hex(XCTRB)?.into();
        let staking_contract = evm::deploy_staking_contract(
            registry_contract.address(),
            xctrb,
            (&client, rpc_client.clone()),
        )
        .await?;
        // Deploy governance contract
        let multisig: AccountId20 = signer.0.public_key().into();
        let governance_contract = evm::deploy_governance_contract(
            registry_contract.address(),
            multisig.into(),
            (&client, rpc_client),
        )
        .await?;
        // Init staking contract
        let staking_contract_address = staking_contract.address();
        evm::init_staking_contract(staking_contract, governance_contract.address()).await?;
        // Init governance contract
        evm::init_governance_contract(governance_contract, staking_contract_address).await?;

        // Register TRB as foreign asset
        evm::register_trb_as_foreign_asset(&signer, &client).await?;
        evm::set_asset_units_per_second(&signer, &client).await?;
        // Push revert code to EVM
        evm::push_revert_code(&signer, &client).await?;
        // Fund multi-location derivative account of TellorPallet@OracleConsumerParachain (pallet -> contract xcm fees)
        evm::fund_tellor_pallet_derivative_account(&signer, &client).await?;

        Ok(())
    });

    // Oracle Consumer Parachain
    tasks.spawn(async move {
        let client = OnlineClient::<PolkadotConfig>::from_url(OCP_URL).await?;
        let signer = subxt_signer::sr25519::dev::alice();
        // Fund Tellor pallet for remote xcm fees (contract -> pallet xcm fees)
        oracle_consumer::fund_tellor_pallet(&signer, &client).await?;
        Ok(())
    });

    while let Some(_) = tasks.join_next().await {}

    // Register parachain with controller contracts
    let client = OnlineClient::<PolkadotConfig>::from_url(OCP_URL).await?;
    let signer = subxt_signer::sr25519::dev::alice();
    oracle_consumer::register(&signer, &client).await?;

    // Fund oracle reporter via reserve transfer of TRB (asset reserve) to xcTRB (EVM parachain)
    let client = OnlineClient::<PolkadotConfig>::from_url(ASSET_HUB_URL).await?;
    let signer = subxt_signer::sr25519::dev::alice();
    asset_reserve::fund_oracle_reporter_via_reserve_transfer(&signer, &client).await?;

    println!("Network initialization complete");
    Ok(())
}

mod asset_reserve {
    use super::chains::asset_hub::{
        api,
        api::{
            assets::events::{Created, Issued, MetadataSet},
            balances::events::Endowed,
            runtime_types::xcm::v3::{
                junction::Junction::{AccountKey20, GeneralIndex, PalletInstance, Parachain},
                junctions::Junctions::{X1, X2},
                multiasset::{AssetId, Fungibility, MultiAsset, MultiAssets},
                multilocation::MultiLocation,
                WeightLimit::Unlimited,
            },
            runtime_types::xcm::{VersionedMultiAssets, VersionedMultiLocation},
        },
    };
    use super::*;

    const TRB_ID: u32 = 872;

    pub(super) async fn create_trb(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        if let Some(_) = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().assets().asset(TRB_ID))
            .await?
        {
            return Ok(());
        }

        println!("[Asset Hub] Creating TRB token...");
        let create = api::tx().assets().create(
            TRB_ID,
            MultiAddress::Id(signer.public_key().into()),
            1_000_000_000_000_000,
        );
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&create, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<Created>()?
            .expect("Asset Created event not emitted");
        assert_eq!(TRB_ID, event.asset_id);
        Ok(())
    }

    pub(super) async fn set_trb_metadata(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        println!("[Asset Hub] Setting TRB token metadata...");
        let set_metadata = api::tx().assets().set_metadata(
            TRB_ID,
            "Tellor Tribute".as_bytes().to_vec(),
            "TRB".as_bytes().to_vec(),
            18,
        );
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&set_metadata, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<MetadataSet>()?
            .expect("Asset Created event not emitted");
        assert_eq!(TRB_ID, event.asset_id);
        Ok(())
    }

    pub(super) async fn mint_trb(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        let supply = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().assets().asset(TRB_ID))
            .await?
            .map_or(0, |a| a.supply);

        let amount = 10_000_000_000_000_000_000_000 - supply;
        if amount == 0 {
            return Ok(());
        }

        println!("[Asset Hub] Minting {amount} TRB...");
        let mint =
            api::tx()
                .assets()
                .mint(TRB_ID, MultiAddress::Id(signer.public_key().into()), amount);
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&mint, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<Issued>()?
            .expect("Asset Issued event not emitted");
        assert_eq!(TRB_ID, event.asset_id);
        Ok(())
    }

    pub(super) async fn init_evm_parachain_sovereign_account(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        // sibling(2000)
        let sovereign_account =
            AccountId32::from_str("5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8")
                .expect("Sovereign account identifier is valid");
        if let Some(a) = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().system().account(&sovereign_account))
            .await?
        {
            if a.data.free > 0 {
                return Ok(());
            }
        }

        println!("[Asset Hub] Initializing EVM Parachain sovereign account...");

        let transfer = api::tx()
            .balances()
            .transfer(MultiAddress::Id(sovereign_account.clone()), 1000000000000);
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&transfer, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<Endowed>()?
            .expect("Balances Transfer event not emitted");
        assert_eq!(sovereign_account, event.account);
        Ok(())
    }

    pub(super) async fn fund_oracle_reporter_via_reserve_transfer(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        println!("[Asset Hub] Funding oracle reporter via reserve transfer...");

        // Grant Balthazar stake amount
        let transfer = api::tx().polkadot_xcm().limited_reserve_transfer_assets(
            VersionedMultiLocation::V3(MultiLocation {
                parents: 1,
                interior: X1(Parachain(2_000)),
            }),
            VersionedMultiLocation::V3(MultiLocation {
                parents: 0,
                interior: X1(AccountKey20 {
                    network: None,
                    key: <[u8; 20]>::from_hex("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")?,
                }),
            }),
            VersionedMultiAssets::V3(MultiAssets(vec![MultiAsset {
                id: AssetId::Concrete(MultiLocation {
                    parents: 0,
                    interior: X2(PalletInstance(50), GeneralIndex(TRB_ID.into())),
                }),
                fun: Fungibility::Fungible(105_000_000_000_000_000_000),
            }])),
            0,
            Unlimited,
        );
        client
            .tx()
            .sign_and_submit_then_watch_default(&transfer, signer)
            .await?
            .wait_for_in_block()
            .await?;
        Ok(())
    }
}

mod evm {
    use super::chains::{
        evm::api,
        evm::api::{
            asset_manager::events::{ForeignAssetRegistered, UnitsPerSecondChanged},
            balances::events::Endowed,
            runtime_types::{
                frame_system::pallet::Call::set_storage,
                moonbase_runtime::{
                    asset_config::AssetRegistrarMetadata,
                    xcm_config::AssetType,
                    RuntimeCall::{AssetManager, System},
                },
                pallet_asset_manager::pallet::Call::{
                    register_foreign_asset, set_asset_units_per_second,
                },
                xcm::v3::{
                    junction::Junction::{GeneralIndex, PalletInstance, Parachain},
                    junctions::Junctions,
                    multilocation::MultiLocation,
                },
            },
        },
    };
    use super::*;
    use ethabi::Token::Address;
    use ethers::{abi::Token, prelude::*};
    use parity_scale_codec::Encode;
    use serde::de::Visitor;
    use serde::Deserialize;
    use subxt::utils::H160;

    const ASSET_TYPE: AssetType = AssetType::Xcm(MultiLocation {
        parents: 1,
        interior: Junctions::X3(Parachain(1000), PalletInstance(50), GeneralIndex(872)),
    });

    pub(super) async fn deploy_registry_contract<M: Middleware + 'static>(
        clients: (&OnlineClient<MoonbeamConfig>, Arc<M>),
    ) -> Result<ethers::contract::Contract<M>> {
        let contract = include_str!("../contracts/ParachainRegistry.json");
        let address = <[u8; 20]>::from_hex("c01ee7f10ea4af4673cfff62710e1d7792aba8f3")?.into();
        deploy_contract("Parachain Registry", contract, vec![], address, clients).await
    }

    pub(super) async fn deploy_staking_contract<M: Middleware + 'static>(
        registry: H160,
        token: H160,
        clients: (&OnlineClient<MoonbeamConfig>, Arc<M>),
    ) -> Result<ethers::contract::Contract<M>> {
        let contract = include_str!("../contracts/ParachainStaking.json");
        let address = <[u8; 20]>::from_hex("970951a12f975e6762482aca81e57d5a2a4e73f4")?.into();
        deploy_contract(
            "Parachain Staking",
            contract,
            vec![Address(registry), Address(token)],
            address,
            clients,
        )
        .await
    }

    pub(super) async fn init_staking_contract<M: Middleware + 'static>(
        staking_contract: ethers::contract::Contract<M>,
        governance_address: H160,
    ) -> Result<()> {
        println!("[EVM Parachain] Initialising staking contract...");
        staking_contract
            .method::<H160, ()>("init", governance_address)?
            .legacy()
            .send()
            .await?
            .confirmations(0)
            .await?;
        println!("[EVM Parachain] Staking contract initialised with governance contract address.");
        Ok(())
    }

    pub(super) async fn deploy_governance_contract<M: Middleware + 'static>(
        registry: H160,
        team_multisig: H160,
        clients: (&OnlineClient<MoonbeamConfig>, Arc<M>),
    ) -> Result<ethers::contract::Contract<M>> {
        let contract = include_str!("../contracts/ParachainGovernance.json");
        let address = <[u8; 20]>::from_hex("3ed62137c5db927cb137c26455969116bf0c23cb")?.into();
        deploy_contract(
            "Parachain Governance",
            contract,
            vec![Address(registry), Address(team_multisig)],
            address,
            clients,
        )
        .await
    }

    pub(super) async fn init_governance_contract<M: Middleware + 'static>(
        governance_contract: ethers::contract::Contract<M>,
        staking_address: H160,
    ) -> Result<()> {
        println!("[EVM Parachain] Initialising governance contract...");
        governance_contract
            .method::<H160, ()>("init", staking_address)?
            .legacy()
            .send()
            .await?
            .confirmations(0)
            .await?;
        println!("[EVM Parachain] Governance contract initialised with staking contract address.");
        Ok(())
    }

    pub(super) async fn deploy_contract<M: Middleware + 'static>(
        name: &str,
        contract: &str,
        constructor_args: Vec<Token>,
        address: H160,
        clients: (&OnlineClient<MoonbeamConfig>, Arc<M>),
    ) -> Result<ethers::contract::Contract<M>> {
        // Check if contract already deployed at expected address
        let contract: Contract = serde_json::from_str(contract)?;
        if let Some(_) = clients
            .0
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().evm().account_codes_metadata(address))
            .await?
        {
            return Ok(ethers::contract::Contract::new(
                address,
                contract.abi,
                clients.1,
            ));
        }

        // Deploy contract and return result
        println!("[EVM Parachain] Deploying {name} contract...");
        let factory =
            ContractFactory::new(contract.abi, contract.bytecode.object.into(), clients.1);
        let contract = factory
            .deploy_tokens(constructor_args)?
            .legacy()
            .send()
            .await?;
        println!(
            "[EVM Parachain] {name} contract deployed to {}",
            contract.address()
        );
        Ok(contract)
    }

    pub fn deserialize_bytecode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HexStrVisitor;

        impl<'de> Visitor<'de> for HexStrVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a hex encoded string")
            }

            fn visit_borrowed_str<E>(self, data: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                FromHex::from_hex(data.strip_prefix("0x").expect("Expected 0x hex prefix"))
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(HexStrVisitor)
    }

    #[derive(Deserialize)]
    struct Contract {
        abi: abi::Abi,
        bytecode: Bytecode,
    }

    #[derive(Deserialize)]
    struct Bytecode {
        #[serde(deserialize_with = "deserialize_bytecode")]
        object: Vec<u8>,
    }

    pub(super) async fn register_trb_as_foreign_asset(
        signer: &EthereumSigner,
        client: &OnlineClient<MoonbeamConfig>,
    ) -> Result<()> {
        const ASSET_ID: u128 = 266833909807825597260388579262846980782;
        if let Some(_) = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().asset_manager().asset_id_type(ASSET_ID))
            .await?
        {
            return Ok(());
        }

        // Prepare call
        println!("[EVM Parachain] Registering TRB as foreign asset...");
        let register = api::tx().sudo().sudo(AssetManager(register_foreign_asset {
            asset: ASSET_TYPE,
            metadata: AssetRegistrarMetadata {
                name: "Tellor Tribute".as_bytes().to_vec(),
                symbol: "xcTRB".as_bytes().to_vec(),
                decimals: 18,
                is_frozen: false,
            },
            min_amount: 1,
            is_sufficient: true,
        }));

        // Register foreign asset
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&register, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<ForeignAssetRegistered>()?
            .expect("contract could not be called");
        assert_eq!(ASSET_ID, event.asset_id);
        Ok(())
    }

    pub(super) async fn set_asset_units_per_second(
        signer: &EthereumSigner,
        client: &OnlineClient<MoonbeamConfig>,
    ) -> Result<()> {
        if let Some(_) = client
            .storage()
            .at_latest()
            .await?
            .fetch(
                &api::storage()
                    .asset_manager()
                    .asset_type_units_per_second(ASSET_TYPE),
            )
            .await?
        {
            return Ok(());
        }

        // Prepare call
        println!("[EVM Parachain] Setting asset units per second...");
        let set = api::tx()
            .sudo()
            .sudo(AssetManager(set_asset_units_per_second {
                asset_type: ASSET_TYPE,
                units_per_second: 100_000, // todo: size correctly
                num_assets_weight_hint: 5, // todo: size correctly
            }));

        // Set asset units per second
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&set, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<UnitsPerSecondChanged>()?
            .expect("contract could not be called");
        assert_eq!(ASSET_TYPE.encode(), event.asset_type.encode());
        Ok(())
    }

    // Required for transferFrom usage within a smart contract, especially to return revert specifics
    // https://github.com/PureStake/xcm-tools/blob/4a3dcdb49434bcc019106677d01be54f9f17b30b/scripts/xcm-asset-registrator.ts#L87-L107
    pub(super) async fn push_revert_code(
        signer: &EthereumSigner,
        client: &OnlineClient<MoonbeamConfig>,
    ) -> Result<()> {
        // Prepare call
        println!("[EVM Parachain] Setting revert code...");
        let set_storage = api::tx().sudo().sudo(System(set_storage { items: vec![(
            // Note: hardcoded based on output of setStorage (decoded) from manual run of xcm-tools: yarn register-asset .. --revert-code true
            Vec::from_hex("1da53b775b270400e7e61ed5cbc5a146ea70f53d5a3306ce02aaf97049cf181a8d25f78201571c23f3f5096d309394ecffffffffc8be577a279484431b9444687ec3d2ae")?,
            Vec::from_hex("1460006000fd")?,
            )] }));

        // Set storage
        client
            .tx()
            .sign_and_submit_then_watch_default(&set_storage, signer)
            .await?
            .wait_for_in_block()
            .await?;
        Ok(())
    }

    pub(super) async fn fund_tellor_pallet_derivative_account(
        signer: &EthereumSigner,
        client: &OnlineClient<MoonbeamConfig>,
    ) -> Result<()> {
        let pallet_account = AccountId20(
            <[u8; 20]>::from_hex("26ab7997cf6d531fed12b2fa6bc3cf2248724195")
                .expect("Derivative account identifier is valid"),
        );
        if let Some(a) = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().system().account(&pallet_account))
            .await?
        {
            if a.data.free > 0 {
                return Ok(());
            }
        }

        println!("[EVM Parachain] Funding pallet derivative account...");
        let transfer = api::tx()
            .balances()
            .transfer(pallet_account.clone(), 1000000000000000000);
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&transfer, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<Endowed>()?
            .expect("Balances Transfer event not emitted");
        assert_eq!(pallet_account.0, event.account.0);
        Ok(())
    }

    impl From<U256> for chains::evm::api::runtime_types::primitive_types::U256 {
        fn from(value: U256) -> Self {
            chains::evm::api::runtime_types::primitive_types::U256(value.0)
        }
    }
}

mod oracle_consumer {
    use super::chains::oracle_consumer::{
        api,
        api::{
            balances::events::Endowed,
            runtime_types::{
                parachain_template_runtime::RuntimeCall::Tellor, tellor::pallet::Call::register,
            },
            tellor::events::RegistrationSent,
        },
    };
    use super::*;
    use subxt::{
        utils::{AccountId32, MultiAddress},
        PolkadotConfig,
    };

    pub(super) async fn fund_tellor_pallet(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        let pallet_account =
            AccountId32::from_str("5EYCAe5ijiYfyFR5Zp1cdckhKCKaUyo9KQsTcU1NR9u2mwTk")
                .expect("Sovereign account identifier is valid");
        if let Some(a) = client
            .storage()
            .at_latest()
            .await?
            .fetch(&api::storage().system().account(&pallet_account))
            .await?
        {
            if a.data.free > 0 {
                return Ok(());
            }
        }

        println!("[Oracle Consumer Parachain] Funding pallet account...");
        let transfer = api::tx().balances().transfer(
            MultiAddress::Id(pallet_account.clone()),
            100_000_000_000_000_000,
        );
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&transfer, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?
            .find_first::<Endowed>()?
            .expect("Balances Transfer event not emitted");
        assert_eq!(pallet_account, event.account);
        Ok(())
    }

    pub(super) async fn register(
        signer: &subxt_signer::sr25519::Keypair,
        client: &OnlineClient<PolkadotConfig>,
    ) -> Result<()> {
        println!("[Oracle Consumer Parachain] Registering parachain with controller contracts...");
        let create = api::tx().sudo().sudo(Tellor(register { gas_limit: None }));

        // Call contract
        let events = client
            .tx()
            .sign_and_submit_then_watch_default(&create, signer)
            .await?
            .wait_for_in_block()
            .await?
            .fetch_events()
            .await?;
        let event = events
            .find_first::<RegistrationSent>()
            .expect("Parachain could not be registered")
            .expect("Parachain could not be registered");
        assert_eq!(2_000, event.para_id);
        assert_eq!(2_000, event.para_id);
        Ok(())
    }
}

mod moonbeam {
    pub use super::chains::evm::api::runtime_types::account::AccountId20;
    use super::chains::evm::api::runtime_types::{
        account::EthereumSignature, sp_core::ecdsa::Signature,
    };
    use parity_scale_codec::{Compact, Encode};
    use subxt::{
        client::OfflineClientT,
        config::{ExtrinsicParams, ExtrinsicParamsEncoder, SignedExtension},
        utils::H160,
        Config, PolkadotConfig,
    };
    use subxt_signer::ecdsa::PublicKey;
    pub use subxt_signer::ecdsa::Seed;

    impl From<subxt_signer::ecdsa::PublicKey> for AccountId20 {
        fn from(value: PublicKey) -> Self {
            use k256::{elliptic_curve::sec1::ToEncodedPoint, PublicKey};

            AccountId20(
                PublicKey::from_sec1_bytes(value.0.as_slice())
                    .map_err(drop)
                    .and_then(|pub_key| {
                        // uncompress the key
                        let uncompressed = pub_key.to_encoded_point(false);
                        // convert to ETH address
                        <[u8; 20]>::try_from(
                            sp_io::hashing::keccak_256(&uncompressed.as_bytes()[1..])[12..]
                                .as_ref(),
                        )
                        .map_err(drop)
                    })
                    .unwrap(),
            )
        }
    }

    impl From<AccountId20> for H160 {
        fn from(value: AccountId20) -> Self {
            H160(value.0)
        }
    }

    pub struct EthereumSigner(pub(super) subxt_signer::ecdsa::Keypair);

    impl EthereumSigner {
        pub fn from_seed(seed: Seed) -> Result<Self, subxt_signer::ecdsa::Error> {
            Ok(Self(subxt_signer::ecdsa::Keypair::from_seed(seed)?))
        }
    }

    impl<T: Config> subxt::tx::Signer<T> for EthereumSigner
    where
        T::AccountId: From<AccountId20>,
        T::Address: From<AccountId20>,
        T::Signature: From<EthereumSignature>,
    {
        fn account_id(&self) -> T::AccountId {
            AccountId20::from(self.0.public_key()).into()
        }

        fn address(&self) -> T::Address {
            AccountId20::from(self.0.public_key()).into()
        }

        fn sign(&self, message: &[u8]) -> T::Signature {
            use secp256k1::{ecdsa::RecoverableSignature, Message, SECP256K1};

            // From sp_core::ecdsa::sign:
            let message_hash = sp_core::hashing::keccak_256(message);
            // From sp_core::ecdsa::sign_prehashed:
            let wrapped = Message::from_slice(&message_hash).expect("Message is 32 bytes; qed");
            let recsig: RecoverableSignature =
                SECP256K1.sign_ecdsa_recoverable(&wrapped, &self.0 .0.secret_key());
            // From sp_core::ecdsa's `impl From<RecoverableSignature> for Signature`:
            let (recid, sig) = recsig.serialize_compact();
            let mut signature_bytes: [u8; 65] = [0; 65];
            signature_bytes[..64].copy_from_slice(&sig);
            signature_bytes[64] = (recid.to_i32() & 0xFF) as u8;

            EthereumSignature(Signature(signature_bytes)).into()
        }
    }

    pub enum MoonbeamConfig {}
    impl Config for MoonbeamConfig {
        type Hash = <PolkadotConfig as Config>::Hash;
        type AccountId = AccountId20;
        type Address = Self::AccountId;
        type Signature = EthereumSignature;
        type Hasher = <PolkadotConfig as Config>::Hasher;
        type Header = <PolkadotConfig as Config>::Header;
        type ExtrinsicParams = MoonbeamExtrinsicParams<Self>;
    }

    pub type MoonbeamExtrinsicParams<T> = subxt::config::signed_extensions::AnyOf<
        T,
        (
            // CheckNonZeroSender
            subxt::config::signed_extensions::CheckSpecVersion,
            subxt::config::signed_extensions::CheckTxVersion,
            subxt::config::signed_extensions::CheckGenesis<T>,
            subxt::config::signed_extensions::CheckMortality<T>,
            CheckNonce,
            // CheckWeight
            subxt::config::signed_extensions::ChargeTransactionPayment,
        ),
    >;

    /// The [`CheckNonce`] signed extension.
    #[derive(Debug)]
    pub struct CheckNonce(Compact<u32>);

    impl<T: Config> ExtrinsicParams<T> for CheckNonce {
        type OtherParams = ();
        type Error = std::convert::Infallible;

        fn new<Client: OfflineClientT<T>>(
            nonce: u64,
            _client: Client,
            _other_params: Self::OtherParams,
        ) -> Result<Self, Self::Error> {
            Ok(CheckNonce(Compact(nonce as u32)))
        }
    }

    impl ExtrinsicParamsEncoder for CheckNonce {
        fn encode_extra_to(&self, v: &mut Vec<u8>) {
            self.0.encode_to(v);
        }
    }

    impl<T: Config> SignedExtension<T> for CheckNonce {
        const NAME: &'static str = "CheckNonce";
    }
}
