#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 18usize] = [
        "System",
        "ParachainSystem",
        "Timestamp",
        "ParachainInfo",
        "Balances",
        "TransactionPayment",
        "Authorship",
        "CollatorSelection",
        "Session",
        "Aura",
        "AuraExt",
        "XcmpQueue",
        "PolkadotXcm",
        "CumulusXcm",
        "DmpQueue",
        "Sudo",
        "Tellor",
        "UsingTellor",
    ];
    pub static RUNTIME_APIS: [&str; 0usize] = [];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::parachain_template_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::parachain_template_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::parachain_template_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn tellor(&self) -> tellor::constants::ConstantsApi {
            tellor::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
            parachain_system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
            parachain_info::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi {
            authorship::storage::StorageApi
        }
        pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
            collator_selection::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
            aura_ext::storage::StorageApi
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
            xcmp_queue::storage::StorageApi
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
            polkadot_xcm::storage::StorageApi
        }
        pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi {
            dmp_queue::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn tellor(&self) -> tellor::storage::StorageApi {
            tellor::storage::StorageApi
        }
        pub fn using_tellor(&self) -> using_tellor::storage::StorageApi {
            using_tellor::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
            parachain_system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
            parachain_info::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
            collator_selection::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
            xcmp_queue::calls::TransactionApi
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
            polkadot_xcm::calls::TransactionApi
        }
        pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
            cumulus_xcm::calls::TransactionApi
        }
        pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi {
            dmp_queue::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn tellor(&self) -> tellor::calls::TransactionApi {
            tellor::calls::TransactionApi
        }
        pub fn using_tellor(&self) -> using_tellor::calls::TransactionApi {
            using_tellor::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                34u8, 56u8, 122u8, 235u8, 218u8, 197u8, 48u8, 224u8, 202u8, 25u8, 176u8, 251u8,
                123u8, 85u8, 203u8, 206u8, 150u8, 192u8, 34u8, 228u8, 57u8, 251u8, 10u8, 61u8,
                206u8, 146u8, 148u8, 232u8, 17u8, 214u8, 122u8, 53u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                    pub subkeys: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "- `O(1)`"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::utils::AccountId32,
                pub hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::parachain_template_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            68u8, 173u8, 60u8, 119u8, 35u8, 94u8, 227u8, 132u8, 70u8, 235u8, 30u8,
                            146u8, 79u8, 80u8, 227u8, 87u8, 245u8, 32u8, 31u8, 77u8, 196u8, 170u8,
                            22u8, 82u8, 97u8, 95u8, 19u8, 207u8, 4u8, 110u8, 100u8, 156u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::Phase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod parachain_system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::cumulus_pallet_parachain_system::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::cumulus_pallet_parachain_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetValidationData {
                    pub data:
                        runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetValidationData {
                    const PALLET: &'static str = "ParachainSystem";
                    const CALL: &'static str = "set_validation_data";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoSendUpwardMessage {
                    pub message: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoSendUpwardMessage {
                    const PALLET: &'static str = "ParachainSystem";
                    const CALL: &'static str = "sudo_send_upward_message";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AuthorizeUpgrade {
                    pub code_hash: ::subxt::utils::H256,
                    pub check_version: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "ParachainSystem";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct EnactAuthorizedUpgrade {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for EnactAuthorizedUpgrade {
                    const PALLET: &'static str = "ParachainSystem";
                    const CALL: &'static str = "enact_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current validation data."]
                #[doc = ""]
                #[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase if the call was not invoked."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`"]
                #[doc = ""]
                #[doc = "As a side effect, this function upgrades the current validation function"]
                #[doc = "if the appropriate time has come."]
                pub fn set_validation_data(
                    &self,
                    data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
                ) -> ::subxt::tx::Payload<types::SetValidationData> {
                    ::subxt::tx::Payload::new_static(
                        "ParachainSystem",
                        "set_validation_data",
                        types::SetValidationData { data },
                        [
                            167u8, 126u8, 75u8, 137u8, 220u8, 60u8, 106u8, 214u8, 92u8, 170u8,
                            136u8, 176u8, 98u8, 0u8, 234u8, 217u8, 146u8, 113u8, 149u8, 88u8,
                            114u8, 141u8, 228u8, 105u8, 136u8, 71u8, 233u8, 18u8, 70u8, 36u8, 24u8,
                            249u8,
                        ],
                    )
                }
                pub fn sudo_send_upward_message(
                    &self,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SudoSendUpwardMessage> {
                    ::subxt::tx::Payload::new_static(
                        "ParachainSystem",
                        "sudo_send_upward_message",
                        types::SudoSendUpwardMessage { message },
                        [
                            1u8, 231u8, 11u8, 78u8, 127u8, 117u8, 248u8, 67u8, 230u8, 199u8, 126u8,
                            47u8, 20u8, 62u8, 252u8, 138u8, 199u8, 48u8, 41u8, 21u8, 28u8, 157u8,
                            218u8, 143u8, 4u8, 253u8, 62u8, 192u8, 94u8, 252u8, 92u8, 180u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "The `check_version` parameter sets a boolean flag for whether or not the runtime's spec"]
                #[doc = "version and name should be verified on upgrade. Since the authorization only has a hash,"]
                #[doc = "it cannot actually perform the verification."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade(
                    &self,
                    code_hash: ::subxt::utils::H256,
                    check_version: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::AuthorizeUpgrade> {
                    ::subxt::tx::Payload::new_static(
                        "ParachainSystem",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade {
                            code_hash,
                            check_version,
                        },
                        [
                            213u8, 114u8, 107u8, 169u8, 223u8, 147u8, 205u8, 204u8, 3u8, 81u8,
                            228u8, 0u8, 82u8, 57u8, 43u8, 95u8, 12u8, 59u8, 241u8, 176u8, 143u8,
                            131u8, 253u8, 166u8, 98u8, 187u8, 94u8, 235u8, 177u8, 110u8, 162u8,
                            218u8,
                        ],
                    )
                }
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Note that this function will not apply the new `code`, but only attempt to schedule the"]
                #[doc = "upgrade with the Relay Chain."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub fn enact_authorized_upgrade(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::EnactAuthorizedUpgrade> {
                    ::subxt::tx::Payload::new_static(
                        "ParachainSystem",
                        "enact_authorized_upgrade",
                        types::EnactAuthorizedUpgrade { code },
                        [
                            232u8, 135u8, 114u8, 87u8, 196u8, 146u8, 244u8, 19u8, 106u8, 73u8,
                            88u8, 193u8, 48u8, 14u8, 72u8, 133u8, 247u8, 147u8, 50u8, 95u8, 252u8,
                            213u8, 192u8, 47u8, 244u8, 102u8, 195u8, 120u8, 179u8, 87u8, 94u8, 8u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The validation function has been scheduled to apply."]
            pub struct ValidationFunctionStored;
            impl ::subxt::events::StaticEvent for ValidationFunctionStored {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The validation function was applied as of the contained relay chain block number."]
            pub struct ValidationFunctionApplied {
                pub relay_chain_block_num: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionApplied";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The relay-chain aborted the upgrade process."]
            pub struct ValidationFunctionDiscarded;
            impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionDiscarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An upgrade has been authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some downward messages have been received and will be processed."]
            pub struct DownwardMessagesReceived {
                pub count: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward messages were processed using the given weight."]
            pub struct DownwardMessagesProcessed {
                pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
                pub dmq_head: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesProcessed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An upward message was sent to the relay chain."]
            pub struct UpwardMessageSent {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for UpwardMessageSent {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpwardMessageSent";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be applied."]
                #[doc = ""]
                #[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]"]
                #[doc = " which will result the next block process with the new validation code. This concludes the upgrade process."]
                #[doc = ""]
                #[doc = " [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE"]
                pub fn pending_validation_code(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "PendingValidationCode",
                        vec![],
                        [
                            78u8, 159u8, 219u8, 211u8, 177u8, 80u8, 102u8, 93u8, 83u8, 146u8, 90u8,
                            233u8, 232u8, 11u8, 104u8, 172u8, 93u8, 68u8, 44u8, 228u8, 99u8, 197u8,
                            254u8, 28u8, 181u8, 215u8, 247u8, 238u8, 49u8, 49u8, 195u8, 249u8,
                        ],
                    )
                }
                #[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
                #[doc = " consequently the relay-chain."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
                #[doc = " the value."]
                pub fn new_validation_code(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "NewValidationCode",
                        vec![],
                        [
                            185u8, 123u8, 152u8, 122u8, 230u8, 136u8, 79u8, 73u8, 206u8, 19u8,
                            59u8, 57u8, 75u8, 250u8, 83u8, 185u8, 29u8, 76u8, 89u8, 137u8, 77u8,
                            163u8, 25u8, 125u8, 182u8, 67u8, 2u8, 180u8, 48u8, 237u8, 49u8, 171u8,
                        ],
                    )
                }
                #[doc = " The [`PersistedValidationData`] set for this block."]
                #[doc = " This value is expected to be set only once per block and it's never stored"]
                #[doc = " in the trie."]
                pub fn validation_data(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::polkadot_primitives::v4::PersistedValidationData<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "ValidationData",
                        vec![],
                        [
                            193u8, 240u8, 25u8, 56u8, 103u8, 173u8, 56u8, 56u8, 229u8, 243u8, 91u8,
                            25u8, 249u8, 95u8, 122u8, 93u8, 37u8, 181u8, 54u8, 244u8, 217u8, 200u8,
                            62u8, 136u8, 80u8, 148u8, 16u8, 177u8, 124u8, 211u8, 95u8, 24u8,
                        ],
                    )
                }
                #[doc = " Were the validation data set to notify the relay chain?"]
                pub fn did_set_validation_code(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "DidSetValidationCode",
                        vec![],
                        [
                            233u8, 228u8, 48u8, 111u8, 200u8, 35u8, 30u8, 139u8, 251u8, 77u8,
                            196u8, 252u8, 35u8, 222u8, 129u8, 235u8, 7u8, 19u8, 156u8, 82u8, 126u8,
                            173u8, 29u8, 62u8, 20u8, 67u8, 166u8, 116u8, 108u8, 182u8, 57u8, 246u8,
                        ],
                    )
                }
                #[doc = " The relay chain block number associated with the last parachain block."]
                pub fn last_relay_chain_block_number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "LastRelayChainBlockNumber",
                        vec![],
                        [
                            17u8, 65u8, 131u8, 169u8, 195u8, 243u8, 195u8, 93u8, 220u8, 174u8,
                            75u8, 216u8, 214u8, 227u8, 96u8, 40u8, 8u8, 153u8, 116u8, 160u8, 79u8,
                            255u8, 35u8, 232u8, 242u8, 42u8, 100u8, 150u8, 208u8, 210u8, 142u8,
                            186u8,
                        ],
                    )
                }
                #[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
                #[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
                #[doc = " candidate will be invalid."]
                #[doc = ""]
                #[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
                #[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
                #[doc = " set after the inherent."]
                pub fn upgrade_restriction_signal(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v4::UpgradeRestriction,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "UpgradeRestrictionSignal",
                        vec![],
                        [
                            235u8, 240u8, 37u8, 44u8, 181u8, 52u8, 7u8, 216u8, 20u8, 139u8, 69u8,
                            124u8, 21u8, 173u8, 237u8, 64u8, 105u8, 88u8, 49u8, 69u8, 123u8, 55u8,
                            181u8, 167u8, 112u8, 183u8, 190u8, 231u8, 231u8, 127u8, 77u8, 148u8,
                        ],
                    )
                }
                #[doc = " The state proof for the last relay parent block."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]
                pub fn relay_state_proof(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_trie::storage_proof::StorageProof,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "RelayStateProof",
                        vec![],
                        [
                            46u8, 115u8, 163u8, 190u8, 246u8, 47u8, 200u8, 159u8, 206u8, 204u8,
                            94u8, 250u8, 127u8, 112u8, 109u8, 111u8, 210u8, 195u8, 244u8, 41u8,
                            36u8, 187u8, 71u8, 150u8, 149u8, 253u8, 143u8, 33u8, 83u8, 189u8,
                            182u8, 238u8,
                        ],
                    )
                }
                #[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
                #[doc = " the relay parent."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]                pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "RelevantMessagingState",
                        vec![],
                        [
                            122u8, 98u8, 235u8, 251u8, 121u8, 64u8, 21u8, 37u8, 101u8, 8u8, 217u8,
                            99u8, 53u8, 5u8, 101u8, 48u8, 248u8, 181u8, 61u8, 235u8, 119u8, 203u8,
                            252u8, 210u8, 183u8, 171u8, 146u8, 31u8, 22u8, 211u8, 88u8, 96u8,
                        ],
                    )
                }
                #[doc = " The parachain host configuration that was obtained from the relay parent."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]
                pub fn host_configuration(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::polkadot_primitives::v4::AbridgedHostConfiguration,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "HostConfiguration",
                        vec![],
                        [
                            224u8, 81u8, 104u8, 216u8, 84u8, 180u8, 220u8, 34u8, 251u8, 192u8,
                            110u8, 151u8, 172u8, 254u8, 133u8, 68u8, 16u8, 230u8, 99u8, 164u8,
                            162u8, 159u8, 189u8, 125u8, 249u8, 187u8, 148u8, 253u8, 71u8, 64u8,
                            89u8, 88u8,
                        ],
                    )
                }
                #[doc = " The last downward message queue chain head we have observed."]
                #[doc = ""]
                #[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
                #[doc = " by the system inherent."]
                pub fn last_dmq_mqc_head(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "LastDmqMqcHead",
                        vec![],
                        [
                            1u8, 70u8, 140u8, 40u8, 51u8, 127u8, 75u8, 80u8, 5u8, 49u8, 196u8,
                            31u8, 30u8, 61u8, 54u8, 252u8, 0u8, 0u8, 100u8, 115u8, 177u8, 250u8,
                            138u8, 48u8, 107u8, 41u8, 93u8, 87u8, 195u8, 107u8, 206u8, 227u8,
                        ],
                    )
                }
                #[doc = " The message queue chain heads we have observed per each channel incoming channel."]
                #[doc = ""]
                #[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
                #[doc = " by the system inherent."]
                pub fn last_hrmp_mqc_heads(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::KeyedVec<
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "LastHrmpMqcHeads",
                        vec![],
                        [
                            131u8, 170u8, 142u8, 30u8, 101u8, 113u8, 131u8, 81u8, 38u8, 168u8,
                            98u8, 3u8, 9u8, 109u8, 96u8, 179u8, 115u8, 177u8, 128u8, 11u8, 238u8,
                            54u8, 81u8, 60u8, 97u8, 112u8, 224u8, 175u8, 86u8, 133u8, 182u8, 76u8,
                        ],
                    )
                }
                #[doc = " Number of downward messages processed in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn processed_downward_messages(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "ProcessedDownwardMessages",
                        vec![],
                        [
                            151u8, 234u8, 196u8, 87u8, 130u8, 79u8, 4u8, 102u8, 47u8, 10u8, 33u8,
                            132u8, 149u8, 118u8, 61u8, 141u8, 5u8, 1u8, 30u8, 120u8, 220u8, 156u8,
                            16u8, 11u8, 14u8, 52u8, 126u8, 151u8, 244u8, 149u8, 197u8, 51u8,
                        ],
                    )
                }
                #[doc = " HRMP watermark that was set in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn hrmp_watermark(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "HrmpWatermark",
                        vec![],
                        [
                            77u8, 62u8, 59u8, 220u8, 7u8, 125u8, 98u8, 249u8, 108u8, 212u8, 223u8,
                            99u8, 152u8, 13u8, 29u8, 80u8, 166u8, 65u8, 232u8, 113u8, 145u8, 128u8,
                            123u8, 35u8, 238u8, 31u8, 113u8, 156u8, 220u8, 104u8, 217u8, 165u8,
                        ],
                    )
                }
                #[doc = " HRMP messages that were sent in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn hrmp_outbound_messages(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "HrmpOutboundMessages",
                        vec![],
                        [
                            42u8, 9u8, 96u8, 217u8, 25u8, 101u8, 129u8, 147u8, 150u8, 20u8, 164u8,
                            186u8, 217u8, 178u8, 15u8, 201u8, 233u8, 104u8, 92u8, 120u8, 29u8,
                            245u8, 196u8, 13u8, 141u8, 210u8, 102u8, 62u8, 216u8, 80u8, 246u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Upward messages that were sent in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn upward_messages(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "UpwardMessages",
                        vec![],
                        [
                            179u8, 127u8, 8u8, 94u8, 194u8, 246u8, 53u8, 79u8, 80u8, 22u8, 18u8,
                            75u8, 116u8, 163u8, 90u8, 161u8, 30u8, 140u8, 57u8, 126u8, 60u8, 91u8,
                            23u8, 30u8, 120u8, 245u8, 125u8, 96u8, 152u8, 25u8, 248u8, 85u8,
                        ],
                    )
                }
                #[doc = " Upward messages that are still pending and not yet send to the relay chain."]
                pub fn pending_upward_messages(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "PendingUpwardMessages",
                        vec![],
                        [
                            239u8, 45u8, 18u8, 173u8, 148u8, 150u8, 55u8, 176u8, 173u8, 156u8,
                            246u8, 226u8, 198u8, 214u8, 104u8, 187u8, 186u8, 13u8, 83u8, 194u8,
                            153u8, 29u8, 228u8, 109u8, 26u8, 18u8, 212u8, 151u8, 246u8, 24u8,
                            133u8, 216u8,
                        ],
                    )
                }
                #[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
                #[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
                pub fn announced_hrmp_messages_per_candidate(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "AnnouncedHrmpMessagesPerCandidate",
                        vec![],
                        [
                            93u8, 11u8, 229u8, 172u8, 73u8, 87u8, 13u8, 149u8, 15u8, 94u8, 163u8,
                            107u8, 156u8, 22u8, 131u8, 177u8, 96u8, 247u8, 213u8, 224u8, 41u8,
                            126u8, 157u8, 33u8, 154u8, 194u8, 95u8, 234u8, 65u8, 19u8, 58u8, 161u8,
                        ],
                    )
                }
                #[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
                #[doc = " overrides the amount set in the Config trait."]
                pub fn reserved_xcmp_weight_override(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_weights::weight_v2::Weight,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "ReservedXcmpWeightOverride",
                        vec![],
                        [
                            176u8, 93u8, 203u8, 74u8, 18u8, 170u8, 246u8, 203u8, 109u8, 89u8, 86u8,
                            77u8, 96u8, 66u8, 189u8, 79u8, 184u8, 253u8, 11u8, 230u8, 87u8, 120u8,
                            1u8, 254u8, 215u8, 41u8, 210u8, 86u8, 239u8, 206u8, 60u8, 2u8,
                        ],
                    )
                }
                #[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
                #[doc = " overrides the amount set in the Config trait."]
                pub fn reserved_dmp_weight_override(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_weights::weight_v2::Weight,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "ReservedDmpWeightOverride",
                        vec![],
                        [
                            205u8, 124u8, 9u8, 156u8, 255u8, 207u8, 208u8, 23u8, 179u8, 132u8,
                            254u8, 157u8, 237u8, 240u8, 167u8, 203u8, 253u8, 111u8, 136u8, 32u8,
                            100u8, 152u8, 16u8, 19u8, 175u8, 14u8, 108u8, 61u8, 59u8, 231u8, 70u8,
                            112u8,
                        ],
                    )
                }
                #[doc = " The next authorized upgrade, if there is one."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::cumulus_pallet_parachain_system::CodeUpgradeAuthorization,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "AuthorizedUpgrade",
                        vec![],
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
                #[doc = " A custom head data that should be returned as result of `validate_block`."]
                #[doc = ""]
                #[doc = " See [`Pallet::set_custom_validation_head_data`] for more information."]
                pub fn custom_validation_head_data(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainSystem",
                        "CustomValidationHeadData",
                        vec![],
                        [
                            52u8, 186u8, 187u8, 57u8, 245u8, 171u8, 202u8, 23u8, 92u8, 80u8, 118u8,
                            66u8, 251u8, 156u8, 175u8, 254u8, 141u8, 185u8, 115u8, 209u8, 170u8,
                            165u8, 1u8, 242u8, 120u8, 234u8, 162u8, 24u8, 135u8, 105u8, 8u8, 177u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Set {
                    #[codec(compact)]
                    pub now: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
                    ::subxt::tx::Payload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod parachain_info {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::parachain_info::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
            }
            pub struct TransactionApi;
            impl TransactionApi {}
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn parachain_id(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParachainInfo",
                        "ParachainId",
                        vec![],
                        [
                            160u8, 130u8, 74u8, 181u8, 231u8, 180u8, 246u8, 152u8, 204u8, 44u8,
                            245u8, 91u8, 113u8, 246u8, 218u8, 50u8, 254u8, 248u8, 35u8, 219u8,
                            83u8, 144u8, 228u8, 245u8, 122u8, 53u8, 194u8, 172u8, 222u8, 118u8,
                            202u8, 91u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalanceDeprecated {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                    #[codec(compact)]
                    pub old_reserved: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "set_balance_deprecated";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub keep_alive: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Transfer {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub fn transfer_allow_death(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account; it also takes a reserved balance but this"]
                #[doc = "must be the same as the account's current reserved balance."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                #[doc = ""]
                #[doc = "WARNING: This call is DEPRECATED! Use `force_set_balance` instead."]
                pub fn set_balance_deprecated(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                    old_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            125u8, 171u8, 21u8, 186u8, 108u8, 185u8, 241u8, 145u8, 125u8, 8u8,
                            12u8, 42u8, 96u8, 114u8, 80u8, 80u8, 227u8, 76u8, 20u8, 208u8, 93u8,
                            219u8, 36u8, 50u8, 209u8, 155u8, 70u8, 45u8, 6u8, 57u8, 156u8, 77u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibililty of churn)."]
                pub fn upgrade_accounts(
                    &self,
                    who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "Alias for `transfer_allow_death`, provided only for name-wise compatibility."]
                #[doc = ""]
                #[doc = "WARNING: DEPRECATED! Will be released in approximately 3 months."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer { dest, value },
                        [
                            154u8, 145u8, 140u8, 54u8, 50u8, 123u8, 225u8, 249u8, 200u8, 217u8,
                            172u8, 110u8, 233u8, 198u8, 77u8, 198u8, 211u8, 89u8, 8u8, 13u8, 240u8,
                            94u8, 28u8, 13u8, 242u8, 217u8, 168u8, 23u8, 106u8, 254u8, 249u8,
                            120u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn force_set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::utils::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        Vec::new(),
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        Vec::new(),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of holds that can exist on an account at any time."]
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::utils::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Author of current block."]
                pub fn author(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Authorship",
                        "Author",
                        vec![],
                        [
                            247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
                            220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
                            61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
                            18u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod collator_selection {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_collator_selection::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_collator_selection::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetInvulnerables {
                    pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
                    const PALLET: &'static str = "CollatorSelection";
                    const CALL: &'static str = "set_invulnerables";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetDesiredCandidates {
                    pub max: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetDesiredCandidates {
                    const PALLET: &'static str = "CollatorSelection";
                    const CALL: &'static str = "set_desired_candidates";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCandidacyBond {
                    pub bond: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCandidacyBond {
                    const PALLET: &'static str = "CollatorSelection";
                    const CALL: &'static str = "set_candidacy_bond";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RegisterAsCandidate;
                impl ::subxt::blocks::StaticExtrinsic for RegisterAsCandidate {
                    const PALLET: &'static str = "CollatorSelection";
                    const CALL: &'static str = "register_as_candidate";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct LeaveIntent;
                impl ::subxt::blocks::StaticExtrinsic for LeaveIntent {
                    const PALLET: &'static str = "CollatorSelection";
                    const CALL: &'static str = "leave_intent";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the list of invulnerable (fixed) collators."]
                pub fn set_invulnerables(
                    &self,
                    new: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::SetInvulnerables> {
                    ::subxt::tx::Payload::new_static(
                        "CollatorSelection",
                        "set_invulnerables",
                        types::SetInvulnerables { new },
                        [
                            113u8, 217u8, 14u8, 48u8, 6u8, 198u8, 8u8, 170u8, 8u8, 237u8, 230u8,
                            184u8, 17u8, 181u8, 15u8, 126u8, 117u8, 3u8, 208u8, 215u8, 40u8, 16u8,
                            150u8, 162u8, 37u8, 196u8, 235u8, 36u8, 247u8, 24u8, 187u8, 17u8,
                        ],
                    )
                }
                #[doc = "Set the ideal number of collators (not including the invulnerables)."]
                #[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
                #[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
                pub fn set_desired_candidates(
                    &self,
                    max: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::SetDesiredCandidates> {
                    ::subxt::tx::Payload::new_static(
                        "CollatorSelection",
                        "set_desired_candidates",
                        types::SetDesiredCandidates { max },
                        [
                            174u8, 44u8, 232u8, 155u8, 228u8, 219u8, 239u8, 75u8, 86u8, 150u8,
                            135u8, 214u8, 58u8, 9u8, 25u8, 133u8, 245u8, 101u8, 85u8, 246u8, 15u8,
                            248u8, 165u8, 87u8, 88u8, 28u8, 10u8, 196u8, 86u8, 89u8, 28u8, 165u8,
                        ],
                    )
                }
                #[doc = "Set the candidacy bond amount."]
                pub fn set_candidacy_bond(
                    &self,
                    bond: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetCandidacyBond> {
                    ::subxt::tx::Payload::new_static(
                        "CollatorSelection",
                        "set_candidacy_bond",
                        types::SetCandidacyBond { bond },
                        [
                            250u8, 4u8, 185u8, 228u8, 101u8, 223u8, 49u8, 44u8, 172u8, 148u8,
                            216u8, 242u8, 192u8, 88u8, 228u8, 59u8, 225u8, 222u8, 171u8, 40u8,
                            23u8, 1u8, 46u8, 183u8, 189u8, 191u8, 156u8, 12u8, 218u8, 116u8, 76u8,
                            59u8,
                        ],
                    )
                }
                #[doc = "Register this account as a collator candidate. The account must (a) already have"]
                #[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
                #[doc = ""]
                #[doc = "This call is not available to `Invulnerable` collators."]
                pub fn register_as_candidate(
                    &self,
                ) -> ::subxt::tx::Payload<types::RegisterAsCandidate> {
                    ::subxt::tx::Payload::new_static(
                        "CollatorSelection",
                        "register_as_candidate",
                        types::RegisterAsCandidate {},
                        [
                            69u8, 222u8, 214u8, 106u8, 105u8, 168u8, 82u8, 239u8, 158u8, 117u8,
                            224u8, 89u8, 228u8, 51u8, 221u8, 244u8, 88u8, 63u8, 72u8, 119u8, 224u8,
                            111u8, 93u8, 39u8, 18u8, 66u8, 72u8, 105u8, 70u8, 66u8, 178u8, 173u8,
                        ],
                    )
                }
                #[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
                #[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
                #[doc = ""]
                #[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
                #[doc = ""]
                #[doc = "This call is not available to `Invulnerable` collators."]
                pub fn leave_intent(&self) -> ::subxt::tx::Payload<types::LeaveIntent> {
                    ::subxt::tx::Payload::new_static(
                        "CollatorSelection",
                        "leave_intent",
                        types::LeaveIntent {},
                        [
                            126u8, 57u8, 10u8, 67u8, 120u8, 229u8, 70u8, 23u8, 154u8, 215u8, 226u8,
                            178u8, 203u8, 152u8, 195u8, 177u8, 157u8, 158u8, 40u8, 17u8, 93u8,
                            225u8, 253u8, 217u8, 48u8, 165u8, 55u8, 79u8, 43u8, 123u8, 193u8,
                            147u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct NewInvulnerables {
                pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for NewInvulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewInvulnerables";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct NewDesiredCandidates {
                pub desired_candidates: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewDesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewDesiredCandidates";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct NewCandidacyBond {
                pub bond_amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for NewCandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewCandidacyBond";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateAdded {
                pub account_id: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for CandidateAdded {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateRemoved {
                pub account_id: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for CandidateRemoved {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The invulnerable, fixed collators."]
                pub fn invulnerables(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::utils::AccountId32,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "Invulnerables",
                        vec![],
                        [
                            109u8, 180u8, 25u8, 41u8, 152u8, 158u8, 186u8, 214u8, 89u8, 222u8,
                            103u8, 14u8, 91u8, 3u8, 65u8, 6u8, 255u8, 62u8, 47u8, 255u8, 132u8,
                            164u8, 217u8, 200u8, 130u8, 29u8, 168u8, 23u8, 81u8, 217u8, 35u8,
                            123u8,
                        ],
                    )
                }
                #[doc = " The (community, limited) collation candidates."]
                pub fn candidates(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_collator_selection::pallet::CandidateInfo<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "Candidates",
                        vec![],
                        [
                            95u8, 142u8, 119u8, 195u8, 123u8, 1u8, 212u8, 104u8, 23u8, 112u8,
                            215u8, 11u8, 254u8, 30u8, 40u8, 19u8, 86u8, 187u8, 3u8, 179u8, 34u8,
                            255u8, 215u8, 181u8, 162u8, 57u8, 23u8, 220u8, 223u8, 55u8, 180u8,
                            88u8,
                        ],
                    )
                }
                #[doc = " Last block authored by collator."]
                pub fn last_authored_block(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "LastAuthoredBlock",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
                            72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
                            126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
                        ],
                    )
                }
                #[doc = " Last block authored by collator."]
                pub fn last_authored_block_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "LastAuthoredBlock",
                        Vec::new(),
                        [
                            176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
                            72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
                            126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
                        ],
                    )
                }
                #[doc = " Desired number of candidates."]
                #[doc = ""]
                #[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
                pub fn desired_candidates(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "DesiredCandidates",
                        vec![],
                        [
                            69u8, 199u8, 130u8, 132u8, 10u8, 127u8, 204u8, 220u8, 59u8, 107u8,
                            96u8, 180u8, 42u8, 235u8, 14u8, 126u8, 231u8, 242u8, 162u8, 126u8,
                            63u8, 223u8, 15u8, 250u8, 22u8, 210u8, 54u8, 34u8, 235u8, 191u8, 250u8,
                            21u8,
                        ],
                    )
                }
                #[doc = " Fixed amount to deposit to become a collator."]
                #[doc = ""]
                #[doc = " When a collator calls `leave_intent` they immediately receive the deposit back."]
                pub fn candidacy_bond(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "CollatorSelection",
                        "CandidacyBond",
                        vec![],
                        [
                            71u8, 134u8, 156u8, 102u8, 201u8, 83u8, 240u8, 251u8, 189u8, 213u8,
                            211u8, 182u8, 126u8, 122u8, 41u8, 174u8, 105u8, 29u8, 216u8, 23u8,
                            255u8, 55u8, 245u8, 187u8, 234u8, 234u8, 178u8, 155u8, 145u8, 49u8,
                            196u8, 214u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the session pallet."]
        pub type Error = runtime_types::pallet_session::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_session::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetKeys {
                    pub keys: runtime_types::parachain_template_runtime::SessionKeys,
                    pub proof: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "set_keys";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PurgeKeys;
                impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "purge_keys";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Sets the session key(s) of the function caller to `keys`."]
                #[doc = "Allows an account to set its session key prior to becoming a validator."]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be signed."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
                #[doc = "  fixed."]
                pub fn set_keys(
                    &self,
                    keys: runtime_types::parachain_template_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "set_keys",
                        types::SetKeys { keys, proof },
                        [
                            10u8, 183u8, 202u8, 82u8, 236u8, 202u8, 212u8, 220u8, 51u8, 217u8,
                            229u8, 169u8, 238u8, 141u8, 129u8, 231u8, 203u8, 176u8, 97u8, 148u8,
                            240u8, 87u8, 177u8, 245u8, 33u8, 109u8, 243u8, 52u8, 46u8, 118u8,
                            164u8, 35u8,
                        ],
                    )
                }
                #[doc = "Removes any session key(s) of the function caller."]
                #[doc = ""]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                #[doc = "usually means being a stash account)."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
                #[doc = "  `T::Keys::key_ids()` which is fixed."]
                pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "purge_keys",
                        types::PurgeKeys {},
                        [
                            215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
                            151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
                            67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
                            209u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current set of validators."]
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]
                pub fn current_index(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
                            135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
                            134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
                            221u8, 230u8,
                        ],
                    )
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]
                pub fn queued_changed(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
                            198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
                            36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
                            153u8,
                        ],
                    )
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]
                pub fn queued_keys(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::parachain_template_runtime::SessionKeys,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            3u8, 214u8, 191u8, 168u8, 90u8, 94u8, 107u8, 111u8, 170u8, 31u8, 78u8,
                            61u8, 240u8, 184u8, 170u8, 104u8, 178u8, 229u8, 159u8, 89u8, 207u8,
                            37u8, 49u8, 209u8, 131u8, 165u8, 14u8, 169u8, 13u8, 68u8, 151u8, 144u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
                            36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
                            183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::parachain_template_runtime::SessionKeys,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            193u8, 216u8, 53u8, 103u8, 143u8, 241u8, 201u8, 54u8, 108u8, 149u8,
                            241u8, 42u8, 3u8, 151u8, 223u8, 246u8, 30u8, 6u8, 239u8, 206u8, 27u8,
                            172u8, 43u8, 226u8, 177u8, 111u8, 203u8, 78u8, 49u8, 34u8, 200u8, 6u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::parachain_template_runtime::SessionKeys,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        Vec::new(),
                        [
                            193u8, 216u8, 53u8, 103u8, 143u8, 241u8, 201u8, 54u8, 108u8, 149u8,
                            241u8, 42u8, 3u8, 151u8, 223u8, 246u8, 30u8, 6u8, 239u8, 206u8, 27u8,
                            172u8, 43u8, 226u8, 177u8, 111u8, 203u8, 78u8, 49u8, 34u8, 200u8, 6u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        Vec::new(),
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura_ext {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Serves as cache for the authorities."]
                #[doc = ""]
                #[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
                #[doc = " but we require the old authorities to verify the seal when validating a PoV. This will always"]
                #[doc = " be updated to the latest AuRa authorities in `on_finalize`."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "AuraExt",
                        "Authorities",
                        vec![],
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod xcmp_queue {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::cumulus_pallet_xcmp_queue::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::cumulus_pallet_xcmp_queue::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ServiceOverweight {
                    pub index: ::core::primitive::u64,
                    pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "service_overweight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SuspendXcmExecution;
                impl ::subxt::blocks::StaticExtrinsic for SuspendXcmExecution {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "suspend_xcm_execution";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ResumeXcmExecution;
                impl ::subxt::blocks::StaticExtrinsic for ResumeXcmExecution {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "resume_xcm_execution";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateSuspendThreshold {
                    pub new: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateSuspendThreshold {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_suspend_threshold";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateDropThreshold {
                    pub new: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateDropThreshold {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_drop_threshold";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateResumeThreshold {
                    pub new: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateResumeThreshold {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_resume_threshold";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateThresholdWeight {
                    pub new: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateThresholdWeight {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_threshold_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateWeightRestrictDecay {
                    pub new: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateWeightRestrictDecay {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_weight_restrict_decay";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateXcmpMaxIndividualWeight {
                    pub new: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateXcmpMaxIndividualWeight {
                    const PALLET: &'static str = "XcmpQueue";
                    const CALL: &'static str = "update_xcmp_max_individual_weight";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Services a single overweight XCM."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                #[doc = "- `index`: The index of the overweight XCM to service"]
                #[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
                #[doc = ""]
                #[doc = "Errors:"]
                #[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
                #[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
                #[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
                #[doc = ""]
                #[doc = "Events:"]
                #[doc = "- `OverweightServiced`: On success."]
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::ServiceOverweight> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "service_overweight",
                        types::ServiceOverweight {
                            index,
                            weight_limit,
                        },
                        [
                            235u8, 203u8, 220u8, 162u8, 173u8, 117u8, 224u8, 194u8, 176u8, 125u8,
                            50u8, 74u8, 180u8, 37u8, 126u8, 227u8, 138u8, 213u8, 227u8, 35u8,
                            247u8, 18u8, 160u8, 231u8, 97u8, 149u8, 144u8, 49u8, 34u8, 146u8, 32u8,
                            7u8,
                        ],
                    )
                }
                #[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                pub fn suspend_xcm_execution(
                    &self,
                ) -> ::subxt::tx::Payload<types::SuspendXcmExecution> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "suspend_xcm_execution",
                        types::SuspendXcmExecution {},
                        [
                            54u8, 120u8, 33u8, 251u8, 74u8, 56u8, 29u8, 76u8, 104u8, 218u8, 115u8,
                            198u8, 148u8, 237u8, 9u8, 191u8, 241u8, 48u8, 33u8, 24u8, 60u8, 144u8,
                            22u8, 78u8, 58u8, 50u8, 26u8, 188u8, 231u8, 42u8, 201u8, 76u8,
                        ],
                    )
                }
                #[doc = "Resumes all XCM executions for the XCMP queue."]
                #[doc = ""]
                #[doc = "Note that this function doesn't change the status of the in/out bound channels."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                pub fn resume_xcm_execution(
                    &self,
                ) -> ::subxt::tx::Payload<types::ResumeXcmExecution> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "resume_xcm_execution",
                        types::ResumeXcmExecution {},
                        [
                            173u8, 231u8, 78u8, 253u8, 108u8, 234u8, 199u8, 124u8, 184u8, 154u8,
                            95u8, 194u8, 13u8, 77u8, 175u8, 7u8, 7u8, 112u8, 161u8, 72u8, 133u8,
                            71u8, 63u8, 218u8, 97u8, 226u8, 133u8, 6u8, 93u8, 177u8, 247u8, 109u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
                #[doc = "suspend their sending."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
                pub fn update_suspend_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::UpdateSuspendThreshold> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_suspend_threshold",
                        types::UpdateSuspendThreshold { new },
                        [
                            64u8, 91u8, 172u8, 51u8, 220u8, 174u8, 54u8, 47u8, 57u8, 89u8, 75u8,
                            39u8, 126u8, 198u8, 143u8, 35u8, 70u8, 125u8, 167u8, 14u8, 17u8, 18u8,
                            146u8, 222u8, 100u8, 92u8, 81u8, 239u8, 173u8, 43u8, 42u8, 174u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
                #[doc = "messages from the channel."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
                pub fn update_drop_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::UpdateDropThreshold> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_drop_threshold",
                        types::UpdateDropThreshold { new },
                        [
                            123u8, 54u8, 12u8, 180u8, 165u8, 198u8, 141u8, 200u8, 149u8, 168u8,
                            186u8, 237u8, 162u8, 91u8, 89u8, 242u8, 229u8, 16u8, 32u8, 254u8, 59u8,
                            168u8, 31u8, 134u8, 217u8, 251u8, 0u8, 102u8, 113u8, 194u8, 175u8, 9u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
                #[doc = "message sending may recommence after it has been suspended."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
                pub fn update_resume_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::UpdateResumeThreshold> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_resume_threshold",
                        types::UpdateResumeThreshold { new },
                        [
                            172u8, 136u8, 11u8, 106u8, 42u8, 157u8, 167u8, 183u8, 87u8, 62u8,
                            182u8, 17u8, 184u8, 59u8, 215u8, 230u8, 18u8, 243u8, 212u8, 34u8, 54u8,
                            188u8, 95u8, 119u8, 173u8, 20u8, 91u8, 206u8, 212u8, 57u8, 136u8, 77u8,
                        ],
                    )
                }
                #[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
                pub fn update_threshold_weight(
                    &self,
                    new: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::UpdateThresholdWeight> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_threshold_weight",
                        types::UpdateThresholdWeight { new },
                        [
                            79u8, 1u8, 102u8, 119u8, 93u8, 104u8, 197u8, 189u8, 248u8, 215u8, 30u8,
                            227u8, 83u8, 26u8, 149u8, 99u8, 174u8, 191u8, 97u8, 82u8, 168u8, 128u8,
                            130u8, 136u8, 185u8, 54u8, 104u8, 186u8, 231u8, 11u8, 66u8, 184u8,
                        ],
                    )
                }
                #[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
                #[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
                pub fn update_weight_restrict_decay(
                    &self,
                    new: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::UpdateWeightRestrictDecay> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_weight_restrict_decay",
                        types::UpdateWeightRestrictDecay { new },
                        [
                            37u8, 210u8, 52u8, 253u8, 67u8, 66u8, 63u8, 238u8, 117u8, 80u8, 77u8,
                            102u8, 166u8, 103u8, 173u8, 135u8, 54u8, 139u8, 100u8, 225u8, 115u8,
                            214u8, 160u8, 228u8, 195u8, 221u8, 160u8, 62u8, 192u8, 105u8, 188u8,
                            139u8,
                        ],
                    )
                }
                #[doc = "Overwrite the maximum amount of weight any individual message may consume."]
                #[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
                pub fn update_xcmp_max_individual_weight(
                    &self,
                    new: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::UpdateXcmpMaxIndividualWeight> {
                    ::subxt::tx::Payload::new_static(
                        "XcmpQueue",
                        "update_xcmp_max_individual_weight",
                        types::UpdateXcmpMaxIndividualWeight { new },
                        [
                            185u8, 199u8, 32u8, 102u8, 179u8, 139u8, 101u8, 14u8, 48u8, 173u8,
                            123u8, 158u8, 161u8, 153u8, 81u8, 109u8, 196u8, 217u8, 235u8, 150u8,
                            176u8, 55u8, 168u8, 31u8, 34u8, 251u8, 128u8, 53u8, 160u8, 220u8,
                            140u8, 174u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some XCM was executed ok."]
            pub struct Success {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for Success {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Success";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some XCM failed."]
            pub struct Fail {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                pub error: runtime_types::xcm::v3::traits::Error,
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for Fail {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Fail";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Bad XCM version used."]
            pub struct BadVersion {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for BadVersion {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Bad XCM format used."]
            pub struct BadFormat {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for BadFormat {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An HRMP message was sent to a sibling parachain."]
            pub struct XcmpMessageSent {
                pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for XcmpMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "XcmpMessageSent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An XCM exceeded the individual message weight budget."]
            pub struct OverweightEnqueued {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub sent_at: ::core::primitive::u32,
                pub index: ::core::primitive::u64,
                pub required: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for OverweightEnqueued {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
            pub struct OverweightServiced {
                pub index: ::core::primitive::u64,
                pub used: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for OverweightServiced {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Status of the inbound XCMP channels."]
                pub fn inbound_xcmp_status(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "InboundXcmpStatus",
                        vec![],
                        [
                            216u8, 138u8, 138u8, 71u8, 210u8, 155u8, 255u8, 91u8, 44u8, 147u8,
                            80u8, 187u8, 203u8, 88u8, 34u8, 54u8, 80u8, 232u8, 249u8, 20u8, 169u8,
                            138u8, 123u8, 139u8, 182u8, 184u8, 0u8, 205u8, 101u8, 9u8, 194u8,
                            122u8,
                        ],
                    )
                }
                #[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
                pub fn inbound_xcmp_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "InboundXcmpMessages",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            60u8, 227u8, 118u8, 144u8, 41u8, 170u8, 15u8, 80u8, 148u8, 229u8,
                            213u8, 6u8, 213u8, 186u8, 20u8, 199u8, 229u8, 159u8, 17u8, 39u8, 116u8,
                            85u8, 34u8, 82u8, 109u8, 100u8, 174u8, 85u8, 245u8, 247u8, 84u8, 116u8,
                        ],
                    )
                }
                #[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
                pub fn inbound_xcmp_messages_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "InboundXcmpMessages",
                        Vec::new(),
                        [
                            60u8, 227u8, 118u8, 144u8, 41u8, 170u8, 15u8, 80u8, 148u8, 229u8,
                            213u8, 6u8, 213u8, 186u8, 20u8, 199u8, 229u8, 159u8, 17u8, 39u8, 116u8,
                            85u8, 34u8, 82u8, 109u8, 100u8, 174u8, 85u8, 245u8, 247u8, 84u8, 116u8,
                        ],
                    )
                }
                #[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
                #[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
                #[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
                #[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
                #[doc = " case of the need to send a high-priority signal message this block."]
                #[doc = " The bool is true if there is a signal message waiting to be sent."]
                pub fn outbound_xcmp_status(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "OutboundXcmpStatus",
                        vec![],
                        [
                            181u8, 5u8, 216u8, 176u8, 154u8, 233u8, 116u8, 14u8, 151u8, 1u8, 114u8,
                            16u8, 42u8, 20u8, 63u8, 233u8, 79u8, 122u8, 87u8, 255u8, 75u8, 149u8,
                            176u8, 106u8, 23u8, 101u8, 228u8, 120u8, 217u8, 167u8, 127u8, 117u8,
                        ],
                    )
                }
                #[doc = " The messages outbound in a given XCMP channel."]
                pub fn outbound_xcmp_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u16>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "OutboundXcmpMessages",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
                            159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
                            144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
                        ],
                    )
                }
                #[doc = " The messages outbound in a given XCMP channel."]
                pub fn outbound_xcmp_messages_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "OutboundXcmpMessages",
                        Vec::new(),
                        [
                            156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
                            159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
                            144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
                        ],
                    )
                }
                #[doc = " Any signal messages waiting to be sent."]
                pub fn signal_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "SignalMessages",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
                            229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
                            249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
                            180u8, 24u8,
                        ],
                    )
                }
                #[doc = " Any signal messages waiting to be sent."]
                pub fn signal_messages_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "SignalMessages",
                        Vec::new(),
                        [
                            182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
                            229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
                            249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
                            180u8, 24u8,
                        ],
                    )
                }
                #[doc = " The configuration which controls the dynamics of the outbound queue."]
                pub fn queue_config(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "QueueConfig",
                        vec![],
                        [
                            112u8, 136u8, 198u8, 133u8, 5u8, 66u8, 33u8, 29u8, 99u8, 72u8, 70u8,
                            56u8, 182u8, 57u8, 48u8, 10u8, 135u8, 63u8, 103u8, 13u8, 143u8, 121u8,
                            12u8, 126u8, 207u8, 56u8, 244u8, 63u8, 126u8, 51u8, 100u8, 69u8,
                        ],
                    )
                }
                #[doc = " The messages that exceeded max individual message weight budget."]
                #[doc = ""]
                #[doc = " These message stay in this storage map until they are manually dispatched via"]
                #[doc = " `service_overweight`."]
                pub fn overweight(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "Overweight",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            4u8, 180u8, 123u8, 50u8, 174u8, 195u8, 68u8, 214u8, 187u8, 92u8, 131u8,
                            234u8, 166u8, 124u8, 19u8, 202u8, 0u8, 249u8, 246u8, 239u8, 199u8,
                            27u8, 129u8, 252u8, 22u8, 92u8, 206u8, 159u8, 136u8, 222u8, 238u8,
                            81u8,
                        ],
                    )
                }
                #[doc = " The messages that exceeded max individual message weight budget."]
                #[doc = ""]
                #[doc = " These message stay in this storage map until they are manually dispatched via"]
                #[doc = " `service_overweight`."]
                pub fn overweight_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "Overweight",
                        Vec::new(),
                        [
                            4u8, 180u8, 123u8, 50u8, 174u8, 195u8, 68u8, 214u8, 187u8, 92u8, 131u8,
                            234u8, 166u8, 124u8, 19u8, 202u8, 0u8, 249u8, 246u8, 239u8, 199u8,
                            27u8, 129u8, 252u8, 22u8, 92u8, 206u8, 159u8, 136u8, 222u8, 238u8,
                            81u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]
                pub fn counter_for_overweight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "CounterForOverweight",
                        vec![],
                        [
                            44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
                            28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
                            143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
                            247u8, 0u8,
                        ],
                    )
                }
                #[doc = " The number of overweight messages ever recorded in `Overweight`. Also doubles as the next"]
                #[doc = " available free overweight index."]
                pub fn overweight_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "OverweightCount",
                        vec![],
                        [
                            28u8, 72u8, 218u8, 167u8, 253u8, 30u8, 10u8, 51u8, 49u8, 101u8, 86u8,
                            26u8, 146u8, 2u8, 153u8, 232u8, 129u8, 38u8, 111u8, 105u8, 246u8, 84u8,
                            192u8, 157u8, 193u8, 57u8, 222u8, 122u8, 38u8, 160u8, 56u8, 39u8,
                        ],
                    )
                }
                #[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
                pub fn queue_suspended(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "XcmpQueue",
                        "QueueSuspended",
                        vec![],
                        [
                            165u8, 66u8, 105u8, 244u8, 113u8, 43u8, 177u8, 252u8, 212u8, 243u8,
                            143u8, 184u8, 87u8, 51u8, 163u8, 104u8, 29u8, 84u8, 119u8, 74u8, 233u8,
                            129u8, 203u8, 105u8, 2u8, 101u8, 19u8, 170u8, 69u8, 253u8, 80u8, 132u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod polkadot_xcm {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_xcm::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_xcm::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Send {
                    pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Send {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "send";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TeleportAssets {
                    pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    pub fee_asset_item: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for TeleportAssets {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "teleport_assets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveTransferAssets {
                    pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    pub fee_asset_item: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReserveTransferAssets {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "reserve_transfer_assets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Execute {
                    pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for Execute {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "execute";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceXcmVersion {
                    pub location:
                        ::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
                    pub xcm_version: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceXcmVersion {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "force_xcm_version";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceDefaultXcmVersion {
                    pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceDefaultXcmVersion {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "force_default_xcm_version";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSubscribeVersionNotify {
                    pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSubscribeVersionNotify {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "force_subscribe_version_notify";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnsubscribeVersionNotify {
                    pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnsubscribeVersionNotify {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "force_unsubscribe_version_notify";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct LimitedReserveTransferAssets {
                    pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    pub fee_asset_item: ::core::primitive::u32,
                    pub weight_limit: runtime_types::xcm::v3::WeightLimit,
                }
                impl ::subxt::blocks::StaticExtrinsic for LimitedReserveTransferAssets {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "limited_reserve_transfer_assets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct LimitedTeleportAssets {
                    pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    pub fee_asset_item: ::core::primitive::u32,
                    pub weight_limit: runtime_types::xcm::v3::WeightLimit,
                }
                impl ::subxt::blocks::StaticExtrinsic for LimitedTeleportAssets {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "limited_teleport_assets";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSuspension {
                    pub suspended: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSuspension {
                    const PALLET: &'static str = "PolkadotXcm";
                    const CALL: &'static str = "force_suspension";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::tx::Payload<types::Send> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "send",
                        types::Send {
                            dest: ::std::boxed::Box::new(dest),
                            message: ::std::boxed::Box::new(message),
                        },
                        [
                            147u8, 255u8, 86u8, 82u8, 17u8, 159u8, 225u8, 145u8, 220u8, 89u8, 71u8,
                            23u8, 193u8, 249u8, 12u8, 70u8, 19u8, 140u8, 232u8, 97u8, 12u8, 220u8,
                            113u8, 65u8, 4u8, 255u8, 138u8, 10u8, 231u8, 122u8, 67u8, 105u8,
                        ],
                    )
                }
                #[doc = "Teleport some assets from the local chain to some destination chain."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                #[doc = "with all fees taken as needed from the asset."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                #[doc = "  `dest` side. May not be empty."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::TeleportAssets> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "teleport_assets",
                        types::TeleportAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                        },
                        [
                            56u8, 144u8, 237u8, 60u8, 157u8, 5u8, 7u8, 129u8, 41u8, 149u8, 160u8,
                            100u8, 233u8, 102u8, 181u8, 140u8, 115u8, 213u8, 29u8, 132u8, 16u8,
                            30u8, 23u8, 82u8, 140u8, 134u8, 37u8, 87u8, 3u8, 99u8, 172u8, 42u8,
                        ],
                    )
                }
                #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                #[doc = "chain and forward a notification XCM."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                #[doc = "with all fees taken as needed from the asset."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                #[doc = "  `dest` side."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::ReserveTransferAssets> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "reserve_transfer_assets",
                        types::ReserveTransferAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                        },
                        [
                            21u8, 167u8, 44u8, 22u8, 210u8, 73u8, 148u8, 7u8, 91u8, 108u8, 148u8,
                            205u8, 170u8, 243u8, 142u8, 224u8, 205u8, 119u8, 252u8, 22u8, 203u8,
                            32u8, 73u8, 200u8, 178u8, 14u8, 167u8, 147u8, 166u8, 55u8, 14u8, 231u8,
                        ],
                    )
                }
                #[doc = "Execute an XCM message from a local, signed, origin."]
                #[doc = ""]
                #[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
                #[doc = "partially."]
                #[doc = ""]
                #[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
                #[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
                #[doc = "attempt will be made."]
                #[doc = ""]
                #[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
                #[doc = "to completion; only that *some* of it was executed."]
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm2,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::Execute> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "execute",
                        types::Execute {
                            message: ::std::boxed::Box::new(message),
                            max_weight,
                        },
                        [
                            15u8, 97u8, 86u8, 111u8, 105u8, 116u8, 109u8, 206u8, 70u8, 8u8, 57u8,
                            232u8, 133u8, 132u8, 30u8, 219u8, 34u8, 69u8, 0u8, 213u8, 98u8, 241u8,
                            186u8, 93u8, 216u8, 39u8, 73u8, 24u8, 193u8, 87u8, 92u8, 31u8,
                        ],
                    )
                }
                #[doc = "Extoll that a particular destination can be communicated with through a particular"]
                #[doc = "version of XCM."]
                #[doc = ""]
                #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                #[doc = "- `location`: The destination that is being described."]
                #[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::ForceXcmVersion> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "force_xcm_version",
                        types::ForceXcmVersion {
                            location: ::std::boxed::Box::new(location),
                            xcm_version,
                        },
                        [
                            84u8, 212u8, 64u8, 161u8, 17u8, 129u8, 213u8, 129u8, 79u8, 86u8, 117u8,
                            246u8, 93u8, 1u8, 161u8, 23u8, 35u8, 171u8, 163u8, 200u8, 69u8, 157u8,
                            71u8, 8u8, 225u8, 149u8, 254u8, 124u8, 38u8, 250u8, 164u8, 218u8,
                        ],
                    )
                }
                #[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
                #[doc = "version a destination can accept is unknown)."]
                #[doc = ""]
                #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                #[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::tx::Payload<types::ForceDefaultXcmVersion> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "force_default_xcm_version",
                        types::ForceDefaultXcmVersion { maybe_xcm_version },
                        [
                            43u8, 114u8, 102u8, 104u8, 209u8, 234u8, 108u8, 173u8, 109u8, 188u8,
                            94u8, 214u8, 136u8, 43u8, 153u8, 75u8, 161u8, 192u8, 76u8, 12u8, 221u8,
                            237u8, 158u8, 247u8, 41u8, 193u8, 35u8, 174u8, 183u8, 207u8, 79u8,
                            213u8,
                        ],
                    )
                }
                #[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
                #[doc = ""]
                #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                #[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::tx::Payload<types::ForceSubscribeVersionNotify> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "force_subscribe_version_notify",
                        types::ForceSubscribeVersionNotify {
                            location: ::std::boxed::Box::new(location),
                        },
                        [
                            112u8, 254u8, 138u8, 12u8, 203u8, 176u8, 251u8, 167u8, 223u8, 0u8,
                            71u8, 148u8, 19u8, 179u8, 47u8, 96u8, 188u8, 189u8, 14u8, 172u8, 1u8,
                            1u8, 192u8, 107u8, 137u8, 158u8, 22u8, 9u8, 138u8, 241u8, 32u8, 47u8,
                        ],
                    )
                }
                #[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
                #[doc = "version changes."]
                #[doc = ""]
                #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                #[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
                #[doc = "  notifications which we no longer desire."]
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::tx::Payload<types::ForceUnsubscribeVersionNotify> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "force_unsubscribe_version_notify",
                        types::ForceUnsubscribeVersionNotify {
                            location: ::std::boxed::Box::new(location),
                        },
                        [
                            205u8, 143u8, 230u8, 143u8, 166u8, 184u8, 53u8, 252u8, 118u8, 184u8,
                            209u8, 227u8, 225u8, 184u8, 254u8, 244u8, 101u8, 56u8, 27u8, 128u8,
                            40u8, 159u8, 178u8, 62u8, 63u8, 164u8, 59u8, 236u8, 1u8, 168u8, 202u8,
                            42u8,
                        ],
                    )
                }
                #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                #[doc = "chain and forward a notification XCM."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                #[doc = "at risk."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                #[doc = "  `dest` side."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                pub fn limited_reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                ) -> ::subxt::tx::Payload<types::LimitedReserveTransferAssets> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "limited_reserve_transfer_assets",
                        types::LimitedReserveTransferAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                            weight_limit,
                        },
                        [
                            10u8, 139u8, 165u8, 239u8, 92u8, 178u8, 169u8, 62u8, 166u8, 236u8,
                            50u8, 12u8, 196u8, 3u8, 233u8, 209u8, 3u8, 159u8, 184u8, 234u8, 171u8,
                            46u8, 145u8, 134u8, 241u8, 155u8, 221u8, 173u8, 166u8, 94u8, 147u8,
                            88u8,
                        ],
                    )
                }
                #[doc = "Teleport some assets from the local chain to some destination chain."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                #[doc = "at risk."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                #[doc = "  `dest` side. May not be empty."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                pub fn limited_teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                ) -> ::subxt::tx::Payload<types::LimitedTeleportAssets> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "limited_teleport_assets",
                        types::LimitedTeleportAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                            weight_limit,
                        },
                        [
                            156u8, 205u8, 105u8, 18u8, 120u8, 130u8, 144u8, 67u8, 152u8, 188u8,
                            109u8, 121u8, 4u8, 240u8, 123u8, 112u8, 72u8, 153u8, 2u8, 111u8, 183u8,
                            170u8, 199u8, 82u8, 33u8, 117u8, 43u8, 133u8, 208u8, 44u8, 118u8,
                            107u8,
                        ],
                    )
                }
                #[doc = "Set or unset the global suspension state of the XCM executor."]
                #[doc = ""]
                #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                #[doc = "- `suspended`: `true` to suspend, `false` to resume."]
                pub fn force_suspension(
                    &self,
                    suspended: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::ForceSuspension> {
                    ::subxt::tx::Payload::new_static(
                        "PolkadotXcm",
                        "force_suspension",
                        types::ForceSuspension { suspended },
                        [
                            78u8, 125u8, 93u8, 55u8, 129u8, 44u8, 36u8, 227u8, 75u8, 46u8, 68u8,
                            202u8, 81u8, 127u8, 111u8, 92u8, 149u8, 38u8, 225u8, 185u8, 183u8,
                            154u8, 89u8, 159u8, 79u8, 10u8, 229u8, 1u8, 226u8, 243u8, 65u8, 238u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Execution of an XCM message was attempted."]
            #[doc = ""]
            #[doc = "\\[ outcome \\]"]
            pub struct Attempted(pub runtime_types::xcm::v3::traits::Outcome);
            impl ::subxt::events::StaticEvent for Attempted {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A XCM message was sent."]
            #[doc = ""]
            #[doc = "\\[ origin, destination, message \\]"]
            pub struct Sent(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::Xcm,
            );
            impl ::subxt::events::StaticEvent for Sent {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Sent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response received which does not match a registered query. This may be because a"]
            #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
            #[doc = "because the query timed out."]
            #[doc = ""]
            #[doc = "\\[ origin location, id \\]"]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for UnexpectedResponse {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
            #[doc = "no registered notification call."]
            #[doc = ""]
            #[doc = "\\[ id, response \\]"]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v3::Response,
            );
            impl ::subxt::events::StaticEvent for ResponseReady {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response has been received and query is removed. The registered notification has"]
            #[doc = "been dispatched and executed successfully."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for Notified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Notified";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response has been received and query is removed. The registered notification could"]
            #[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
            #[doc = "originally budgeted by this runtime for the query result."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub runtime_types::sp_weights::weight_v2::Weight,
                pub runtime_types::sp_weights::weight_v2::Weight,
            );
            impl ::subxt::events::StaticEvent for NotifyOverweight {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response has been received and query is removed. There was a general error with"]
            #[doc = "dispatching the notification call."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for NotifyDispatchError {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
            #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
            #[doc = "is not `(origin, QueryId, Response)`."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Expected query response has been received but the origin location of the response does"]
            #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
            #[doc = "be received and acted upon."]
            #[doc = ""]
            #[doc = "\\[ origin location, id, expected location \\]"]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
            );
            impl ::subxt::events::StaticEvent for InvalidResponder {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Expected query response has been received but the expected origin location placed in"]
            #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
            #[doc = ""]
            #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
            #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
            #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
            #[doc = "needed."]
            #[doc = ""]
            #[doc = "\\[ origin location, id \\]"]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for InvalidResponderVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Received query response has been read and removed."]
            #[doc = ""]
            #[doc = "\\[ id \\]"]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::events::StaticEvent for ResponseTaken {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some assets have been placed in an asset trap."]
            #[doc = ""]
            #[doc = "\\[ hash, origin, assets \\]"]
            pub struct AssetsTrapped(
                pub ::subxt::utils::H256,
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::events::StaticEvent for AssetsTrapped {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An XCM version change notification message has been attempted to be sent."]
            #[doc = ""]
            #[doc = "The cost of sending it (borne by the chain) is included."]
            #[doc = ""]
            #[doc = "\\[ destination, result, cost \\]"]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u32,
                pub runtime_types::xcm::v3::multiasset::MultiAssets,
            );
            impl ::subxt::events::StaticEvent for VersionChangeNotified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The supported version of a location has been changed. This might be through an"]
            #[doc = "automatic notification or a manual intervention."]
            #[doc = ""]
            #[doc = "\\[ location, XCM version \\]"]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for SupportedVersionChanged {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A given location which had a version change subscription was dropped owing to an error"]
            #[doc = "sending the notification to it."]
            #[doc = ""]
            #[doc = "\\[ location, query ID, error \\]"]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v3::traits::Error,
            );
            impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A given location which had a version change subscription was dropped owing to an error"]
            #[doc = "migrating the location to our new XCM format."]
            #[doc = ""]
            #[doc = "\\[ location, query ID \\]"]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Expected query response has been received but the expected querier location placed in"]
            #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
            #[doc = ""]
            #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
            #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
            #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
            #[doc = "needed."]
            #[doc = ""]
            #[doc = "\\[ origin location, id \\]"]
            pub struct InvalidQuerierVersion(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for InvalidQuerierVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidQuerierVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Expected query response has been received but the querier location of the response does"]
            #[doc = "not match the expected. The query remains registered for a later, valid, response to"]
            #[doc = "be received and acted upon."]
            #[doc = ""]
            #[doc = "\\[ origin location, id, expected querier, maybe actual querier \\]"]
            pub struct InvalidQuerier(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
            );
            impl ::subxt::events::StaticEvent for InvalidQuerier {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidQuerier";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A remote has requested XCM version change notification from us and we have honored it."]
            #[doc = "A version information message is sent to them and its cost is included."]
            #[doc = ""]
            #[doc = "\\[ destination location, cost \\]"]
            pub struct VersionNotifyStarted(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::multiasset::MultiAssets,
            );
            impl ::subxt::events::StaticEvent for VersionNotifyStarted {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionNotifyStarted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "We have requested that a remote chain sends us XCM version change notifications."]
            #[doc = ""]
            #[doc = "\\[ destination location, cost \\]"]
            pub struct VersionNotifyRequested(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::multiasset::MultiAssets,
            );
            impl ::subxt::events::StaticEvent for VersionNotifyRequested {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionNotifyRequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "We have requested that a remote chain stops sending us XCM version change notifications."]
            #[doc = ""]
            #[doc = "\\[ destination location, cost \\]"]
            pub struct VersionNotifyUnrequested(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::multiasset::MultiAssets,
            );
            impl ::subxt::events::StaticEvent for VersionNotifyUnrequested {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionNotifyUnrequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
            #[doc = ""]
            #[doc = "\\[ paying location, fees \\]"]
            pub struct FeesPaid(
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::v3::multiasset::MultiAssets,
            );
            impl ::subxt::events::StaticEvent for FeesPaid {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "FeesPaid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some assets have been claimed from an asset trap"]
            #[doc = ""]
            #[doc = "\\[ hash, origin, assets \\]"]
            pub struct AssetsClaimed(
                pub ::subxt::utils::H256,
                pub runtime_types::xcm::v3::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::events::StaticEvent for AssetsClaimed {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "AssetsClaimed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The latest available query index."]
                pub fn query_counter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "QueryCounter",
                        vec![],
                        [
                            216u8, 73u8, 160u8, 232u8, 60u8, 245u8, 218u8, 219u8, 152u8, 68u8,
                            146u8, 219u8, 255u8, 7u8, 86u8, 112u8, 83u8, 49u8, 94u8, 173u8, 64u8,
                            203u8, 147u8, 226u8, 236u8, 39u8, 129u8, 106u8, 209u8, 113u8, 150u8,
                            50u8,
                        ],
                    )
                }
                #[doc = " The ongoing queries."]
                pub fn queries(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "Queries",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            119u8, 5u8, 12u8, 91u8, 117u8, 240u8, 52u8, 192u8, 135u8, 139u8, 220u8,
                            78u8, 207u8, 199u8, 71u8, 163u8, 100u8, 17u8, 6u8, 65u8, 200u8, 245u8,
                            191u8, 82u8, 232u8, 128u8, 126u8, 70u8, 39u8, 63u8, 148u8, 219u8,
                        ],
                    )
                }
                #[doc = " The ongoing queries."]
                pub fn queries_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "Queries",
                        Vec::new(),
                        [
                            119u8, 5u8, 12u8, 91u8, 117u8, 240u8, 52u8, 192u8, 135u8, 139u8, 220u8,
                            78u8, 207u8, 199u8, 71u8, 163u8, 100u8, 17u8, 6u8, 65u8, 200u8, 245u8,
                            191u8, 82u8, 232u8, 128u8, 126u8, 70u8, 39u8, 63u8, 148u8, 219u8,
                        ],
                    )
                }
                #[doc = " The existing asset traps."]
                #[doc = ""]
                #[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
                #[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
                pub fn asset_traps(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "AssetTraps",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
                            50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
                            126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
                        ],
                    )
                }
                #[doc = " The existing asset traps."]
                #[doc = ""]
                #[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
                #[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
                pub fn asset_traps_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "AssetTraps",
                        Vec::new(),
                        [
                            148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
                            50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
                            126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
                        ],
                    )
                }
                #[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
                #[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
                pub fn safe_xcm_version(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "SafeXcmVersion",
                        vec![],
                        [
                            187u8, 8u8, 74u8, 126u8, 80u8, 215u8, 177u8, 60u8, 223u8, 123u8, 196u8,
                            155u8, 166u8, 66u8, 25u8, 164u8, 191u8, 66u8, 116u8, 131u8, 116u8,
                            188u8, 224u8, 122u8, 75u8, 195u8, 246u8, 188u8, 83u8, 134u8, 49u8,
                            143u8,
                        ],
                    )
                }
                #[doc = " The Latest versions that we know various locations support."]
                pub fn supported_version(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "SupportedVersion",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            144u8, 22u8, 91u8, 30u8, 139u8, 164u8, 95u8, 149u8, 97u8, 247u8, 12u8,
                            212u8, 96u8, 16u8, 134u8, 236u8, 74u8, 57u8, 244u8, 169u8, 68u8, 63u8,
                            111u8, 86u8, 65u8, 229u8, 104u8, 51u8, 44u8, 100u8, 47u8, 191u8,
                        ],
                    )
                }
                #[doc = " The Latest versions that we know various locations support."]
                pub fn supported_version_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "SupportedVersion",
                        Vec::new(),
                        [
                            144u8, 22u8, 91u8, 30u8, 139u8, 164u8, 95u8, 149u8, 97u8, 247u8, 12u8,
                            212u8, 96u8, 16u8, 134u8, 236u8, 74u8, 57u8, 244u8, 169u8, 68u8, 63u8,
                            111u8, 86u8, 65u8, 229u8, 104u8, 51u8, 44u8, 100u8, 47u8, 191u8,
                        ],
                    )
                }
                #[doc = " All locations that we have requested version notifications from."]
                pub fn version_notifiers(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "VersionNotifiers",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            49u8, 190u8, 73u8, 67u8, 91u8, 69u8, 121u8, 206u8, 25u8, 82u8, 29u8,
                            170u8, 157u8, 201u8, 168u8, 93u8, 181u8, 55u8, 226u8, 142u8, 136u8,
                            46u8, 117u8, 208u8, 130u8, 90u8, 129u8, 39u8, 151u8, 92u8, 118u8, 75u8,
                        ],
                    )
                }
                #[doc = " All locations that we have requested version notifications from."]
                pub fn version_notifiers_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "VersionNotifiers",
                        Vec::new(),
                        [
                            49u8, 190u8, 73u8, 67u8, 91u8, 69u8, 121u8, 206u8, 25u8, 82u8, 29u8,
                            170u8, 157u8, 201u8, 168u8, 93u8, 181u8, 55u8, 226u8, 142u8, 136u8,
                            46u8, 117u8, 208u8, 130u8, 90u8, 129u8, 39u8, 151u8, 92u8, 118u8, 75u8,
                        ],
                    )
                }
                #[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
                #[doc = " of our versions we informed them of."]
                pub fn version_notify_targets(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u64,
                        runtime_types::sp_weights::weight_v2::Weight,
                        ::core::primitive::u32,
                    ),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "VersionNotifyTargets",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            1u8, 195u8, 40u8, 83u8, 216u8, 175u8, 241u8, 95u8, 42u8, 7u8, 85u8,
                            253u8, 223u8, 241u8, 195u8, 41u8, 41u8, 21u8, 17u8, 171u8, 216u8,
                            150u8, 39u8, 165u8, 215u8, 194u8, 201u8, 225u8, 179u8, 12u8, 52u8,
                            173u8,
                        ],
                    )
                }
                #[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
                #[doc = " of our versions we informed them of."]
                pub fn version_notify_targets_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u64,
                        runtime_types::sp_weights::weight_v2::Weight,
                        ::core::primitive::u32,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "VersionNotifyTargets",
                        Vec::new(),
                        [
                            1u8, 195u8, 40u8, 83u8, 216u8, 175u8, 241u8, 95u8, 42u8, 7u8, 85u8,
                            253u8, 223u8, 241u8, 195u8, 41u8, 41u8, 21u8, 17u8, 171u8, 216u8,
                            150u8, 39u8, 165u8, 215u8, 194u8, 201u8, 225u8, 179u8, 12u8, 52u8,
                            173u8,
                        ],
                    )
                }
                #[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
                #[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
                #[doc = " which is used as a prioritization."]
                pub fn version_discovery_queue(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "VersionDiscoveryQueue",
                        vec![],
                        [
                            110u8, 87u8, 102u8, 193u8, 125u8, 129u8, 0u8, 221u8, 218u8, 229u8,
                            101u8, 94u8, 74u8, 229u8, 246u8, 180u8, 113u8, 11u8, 15u8, 159u8, 98u8,
                            90u8, 30u8, 112u8, 164u8, 236u8, 151u8, 220u8, 19u8, 83u8, 67u8, 248u8,
                        ],
                    )
                }
                #[doc = " The current migration's stage, if any."]
                pub fn current_migration(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_xcm::pallet::VersionMigrationStage,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "CurrentMigration",
                        vec![],
                        [
                            74u8, 138u8, 181u8, 162u8, 59u8, 251u8, 37u8, 28u8, 232u8, 51u8, 30u8,
                            152u8, 252u8, 133u8, 95u8, 195u8, 47u8, 127u8, 21u8, 44u8, 62u8, 143u8,
                            170u8, 234u8, 160u8, 37u8, 131u8, 179u8, 57u8, 241u8, 140u8, 124u8,
                        ],
                    )
                }
                #[doc = " Fungible assets which we know are locked on a remote chain."]
                pub fn remote_locked_fungibles(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _2: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedAssetId>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "RemoteLockedFungibles",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
                        ],
                        [
                            74u8, 249u8, 83u8, 245u8, 44u8, 230u8, 152u8, 82u8, 4u8, 163u8, 230u8,
                            121u8, 87u8, 143u8, 184u8, 12u8, 117u8, 112u8, 131u8, 160u8, 232u8,
                            62u8, 175u8, 15u8, 81u8, 198u8, 182u8, 255u8, 37u8, 81u8, 6u8, 57u8,
                        ],
                    )
                }
                #[doc = " Fungible assets which we know are locked on a remote chain."]
                pub fn remote_locked_fungibles_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "RemoteLockedFungibles",
                        Vec::new(),
                        [
                            74u8, 249u8, 83u8, 245u8, 44u8, 230u8, 152u8, 82u8, 4u8, 163u8, 230u8,
                            121u8, 87u8, 143u8, 184u8, 12u8, 117u8, 112u8, 131u8, 160u8, 232u8,
                            62u8, 175u8, 15u8, 81u8, 198u8, 182u8, 255u8, 37u8, 81u8, 6u8, 57u8,
                        ],
                    )
                }
                #[doc = " Fungible assets which we know are locked on this chain."]
                pub fn locked_fungibles(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::core::primitive::u128,
                        runtime_types::xcm::VersionedMultiLocation,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "LockedFungibles",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            110u8, 220u8, 127u8, 176u8, 219u8, 23u8, 132u8, 36u8, 224u8, 187u8,
                            25u8, 103u8, 126u8, 99u8, 34u8, 105u8, 57u8, 182u8, 162u8, 69u8, 24u8,
                            67u8, 221u8, 103u8, 79u8, 139u8, 187u8, 162u8, 113u8, 109u8, 163u8,
                            35u8,
                        ],
                    )
                }
                #[doc = " Fungible assets which we know are locked on this chain."]
                pub fn locked_fungibles_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::core::primitive::u128,
                        runtime_types::xcm::VersionedMultiLocation,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "LockedFungibles",
                        Vec::new(),
                        [
                            110u8, 220u8, 127u8, 176u8, 219u8, 23u8, 132u8, 36u8, 224u8, 187u8,
                            25u8, 103u8, 126u8, 99u8, 34u8, 105u8, 57u8, 182u8, 162u8, 69u8, 24u8,
                            67u8, 221u8, 103u8, 79u8, 139u8, 187u8, 162u8, 113u8, 109u8, 163u8,
                            35u8,
                        ],
                    )
                }
                #[doc = " Global suspension state of the XCM executor."]
                pub fn xcm_execution_suspended(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PolkadotXcm",
                        "XcmExecutionSuspended",
                        vec![],
                        [
                            182u8, 54u8, 69u8, 68u8, 78u8, 76u8, 103u8, 79u8, 47u8, 136u8, 99u8,
                            104u8, 128u8, 129u8, 249u8, 54u8, 214u8, 136u8, 97u8, 48u8, 178u8,
                            42u8, 26u8, 27u8, 82u8, 24u8, 33u8, 77u8, 33u8, 27u8, 20u8, 127u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod cumulus_xcm {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::cumulus_pallet_xcm::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::cumulus_pallet_xcm::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
            }
            pub struct TransactionApi;
            impl TransactionApi {}
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message is invalid XCM."]
            #[doc = "\\[ id \\]"]
            pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::events::StaticEvent for InvalidFormat {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message is unsupported version of XCM."]
            #[doc = "\\[ id \\]"]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::events::StaticEvent for UnsupportedVersion {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message executed with the given outcome."]
            #[doc = "\\[ id, outcome \\]"]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 32usize],
                pub runtime_types::xcm::v3::traits::Outcome,
            );
            impl ::subxt::events::StaticEvent for ExecutedDownward {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "ExecutedDownward";
            }
        }
    }
    pub mod dmp_queue {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::cumulus_pallet_dmp_queue::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::cumulus_pallet_dmp_queue::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ServiceOverweight {
                    pub index: ::core::primitive::u64,
                    pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
                    const PALLET: &'static str = "DmpQueue";
                    const CALL: &'static str = "service_overweight";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Service a single overweight message."]
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::ServiceOverweight> {
                    ::subxt::tx::Payload::new_static(
                        "DmpQueue",
                        "service_overweight",
                        types::ServiceOverweight {
                            index,
                            weight_limit,
                        },
                        [
                            235u8, 203u8, 220u8, 162u8, 173u8, 117u8, 224u8, 194u8, 176u8, 125u8,
                            50u8, 74u8, 180u8, 37u8, 126u8, 227u8, 138u8, 213u8, 227u8, 35u8,
                            247u8, 18u8, 160u8, 231u8, 97u8, 149u8, 144u8, 49u8, 34u8, 146u8, 32u8,
                            7u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message is invalid XCM."]
            pub struct InvalidFormat {
                pub message_id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for InvalidFormat {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message is unsupported version of XCM."]
            pub struct UnsupportedVersion {
                pub message_id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for UnsupportedVersion {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message executed with the given outcome."]
            pub struct ExecutedDownward {
                pub message_id: [::core::primitive::u8; 32usize],
                pub outcome: runtime_types::xcm::v3::traits::Outcome,
            }
            impl ::subxt::events::StaticEvent for ExecutedDownward {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "ExecutedDownward";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The weight limit for handling downward messages was reached."]
            pub struct WeightExhausted {
                pub message_id: [::core::primitive::u8; 32usize],
                pub remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
                pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for WeightExhausted {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message is overweight and was placed in the overweight queue."]
            pub struct OverweightEnqueued {
                pub message_id: [::core::primitive::u8; 32usize],
                pub overweight_index: ::core::primitive::u64,
                pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for OverweightEnqueued {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Downward message from the overweight queue was executed."]
            pub struct OverweightServiced {
                pub overweight_index: ::core::primitive::u64,
                pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
            }
            impl ::subxt::events::StaticEvent for OverweightServiced {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The maximum number of downward messages was."]
            pub struct MaxMessagesExhausted {
                pub message_id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for MaxMessagesExhausted {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "MaxMessagesExhausted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The configuration."]
                pub fn configuration(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::cumulus_pallet_dmp_queue::ConfigData,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "Configuration",
                        vec![],
                        [
                            28u8, 58u8, 57u8, 84u8, 115u8, 69u8, 158u8, 234u8, 180u8, 37u8, 138u8,
                            120u8, 182u8, 145u8, 109u8, 203u8, 62u8, 102u8, 168u8, 56u8, 236u8,
                            10u8, 236u8, 104u8, 232u8, 245u8, 107u8, 143u8, 247u8, 232u8, 135u8,
                            131u8,
                        ],
                    )
                }
                #[doc = " The page index."]
                pub fn page_index(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "PageIndex",
                        vec![],
                        [
                            246u8, 129u8, 111u8, 255u8, 168u8, 54u8, 121u8, 21u8, 159u8, 142u8,
                            252u8, 173u8, 3u8, 191u8, 202u8, 158u8, 86u8, 26u8, 76u8, 134u8, 201u8,
                            138u8, 103u8, 75u8, 223u8, 57u8, 36u8, 45u8, 171u8, 190u8, 21u8, 60u8,
                        ],
                    )
                }
                #[doc = " The queue pages."]
                pub fn pages(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "Pages",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            24u8, 215u8, 210u8, 131u8, 23u8, 56u8, 71u8, 143u8, 35u8, 151u8, 223u8,
                            133u8, 42u8, 32u8, 180u8, 85u8, 146u8, 166u8, 6u8, 168u8, 227u8, 128u8,
                            30u8, 108u8, 103u8, 16u8, 169u8, 235u8, 238u8, 224u8, 247u8, 233u8,
                        ],
                    )
                }
                #[doc = " The queue pages."]
                pub fn pages_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "Pages",
                        Vec::new(),
                        [
                            24u8, 215u8, 210u8, 131u8, 23u8, 56u8, 71u8, 143u8, 35u8, 151u8, 223u8,
                            133u8, 42u8, 32u8, 180u8, 85u8, 146u8, 166u8, 6u8, 168u8, 227u8, 128u8,
                            30u8, 108u8, 103u8, 16u8, 169u8, 235u8, 238u8, 224u8, 247u8, 233u8,
                        ],
                    )
                }
                #[doc = " The overweight messages."]
                pub fn overweight(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "Overweight",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
                            48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
                            117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
                        ],
                    )
                }
                #[doc = " The overweight messages."]
                pub fn overweight_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "Overweight",
                        Vec::new(),
                        [
                            86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
                            48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
                            117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]
                pub fn counter_for_overweight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "DmpQueue",
                        "CounterForOverweight",
                        vec![],
                        [
                            44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
                            28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
                            143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
                            247u8, 0u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet"]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Sudo {
                    pub call:
                        ::std::boxed::Box<runtime_types::parachain_template_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoUncheckedWeight {
                    pub call:
                        ::std::boxed::Box<runtime_types::parachain_template_runtime::RuntimeCall>,
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetKey {
                    pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoAs {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub call:
                        ::std::boxed::Box<runtime_types::parachain_template_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo(
                    &self,
                    call: runtime_types::parachain_template_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::Sudo> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            63u8, 190u8, 231u8, 170u8, 220u8, 87u8, 204u8, 118u8, 126u8, 187u8,
                            46u8, 187u8, 190u8, 6u8, 15u8, 77u8, 44u8, 92u8, 80u8, 159u8, 6u8,
                            149u8, 246u8, 219u8, 97u8, 102u8, 134u8, 17u8, 204u8, 238u8, 20u8,
                            189u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::parachain_template_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            59u8, 91u8, 152u8, 251u8, 199u8, 66u8, 40u8, 208u8, 125u8, 159u8,
                            157u8, 123u8, 95u8, 153u8, 9u8, 112u8, 129u8, 146u8, 251u8, 99u8,
                            229u8, 150u8, 0u8, 8u8, 227u8, 103u8, 116u8, 106u8, 204u8, 244u8,
                            216u8, 68u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn set_key(
                    &self,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::SetKey> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call: runtime_types::parachain_template_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::SudoAs> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            10u8, 209u8, 126u8, 232u8, 228u8, 185u8, 233u8, 91u8, 8u8, 37u8, 238u8,
                            253u8, 42u8, 95u8, 25u8, 45u8, 129u8, 13u8, 162u8, 224u8, 119u8, 21u8,
                            88u8, 42u8, 9u8, 217u8, 164u8, 170u8, 30u8, 177u8, 220u8, 251u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod tellor {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::tellor::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::tellor::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Register {
                    pub gas_limit: ::core::option::Option<::core::primitive::u64>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Register {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "register";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ClaimOnetimeTip {
                    pub query_id: ::subxt::utils::H256,
                    pub timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::codec::Compact<::core::primitive::u64>,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for ClaimOnetimeTip {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "claim_onetime_tip";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ClaimTip {
                    pub feed_id: ::subxt::utils::H256,
                    pub query_id: ::subxt::utils::H256,
                    pub timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::codec::Compact<::core::primitive::u64>,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for ClaimTip {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "claim_tip";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FundFeed {
                    pub feed_id: ::subxt::utils::H256,
                    pub query_id: ::subxt::utils::H256,
                    #[codec(compact)]
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for FundFeed {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "fund_feed";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetupDataFeed {
                    pub query_id: ::subxt::utils::H256,
                    #[codec(compact)]
                    pub reward: ::core::primitive::u128,
                    #[codec(compact)]
                    pub start_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub interval: ::core::primitive::u64,
                    #[codec(compact)]
                    pub window: ::core::primitive::u64,
                    pub price_threshold: ::core::primitive::u16,
                    #[codec(compact)]
                    pub reward_increase_per_second: ::core::primitive::u128,
                    pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetupDataFeed {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "setup_data_feed";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Tip {
                    pub query_id: ::subxt::utils::H256,
                    #[codec(compact)]
                    pub amount: ::core::primitive::u128,
                    pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for Tip {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "tip";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddStakingRewards {
                    #[codec(compact)]
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddStakingRewards {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "add_staking_rewards";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitValue {
                    pub query_id: ::subxt::utils::H256,
                    pub value: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub nonce: ::core::primitive::u32,
                    pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SubmitValue {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "submit_value";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateStakeAmount;
                impl ::subxt::blocks::StaticExtrinsic for UpdateStakeAmount {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "update_stake_amount";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BeginDispute {
                    pub query_id: ::subxt::utils::H256,
                    #[codec(compact)]
                    pub timestamp: ::core::primitive::u64,
                    pub beneficiary: ::core::option::Option<::subxt::utils::H160>,
                }
                impl ::subxt::blocks::StaticExtrinsic for BeginDispute {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "begin_dispute";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Vote {
                    pub dispute_id: ::subxt::utils::H256,
                    pub supports: ::core::option::Option<::core::primitive::bool>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Vote {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "vote";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct VoteOnMultipleDisputes {
                    pub votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::subxt::utils::H256,
                        ::core::option::Option<::core::primitive::bool>,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for VoteOnMultipleDisputes {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "vote_on_multiple_disputes";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SendVotes {
                    pub max_votes: ::core::primitive::u8,
                }
                impl ::subxt::blocks::StaticExtrinsic for SendVotes {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "send_votes";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportStakeDeposited {
                    pub reporter: ::subxt::utils::AccountId32,
                    pub amount: runtime_types::primitive_types::U256,
                    pub address: ::subxt::utils::H160,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportStakeDeposited {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_stake_deposited";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportStakingWithdrawRequest {
                    pub reporter: ::subxt::utils::AccountId32,
                    pub amount: runtime_types::primitive_types::U256,
                    pub address: ::subxt::utils::H160,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportStakingWithdrawRequest {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_staking_withdraw_request";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportStakeWithdrawn {
                    pub reporter: ::subxt::utils::AccountId32,
                    pub amount: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportStakeWithdrawn {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_stake_withdrawn";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportSlash {
                    pub reporter: ::subxt::utils::AccountId32,
                    pub amount: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportSlash {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_slash";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportVoteTallied {
                    pub dispute_id: ::subxt::utils::H256,
                    pub result: runtime_types::tellor::types::governance::VoteResult,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportVoteTallied {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_vote_tallied";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportVoteExecuted {
                    pub dispute_id: ::subxt::utils::H256,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportVoteExecuted {
                    const PALLET: &'static str = "Tellor";
                    const CALL: &'static str = "report_vote_executed";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Registers the parachain with the Tellor controller contracts."]
                pub fn register(
                    &self,
                    gas_limit: ::core::option::Option<::core::primitive::u64>,
                ) -> ::subxt::tx::Payload<types::Register> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "register",
                        types::Register { gas_limit },
                        [
                            198u8, 159u8, 174u8, 145u8, 9u8, 205u8, 42u8, 247u8, 161u8, 87u8, 7u8,
                            194u8, 105u8, 0u8, 1u8, 168u8, 154u8, 101u8, 239u8, 241u8, 49u8, 109u8,
                            175u8, 112u8, 35u8, 154u8, 17u8, 158u8, 5u8, 207u8, 234u8, 148u8,
                        ],
                    )
                }
                #[doc = "Function to claim singular tip."]
                #[doc = ""]
                #[doc = "- `query_id`: Identifier of reported data."]
                #[doc = "- `timestamps`: Batch of timestamps of reported data eligible for reward."]
                pub fn claim_onetime_tip(
                    &self,
                    query_id: ::subxt::utils::H256,
                    timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::codec::Compact<::core::primitive::u64>,
                    >,
                ) -> ::subxt::tx::Payload<types::ClaimOnetimeTip> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "claim_onetime_tip",
                        types::ClaimOnetimeTip {
                            query_id,
                            timestamps,
                        },
                        [
                            13u8, 169u8, 67u8, 34u8, 61u8, 99u8, 60u8, 138u8, 61u8, 39u8, 208u8,
                            237u8, 184u8, 73u8, 12u8, 197u8, 145u8, 97u8, 94u8, 249u8, 190u8,
                            171u8, 49u8, 253u8, 75u8, 218u8, 89u8, 252u8, 100u8, 110u8, 204u8,
                            132u8,
                        ],
                    )
                }
                #[doc = "Allows Tellor reporters to claim their tips in batches."]
                #[doc = ""]
                #[doc = "- `feed_id`: Unique feed identifier."]
                #[doc = "- `query_id`: Identifier of reported data."]
                #[doc = "- `timestamps`: Batch of timestamps of reported data eligible for reward."]
                pub fn claim_tip(
                    &self,
                    feed_id: ::subxt::utils::H256,
                    query_id: ::subxt::utils::H256,
                    timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::ext::codec::Compact<::core::primitive::u64>,
                    >,
                ) -> ::subxt::tx::Payload<types::ClaimTip> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "claim_tip",
                        types::ClaimTip {
                            feed_id,
                            query_id,
                            timestamps,
                        },
                        [
                            196u8, 224u8, 184u8, 199u8, 236u8, 148u8, 210u8, 10u8, 27u8, 49u8,
                            90u8, 15u8, 45u8, 130u8, 170u8, 222u8, 205u8, 132u8, 13u8, 149u8,
                            121u8, 164u8, 43u8, 213u8, 11u8, 61u8, 78u8, 147u8, 75u8, 196u8, 60u8,
                            127u8,
                        ],
                    )
                }
                #[doc = "Allows data feed account to be filled with tokens."]
                #[doc = ""]
                #[doc = "- `feed_id`: Unique feed identifier."]
                #[doc = "- `query_id`: Identifier of reported data type associated with feed."]
                #[doc = "- `amount`: Quantity of tokens to fund feed."]
                pub fn fund_feed(
                    &self,
                    feed_id: ::subxt::utils::H256,
                    query_id: ::subxt::utils::H256,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::FundFeed> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "fund_feed",
                        types::FundFeed {
                            feed_id,
                            query_id,
                            amount,
                        },
                        [
                            74u8, 251u8, 208u8, 172u8, 252u8, 79u8, 165u8, 33u8, 203u8, 59u8, 42u8,
                            216u8, 30u8, 29u8, 157u8, 101u8, 138u8, 166u8, 96u8, 229u8, 250u8,
                            173u8, 164u8, 254u8, 102u8, 238u8, 0u8, 33u8, 180u8, 73u8, 60u8, 170u8,
                        ],
                    )
                }
                #[doc = "Initializes data feed parameters."]
                #[doc = ""]
                #[doc = "- `query_id`: Unique identifier of desired data feed."]
                #[doc = "- `reward`: Tip amount per eligible data submission."]
                #[doc = "- `start_time`: Timestamp of first autopay window."]
                #[doc = "- `interval`: Amount of time between autopay windows."]
                #[doc = "- `window`: Amount of time after each new interval when reports are eligible for tips."]
                #[doc = "- `price_threshold`: Amount price must change to automate update regardless of time (negated if 0, 100 = 1%)."]
                #[doc = "- `reward_increase_per_second`: Amount reward increases per second within a window (0 for flat reward)."]
                #[doc = "- `query_data`: The data used by reporters to fulfil the query."]
                #[doc = "- `amount`: Optional initial amount to fund it with."]
                pub fn setup_data_feed(
                    &self,
                    query_id: ::subxt::utils::H256,
                    reward: ::core::primitive::u128,
                    start_time: ::core::primitive::u64,
                    interval: ::core::primitive::u64,
                    window: ::core::primitive::u64,
                    price_threshold: ::core::primitive::u16,
                    reward_increase_per_second: ::core::primitive::u128,
                    query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetupDataFeed> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "setup_data_feed",
                        types::SetupDataFeed {
                            query_id,
                            reward,
                            start_time,
                            interval,
                            window,
                            price_threshold,
                            reward_increase_per_second,
                            query_data,
                            amount,
                        },
                        [
                            221u8, 244u8, 243u8, 108u8, 247u8, 207u8, 78u8, 101u8, 181u8, 77u8,
                            30u8, 209u8, 91u8, 79u8, 89u8, 250u8, 166u8, 97u8, 50u8, 226u8, 184u8,
                            12u8, 158u8, 180u8, 136u8, 129u8, 25u8, 30u8, 214u8, 194u8, 109u8,
                            142u8,
                        ],
                    )
                }
                #[doc = "Function to run a single tip."]
                #[doc = ""]
                #[doc = "- `query_id`: Identifier of tipped data."]
                #[doc = "- `amount`: Amount to tip."]
                #[doc = "- `query_data`: The data used by reporters to fulfil the query."]
                pub fn tip(
                    &self,
                    query_id: ::subxt::utils::H256,
                    amount: ::core::primitive::u128,
                    query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::Payload<types::Tip> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "tip",
                        types::Tip {
                            query_id,
                            amount,
                            query_data,
                        },
                        [
                            64u8, 60u8, 23u8, 183u8, 122u8, 111u8, 199u8, 231u8, 128u8, 163u8,
                            207u8, 219u8, 223u8, 148u8, 34u8, 17u8, 181u8, 24u8, 75u8, 93u8, 92u8,
                            251u8, 224u8, 200u8, 36u8, 104u8, 92u8, 38u8, 63u8, 153u8, 104u8,
                            238u8,
                        ],
                    )
                }
                #[doc = "Funds the staking account with staking rewards."]
                #[doc = ""]
                #[doc = "- `amount`: Amount of tokens to fund staking account with."]
                pub fn add_staking_rewards(
                    &self,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::AddStakingRewards> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "add_staking_rewards",
                        types::AddStakingRewards { amount },
                        [
                            140u8, 161u8, 143u8, 178u8, 247u8, 146u8, 72u8, 22u8, 115u8, 210u8,
                            209u8, 96u8, 233u8, 160u8, 120u8, 242u8, 141u8, 141u8, 202u8, 72u8,
                            175u8, 54u8, 68u8, 84u8, 207u8, 5u8, 245u8, 150u8, 33u8, 136u8, 16u8,
                            247u8,
                        ],
                    )
                }
                #[doc = "Allows a reporter to submit a value to the oracle."]
                #[doc = ""]
                #[doc = "- `query_id`: Identifier of the specific data feed."]
                #[doc = "- `value`: Value the user submits to the oracle."]
                #[doc = "- `nonce`: The current value count for the query identifier."]
                #[doc = "- `query_data`: The data used to fulfil the data query."]
                pub fn submit_value(
                    &self,
                    query_id: ::subxt::utils::H256,
                    value: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    nonce: ::core::primitive::u32,
                    query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::Payload<types::SubmitValue> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "submit_value",
                        types::SubmitValue {
                            query_id,
                            value,
                            nonce,
                            query_data,
                        },
                        [
                            55u8, 246u8, 139u8, 89u8, 224u8, 48u8, 233u8, 29u8, 69u8, 139u8, 118u8,
                            8u8, 48u8, 97u8, 234u8, 111u8, 37u8, 197u8, 206u8, 0u8, 119u8, 243u8,
                            85u8, 255u8, 69u8, 58u8, 236u8, 57u8, 56u8, 0u8, 105u8, 196u8,
                        ],
                    )
                }
                #[doc = "Updates the stake amount after retrieving the latest token price from oracle."]
                pub fn update_stake_amount(
                    &self,
                ) -> ::subxt::tx::Payload<types::UpdateStakeAmount> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "update_stake_amount",
                        types::UpdateStakeAmount {},
                        [
                            124u8, 35u8, 144u8, 18u8, 237u8, 156u8, 234u8, 248u8, 119u8, 208u8,
                            29u8, 253u8, 68u8, 64u8, 122u8, 211u8, 172u8, 100u8, 29u8, 82u8, 27u8,
                            166u8, 20u8, 185u8, 130u8, 142u8, 201u8, 110u8, 214u8, 130u8, 51u8,
                            241u8,
                        ],
                    )
                }
                #[doc = "Initialises a dispute/vote in the system."]
                #[doc = ""]
                #[doc = "- `query_id`: Query identifier being disputed."]
                #[doc = "- `timestamp`: Timestamp being disputed."]
                #[doc = "- 'beneficiary`: address on controller chain to potentially receive the slash amount if dispute successful"]
                pub fn begin_dispute(
                    &self,
                    query_id: ::subxt::utils::H256,
                    timestamp: ::core::primitive::u64,
                    beneficiary: ::core::option::Option<::subxt::utils::H160>,
                ) -> ::subxt::tx::Payload<types::BeginDispute> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "begin_dispute",
                        types::BeginDispute {
                            query_id,
                            timestamp,
                            beneficiary,
                        },
                        [
                            228u8, 178u8, 208u8, 229u8, 183u8, 60u8, 3u8, 209u8, 66u8, 178u8, 59u8,
                            34u8, 25u8, 233u8, 171u8, 2u8, 187u8, 159u8, 178u8, 102u8, 165u8,
                            185u8, 252u8, 219u8, 126u8, 107u8, 238u8, 162u8, 184u8, 85u8, 107u8,
                            126u8,
                        ],
                    )
                }
                #[doc = "Enables the caller to cast a vote."]
                #[doc = ""]
                #[doc = "- `dispute_id`: The identifier of the dispute."]
                #[doc = "- `supports`: Whether the caller supports or is against the vote. None indicates the caller’s classification of the dispute as invalid."]
                pub fn vote(
                    &self,
                    dispute_id: ::subxt::utils::H256,
                    supports: ::core::option::Option<::core::primitive::bool>,
                ) -> ::subxt::tx::Payload<types::Vote> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "vote",
                        types::Vote {
                            dispute_id,
                            supports,
                        },
                        [
                            61u8, 95u8, 233u8, 238u8, 87u8, 80u8, 240u8, 202u8, 138u8, 219u8, 13u8,
                            201u8, 59u8, 70u8, 90u8, 92u8, 156u8, 50u8, 202u8, 226u8, 251u8, 161u8,
                            227u8, 81u8, 223u8, 192u8, 211u8, 55u8, 205u8, 25u8, 23u8, 172u8,
                        ],
                    )
                }
                #[doc = "Enables the caller to cast votes for multiple disputes."]
                #[doc = ""]
                #[doc = "- `votes`: The votes for disputes, containing the dispute identifier and whether the caller supports or is against the vote. None indicates the caller’s classification of the dispute as invalid."]
                pub fn vote_on_multiple_disputes(
                    &self,
                    votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::subxt::utils::H256,
                        ::core::option::Option<::core::primitive::bool>,
                    )>,
                ) -> ::subxt::tx::Payload<types::VoteOnMultipleDisputes> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "vote_on_multiple_disputes",
                        types::VoteOnMultipleDisputes { votes },
                        [
                            25u8, 204u8, 165u8, 101u8, 86u8, 4u8, 188u8, 135u8, 84u8, 4u8, 190u8,
                            17u8, 219u8, 4u8, 165u8, 69u8, 53u8, 138u8, 25u8, 99u8, 222u8, 189u8,
                            108u8, 86u8, 11u8, 114u8, 151u8, 103u8, 37u8, 58u8, 77u8, 139u8,
                        ],
                    )
                }
                #[doc = "Sends any pending dispute votes due to the governance controller contract for tallying."]
                #[doc = ""]
                #[doc = "- `max_votes`: The maximum number of votes to be sent."]
                pub fn send_votes(
                    &self,
                    max_votes: ::core::primitive::u8,
                ) -> ::subxt::tx::Payload<types::SendVotes> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "send_votes",
                        types::SendVotes { max_votes },
                        [
                            109u8, 247u8, 58u8, 65u8, 33u8, 192u8, 146u8, 198u8, 216u8, 241u8,
                            229u8, 165u8, 89u8, 69u8, 163u8, 26u8, 76u8, 12u8, 117u8, 116u8, 198u8,
                            119u8, 233u8, 30u8, 88u8, 82u8, 228u8, 152u8, 83u8, 104u8, 249u8,
                            175u8,
                        ],
                    )
                }
                #[doc = "Reports a stake deposited by a reporter."]
                #[doc = ""]
                #[doc = "- `reporter`: The reporter who deposited a stake."]
                #[doc = "- `amount`: The amount staked."]
                #[doc = "- `address`: The corresponding address on the controlling chain."]
                pub fn report_stake_deposited(
                    &self,
                    reporter: ::subxt::utils::AccountId32,
                    amount: runtime_types::primitive_types::U256,
                    address: ::subxt::utils::H160,
                ) -> ::subxt::tx::Payload<types::ReportStakeDeposited> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_stake_deposited",
                        types::ReportStakeDeposited {
                            reporter,
                            amount,
                            address,
                        },
                        [
                            33u8, 185u8, 38u8, 90u8, 53u8, 142u8, 73u8, 59u8, 194u8, 150u8, 142u8,
                            73u8, 200u8, 71u8, 94u8, 90u8, 239u8, 91u8, 96u8, 128u8, 25u8, 20u8,
                            57u8, 114u8, 63u8, 212u8, 68u8, 15u8, 146u8, 81u8, 89u8, 191u8,
                        ],
                    )
                }
                #[doc = "Reports a staking withdrawal request by a reporter."]
                #[doc = ""]
                #[doc = "- `reporter`: The reporter who requested a withdrawal."]
                #[doc = "- `amount`: The amount requested to withdraw."]
                #[doc = "- `address`: The corresponding address on the controlling chain."]
                pub fn report_staking_withdraw_request(
                    &self,
                    reporter: ::subxt::utils::AccountId32,
                    amount: runtime_types::primitive_types::U256,
                    address: ::subxt::utils::H160,
                ) -> ::subxt::tx::Payload<types::ReportStakingWithdrawRequest> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_staking_withdraw_request",
                        types::ReportStakingWithdrawRequest {
                            reporter,
                            amount,
                            address,
                        },
                        [
                            243u8, 58u8, 223u8, 111u8, 211u8, 229u8, 102u8, 27u8, 55u8, 96u8,
                            102u8, 221u8, 103u8, 24u8, 3u8, 194u8, 57u8, 155u8, 180u8, 194u8,
                            231u8, 17u8, 19u8, 26u8, 160u8, 110u8, 79u8, 170u8, 186u8, 255u8,
                            237u8, 104u8,
                        ],
                    )
                }
                #[doc = "Reports a stake withdrawal by a reporter."]
                #[doc = ""]
                #[doc = "- `reporter`: The reporter who withdrew a stake."]
                #[doc = "- `amount`: The total amount withdrawn."]
                #[doc = "- `address`: The corresponding address on the controlling chain."]
                pub fn report_stake_withdrawn(
                    &self,
                    reporter: ::subxt::utils::AccountId32,
                    amount: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::ReportStakeWithdrawn> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_stake_withdrawn",
                        types::ReportStakeWithdrawn { reporter, amount },
                        [
                            21u8, 79u8, 200u8, 145u8, 184u8, 57u8, 38u8, 213u8, 9u8, 85u8, 109u8,
                            108u8, 214u8, 28u8, 27u8, 240u8, 186u8, 190u8, 123u8, 213u8, 163u8,
                            240u8, 6u8, 249u8, 132u8, 63u8, 76u8, 101u8, 147u8, 70u8, 172u8, 141u8,
                        ],
                    )
                }
                #[doc = "Reports a slashing of a reporter."]
                #[doc = ""]
                #[doc = "- `reporter`: The address of the slashed reporter."]
                #[doc = "- `amount`: The slashed amount."]
                pub fn report_slash(
                    &self,
                    reporter: ::subxt::utils::AccountId32,
                    amount: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::ReportSlash> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_slash",
                        types::ReportSlash { reporter, amount },
                        [
                            174u8, 94u8, 111u8, 250u8, 158u8, 194u8, 199u8, 68u8, 207u8, 202u8,
                            62u8, 14u8, 160u8, 6u8, 116u8, 100u8, 43u8, 147u8, 120u8, 33u8, 185u8,
                            109u8, 172u8, 221u8, 120u8, 158u8, 250u8, 36u8, 147u8, 40u8, 19u8,
                            67u8,
                        ],
                    )
                }
                #[doc = "Reports the tally of a vote."]
                #[doc = ""]
                #[doc = "- `dispute_id`: The identifier of the dispute."]
                #[doc = "- `result`: The outcome of the vote, as determined by governance."]
                pub fn report_vote_tallied(
                    &self,
                    dispute_id: ::subxt::utils::H256,
                    result: runtime_types::tellor::types::governance::VoteResult,
                ) -> ::subxt::tx::Payload<types::ReportVoteTallied> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_vote_tallied",
                        types::ReportVoteTallied { dispute_id, result },
                        [
                            99u8, 135u8, 116u8, 130u8, 184u8, 76u8, 21u8, 93u8, 190u8, 201u8, 17u8,
                            184u8, 251u8, 30u8, 13u8, 46u8, 65u8, 95u8, 57u8, 150u8, 124u8, 243u8,
                            50u8, 0u8, 51u8, 121u8, 252u8, 134u8, 24u8, 135u8, 207u8, 83u8,
                        ],
                    )
                }
                #[doc = "Reports the execution of a vote."]
                #[doc = ""]
                #[doc = "- `dispute_id`: The identifier of the dispute."]
                pub fn report_vote_executed(
                    &self,
                    dispute_id: ::subxt::utils::H256,
                ) -> ::subxt::tx::Payload<types::ReportVoteExecuted> {
                    ::subxt::tx::Payload::new_static(
                        "Tellor",
                        "report_vote_executed",
                        types::ReportVoteExecuted { dispute_id },
                        [
                            148u8, 165u8, 151u8, 67u8, 168u8, 121u8, 76u8, 48u8, 128u8, 201u8,
                            217u8, 26u8, 76u8, 3u8, 185u8, 171u8, 60u8, 29u8, 116u8, 239u8, 59u8,
                            202u8, 208u8, 147u8, 0u8, 97u8, 240u8, 34u8, 200u8, 86u8, 96u8, 100u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::tellor::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a data feed is funded."]
            pub struct DataFeedFunded {
                pub query_id: ::subxt::utils::H256,
                pub feed_id: ::subxt::utils::H256,
                pub amount: ::core::primitive::u128,
                pub feed_funder: ::subxt::utils::AccountId32,
                pub feed_details:
                    runtime_types::tellor::types::autopay::Feed<::core::primitive::u128>,
            }
            impl ::subxt::events::StaticEvent for DataFeedFunded {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "DataFeedFunded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a data feed is set up."]
            pub struct NewDataFeed {
                pub query_id: ::subxt::utils::H256,
                pub feed_id: ::subxt::utils::H256,
                pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub feed_creator: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewDataFeed {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewDataFeed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a onetime tip is claimed."]
            pub struct OneTimeTipClaimed {
                pub query_id: ::subxt::utils::H256,
                pub amount: ::core::primitive::u128,
                pub reporter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for OneTimeTipClaimed {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "OneTimeTipClaimed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a tip is added."]
            pub struct TipAdded {
                pub query_id: ::subxt::utils::H256,
                pub amount: ::core::primitive::u128,
                pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub tipper: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for TipAdded {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "TipAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a tip is claimed."]
            pub struct TipClaimed {
                pub feed_id: ::subxt::utils::H256,
                pub query_id: ::subxt::utils::H256,
                pub amount: ::core::primitive::u128,
                pub reporter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for TipClaimed {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "TipClaimed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a new value is submitted."]
            pub struct NewReport {
                pub query_id: ::subxt::utils::H256,
                pub time: ::core::primitive::u64,
                pub value: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub nonce: ::core::primitive::u32,
                pub query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub reporter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewReport {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewReport";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when the stake amount has changed."]
            pub struct NewStakeAmount {
                pub amount: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for NewStakeAmount {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewStakeAmount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a new staker is reported."]
            pub struct NewStakerReported {
                pub staker: ::subxt::utils::AccountId32,
                pub amount: runtime_types::primitive_types::U256,
                pub address: ::subxt::utils::H160,
            }
            impl ::subxt::events::StaticEvent for NewStakerReported {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewStakerReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a stake slash is reported."]
            pub struct SlashReported {
                pub reporter: ::subxt::utils::AccountId32,
                pub amount: runtime_types::primitive_types::U256,
            }
            impl ::subxt::events::StaticEvent for SlashReported {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "SlashReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a stake withdrawal is reported."]
            pub struct StakeWithdrawnReported {
                pub staker: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for StakeWithdrawnReported {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "StakeWithdrawnReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a stake withdrawal request is reported."]
            pub struct StakeWithdrawRequestReported {
                pub reporter: ::subxt::utils::AccountId32,
                pub amount: runtime_types::primitive_types::U256,
                pub address: ::subxt::utils::H160,
            }
            impl ::subxt::events::StaticEvent for StakeWithdrawRequestReported {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "StakeWithdrawRequestReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when staking rewards are added."]
            pub struct StakingRewardsAdded {
                pub source: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for StakingRewardsAdded {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "StakingRewardsAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a value is removed (via governance)."]
            pub struct ValueRemoved {
                pub query_id: ::subxt::utils::H256,
                pub timestamp: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for ValueRemoved {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "ValueRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a new dispute is opened."]
            pub struct NewDispute {
                pub dispute_id: ::subxt::utils::H256,
                pub query_id: ::subxt::utils::H256,
                pub timestamp: ::core::primitive::u64,
                pub reporter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewDispute {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewDispute";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a new dispute is sent to the governance controller contract."]
            pub struct NewDisputeSent {
                pub para_id: ::core::primitive::u32,
                pub contract_address: ::subxt::utils::H160,
            }
            impl ::subxt::events::StaticEvent for NewDisputeSent {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewDisputeSent";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when the dispute fee has changed."]
            pub struct NewDisputeFee {
                pub dispute_fee: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for NewDisputeFee {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "NewDisputeFee";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when an address casts their vote."]
            pub struct Voted {
                pub dispute_id: ::subxt::utils::H256,
                pub supports: ::core::option::Option<::core::primitive::bool>,
                pub voter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Voted {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "Voted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a vote is sent to the governance controller contract for tallying."]
            pub struct VoteSent {
                pub para_id: ::core::primitive::u32,
                pub contract_address: ::subxt::utils::H160,
                pub dispute_id: ::subxt::utils::H256,
                pub vote_round: ::core::primitive::u8,
            }
            impl ::subxt::events::StaticEvent for VoteSent {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "VoteSent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when all casting for a vote is tallied."]
            pub struct VoteTallied {
                pub dispute_id: ::subxt::utils::H256,
                pub result: runtime_types::tellor::types::governance::VoteResult,
                pub initiator: ::subxt::utils::AccountId32,
                pub reporter: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for VoteTallied {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "VoteTallied";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when a vote is executed."]
            pub struct VoteExecuted {
                pub dispute_id: ::subxt::utils::H256,
                pub result: runtime_types::tellor::types::governance::VoteResult,
            }
            impl ::subxt::events::StaticEvent for VoteExecuted {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "VoteExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when query data is stored."]
            pub struct QueryDataStored {
                pub query_id: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for QueryDataStored {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "QueryDataStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Emitted when registration is sent to the controller contracts."]
            pub struct RegistrationSent {
                pub para_id: ::core::primitive::u32,
                pub contract_address: ::subxt::utils::H160,
                pub weights: runtime_types::tellor::types::Weights,
            }
            impl ::subxt::events::StaticEvent for RegistrationSent {
                const PALLET: &'static str = "Tellor";
                const EVENT: &'static str = "RegistrationSent";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Mapping query identifier and feed identifier to feed details"]
                pub fn data_feeds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::autopay::Feed<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DataFeeds",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            94u8, 61u8, 116u8, 35u8, 62u8, 222u8, 204u8, 137u8, 209u8, 92u8, 101u8,
                            198u8, 190u8, 119u8, 193u8, 67u8, 171u8, 41u8, 94u8, 79u8, 128u8, 45u8,
                            157u8, 94u8, 215u8, 5u8, 186u8, 103u8, 155u8, 175u8, 187u8, 20u8,
                        ],
                    )
                }
                #[doc = " Mapping query identifier and feed identifier to feed details"]
                pub fn data_feeds_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::autopay::Feed<::core::primitive::u128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DataFeeds",
                        Vec::new(),
                        [
                            94u8, 61u8, 116u8, 35u8, 62u8, 222u8, 204u8, 137u8, 209u8, 92u8, 101u8,
                            198u8, 190u8, 119u8, 193u8, 67u8, 171u8, 41u8, 94u8, 79u8, 128u8, 45u8,
                            157u8, 94u8, 215u8, 5u8, 186u8, 103u8, 155u8, 175u8, 187u8, 20u8,
                        ],
                    )
                }
                #[doc = " Tracks which tips were already paid out."]
                pub fn data_feed_reward_claimed(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _2: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DataFeedRewardClaimed",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
                        ],
                        [
                            104u8, 104u8, 90u8, 129u8, 35u8, 107u8, 140u8, 233u8, 161u8, 191u8,
                            30u8, 101u8, 128u8, 99u8, 176u8, 35u8, 114u8, 183u8, 24u8, 138u8,
                            180u8, 24u8, 169u8, 96u8, 55u8, 10u8, 88u8, 226u8, 195u8, 169u8, 84u8,
                            137u8,
                        ],
                    )
                }
                #[doc = " Tracks which tips were already paid out."]
                pub fn data_feed_reward_claimed_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DataFeedRewardClaimed",
                        Vec::new(),
                        [
                            104u8, 104u8, 90u8, 129u8, 35u8, 107u8, 140u8, 233u8, 161u8, 191u8,
                            30u8, 101u8, 128u8, 99u8, 176u8, 35u8, 114u8, 183u8, 24u8, 138u8,
                            180u8, 24u8, 169u8, 96u8, 55u8, 10u8, 88u8, 226u8, 195u8, 169u8, 84u8,
                            137u8,
                        ],
                    )
                }
                #[doc = " Feed identifiers that have funding"]
                pub fn feeds_with_funding(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "FeedsWithFunding",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            171u8, 248u8, 165u8, 50u8, 11u8, 222u8, 151u8, 100u8, 87u8, 61u8, 42u8,
                            117u8, 38u8, 110u8, 89u8, 115u8, 46u8, 14u8, 211u8, 121u8, 175u8,
                            233u8, 36u8, 100u8, 140u8, 68u8, 12u8, 241u8, 13u8, 131u8, 159u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " Feed identifiers that have funding"]
                pub fn feeds_with_funding_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "FeedsWithFunding",
                        Vec::new(),
                        [
                            171u8, 248u8, 165u8, 50u8, 11u8, 222u8, 151u8, 100u8, 87u8, 61u8, 42u8,
                            117u8, 38u8, 110u8, 89u8, 115u8, 46u8, 14u8, 211u8, 121u8, 175u8,
                            233u8, 36u8, 100u8, 140u8, 68u8, 12u8, 241u8, 13u8, 131u8, 159u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " Mapping feed identifier to query identifier"]
                pub fn query_id_from_data_feed_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryIdFromDataFeedId",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            169u8, 253u8, 243u8, 104u8, 145u8, 105u8, 134u8, 203u8, 59u8, 22u8,
                            148u8, 187u8, 231u8, 173u8, 155u8, 149u8, 219u8, 214u8, 35u8, 255u8,
                            117u8, 37u8, 72u8, 55u8, 2u8, 103u8, 240u8, 97u8, 19u8, 136u8, 159u8,
                            69u8,
                        ],
                    )
                }
                #[doc = " Mapping feed identifier to query identifier"]
                pub fn query_id_from_data_feed_id_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryIdFromDataFeedId",
                        Vec::new(),
                        [
                            169u8, 253u8, 243u8, 104u8, 145u8, 105u8, 134u8, 203u8, 59u8, 22u8,
                            148u8, 187u8, 231u8, 173u8, 155u8, 149u8, 219u8, 214u8, 35u8, 255u8,
                            117u8, 37u8, 72u8, 55u8, 2u8, 103u8, 240u8, 97u8, 19u8, 136u8, 159u8,
                            69u8,
                        ],
                    )
                }
                pub fn query_ids_with_funding(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryIdsWithFunding",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            84u8, 238u8, 210u8, 193u8, 80u8, 255u8, 33u8, 79u8, 65u8, 74u8, 82u8,
                            151u8, 198u8, 231u8, 15u8, 69u8, 168u8, 10u8, 17u8, 129u8, 145u8,
                            208u8, 103u8, 173u8, 104u8, 60u8, 79u8, 39u8, 199u8, 13u8, 157u8, 64u8,
                        ],
                    )
                }
                pub fn query_ids_with_funding_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryIdsWithFunding",
                        Vec::new(),
                        [
                            84u8, 238u8, 210u8, 193u8, 80u8, 255u8, 33u8, 79u8, 65u8, 74u8, 82u8,
                            151u8, 198u8, 231u8, 15u8, 69u8, 168u8, 10u8, 17u8, 129u8, 145u8,
                            208u8, 103u8, 173u8, 104u8, 60u8, 79u8, 39u8, 199u8, 13u8, 157u8, 64u8,
                        ],
                    )
                }
                #[doc = " Mapping query identifier (and index) to tips"]
                pub fn tips(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::autopay::Tip<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Tips",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            254u8, 30u8, 182u8, 94u8, 23u8, 35u8, 68u8, 167u8, 84u8, 67u8, 252u8,
                            254u8, 30u8, 243u8, 215u8, 39u8, 13u8, 251u8, 58u8, 237u8, 82u8, 94u8,
                            9u8, 168u8, 81u8, 157u8, 59u8, 148u8, 140u8, 85u8, 200u8, 58u8,
                        ],
                    )
                }
                #[doc = " Mapping query identifier (and index) to tips"]
                pub fn tips_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::autopay::Tip<::core::primitive::u128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Tips",
                        Vec::new(),
                        [
                            254u8, 30u8, 182u8, 94u8, 23u8, 35u8, 68u8, 167u8, 84u8, 67u8, 252u8,
                            254u8, 30u8, 243u8, 215u8, 39u8, 13u8, 251u8, 58u8, 237u8, 82u8, 94u8,
                            9u8, 168u8, 81u8, 157u8, 59u8, 148u8, 140u8, 85u8, 200u8, 58u8,
                        ],
                    )
                }
                #[doc = " Total tip count per query identifier"]
                pub fn tip_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TipCount",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            141u8, 149u8, 64u8, 87u8, 82u8, 252u8, 234u8, 203u8, 228u8, 213u8,
                            113u8, 205u8, 181u8, 60u8, 14u8, 218u8, 119u8, 207u8, 130u8, 230u8,
                            176u8, 254u8, 192u8, 252u8, 160u8, 169u8, 116u8, 248u8, 152u8, 122u8,
                            10u8, 28u8,
                        ],
                    )
                }
                #[doc = " Total tip count per query identifier"]
                pub fn tip_count_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TipCount",
                        Vec::new(),
                        [
                            141u8, 149u8, 64u8, 87u8, 82u8, 252u8, 234u8, 203u8, 228u8, 213u8,
                            113u8, 205u8, 181u8, 60u8, 14u8, 218u8, 119u8, 207u8, 130u8, 230u8,
                            176u8, 254u8, 192u8, 252u8, 160u8, 169u8, 116u8, 248u8, 152u8, 122u8,
                            10u8, 28u8,
                        ],
                    )
                }
                #[doc = " Tracks user tip total per user"]
                pub fn user_tips_total(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "UserTipsTotal",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            128u8, 190u8, 246u8, 140u8, 167u8, 221u8, 59u8, 63u8, 117u8, 168u8,
                            248u8, 90u8, 157u8, 163u8, 192u8, 110u8, 158u8, 151u8, 43u8, 110u8,
                            176u8, 248u8, 1u8, 126u8, 232u8, 195u8, 185u8, 224u8, 244u8, 78u8,
                            184u8, 94u8,
                        ],
                    )
                }
                #[doc = " Tracks user tip total per user"]
                pub fn user_tips_total_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "UserTipsTotal",
                        Vec::new(),
                        [
                            128u8, 190u8, 246u8, 140u8, 167u8, 221u8, 59u8, 63u8, 117u8, 168u8,
                            248u8, 90u8, 157u8, 163u8, 192u8, 110u8, 158u8, 151u8, 43u8, 110u8,
                            176u8, 248u8, 1u8, 126u8, 232u8, 195u8, 185u8, 224u8, 244u8, 78u8,
                            184u8, 94u8,
                        ],
                    )
                }
                #[doc = " Accumulated staking reward per staked token"]
                pub fn accumulated_reward_per_share(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "AccumulatedRewardPerShare",
                        vec![],
                        [
                            161u8, 193u8, 140u8, 149u8, 38u8, 133u8, 6u8, 75u8, 196u8, 123u8,
                            217u8, 192u8, 142u8, 107u8, 92u8, 219u8, 246u8, 95u8, 92u8, 89u8,
                            221u8, 44u8, 2u8, 148u8, 246u8, 199u8, 145u8, 182u8, 89u8, 247u8,
                            234u8, 83u8,
                        ],
                    )
                }
                #[doc = " The last (non-disputed) reported timestamp (by query identifier)."]
                pub fn last_reported_timestamp(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "LastReportedTimestamp",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            133u8, 27u8, 85u8, 152u8, 179u8, 1u8, 172u8, 40u8, 95u8, 102u8, 89u8,
                            245u8, 252u8, 161u8, 112u8, 90u8, 65u8, 227u8, 98u8, 241u8, 85u8,
                            183u8, 104u8, 212u8, 121u8, 232u8, 9u8, 119u8, 167u8, 154u8, 51u8,
                            105u8,
                        ],
                    )
                }
                #[doc = " The last (non-disputed) reported timestamp (by query identifier)."]
                pub fn last_reported_timestamp_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "LastReportedTimestamp",
                        Vec::new(),
                        [
                            133u8, 27u8, 85u8, 152u8, 179u8, 1u8, 172u8, 40u8, 95u8, 102u8, 89u8,
                            245u8, 252u8, 161u8, 112u8, 90u8, 65u8, 227u8, 98u8, 241u8, 85u8,
                            183u8, 104u8, 212u8, 121u8, 232u8, 9u8, 119u8, 167u8, 154u8, 51u8,
                            105u8,
                        ],
                    )
                }
                #[doc = " A timestamp at which the stake amount was last updated."]
                pub fn last_stake_amount_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "LastStakeAmountUpdate",
                        vec![],
                        [
                            236u8, 30u8, 196u8, 61u8, 154u8, 31u8, 55u8, 157u8, 78u8, 224u8, 209u8,
                            142u8, 63u8, 141u8, 67u8, 113u8, 205u8, 6u8, 156u8, 217u8, 234u8,
                            128u8, 131u8, 117u8, 208u8, 76u8, 64u8, 139u8, 136u8, 20u8, 173u8,
                            170u8,
                        ],
                    )
                }
                #[doc = " Mapping of reports by query identifier and timestamp."]
                pub fn reports(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::oracle::Report<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Reports",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            204u8, 195u8, 95u8, 76u8, 160u8, 195u8, 167u8, 145u8, 24u8, 144u8,
                            85u8, 153u8, 7u8, 246u8, 7u8, 84u8, 183u8, 149u8, 75u8, 42u8, 187u8,
                            187u8, 233u8, 61u8, 123u8, 1u8, 245u8, 14u8, 45u8, 172u8, 36u8, 245u8,
                        ],
                    )
                }
                #[doc = " Mapping of reports by query identifier and timestamp."]
                pub fn reports_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::oracle::Report<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Reports",
                        Vec::new(),
                        [
                            204u8, 195u8, 95u8, 76u8, 160u8, 195u8, 167u8, 145u8, 24u8, 144u8,
                            85u8, 153u8, 7u8, 246u8, 7u8, 84u8, 183u8, 149u8, 75u8, 42u8, 187u8,
                            187u8, 233u8, 61u8, 123u8, 1u8, 245u8, 14u8, 45u8, 172u8, 36u8, 245u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamps (by query identifier) to respective indices."]
                pub fn reported_timestamps_by_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedTimestampsByIndex",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            102u8, 204u8, 210u8, 117u8, 32u8, 70u8, 65u8, 243u8, 182u8, 160u8,
                            59u8, 235u8, 79u8, 195u8, 228u8, 118u8, 145u8, 203u8, 196u8, 185u8,
                            236u8, 117u8, 210u8, 184u8, 11u8, 180u8, 3u8, 101u8, 253u8, 89u8, 84u8,
                            4u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamps (by query identifier) to respective indices."]
                pub fn reported_timestamps_by_index_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedTimestampsByIndex",
                        Vec::new(),
                        [
                            102u8, 204u8, 210u8, 117u8, 32u8, 70u8, 65u8, 243u8, 182u8, 160u8,
                            59u8, 235u8, 79u8, 195u8, 228u8, 118u8, 145u8, 203u8, 196u8, 185u8,
                            236u8, 117u8, 210u8, 184u8, 11u8, 180u8, 3u8, 101u8, 253u8, 89u8, 84u8,
                            4u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamp count by query identifier."]
                pub fn reported_timestamp_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedTimestampCount",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            159u8, 21u8, 176u8, 128u8, 24u8, 144u8, 107u8, 4u8, 9u8, 95u8, 101u8,
                            202u8, 112u8, 216u8, 16u8, 47u8, 192u8, 89u8, 235u8, 107u8, 71u8, 33u8,
                            48u8, 68u8, 247u8, 81u8, 205u8, 228u8, 51u8, 192u8, 3u8, 123u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamp count by query identifier."]
                pub fn reported_timestamp_count_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedTimestampCount",
                        Vec::new(),
                        [
                            159u8, 21u8, 176u8, 128u8, 24u8, 144u8, 107u8, 4u8, 9u8, 95u8, 101u8,
                            202u8, 112u8, 216u8, 16u8, 47u8, 192u8, 89u8, 235u8, 107u8, 71u8, 33u8,
                            48u8, 68u8, 247u8, 81u8, 205u8, 228u8, 51u8, 192u8, 3u8, 123u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamps (by query identifier) to values."]
                pub fn reported_values_by_timestamp(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedValuesByTimestamp",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            195u8, 81u8, 72u8, 217u8, 103u8, 204u8, 209u8, 90u8, 248u8, 40u8,
                            150u8, 155u8, 159u8, 203u8, 154u8, 141u8, 161u8, 172u8, 22u8, 194u8,
                            158u8, 185u8, 235u8, 73u8, 223u8, 254u8, 230u8, 174u8, 210u8, 2u8,
                            62u8, 211u8,
                        ],
                    )
                }
                #[doc = " Mapping of reported timestamps (by query identifier) to values."]
                pub fn reported_values_by_timestamp_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ReportedValuesByTimestamp",
                        Vec::new(),
                        [
                            195u8, 81u8, 72u8, 217u8, 103u8, 204u8, 209u8, 90u8, 248u8, 40u8,
                            150u8, 155u8, 159u8, 203u8, 154u8, 141u8, 161u8, 172u8, 22u8, 194u8,
                            158u8, 185u8, 235u8, 73u8, 223u8, 254u8, 230u8, 174u8, 210u8, 2u8,
                            62u8, 211u8,
                        ],
                    )
                }
                #[doc = " Total staking rewards released per second."]
                pub fn reward_rate(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "RewardRate",
                        vec![],
                        [
                            35u8, 153u8, 6u8, 176u8, 236u8, 148u8, 9u8, 5u8, 251u8, 79u8, 160u8,
                            239u8, 19u8, 242u8, 71u8, 16u8, 215u8, 148u8, 29u8, 203u8, 4u8, 122u8,
                            123u8, 27u8, 193u8, 85u8, 13u8, 255u8, 66u8, 77u8, 33u8, 193u8,
                        ],
                    )
                }
                #[doc = " Minimum amount required to be a staker."]
                pub fn stake_amount(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "StakeAmount",
                        vec![],
                        [
                            30u8, 24u8, 165u8, 210u8, 231u8, 16u8, 63u8, 66u8, 17u8, 32u8, 57u8,
                            193u8, 60u8, 28u8, 9u8, 65u8, 207u8, 143u8, 175u8, 45u8, 156u8, 28u8,
                            101u8, 243u8, 91u8, 206u8, 104u8, 201u8, 107u8, 187u8, 158u8, 96u8,
                        ],
                    )
                }
                #[doc = " Mapping from a staker's account identifier to their staking info."]
                pub fn staker_details(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::oracle::StakeInfo<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "StakerDetails",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            176u8, 202u8, 138u8, 120u8, 241u8, 213u8, 33u8, 14u8, 88u8, 198u8,
                            32u8, 201u8, 26u8, 215u8, 14u8, 186u8, 143u8, 140u8, 117u8, 217u8,
                            253u8, 35u8, 255u8, 59u8, 182u8, 134u8, 37u8, 8u8, 238u8, 82u8, 161u8,
                            123u8,
                        ],
                    )
                }
                #[doc = " Mapping from a staker's account identifier to their staking info."]
                pub fn staker_details_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::oracle::StakeInfo<::core::primitive::u128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "StakerDetails",
                        Vec::new(),
                        [
                            176u8, 202u8, 138u8, 120u8, 241u8, 213u8, 33u8, 14u8, 88u8, 198u8,
                            32u8, 201u8, 26u8, 215u8, 14u8, 186u8, 143u8, 140u8, 117u8, 217u8,
                            253u8, 35u8, 255u8, 59u8, 182u8, 134u8, 37u8, 8u8, 238u8, 82u8, 161u8,
                            123u8,
                        ],
                    )
                }
                #[doc = " Mapping of reporter and query identifier to number of reports submitted."]
                pub fn staker_reports_submitted_by_query_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "StakerReportsSubmittedByQueryId",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            30u8, 185u8, 166u8, 105u8, 192u8, 224u8, 78u8, 144u8, 204u8, 150u8,
                            51u8, 136u8, 102u8, 177u8, 77u8, 10u8, 194u8, 141u8, 84u8, 202u8,
                            218u8, 18u8, 90u8, 160u8, 241u8, 237u8, 43u8, 69u8, 93u8, 252u8, 224u8,
                            233u8,
                        ],
                    )
                }
                #[doc = " Mapping of reporter and query identifier to number of reports submitted."]
                pub fn staker_reports_submitted_by_query_id_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "StakerReportsSubmittedByQueryId",
                        Vec::new(),
                        [
                            30u8, 185u8, 166u8, 105u8, 192u8, 224u8, 78u8, 144u8, 204u8, 150u8,
                            51u8, 136u8, 102u8, 177u8, 77u8, 10u8, 194u8, 141u8, 84u8, 202u8,
                            218u8, 18u8, 90u8, 160u8, 241u8, 237u8, 43u8, 69u8, 93u8, 252u8, 224u8,
                            233u8,
                        ],
                    )
                }
                #[doc = " The time of last update to AccumulatedRewardPerShare."]
                pub fn time_of_last_allocation(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TimeOfLastAllocation",
                        vec![],
                        [
                            163u8, 96u8, 231u8, 188u8, 235u8, 56u8, 186u8, 233u8, 167u8, 227u8,
                            42u8, 15u8, 152u8, 147u8, 250u8, 75u8, 162u8, 164u8, 6u8, 109u8, 140u8,
                            212u8, 158u8, 55u8, 158u8, 217u8, 240u8, 75u8, 25u8, 167u8, 106u8,
                            30u8,
                        ],
                    )
                }
                #[doc = " The time of the last new submitted value."]
                pub fn time_of_last_new_value(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TimeOfLastNewValue",
                        vec![],
                        [
                            203u8, 75u8, 167u8, 165u8, 38u8, 101u8, 153u8, 151u8, 64u8, 104u8,
                            65u8, 234u8, 227u8, 18u8, 10u8, 233u8, 247u8, 245u8, 174u8, 145u8,
                            75u8, 195u8, 107u8, 189u8, 134u8, 169u8, 140u8, 97u8, 49u8, 104u8,
                            142u8, 26u8,
                        ],
                    )
                }
                #[doc = " Staking reward debt, used to calculate real staking rewards balance."]
                pub fn total_reward_debt(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TotalRewardDebt",
                        vec![],
                        [
                            254u8, 82u8, 126u8, 148u8, 219u8, 64u8, 80u8, 171u8, 210u8, 160u8,
                            120u8, 222u8, 215u8, 62u8, 107u8, 37u8, 151u8, 148u8, 35u8, 29u8,
                            247u8, 127u8, 88u8, 67u8, 118u8, 52u8, 110u8, 95u8, 122u8, 13u8, 72u8,
                            178u8,
                        ],
                    )
                }
                #[doc = " Total amount of tokens locked in the staking controller contract."]
                pub fn total_stake_amount(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TotalStakeAmount",
                        vec![],
                        [
                            132u8, 190u8, 37u8, 101u8, 38u8, 96u8, 165u8, 142u8, 3u8, 245u8, 106u8,
                            167u8, 2u8, 76u8, 63u8, 96u8, 139u8, 210u8, 58u8, 122u8, 247u8, 166u8,
                            217u8, 97u8, 75u8, 193u8, 180u8, 182u8, 235u8, 1u8, 255u8, 201u8,
                        ],
                    )
                }
                #[doc = " Total number of stakers with at least StakeAmount staked, not exact."]
                pub fn total_stakers(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "TotalStakers",
                        vec![],
                        [
                            161u8, 139u8, 226u8, 52u8, 97u8, 197u8, 155u8, 181u8, 85u8, 139u8,
                            84u8, 12u8, 163u8, 171u8, 11u8, 217u8, 179u8, 194u8, 62u8, 72u8, 125u8,
                            244u8, 38u8, 155u8, 179u8, 150u8, 19u8, 81u8, 246u8, 10u8, 108u8,
                            252u8,
                        ],
                    )
                }
                #[doc = " Amount locked for withdrawal."]
                pub fn to_withdraw(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "ToWithdraw",
                        vec![],
                        [
                            178u8, 168u8, 35u8, 89u8, 199u8, 116u8, 48u8, 251u8, 123u8, 252u8,
                            26u8, 54u8, 208u8, 86u8, 16u8, 18u8, 252u8, 109u8, 217u8, 149u8, 147u8,
                            40u8, 66u8, 75u8, 151u8, 76u8, 18u8, 37u8, 221u8, 245u8, 204u8, 38u8,
                        ],
                    )
                }
                #[doc = " The latest dispute fee."]
                pub fn dispute_fee(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DisputeFee",
                        vec![],
                        [
                            211u8, 149u8, 175u8, 25u8, 114u8, 253u8, 180u8, 33u8, 12u8, 109u8,
                            187u8, 198u8, 218u8, 98u8, 117u8, 60u8, 151u8, 204u8, 102u8, 26u8,
                            142u8, 112u8, 48u8, 58u8, 33u8, 127u8, 229u8, 52u8, 1u8, 48u8, 254u8,
                            65u8,
                        ],
                    )
                }
                #[doc = " Mapping of reporter accounts to dispute identifiers."]
                pub fn dispute_ids_by_reporter(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DisputeIdsByReporter",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            84u8, 105u8, 143u8, 169u8, 115u8, 159u8, 194u8, 46u8, 62u8, 85u8,
                            167u8, 50u8, 71u8, 93u8, 2u8, 146u8, 117u8, 138u8, 71u8, 145u8, 144u8,
                            238u8, 235u8, 120u8, 212u8, 206u8, 136u8, 64u8, 93u8, 229u8, 108u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " Mapping of reporter accounts to dispute identifiers."]
                pub fn dispute_ids_by_reporter_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DisputeIdsByReporter",
                        Vec::new(),
                        [
                            84u8, 105u8, 143u8, 169u8, 115u8, 159u8, 194u8, 46u8, 62u8, 85u8,
                            167u8, 50u8, 71u8, 93u8, 2u8, 146u8, 117u8, 138u8, 71u8, 145u8, 144u8,
                            238u8, 235u8, 120u8, 212u8, 206u8, 136u8, 64u8, 93u8, 229u8, 108u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the details of the dispute."]
                pub fn dispute_info(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::governance::Dispute<
                        ::subxt::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DisputeInfo",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            95u8, 45u8, 85u8, 39u8, 43u8, 254u8, 83u8, 240u8, 243u8, 131u8, 85u8,
                            32u8, 162u8, 80u8, 222u8, 106u8, 166u8, 241u8, 123u8, 230u8, 142u8,
                            146u8, 8u8, 128u8, 3u8, 222u8, 98u8, 81u8, 87u8, 10u8, 30u8, 41u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the details of the dispute."]
                pub fn dispute_info_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::governance::Dispute<
                        ::subxt::utils::AccountId32,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "DisputeInfo",
                        Vec::new(),
                        [
                            95u8, 45u8, 85u8, 39u8, 43u8, 254u8, 83u8, 240u8, 243u8, 131u8, 85u8,
                            32u8, 162u8, 80u8, 222u8, 106u8, 166u8, 241u8, 123u8, 230u8, 142u8,
                            146u8, 8u8, 128u8, 3u8, 222u8, 98u8, 81u8, 87u8, 10u8, 30u8, 41u8,
                        ],
                    )
                }
                #[doc = " Mapping of a query identifier to the number of corresponding open disputes."]
                pub fn open_disputes_on_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "OpenDisputesOnId",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            4u8, 175u8, 9u8, 123u8, 36u8, 149u8, 206u8, 22u8, 4u8, 36u8, 59u8,
                            219u8, 55u8, 102u8, 21u8, 174u8, 227u8, 60u8, 48u8, 100u8, 26u8, 109u8,
                            28u8, 159u8, 88u8, 14u8, 48u8, 39u8, 13u8, 11u8, 34u8, 142u8,
                        ],
                    )
                }
                #[doc = " Mapping of a query identifier to the number of corresponding open disputes."]
                pub fn open_disputes_on_id_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "OpenDisputesOnId",
                        Vec::new(),
                        [
                            4u8, 175u8, 9u8, 123u8, 36u8, 149u8, 206u8, 22u8, 4u8, 36u8, 59u8,
                            219u8, 55u8, 102u8, 21u8, 174u8, 227u8, 60u8, 48u8, 100u8, 26u8, 109u8,
                            28u8, 159u8, 88u8, 14u8, 48u8, 39u8, 13u8, 11u8, 34u8, 142u8,
                        ],
                    )
                }
                #[doc = " Any pending votes which are queued to be sent to the governance controller contract for tallying."]
                pub fn pending_votes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (::core::primitive::u8, ::core::primitive::u64),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "PendingVotes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            127u8, 27u8, 51u8, 104u8, 151u8, 44u8, 153u8, 78u8, 188u8, 241u8,
                            167u8, 195u8, 179u8, 89u8, 71u8, 83u8, 118u8, 9u8, 114u8, 25u8, 233u8,
                            48u8, 108u8, 95u8, 199u8, 126u8, 83u8, 212u8, 157u8, 34u8, 41u8, 242u8,
                        ],
                    )
                }
                #[doc = " Any pending votes which are queued to be sent to the governance controller contract for tallying."]
                pub fn pending_votes_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (::core::primitive::u8, ::core::primitive::u64),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "PendingVotes",
                        Vec::new(),
                        [
                            127u8, 27u8, 51u8, 104u8, 151u8, 44u8, 153u8, 78u8, 188u8, 241u8,
                            167u8, 195u8, 179u8, 89u8, 71u8, 83u8, 118u8, 9u8, 114u8, 25u8, 233u8,
                            48u8, 108u8, 95u8, 199u8, 126u8, 83u8, 212u8, 157u8, 34u8, 41u8, 242u8,
                        ],
                    )
                }
                #[doc = " Total number of votes initiated."]
                pub fn vote_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteCount",
                        vec![],
                        [
                            180u8, 207u8, 124u8, 78u8, 229u8, 147u8, 99u8, 32u8, 247u8, 219u8, 5u8,
                            29u8, 89u8, 219u8, 89u8, 138u8, 82u8, 78u8, 30u8, 217u8, 251u8, 203u8,
                            114u8, 85u8, 193u8, 168u8, 177u8, 220u8, 254u8, 23u8, 127u8, 114u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the details of the vote round."]
                pub fn vote_info(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u8>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::governance::Vote<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteInfo",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            79u8, 108u8, 118u8, 147u8, 103u8, 182u8, 96u8, 203u8, 216u8, 166u8,
                            28u8, 127u8, 84u8, 146u8, 163u8, 84u8, 246u8, 63u8, 38u8, 108u8, 131u8,
                            41u8, 168u8, 175u8, 138u8, 183u8, 27u8, 23u8, 55u8, 141u8, 59u8, 182u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the details of the vote round."]
                pub fn vote_info_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::tellor::types::governance::Vote<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteInfo",
                        Vec::new(),
                        [
                            79u8, 108u8, 118u8, 147u8, 103u8, 182u8, 96u8, 203u8, 216u8, 166u8,
                            28u8, 127u8, 84u8, 146u8, 163u8, 84u8, 246u8, 63u8, 38u8, 108u8, 131u8,
                            41u8, 168u8, 175u8, 138u8, 183u8, 27u8, 23u8, 55u8, 141u8, 59u8, 182u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the number of vote rounds."]
                pub fn vote_rounds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteRounds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            7u8, 113u8, 109u8, 252u8, 154u8, 93u8, 194u8, 233u8, 42u8, 66u8, 133u8,
                            201u8, 97u8, 169u8, 30u8, 194u8, 173u8, 104u8, 110u8, 130u8, 142u8,
                            237u8, 121u8, 19u8, 159u8, 235u8, 9u8, 18u8, 213u8, 51u8, 205u8, 30u8,
                        ],
                    )
                }
                #[doc = " Mapping of dispute identifiers to the number of vote rounds."]
                pub fn vote_rounds_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u8,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteRounds",
                        Vec::new(),
                        [
                            7u8, 113u8, 109u8, 252u8, 154u8, 93u8, 194u8, 233u8, 42u8, 66u8, 133u8,
                            201u8, 97u8, 169u8, 30u8, 194u8, 173u8, 104u8, 110u8, 130u8, 142u8,
                            237u8, 121u8, 19u8, 159u8, 235u8, 9u8, 18u8, 213u8, 51u8, 205u8, 30u8,
                        ],
                    )
                }
                #[doc = " Mapping of accounts to whether they voted for a dispute round or not."]
                pub fn votes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u8>,
                    _2: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Votes",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
                        ],
                        [
                            91u8, 88u8, 160u8, 84u8, 246u8, 10u8, 106u8, 30u8, 179u8, 233u8, 182u8,
                            44u8, 203u8, 53u8, 58u8, 61u8, 77u8, 45u8, 48u8, 120u8, 75u8, 182u8,
                            51u8, 231u8, 217u8, 191u8, 86u8, 102u8, 25u8, 195u8, 26u8, 147u8,
                        ],
                    )
                }
                #[doc = " Mapping of accounts to whether they voted for a dispute round or not."]
                pub fn votes_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "Votes",
                        Vec::new(),
                        [
                            91u8, 88u8, 160u8, 84u8, 246u8, 10u8, 106u8, 30u8, 179u8, 233u8, 182u8,
                            44u8, 203u8, 53u8, 58u8, 61u8, 77u8, 45u8, 48u8, 120u8, 75u8, 182u8,
                            51u8, 231u8, 217u8, 191u8, 86u8, 102u8, 25u8, 195u8, 26u8, 147u8,
                        ],
                    )
                }
                #[doc = " Mapping of addresses to the number of votes they have cast."]
                pub fn vote_tally_by_address(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteTallyByAddress",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            192u8, 39u8, 37u8, 24u8, 79u8, 17u8, 75u8, 175u8, 255u8, 145u8, 167u8,
                            134u8, 29u8, 61u8, 122u8, 122u8, 31u8, 176u8, 6u8, 124u8, 90u8, 242u8,
                            56u8, 80u8, 82u8, 231u8, 78u8, 56u8, 250u8, 172u8, 196u8, 139u8,
                        ],
                    )
                }
                #[doc = " Mapping of addresses to the number of votes they have cast."]
                pub fn vote_tally_by_address_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "VoteTallyByAddress",
                        Vec::new(),
                        [
                            192u8, 39u8, 37u8, 24u8, 79u8, 17u8, 75u8, 175u8, 255u8, 145u8, 167u8,
                            134u8, 29u8, 61u8, 122u8, 122u8, 31u8, 176u8, 6u8, 124u8, 90u8, 242u8,
                            56u8, 80u8, 82u8, 231u8, 78u8, 56u8, 250u8, 172u8, 196u8, 139u8,
                        ],
                    )
                }
                pub fn query_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            30u8, 245u8, 224u8, 183u8, 89u8, 2u8, 158u8, 2u8, 20u8, 90u8, 242u8,
                            64u8, 50u8, 35u8, 232u8, 230u8, 194u8, 222u8, 217u8, 118u8, 229u8,
                            38u8, 98u8, 50u8, 163u8, 203u8, 53u8, 198u8, 20u8, 243u8, 25u8, 213u8,
                        ],
                    )
                }
                pub fn query_data_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Tellor",
                        "QueryData",
                        Vec::new(),
                        [
                            30u8, 245u8, 224u8, 183u8, 89u8, 2u8, 158u8, 2u8, 20u8, 90u8, 242u8,
                            64u8, 50u8, 35u8, 232u8, 230u8, 194u8, 222u8, 217u8, 118u8, 229u8,
                            38u8, 98u8, 50u8, 163u8, 203u8, 53u8, 198u8, 20u8, 243u8, 25u8, 213u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The number of decimals used by the balance unit."]
                pub fn decimals(&self) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "Decimals",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The index of the `pallet-ethereum-xcm` pallet within the destination chain runtime."]
                pub fn ethereum_xcm_pallet_index(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "EthereumXcmPalletIndex",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " Percentage, 1000 is 100%, 50 is 5%, etc"]
                pub fn fee(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "Fee",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                #[doc = " The location of the governance controller contract."]
                pub fn governance(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::tellor::xcm::ContractLocation>
                {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "Governance",
                        [
                            253u8, 167u8, 223u8, 169u8, 148u8, 55u8, 105u8, 158u8, 131u8, 125u8,
                            26u8, 252u8, 36u8, 15u8, 152u8, 86u8, 189u8, 151u8, 100u8, 148u8,
                            172u8, 135u8, 170u8, 179u8, 196u8, 90u8, 144u8, 161u8, 213u8, 234u8,
                            5u8, 90u8,
                        ],
                    )
                }
                #[doc = " Initial dispute fee."]
                pub fn initial_dispute_fee(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "InitialDisputeFee",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of timestamps per claim."]
                pub fn max_claim_timestamps(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MaxClaimTimestamps",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of sequential disputed timestamps."]
                pub fn max_disputed_time_series(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MaxDisputedTimeSeries",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum length of query data."]
                pub fn max_query_data_length(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MaxQueryDataLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum length of an individual value submitted to the oracle."]
                pub fn max_value_length(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MaxValueLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of votes when voting on multiple disputes."]
                pub fn max_votes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MaxVotes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The minimum amount of tokens required to stake."]
                pub fn minimum_stake_amount(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "MinimumStakeAmount",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The identifier of the pallet within the runtime."]
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId>
                {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "PalletId",
                        [
                            56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
                            161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
                            129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
                        ],
                    )
                }
                #[doc = " The local parachain's own identifier."]
                pub fn parachain_id(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "ParachainId",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The location of the registry controller contract."]
                pub fn registry(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::tellor::xcm::ContractLocation>
                {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "Registry",
                        [
                            253u8, 167u8, 223u8, 169u8, 148u8, 55u8, 105u8, 158u8, 131u8, 125u8,
                            26u8, 252u8, 36u8, 15u8, 152u8, 86u8, 189u8, 151u8, 100u8, 148u8,
                            172u8, 135u8, 170u8, 179u8, 196u8, 90u8, 144u8, 161u8, 213u8, 234u8,
                            5u8, 90u8,
                        ],
                    )
                }
                pub fn stake_amount_currency_target(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "StakeAmountCurrencyTarget",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Staking token 'SpotPrice' query identifier, used for updating stake amount."]
                pub fn staking_token_price_query_id(
                    &self,
                ) -> ::subxt::constants::Address<::subxt::utils::H256> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "StakingTokenPriceQueryId",
                        [
                            115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
                            155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
                            204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
                            135u8,
                        ],
                    )
                }
                #[doc = " Staking token to local token 'SpotPrice' query identifier, used for updating dispute fee."]
                pub fn staking_to_local_token_price_query_id(
                    &self,
                ) -> ::subxt::constants::Address<::subxt::utils::H256> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "StakingToLocalTokenPriceQueryId",
                        [
                            115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
                            155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
                            204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
                            135u8,
                        ],
                    )
                }
                #[doc = " Frequency of stake amount updates."]
                pub fn update_stake_amount_interval(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "UpdateStakeAmountInterval",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " The value to convert weight to fee, used by sent to controller contracts to"]
                #[doc = " calculate fees required for XCM execution on this parachain."]
                pub fn weight_to_fee(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "WeightToFee",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount per weight unit in the asset used for fee payment for remote execution on the controller contract chain."]
                pub fn xcm_weight_to_asset(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Tellor",
                        "XcmWeightToAsset",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod using_tellor {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::using_tellor::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::using_tellor::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Configure {
                    pub query_id: ::subxt::utils::H256,
                }
                impl ::subxt::blocks::StaticExtrinsic for Configure {
                    const PALLET: &'static str = "UsingTellor";
                    const CALL: &'static str = "configure";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DoSomething {
                    pub value: runtime_types::primitive_types::U256,
                }
                impl ::subxt::blocks::StaticExtrinsic for DoSomething {
                    const PALLET: &'static str = "UsingTellor";
                    const CALL: &'static str = "do_something";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "A sample dispatchable that takes a query identifier as a parameter, writes it to"]
                #[doc = "storage and emits an event. This function must be dispatched by the configured origin."]
                pub fn configure(
                    &self,
                    query_id: ::subxt::utils::H256,
                ) -> ::subxt::tx::Payload<types::Configure> {
                    ::subxt::tx::Payload::new_static(
                        "UsingTellor",
                        "configure",
                        types::Configure { query_id },
                        [
                            23u8, 104u8, 124u8, 146u8, 175u8, 89u8, 228u8, 220u8, 139u8, 220u8,
                            43u8, 6u8, 189u8, 80u8, 60u8, 17u8, 122u8, 32u8, 236u8, 91u8, 198u8,
                            123u8, 184u8, 191u8, 166u8, 227u8, 70u8, 234u8, 6u8, 113u8, 126u8,
                            133u8,
                        ],
                    )
                }
                #[doc = "A sample dispatchable that takes a single value as a parameter, derives some new value"]
                #[doc = "and then writes that derived value to storage and emits an event. This function must be"]
                #[doc = "dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    value: runtime_types::primitive_types::U256,
                ) -> ::subxt::tx::Payload<types::DoSomething> {
                    ::subxt::tx::Payload::new_static(
                        "UsingTellor",
                        "do_something",
                        types::DoSomething { value },
                        [
                            169u8, 124u8, 4u8, 161u8, 229u8, 39u8, 7u8, 197u8, 199u8, 204u8, 104u8,
                            162u8, 171u8, 212u8, 165u8, 203u8, 117u8, 71u8, 195u8, 196u8, 41u8,
                            171u8, 223u8, 9u8, 13u8, 121u8, 189u8, 136u8, 217u8, 232u8, 137u8,
                            13u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::using_tellor::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The pallet was configured with a query identifier. [queryId]"]
            pub struct Configured {
                pub query_id: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Configured {
                const PALLET: &'static str = "UsingTellor";
                const EVENT: &'static str = "Configured";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A value was stored. [value, who]"]
            pub struct ValueStored {
                pub value: runtime_types::primitive_types::U256,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ValueStored {
                const PALLET: &'static str = "UsingTellor";
                const EVENT: &'static str = "ValueStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn configuration(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "UsingTellor",
                        "Configuration",
                        vec![],
                        [
                            98u8, 97u8, 190u8, 174u8, 159u8, 93u8, 32u8, 75u8, 79u8, 76u8, 102u8,
                            194u8, 138u8, 193u8, 190u8, 128u8, 78u8, 206u8, 134u8, 68u8, 195u8,
                            189u8, 254u8, 254u8, 121u8, 14u8, 79u8, 48u8, 124u8, 62u8, 188u8, 90u8,
                        ],
                    )
                }
                pub fn values(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "UsingTellor",
                        "Values",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            157u8, 131u8, 30u8, 42u8, 140u8, 42u8, 177u8, 0u8, 217u8, 150u8, 203u8,
                            3u8, 206u8, 28u8, 233u8, 76u8, 14u8, 238u8, 242u8, 132u8, 150u8, 169u8,
                            75u8, 58u8, 118u8, 128u8, 5u8, 81u8, 74u8, 61u8, 130u8, 20u8,
                        ],
                    )
                }
                pub fn values_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::primitive_types::U256,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "UsingTellor",
                        "Values",
                        Vec::new(),
                        [
                            157u8, 131u8, 30u8, 42u8, 140u8, 42u8, 177u8, 0u8, 217u8, 150u8, 203u8,
                            3u8, 206u8, 28u8, 233u8, 76u8, 14u8, 238u8, 242u8, 132u8, 150u8, 169u8,
                            75u8, 58u8, 118u8, 128u8, 5u8, 81u8, 74u8, 61u8, 130u8, 20u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod cumulus_pallet_dmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Service a single overweight message."]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The message index given is unknown."]
                    Unknown,
                    #[codec(index = 1)]
                    #[doc = "The amount of weight given is possibly not enough for executing the message."]
                    OverLimit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Downward message is invalid XCM."]
                    InvalidFormat {
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "Downward message is unsupported version of XCM."]
                    UnsupportedVersion {
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "Downward message executed with the given outcome."]
                    ExecutedDownward {
                        message_id: [::core::primitive::u8; 32usize],
                        outcome: runtime_types::xcm::v3::traits::Outcome,
                    },
                    #[codec(index = 3)]
                    #[doc = "The weight limit for handling downward messages was reached."]
                    WeightExhausted {
                        message_id: [::core::primitive::u8; 32usize],
                        remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
                        required_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 4)]
                    #[doc = "Downward message is overweight and was placed in the overweight queue."]
                    OverweightEnqueued {
                        message_id: [::core::primitive::u8; 32usize],
                        overweight_index: ::core::primitive::u64,
                        required_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 5)]
                    #[doc = "Downward message from the overweight queue was executed."]
                    OverweightServiced {
                        overweight_index: ::core::primitive::u64,
                        weight_used: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 6)]
                    #[doc = "The maximum number of downward messages was."]
                    MaxMessagesExhausted {
                        message_id: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ConfigData {
                pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PageIndexData {
                pub begin_used: ::core::primitive::u32,
                pub end_used: ::core::primitive::u32,
                pub overweight_count: ::core::primitive::u64,
            }
        }
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] # [doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"] # [doc = "later."] # [doc = ""] # [doc = "The `check_version` parameter sets a boolean flag for whether or not the runtime's spec"] # [doc = "version and name should be verified on upgrade. Since the authorization only has a hash,"] # [doc = "it cannot actually perform the verification."] # [doc = ""] # [doc = "This call requires Root origin."] authorize_upgrade { code_hash : :: subxt :: utils :: H256 , check_version : :: core :: primitive :: bool , } , # [codec (index = 3)] # [doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."] # [doc = ""] # [doc = "If the authorization required a version check, this call will ensure the spec name"] # [doc = "remains unchanged and that the spec version has increased."] # [doc = ""] # [doc = "Note that this function will not apply the new `code`, but only attempt to schedule the"] # [doc = "upgrade with the Relay Chain."] # [doc = ""] # [doc = "All origins are allowed."] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to upgrade validation function while existing upgrade pending."]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    #[doc = "Polkadot currently prohibits this parachain from upgrading its validation function."]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    #[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
                    #[doc = "willing to run."]
                    TooBig,
                    #[codec(index = 3)]
                    #[doc = "The inherent which supplies the validation data did not run this block."]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    #[doc = "The inherent which supplies the host configuration did not run this block."]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    #[doc = "No validation function upgrade is currently scheduled."]
                    NotScheduled,
                    #[codec(index = 6)]
                    #[doc = "No code upgrade has been authorized."]
                    NothingAuthorized,
                    #[codec(index = 7)]
                    #[doc = "The given code upgrade has not been authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The validation function has been scheduled to apply."]
                    ValidationFunctionStored,
                    #[codec(index = 1)]
                    #[doc = "The validation function was applied as of the contained relay chain block number."]
                    ValidationFunctionApplied {
                        relay_chain_block_num: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The relay-chain aborted the upgrade process."]
                    ValidationFunctionDiscarded,
                    #[codec(index = 3)]
                    #[doc = "An upgrade has been authorized."]
                    UpgradeAuthorized { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 4)]
                    #[doc = "Some downward messages have been received and will be processed."]
                    DownwardMessagesReceived { count: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Downward messages were processed using the given weight."]
                    DownwardMessagesProcessed {
                        weight_used: runtime_types::sp_weights::weight_v2::Weight,
                        dmq_head: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "An upward message was sent to the relay chain."]
                    UpwardMessageSent {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                }
            }
            pub mod relay_state_snapshot {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MessagingStateSnapshot { pub dmq_mqc_head : :: subxt :: utils :: H256 , pub relay_dispatch_queue_size : runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: RelayDispachQueueSize , pub ingress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , pub egress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RelayDispachQueueSize {
                    pub remaining_count: ::core::primitive::u32,
                    pub remaining_size: ::core::primitive::u32,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
        }
        pub mod cumulus_pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Downward message is invalid XCM."]
                    #[doc = "\\[ id \\]"]
                    InvalidFormat([::core::primitive::u8; 32usize]),
                    #[codec(index = 1)]
                    #[doc = "Downward message is unsupported version of XCM."]
                    #[doc = "\\[ id \\]"]
                    UnsupportedVersion([::core::primitive::u8; 32usize]),
                    #[codec(index = 2)]
                    #[doc = "Downward message executed with the given outcome."]
                    #[doc = "\\[ id, outcome \\]"]
                    ExecutedDownward(
                        [::core::primitive::u8; 32usize],
                        runtime_types::xcm::v3::traits::Outcome,
                    ),
                }
            }
        }
        pub mod cumulus_pallet_xcmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Services a single overweight XCM."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                    #[doc = "- `index`: The index of the overweight XCM to service"]
                    #[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
                    #[doc = ""]
                    #[doc = "Errors:"]
                    #[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
                    #[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
                    #[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
                    #[doc = ""]
                    #[doc = "Events:"]
                    #[doc = "- `OverweightServiced`: On success."]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 1)]
                    #[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    suspend_xcm_execution,
                    #[codec(index = 2)]
                    #[doc = "Resumes all XCM executions for the XCMP queue."]
                    #[doc = ""]
                    #[doc = "Note that this function doesn't change the status of the in/out bound channels."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    resume_xcm_execution,
                    #[codec(index = 3)]
                    #[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
                    #[doc = "suspend their sending."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
                    update_suspend_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
                    #[doc = "messages from the channel."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
                    update_drop_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
                    #[doc = "message sending may recommence after it has been suspended."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
                    update_resume_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
                    update_threshold_weight {
                        new: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 7)]
                    #[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
                    #[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
                    update_weight_restrict_decay {
                        new: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 8)]
                    #[doc = "Overwrite the maximum amount of weight any individual message may consume."]
                    #[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
                    update_xcmp_max_individual_weight {
                        new: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Failed to send XCM message."]
                    FailedToSend,
                    #[codec(index = 1)]
                    #[doc = "Bad XCM origin."]
                    BadXcmOrigin,
                    #[codec(index = 2)]
                    #[doc = "Bad XCM data."]
                    BadXcm,
                    #[codec(index = 3)]
                    #[doc = "Bad overweight index."]
                    BadOverweightIndex,
                    #[codec(index = 4)]
                    #[doc = "Provided weight is possibly not enough to execute the message."]
                    WeightOverLimit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Some XCM was executed ok."]
                    Success {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 1)]
                    #[doc = "Some XCM failed."]
                    Fail {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                        error: runtime_types::xcm::v3::traits::Error,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Bad XCM version used."]
                    BadVersion {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Bad XCM format used."]
                    BadFormat {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                    #[codec(index = 4)]
                    #[doc = "An HRMP message was sent to a sibling parachain."]
                    XcmpMessageSent {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                    #[codec(index = 5)]
                    #[doc = "An XCM exceeded the individual message weight budget."]
                    OverweightEnqueued {
                        sender: runtime_types::polkadot_parachain::primitives::Id,
                        sent_at: ::core::primitive::u32,
                        index: ::core::primitive::u64,
                        required: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 6)]
                    #[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
                    OverweightServiced {
                        index: ::core::primitive::u64,
                        used: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InboundChannelDetails {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
                pub message_metadata: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum InboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OutboundChannelDetails {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
                pub signals_exist: ::core::primitive::bool,
                pub first_index: ::core::primitive::u16,
                pub last_index: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum OutboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct QueueConfigData {
                pub suspend_threshold: ::core::primitive::u32,
                pub drop_threshold: ::core::primitive::u32,
                pub resume_threshold: ::core::primitive::u32,
                pub threshold_weight: runtime_types::sp_weights::weight_v2::Weight,
                pub weight_restrict_decay: runtime_types::sp_weights::weight_v2::Weight,
                pub xcmp_max_individual_weight: runtime_types::sp_weights::weight_v2::Weight,
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct MessageQueueChain(pub ::subxt::utils::H256);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParachainInherentData {
                pub validation_data:
                    runtime_types::polkadot_primitives::v4::PersistedValidationData<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
                pub downward_messages: ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >,
                pub horizontal_messages: ::subxt::utils::KeyedVec<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                >,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "- `O(1)`"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the regular balance of a given account; it also takes a reserved balance but this"]
                    #[doc = "must be the same as the account's current reserved balance."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    #[doc = ""]
                    #[doc = "WARNING: This call is DEPRECATED! Use `force_set_balance` instead."]
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibililty of churn)."]
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Alias for `transfer_allow_death`, provided only for name-wise compatibility."]
                    #[doc = ""]
                    #[doc = "WARNING: DEPRECATED! Will be released in approximately 3 months."]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `MaxHolds`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_collator_selection {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the list of invulnerable (fixed) collators."]
                    set_invulnerables {
                        new: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the ideal number of collators (not including the invulnerables)."]
                    #[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
                    #[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
                    set_desired_candidates { max: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "Set the candidacy bond amount."]
                    set_candidacy_bond { bond: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "Register this account as a collator candidate. The account must (a) already have"]
                    #[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
                    #[doc = ""]
                    #[doc = "This call is not available to `Invulnerable` collators."]
                    register_as_candidate,
                    #[codec(index = 4)]
                    #[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
                    #[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
                    #[doc = ""]
                    #[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
                    #[doc = ""]
                    #[doc = "This call is not available to `Invulnerable` collators."]
                    leave_intent,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidateInfo<_0, _1> {
                    pub who: _0,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many candidates"]
                    TooManyCandidates,
                    #[codec(index = 1)]
                    #[doc = "Too few candidates"]
                    TooFewCandidates,
                    #[codec(index = 2)]
                    #[doc = "Unknown error"]
                    Unknown,
                    #[codec(index = 3)]
                    #[doc = "Permission issue"]
                    Permission,
                    #[codec(index = 4)]
                    #[doc = "User is already a candidate"]
                    AlreadyCandidate,
                    #[codec(index = 5)]
                    #[doc = "User is not a candidate"]
                    NotCandidate,
                    #[codec(index = 6)]
                    #[doc = "Too many invulnerables"]
                    TooManyInvulnerables,
                    #[codec(index = 7)]
                    #[doc = "User is already an Invulnerable"]
                    AlreadyInvulnerable,
                    #[codec(index = 8)]
                    #[doc = "Account has no associated validator ID"]
                    NoAssociatedValidatorId,
                    #[codec(index = 9)]
                    #[doc = "Validator ID is not yet registered"]
                    ValidatorNotRegistered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    NewInvulnerables {
                        invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 1)]
                    NewDesiredCandidates {
                        desired_candidates: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    NewCandidacyBond {
                        bond_amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    CandidateAdded {
                        account_id: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    CandidateRemoved {
                        account_id: ::subxt::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the session key(s) of the function caller to `keys`."]
                    #[doc = "Allows an account to set its session key prior to becoming a validator."]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be signed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
                    #[doc = "  fixed."]
                    set_keys {
                        keys: runtime_types::parachain_template_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes any session key(s) of the function caller."]
                    #[doc = ""]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                    #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                    #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                    #[doc = "usually means being a stash account)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
                    #[doc = "  `T::Keys::key_ids()` which is fixed."]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the session pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo {
                        call: ::std::boxed::Box<
                            runtime_types::parachain_template_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<
                            runtime_types::parachain_template_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::std::boxed::Box<
                            runtime_types::parachain_template_runtime::RuntimeCall,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    send {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Teleport some assets from the local chain to some destination chain."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                    #[doc = "with all fees taken as needed from the asset."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                    #[doc = "  `dest` side. May not be empty."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                    #[doc = "chain and forward a notification XCM."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                    #[doc = "with all fees taken as needed from the asset."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                    #[doc = "  `dest` side."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Execute an XCM message from a local, signed, origin."]
                    #[doc = ""]
                    #[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
                    #[doc = "partially."]
                    #[doc = ""]
                    #[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
                    #[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
                    #[doc = "attempt will be made."]
                    #[doc = ""]
                    #[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
                    #[doc = "to completion; only that *some* of it was executed."]
                    execute {
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 4)]
                    #[doc = "Extoll that a particular destination can be communicated with through a particular"]
                    #[doc = "version of XCM."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                    #[doc = "- `location`: The destination that is being described."]
                    #[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
                    force_xcm_version {
                        location:
                            ::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
                        xcm_version: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
                    #[doc = "version a destination can accept is unknown)."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                    #[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
                    force_default_xcm_version {
                        maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                    #[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
                    force_subscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
                    #[doc = "version changes."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                    #[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
                    #[doc = "  notifications which we no longer desire."]
                    force_unsubscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                    #[doc = "chain and forward a notification XCM."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                    #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                    #[doc = "at risk."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                    #[doc = "  `dest` side."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                    limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 9)]
                    #[doc = "Teleport some assets from the local chain to some destination chain."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                    #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                    #[doc = "at risk."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                    #[doc = "  `dest` side. May not be empty."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                    limited_teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 10)]
                    #[doc = "Set or unset the global suspension state of the XCM executor."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
                    #[doc = "- `suspended`: `true` to suspend, `false` to resume."]
                    force_suspension { suspended: ::core::primitive::bool },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
                    #[doc = "to it."]
                    Unreachable,
                    #[codec(index = 1)]
                    #[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
                    #[doc = "a lack of space for buffering the message."]
                    SendFailure,
                    #[codec(index = 2)]
                    #[doc = "The message execution fails the filter."]
                    Filtered,
                    #[codec(index = 3)]
                    #[doc = "The message's weight could not be determined."]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    #[doc = "The destination `MultiLocation` provided cannot be inverted."]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    #[doc = "The assets to be sent are empty."]
                    Empty,
                    #[codec(index = 6)]
                    #[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
                    CannotReanchor,
                    #[codec(index = 7)]
                    #[doc = "Too many assets have been attempted for transfer."]
                    TooManyAssets,
                    #[codec(index = 8)]
                    #[doc = "Origin is invalid for sending."]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    #[doc = "The version of the `Versioned` value used is not able to be interpreted."]
                    BadVersion,
                    #[codec(index = 10)]
                    #[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
                    #[doc = "desired version of XCM)."]
                    BadLocation,
                    #[codec(index = 11)]
                    #[doc = "The referenced subscription could not be found."]
                    NoSubscription,
                    #[codec(index = 12)]
                    #[doc = "The location is invalid since it already has a subscription from us."]
                    AlreadySubscribed,
                    #[codec(index = 13)]
                    #[doc = "Invalid asset for the operation."]
                    InvalidAsset,
                    #[codec(index = 14)]
                    #[doc = "The owner does not own (all) of the asset that they wish to do the operation on."]
                    LowBalance,
                    #[codec(index = 15)]
                    #[doc = "The asset owner has too many locks on the asset."]
                    TooManyLocks,
                    #[codec(index = 16)]
                    #[doc = "The given account is not an identifiable sovereign account for any location."]
                    AccountNotSovereign,
                    #[codec(index = 17)]
                    #[doc = "The operation required fees to be paid which the initiator could not meet."]
                    FeesNotMet,
                    #[codec(index = 18)]
                    #[doc = "A remote lock with the corresponding data could not be found."]
                    LockNotFound,
                    #[codec(index = 19)]
                    #[doc = "The unlock operation cannot succeed because there are still consumers of the lock."]
                    InUse,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Execution of an XCM message was attempted."]
                    #[doc = ""]
                    #[doc = "\\[ outcome \\]"]
                    Attempted(runtime_types::xcm::v3::traits::Outcome),
                    #[codec(index = 1)]
                    #[doc = "A XCM message was sent."]
                    #[doc = ""]
                    #[doc = "\\[ origin, destination, message \\]"]
                    Sent(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::Xcm,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Query response received which does not match a registered query. This may be because a"]
                    #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
                    #[doc = "because the query timed out."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id \\]"]
                    UnexpectedResponse(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
                    #[doc = "no registered notification call."]
                    #[doc = ""]
                    #[doc = "\\[ id, response \\]"]
                    ResponseReady(::core::primitive::u64, runtime_types::xcm::v3::Response),
                    #[codec(index = 4)]
                    #[doc = "Query response has been received and query is removed. The registered notification has"]
                    #[doc = "been dispatched and executed successfully."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 5)]
                    #[doc = "Query response has been received and query is removed. The registered notification could"]
                    #[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
                    #[doc = "originally budgeted by this runtime for the query result."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
                    NotifyOverweight(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                        runtime_types::sp_weights::weight_v2::Weight,
                        runtime_types::sp_weights::weight_v2::Weight,
                    ),
                    #[codec(index = 6)]
                    #[doc = "Query response has been received and query is removed. There was a general error with"]
                    #[doc = "dispatching the notification call."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    NotifyDispatchError(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 7)]
                    #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
                    #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
                    #[doc = "is not `(origin, QueryId, Response)`."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    NotifyDecodeFailed(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 8)]
                    #[doc = "Expected query response has been received but the origin location of the response does"]
                    #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
                    #[doc = "be received and acted upon."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id, expected location \\]"]
                    InvalidResponder(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 9)]
                    #[doc = "Expected query response has been received but the expected origin location placed in"]
                    #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                    #[doc = ""]
                    #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                    #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                    #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                    #[doc = "needed."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id \\]"]
                    InvalidResponderVersion(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    #[doc = "Received query response has been read and removed."]
                    #[doc = ""]
                    #[doc = "\\[ id \\]"]
                    ResponseTaken(::core::primitive::u64),
                    #[codec(index = 11)]
                    #[doc = "Some assets have been placed in an asset trap."]
                    #[doc = ""]
                    #[doc = "\\[ hash, origin, assets \\]"]
                    AssetsTrapped(
                        ::subxt::utils::H256,
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                    #[codec(index = 12)]
                    #[doc = "An XCM version change notification message has been attempted to be sent."]
                    #[doc = ""]
                    #[doc = "The cost of sending it (borne by the chain) is included."]
                    #[doc = ""]
                    #[doc = "\\[ destination, result, cost \\]"]
                    VersionChangeNotified(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u32,
                        runtime_types::xcm::v3::multiasset::MultiAssets,
                    ),
                    #[codec(index = 13)]
                    #[doc = "The supported version of a location has been changed. This might be through an"]
                    #[doc = "automatic notification or a manual intervention."]
                    #[doc = ""]
                    #[doc = "\\[ location, XCM version \\]"]
                    SupportedVersionChanged(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "sending the notification to it."]
                    #[doc = ""]
                    #[doc = "\\[ location, query ID, error \\]"]
                    NotifyTargetSendFail(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v3::traits::Error,
                    ),
                    #[codec(index = 15)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "migrating the location to our new XCM format."]
                    #[doc = ""]
                    #[doc = "\\[ location, query ID \\]"]
                    NotifyTargetMigrationFail(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 16)]
                    #[doc = "Expected query response has been received but the expected querier location placed in"]
                    #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                    #[doc = ""]
                    #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                    #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                    #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                    #[doc = "needed."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id \\]"]
                    InvalidQuerierVersion(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 17)]
                    #[doc = "Expected query response has been received but the querier location of the response does"]
                    #[doc = "not match the expected. The query remains registered for a later, valid, response to"]
                    #[doc = "be received and acted upon."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id, expected querier, maybe actual querier \\]"]
                    InvalidQuerier(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 18)]
                    #[doc = "A remote has requested XCM version change notification from us and we have honored it."]
                    #[doc = "A version information message is sent to them and its cost is included."]
                    #[doc = ""]
                    #[doc = "\\[ destination location, cost \\]"]
                    VersionNotifyStarted(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::multiasset::MultiAssets,
                    ),
                    #[codec(index = 19)]
                    #[doc = "We have requested that a remote chain sends us XCM version change notifications."]
                    #[doc = ""]
                    #[doc = "\\[ destination location, cost \\]"]
                    VersionNotifyRequested(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::multiasset::MultiAssets,
                    ),
                    #[codec(index = 20)]
                    #[doc = "We have requested that a remote chain stops sending us XCM version change notifications."]
                    #[doc = ""]
                    #[doc = "\\[ destination location, cost \\]"]
                    VersionNotifyUnrequested(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::multiasset::MultiAssets,
                    ),
                    #[codec(index = 21)]
                    #[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
                    #[doc = ""]
                    #[doc = "\\[ paying location, fees \\]"]
                    FeesPaid(
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::v3::multiasset::MultiAssets,
                    ),
                    #[codec(index = 22)]
                    #[doc = "Some assets have been claimed from an asset trap"]
                    #[doc = ""]
                    #[doc = "\\[ hash, origin, assets \\]"]
                    AssetsClaimed(
                        ::subxt::utils::H256,
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum QueryStatus<_0> {
                    #[codec(index = 0)]
                    Pending {
                        responder: runtime_types::xcm::VersionedMultiLocation,
                        maybe_match_querier:
                            ::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
                        maybe_notify:
                            ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                        timeout: _0,
                    },
                    #[codec(index = 1)]
                    VersionNotifier {
                        origin: runtime_types::xcm::VersionedMultiLocation,
                        is_active: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Ready {
                        response: runtime_types::xcm::VersionedResponse,
                        at: _0,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoteLockedFungibleRecord<_0> {
                    pub amount: ::core::primitive::u128,
                    pub owner: runtime_types::xcm::VersionedMultiLocation,
                    pub locker: runtime_types::xcm::VersionedMultiLocation,
                    pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        _0,
                        ::core::primitive::u128,
                    )>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum VersionMigrationStage {
                    #[codec(index = 0)]
                    MigrateSupportedVersion,
                    #[codec(index = 1)]
                    MigrateVersionNotifiers,
                    #[codec(index = 2)]
                    NotifyCurrentTargets(
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ),
                    #[codec(index = 3)]
                    MigrateAndNotifyOldTargets,
                }
            }
        }
        pub mod parachain_info {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {}
            }
        }
        pub mod parachain_template_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                ParachainInfo(runtime_types::parachain_info::pallet::Call),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
                #[codec(index = 33)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
                #[codec(index = 34)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 40)]
                Tellor(runtime_types::tellor::pallet::Call),
                #[codec(index = 41)]
                UsingTellor(runtime_types::using_tellor::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Error),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Error),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Error),
                #[codec(index = 33)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Error),
                #[codec(index = 34)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 40)]
                Tellor(runtime_types::tellor::pallet::Error),
                #[codec(index = 41)]
                UsingTellor(runtime_types::using_tellor::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 11)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                #[codec(index = 33)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
                #[codec(index = 34)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 40)]
                Tellor(runtime_types::tellor::pallet::Event),
                #[codec(index = 41)]
                UsingTellor(runtime_types::using_tellor::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SessionKeys {
                pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum XcmpMessageFormat {
                    #[codec(index = 0)]
                    ConcatenatedVersionedXcm,
                    #[codec(index = 1)]
                    ConcatenatedEncodedBlob,
                    #[codec(index = 2)]
                    Signals,
                }
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v4 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AbridgedHostConfiguration {
                    pub max_code_size: ::core::primitive::u32,
                    pub max_head_data_size: ::core::primitive::u32,
                    pub max_upward_queue_count: ::core::primitive::u32,
                    pub max_upward_queue_size: ::core::primitive::u32,
                    pub max_upward_message_size: ::core::primitive::u32,
                    pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                    pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                    pub validation_upgrade_cooldown: ::core::primitive::u32,
                    pub validation_upgrade_delay: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AbridgedHrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct U256(pub [::core::primitive::u64; 4usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct StorageProof {
                    pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod tellor {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Registers the parachain with the Tellor controller contracts."]
                    register {
                        gas_limit: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Function to claim singular tip."]
                    #[doc = ""]
                    #[doc = "- `query_id`: Identifier of reported data."]
                    #[doc = "- `timestamps`: Batch of timestamps of reported data eligible for reward."]
                    claim_onetime_tip {
                        query_id: ::subxt::utils::H256,
                        timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::codec::Compact<::core::primitive::u64>,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Allows Tellor reporters to claim their tips in batches."]
                    #[doc = ""]
                    #[doc = "- `feed_id`: Unique feed identifier."]
                    #[doc = "- `query_id`: Identifier of reported data."]
                    #[doc = "- `timestamps`: Batch of timestamps of reported data eligible for reward."]
                    claim_tip {
                        feed_id: ::subxt::utils::H256,
                        query_id: ::subxt::utils::H256,
                        timestamps: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::codec::Compact<::core::primitive::u64>,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Allows data feed account to be filled with tokens."]
                    #[doc = ""]
                    #[doc = "- `feed_id`: Unique feed identifier."]
                    #[doc = "- `query_id`: Identifier of reported data type associated with feed."]
                    #[doc = "- `amount`: Quantity of tokens to fund feed."]
                    fund_feed {
                        feed_id: ::subxt::utils::H256,
                        query_id: ::subxt::utils::H256,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Initializes data feed parameters."]
                    #[doc = ""]
                    #[doc = "- `query_id`: Unique identifier of desired data feed."]
                    #[doc = "- `reward`: Tip amount per eligible data submission."]
                    #[doc = "- `start_time`: Timestamp of first autopay window."]
                    #[doc = "- `interval`: Amount of time between autopay windows."]
                    #[doc = "- `window`: Amount of time after each new interval when reports are eligible for tips."]
                    #[doc = "- `price_threshold`: Amount price must change to automate update regardless of time (negated if 0, 100 = 1%)."]
                    #[doc = "- `reward_increase_per_second`: Amount reward increases per second within a window (0 for flat reward)."]
                    #[doc = "- `query_data`: The data used by reporters to fulfil the query."]
                    #[doc = "- `amount`: Optional initial amount to fund it with."]
                    setup_data_feed {
                        query_id: ::subxt::utils::H256,
                        #[codec(compact)]
                        reward: ::core::primitive::u128,
                        #[codec(compact)]
                        start_time: ::core::primitive::u64,
                        #[codec(compact)]
                        interval: ::core::primitive::u64,
                        #[codec(compact)]
                        window: ::core::primitive::u64,
                        price_threshold: ::core::primitive::u16,
                        #[codec(compact)]
                        reward_increase_per_second: ::core::primitive::u128,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Function to run a single tip."]
                    #[doc = ""]
                    #[doc = "- `query_id`: Identifier of tipped data."]
                    #[doc = "- `amount`: Amount to tip."]
                    #[doc = "- `query_data`: The data used by reporters to fulfil the query."]
                    tip {
                        query_id: ::subxt::utils::H256,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Funds the staking account with staking rewards."]
                    #[doc = ""]
                    #[doc = "- `amount`: Amount of tokens to fund staking account with."]
                    add_staking_rewards {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Allows a reporter to submit a value to the oracle."]
                    #[doc = ""]
                    #[doc = "- `query_id`: Identifier of the specific data feed."]
                    #[doc = "- `value`: Value the user submits to the oracle."]
                    #[doc = "- `nonce`: The current value count for the query identifier."]
                    #[doc = "- `query_data`: The data used to fulfil the data query."]
                    submit_value {
                        query_id: ::subxt::utils::H256,
                        value: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        #[codec(compact)]
                        nonce: ::core::primitive::u32,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Updates the stake amount after retrieving the latest token price from oracle."]
                    update_stake_amount,
                    #[codec(index = 9)]
                    #[doc = "Initialises a dispute/vote in the system."]
                    #[doc = ""]
                    #[doc = "- `query_id`: Query identifier being disputed."]
                    #[doc = "- `timestamp`: Timestamp being disputed."]
                    #[doc = "- 'beneficiary`: address on controller chain to potentially receive the slash amount if dispute successful"]
                    begin_dispute {
                        query_id: ::subxt::utils::H256,
                        #[codec(compact)]
                        timestamp: ::core::primitive::u64,
                        beneficiary: ::core::option::Option<::subxt::utils::H160>,
                    },
                    #[codec(index = 10)]
                    #[doc = "Enables the caller to cast a vote."]
                    #[doc = ""]
                    #[doc = "- `dispute_id`: The identifier of the dispute."]
                    #[doc = "- `supports`: Whether the caller supports or is against the vote. None indicates the caller’s classification of the dispute as invalid."]
                    vote {
                        dispute_id: ::subxt::utils::H256,
                        supports: ::core::option::Option<::core::primitive::bool>,
                    },
                    #[codec(index = 11)]
                    #[doc = "Enables the caller to cast votes for multiple disputes."]
                    #[doc = ""]
                    #[doc = "- `votes`: The votes for disputes, containing the dispute identifier and whether the caller supports or is against the vote. None indicates the caller’s classification of the dispute as invalid."]
                    vote_on_multiple_disputes {
                        votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                            ::subxt::utils::H256,
                            ::core::option::Option<::core::primitive::bool>,
                        )>,
                    },
                    #[codec(index = 12)]
                    #[doc = "Sends any pending dispute votes due to the governance controller contract for tallying."]
                    #[doc = ""]
                    #[doc = "- `max_votes`: The maximum number of votes to be sent."]
                    send_votes { max_votes: ::core::primitive::u8 },
                    #[codec(index = 13)]
                    #[doc = "Reports a stake deposited by a reporter."]
                    #[doc = ""]
                    #[doc = "- `reporter`: The reporter who deposited a stake."]
                    #[doc = "- `amount`: The amount staked."]
                    #[doc = "- `address`: The corresponding address on the controlling chain."]
                    report_stake_deposited {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                        address: ::subxt::utils::H160,
                    },
                    #[codec(index = 14)]
                    #[doc = "Reports a staking withdrawal request by a reporter."]
                    #[doc = ""]
                    #[doc = "- `reporter`: The reporter who requested a withdrawal."]
                    #[doc = "- `amount`: The amount requested to withdraw."]
                    #[doc = "- `address`: The corresponding address on the controlling chain."]
                    report_staking_withdraw_request {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                        address: ::subxt::utils::H160,
                    },
                    #[codec(index = 15)]
                    #[doc = "Reports a stake withdrawal by a reporter."]
                    #[doc = ""]
                    #[doc = "- `reporter`: The reporter who withdrew a stake."]
                    #[doc = "- `amount`: The total amount withdrawn."]
                    #[doc = "- `address`: The corresponding address on the controlling chain."]
                    report_stake_withdrawn {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 16)]
                    #[doc = "Reports a slashing of a reporter."]
                    #[doc = ""]
                    #[doc = "- `reporter`: The address of the slashed reporter."]
                    #[doc = "- `amount`: The slashed amount."]
                    report_slash {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 17)]
                    #[doc = "Reports the tally of a vote."]
                    #[doc = ""]
                    #[doc = "- `dispute_id`: The identifier of the dispute."]
                    #[doc = "- `result`: The outcome of the vote, as determined by governance."]
                    report_vote_tallied {
                        dispute_id: ::subxt::utils::H256,
                        result: runtime_types::tellor::types::governance::VoteResult,
                    },
                    #[codec(index = 18)]
                    #[doc = "Reports the execution of a vote."]
                    #[doc = ""]
                    #[doc = "- `dispute_id`: The identifier of the dispute."]
                    report_vote_executed { dispute_id: ::subxt::utils::H256 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Claim buffer time has not passed."]
                    ClaimBufferNotPassed,
                    #[codec(index = 1)]
                    #[doc = "Timestamp too old to claim tip."]
                    ClaimPeriodExpired,
                    #[codec(index = 2)]
                    #[doc = "Feed must not be set up already."]
                    FeedAlreadyExists,
                    #[codec(index = 3)]
                    #[doc = "No funds available for this feed or insufficient balance for all submitted timestamps."]
                    InsufficientFeedBalance,
                    #[codec(index = 4)]
                    #[doc = "Amount must be greater than zero."]
                    InvalidAmount,
                    #[codec(index = 5)]
                    #[doc = "Claimer must be the reporter."]
                    InvalidClaimer,
                    #[codec(index = 6)]
                    #[doc = "Feed not set up."]
                    InvalidFeed,
                    #[codec(index = 7)]
                    InvalidIndex,
                    #[codec(index = 8)]
                    #[doc = "Interval must be greater than zero."]
                    InvalidInterval,
                    #[codec(index = 9)]
                    #[doc = "Reward must be greater than zero."]
                    InvalidReward,
                    #[codec(index = 10)]
                    #[doc = "Query identifier must be a hash of bytes data."]
                    InvalidQueryId,
                    #[codec(index = 11)]
                    #[doc = "No value exists at timestamp."]
                    InvalidTimestamp,
                    #[codec(index = 12)]
                    #[doc = "Window must be less than interval length."]
                    InvalidWindow,
                    #[codec(index = 13)]
                    #[doc = "No tips submitted for this query identifier."]
                    NoTipsSubmitted,
                    #[codec(index = 14)]
                    #[doc = "Price threshold not met."]
                    PriceThresholdNotMet,
                    #[codec(index = 15)]
                    #[doc = "Timestamp not eligible for tip."]
                    TimestampIneligibleForTip,
                    #[codec(index = 16)]
                    #[doc = "Tip already claimed."]
                    TipAlreadyClaimed,
                    #[codec(index = 17)]
                    #[doc = "Tip earned by previous submission."]
                    TipAlreadyEarned,
                    #[codec(index = 18)]
                    #[doc = "An error occurred converting an oracle value."]
                    ValueConversionError,
                    #[codec(index = 19)]
                    #[doc = "Value disputed."]
                    ValueDisputed,
                    #[codec(index = 20)]
                    InvalidAddress,
                    #[codec(index = 21)]
                    #[doc = "Balance must be greater than stake amount."]
                    InsufficientStake,
                    #[codec(index = 22)]
                    #[doc = "Nonce must match the timestamp index."]
                    InvalidNonce,
                    #[codec(index = 23)]
                    #[doc = "Invalid token price."]
                    InvalidPrice,
                    #[codec(index = 24)]
                    #[doc = "Invalid staking token price."]
                    InvalidStakingTokenPrice,
                    #[codec(index = 25)]
                    #[doc = "Value must be submitted."]
                    InvalidValue,
                    #[codec(index = 26)]
                    #[doc = "The maximum sequential disputed timestamps has been reached."]
                    MaxDisputedTimeSeriesReached,
                    #[codec(index = 27)]
                    #[doc = "Reporter not locked for withdrawal."]
                    NoWithdrawalRequested,
                    #[codec(index = 28)]
                    #[doc = "Still in reporter time lock, please wait!"]
                    ReporterTimeLocked,
                    #[codec(index = 29)]
                    #[doc = "Timestamp already reported."]
                    TimestampAlreadyReported,
                    #[codec(index = 30)]
                    #[doc = "Withdrawal period didn't pass."]
                    WithdrawalPeriodPending,
                    #[codec(index = 31)]
                    #[doc = "Voter has already voted."]
                    AlreadyVoted,
                    #[codec(index = 32)]
                    #[doc = "Dispute must be started within reporting lock time."]
                    DisputeReportingPeriodExpired,
                    #[codec(index = 33)]
                    #[doc = "New dispute round must be started within a day."]
                    DisputeRoundReportingPeriodExpired,
                    #[codec(index = 34)]
                    #[doc = "Dispute does not exist."]
                    InvalidDispute,
                    #[codec(index = 35)]
                    #[doc = "Vote does not exist."]
                    InvalidVote,
                    #[codec(index = 36)]
                    #[doc = "The maximum number of vote rounds has been reached."]
                    MaxVoteRoundsReached,
                    #[codec(index = 37)]
                    #[doc = "Dispute initiator is not a reporter."]
                    NotReporter,
                    #[codec(index = 38)]
                    #[doc = "No value exists at given timestamp."]
                    NoValueExists,
                    #[codec(index = 39)]
                    #[doc = "One day has to pass after tally to allow for disputes."]
                    TallyDisputePeriodActive,
                    #[codec(index = 40)]
                    #[doc = "Vote has already been executed."]
                    VoteAlreadyExecuted,
                    #[codec(index = 41)]
                    #[doc = "Vote has already been sent."]
                    VoteAlreadySent,
                    #[codec(index = 42)]
                    #[doc = "Vote has already been tallied."]
                    VoteAlreadyTallied,
                    #[codec(index = 43)]
                    #[doc = "Vote must be tallied."]
                    VoteNotTallied,
                    #[codec(index = 44)]
                    #[doc = "Time for voting has not elapsed."]
                    VotingPeriodActive,
                    #[codec(index = 45)]
                    FeesNotMet,
                    #[codec(index = 46)]
                    JunctionOverflow,
                    #[codec(index = 47)]
                    MaxEthereumXcmInputSizeExceeded,
                    #[codec(index = 48)]
                    SendFailure,
                    #[codec(index = 49)]
                    Unreachable,
                    #[codec(index = 50)]
                    WeighingFailure,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Emitted when a data feed is funded."]
                    DataFeedFunded {
                        query_id: ::subxt::utils::H256,
                        feed_id: ::subxt::utils::H256,
                        amount: ::core::primitive::u128,
                        feed_funder: ::subxt::utils::AccountId32,
                        feed_details:
                            runtime_types::tellor::types::autopay::Feed<::core::primitive::u128>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Emitted when a data feed is set up."]
                    NewDataFeed {
                        query_id: ::subxt::utils::H256,
                        feed_id: ::subxt::utils::H256,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        feed_creator: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Emitted when a onetime tip is claimed."]
                    OneTimeTipClaimed {
                        query_id: ::subxt::utils::H256,
                        amount: ::core::primitive::u128,
                        reporter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Emitted when a tip is added."]
                    TipAdded {
                        query_id: ::subxt::utils::H256,
                        amount: ::core::primitive::u128,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        tipper: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Emitted when a tip is claimed."]
                    TipClaimed {
                        feed_id: ::subxt::utils::H256,
                        query_id: ::subxt::utils::H256,
                        amount: ::core::primitive::u128,
                        reporter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Emitted when a new value is submitted."]
                    NewReport {
                        query_id: ::subxt::utils::H256,
                        time: ::core::primitive::u64,
                        value: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        nonce: ::core::primitive::u32,
                        query_data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        reporter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Emitted when the stake amount has changed."]
                    NewStakeAmount {
                        amount: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 7)]
                    #[doc = "Emitted when a new staker is reported."]
                    NewStakerReported {
                        staker: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                        address: ::subxt::utils::H160,
                    },
                    #[codec(index = 8)]
                    #[doc = "Emitted when a stake slash is reported."]
                    SlashReported {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 9)]
                    #[doc = "Emitted when a stake withdrawal is reported."]
                    StakeWithdrawnReported { staker: ::subxt::utils::AccountId32 },
                    #[codec(index = 10)]
                    #[doc = "Emitted when a stake withdrawal request is reported."]
                    StakeWithdrawRequestReported {
                        reporter: ::subxt::utils::AccountId32,
                        amount: runtime_types::primitive_types::U256,
                        address: ::subxt::utils::H160,
                    },
                    #[codec(index = 11)]
                    #[doc = "Emitted when staking rewards are added."]
                    StakingRewardsAdded {
                        source: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Emitted when a value is removed (via governance)."]
                    ValueRemoved {
                        query_id: ::subxt::utils::H256,
                        timestamp: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    #[doc = "Emitted when a new dispute is opened."]
                    NewDispute {
                        dispute_id: ::subxt::utils::H256,
                        query_id: ::subxt::utils::H256,
                        timestamp: ::core::primitive::u64,
                        reporter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 14)]
                    #[doc = "Emitted when a new dispute is sent to the governance controller contract."]
                    NewDisputeSent {
                        para_id: ::core::primitive::u32,
                        contract_address: ::subxt::utils::H160,
                    },
                    #[codec(index = 15)]
                    #[doc = "Emitted when the dispute fee has changed."]
                    NewDisputeFee {
                        dispute_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 16)]
                    #[doc = "Emitted when an address casts their vote."]
                    Voted {
                        dispute_id: ::subxt::utils::H256,
                        supports: ::core::option::Option<::core::primitive::bool>,
                        voter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 17)]
                    #[doc = "Emitted when a vote is sent to the governance controller contract for tallying."]
                    VoteSent {
                        para_id: ::core::primitive::u32,
                        contract_address: ::subxt::utils::H160,
                        dispute_id: ::subxt::utils::H256,
                        vote_round: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    #[doc = "Emitted when all casting for a vote is tallied."]
                    VoteTallied {
                        dispute_id: ::subxt::utils::H256,
                        result: runtime_types::tellor::types::governance::VoteResult,
                        initiator: ::subxt::utils::AccountId32,
                        reporter: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    #[doc = "Emitted when a vote is executed."]
                    VoteExecuted {
                        dispute_id: ::subxt::utils::H256,
                        result: runtime_types::tellor::types::governance::VoteResult,
                    },
                    #[codec(index = 20)]
                    #[doc = "Emitted when query data is stored."]
                    QueryDataStored { query_id: ::subxt::utils::H256 },
                    #[codec(index = 21)]
                    #[doc = "Emitted when registration is sent to the controller contracts."]
                    RegistrationSent {
                        para_id: ::core::primitive::u32,
                        contract_address: ::subxt::utils::H160,
                        weights: runtime_types::tellor::types::Weights,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod autopay {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Feed<_0> {
                        pub reward: _0,
                        pub balance: _0,
                        pub start_time: ::core::primitive::u64,
                        pub interval: ::core::primitive::u64,
                        pub window: ::core::primitive::u64,
                        pub price_threshold: ::core::primitive::u16,
                        pub reward_increase_per_second: _0,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Tip<_0> {
                        pub amount: _0,
                        pub timestamp: ::core::primitive::u64,
                        pub cumulative_tips: _0,
                    }
                }
                pub mod governance {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Dispute<_0, _1> {
                        pub query_id: ::subxt::utils::H256,
                        pub timestamp: ::core::primitive::u64,
                        pub value: _1,
                        pub disputed_reporter: _0,
                        pub slashed_amount: runtime_types::primitive_types::U256,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Tally<_0> {
                        pub does_support: _0,
                        pub against: _0,
                        pub invalid_query: _0,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Vote<_0, _1, _2> {
                        pub identifier: ::subxt::utils::H256,
                        pub vote_round: ::core::primitive::u8,
                        pub start_date: ::core::primitive::u64,
                        pub block_number: _2,
                        pub fee: _1,
                        pub tally_date: ::core::primitive::u64,
                        pub users: runtime_types::tellor::types::governance::Tally<_1>,
                        pub reporters: runtime_types::tellor::types::governance::Tally<_1>,
                        pub sent: ::core::primitive::bool,
                        pub executed: ::core::primitive::bool,
                        pub result: ::core::option::Option<
                            runtime_types::tellor::types::governance::VoteResult,
                        >,
                        pub initiator: _0,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum VoteResult {
                        #[codec(index = 0)]
                        Failed,
                        #[codec(index = 1)]
                        Passed,
                        #[codec(index = 2)]
                        Invalid,
                    }
                }
                pub mod oracle {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Report<_0, _1> {
                        pub index: ::core::primitive::u32,
                        pub block_number: _1,
                        pub reporter: _0,
                        pub is_disputed: ::core::primitive::bool,
                        pub previous: ::core::option::Option<::core::primitive::u64>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct StakeInfo<_0> {
                        pub address: ::subxt::utils::H160,
                        pub start_date: ::core::primitive::u64,
                        pub staked_balance: runtime_types::primitive_types::U256,
                        pub locked_balance: runtime_types::primitive_types::U256,
                        pub reward_debt: _0,
                        pub reporter_last_timestamp: ::core::primitive::u64,
                        pub reports_submitted: ::core::primitive::u32,
                        pub start_vote_count: ::core::primitive::u64,
                        pub start_vote_tally: ::core::primitive::u32,
                        pub staked: ::core::primitive::bool,
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weights {
                    pub report_stake_deposited: ::core::primitive::u64,
                    pub report_staking_withdraw_request: ::core::primitive::u64,
                    pub report_stake_withdrawn: ::core::primitive::u64,
                    pub report_vote_tallied: ::core::primitive::u64,
                    pub report_vote_executed: ::core::primitive::u64,
                    pub report_slash: ::core::primitive::u64,
                }
            }
            pub mod xcm {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ContractLocation {
                    pub para_id: ::core::primitive::u32,
                    pub address: [::core::primitive::u8; 20usize],
                    pub network:
                        ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                }
            }
        }
        pub mod using_tellor {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A sample dispatchable that takes a query identifier as a parameter, writes it to"]
                    #[doc = "storage and emits an event. This function must be dispatched by the configured origin."]
                    configure { query_id: ::subxt::utils::H256 },
                    #[codec(index = 1)]
                    #[doc = "A sample dispatchable that takes a single value as a parameter, derives some new value"]
                    #[doc = "and then writes that derived value to storage and emits an event. This function must be"]
                    #[doc = "dispatched by a signed extrinsic."]
                    do_something {
                        value: runtime_types::primitive_types::U256,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The pallet has not been configured."]
                    NotConfigured,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The pallet was configured with a query identifier. [queryId]"]
                    Configured { query_id: ::subxt::utils::H256 },
                    #[codec(index = 1)]
                    #[doc = "A value was stored. [value, who]"]
                    ValueStored {
                        value: runtime_types::primitive_types::U256,
                        who: ::subxt::utils::AccountId32,
                    },
                }
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DoubleEncoded2 {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: runtime_types::xcm::v2::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v2::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: runtime_types::xcm::v2::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey(
                            runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v2::BodyId,
                            part: runtime_types::xcm::v2::BodyPart,
                        },
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                        #[codec(index = 6)]
                        Blob(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v2::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
                    );
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v2::multiasset::AssetId,
                            fun: runtime_types::xcm::v2::multiasset::WildFungibility,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v2::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                            runtime_types::xcm::v2::junction::Junction,
                        ),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v2::multilocation::Junctions,
                    }
                }
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        MultiLocationFull,
                        #[codec(index = 5)]
                        MultiLocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        UnhandledXcmVersion,
                        #[codec(index = 23)]
                        WeightLimitReached(::core::primitive::u64),
                        #[codec(index = 24)]
                        Barrier,
                        #[codec(index = 25)]
                        WeightNotComputable,
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum BodyId {
                    #[codec(index = 0)]
                    Unit,
                    #[codec(index = 1)]
                    Named(
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    Index(#[codec(compact)] ::core::primitive::u32),
                    #[codec(index = 3)]
                    Executive,
                    #[codec(index = 4)]
                    Technical,
                    #[codec(index = 5)]
                    Legislative,
                    #[codec(index = 6)]
                    Judicial,
                    #[codec(index = 7)]
                    Defense,
                    #[codec(index = 8)]
                    Administration,
                    #[codec(index = 9)]
                    Treasury,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum BodyPart {
                    #[codec(index = 0)]
                    Voice,
                    #[codec(index = 1)]
                    Members {
                        #[codec(compact)]
                        count: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    Fraction {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    AtLeastProportion {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    MoreThanProportion {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v2::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v2::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v2::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Instruction2 {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v2::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded2,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v2::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v2::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm2),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm2),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum NetworkId {
                    #[codec(index = 0)]
                    Any,
                    #[codec(index = 1)]
                    Named(
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    Polkadot,
                    #[codec(index = 3)]
                    Kusama,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v2::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(#[codec(compact)] ::core::primitive::u64),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction2>);
            }
            pub mod v3 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Moniker([::core::primitive::u8; 4usize]),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                        #[codec(index = 7)]
                        Defense,
                        #[codec(index = 8)]
                        Administration,
                        #[codec(index = 9)]
                        Treasury,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey {
                            length: ::core::primitive::u8,
                            data: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v3::junction::BodyId,
                            part: runtime_types::xcm::v3::junction::BodyPart,
                        },
                        #[codec(index = 9)]
                        GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        ByGenesis([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        ByFork {
                            block_number: ::core::primitive::u64,
                            block_hash: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                        #[codec(index = 4)]
                        Westend,
                        #[codec(index = 5)]
                        Rococo,
                        #[codec(index = 6)]
                        Wococo,
                        #[codec(index = 7)]
                        Ethereum {
                            #[codec(compact)]
                            chain_id: ::core::primitive::u64,
                        },
                        #[codec(index = 8)]
                        BitcoinCore,
                        #[codec(index = 9)]
                        BitcoinCash,
                    }
                }
                pub mod junctions {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v3::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v3::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
                    );
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v3::multiasset::AssetId,
                            fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                        },
                        #[codec(index = 2)]
                        AllCounted(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        AllOfCounted {
                            id: runtime_types::xcm::v3::multiasset::AssetId,
                            fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v3::junctions::Junctions,
                    }
                }
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        LocationFull,
                        #[codec(index = 5)]
                        LocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        ExpectationFalse,
                        #[codec(index = 23)]
                        PalletNotFound,
                        #[codec(index = 24)]
                        NameMismatch,
                        #[codec(index = 25)]
                        VersionIncompatible,
                        #[codec(index = 26)]
                        HoldingWouldOverflow,
                        #[codec(index = 27)]
                        ExportError,
                        #[codec(index = 28)]
                        ReanchorFailed,
                        #[codec(index = 29)]
                        NoDeal,
                        #[codec(index = 30)]
                        FeesNotMet,
                        #[codec(index = 31)]
                        LockError,
                        #[codec(index = 32)]
                        NoPermission,
                        #[codec(index = 33)]
                        Unanchored,
                        #[codec(index = 34)]
                        NotDepositable,
                        #[codec(index = 35)]
                        UnhandledXcmVersion,
                        #[codec(index = 36)]
                        WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
                        #[codec(index = 37)]
                        Barrier,
                        #[codec(index = 38)]
                        WeightNotComputable,
                        #[codec(index = 39)]
                        ExceedsStackLimit,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(runtime_types::sp_weights::weight_v2::Weight),
                        #[codec(index = 1)]
                        Incomplete(
                            runtime_types::sp_weights::weight_v2::Weight,
                            runtime_types::xcm::v3::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v3::traits::Error),
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v3::Response,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                        querier: ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_kind: runtime_types::xcm::v2::OriginKind,
                        require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
                    #[codec(index = 12)]
                    ReportError(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        want: runtime_types::xcm::v3::multiasset::MultiAssets,
                        maximal: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 18)]
                    ReportHolding {
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v3::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v3::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v3::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                    #[codec(index = 28)]
                    BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 29)]
                    ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 30)]
                    ExpectOrigin(
                        ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 31)]
                    ExpectError(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 32)]
                    ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                    #[codec(index = 33)]
                    QueryPallet {
                        module_name: ::std::vec::Vec<::core::primitive::u8>,
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                    },
                    #[codec(index = 34)]
                    ExpectPallet {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        module_name: ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        crate_major: ::core::primitive::u32,
                        #[codec(compact)]
                        min_crate_minor: ::core::primitive::u32,
                    },
                    #[codec(index = 35)]
                    ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 36)]
                    ClearTransactStatus,
                    #[codec(index = 37)]
                    UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
                    #[codec(index = 38)]
                    ExportMessage {
                        network: runtime_types::xcm::v3::junction::NetworkId,
                        destination: runtime_types::xcm::v3::junctions::Junctions,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 39)]
                    LockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 40)]
                    UnlockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        target: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 41)]
                    NoteUnlockable {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        owner: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 42)]
                    RequestUnlock {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        locker: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 43)]
                    SetFeesMode {
                        jit_withdraw: ::core::primitive::bool,
                    },
                    #[codec(index = 44)]
                    SetTopic([::core::primitive::u8; 32usize]),
                    #[codec(index = 45)]
                    ClearTopic,
                    #[codec(index = 46)]
                    AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
                    #[codec(index = 47)]
                    UnpaidExecution {
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                        check_origin: ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Instruction2 {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v3::Response,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                        querier: ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_kind: runtime_types::xcm::v2::OriginKind,
                        require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded2,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
                    #[codec(index = 12)]
                    ReportError(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        want: runtime_types::xcm::v3::multiasset::MultiAssets,
                        maximal: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 18)]
                    ReportHolding {
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v3::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v3::Xcm2),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v3::Xcm2),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                    #[codec(index = 28)]
                    BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 29)]
                    ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 30)]
                    ExpectOrigin(
                        ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 31)]
                    ExpectError(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 32)]
                    ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                    #[codec(index = 33)]
                    QueryPallet {
                        module_name: ::std::vec::Vec<::core::primitive::u8>,
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                    },
                    #[codec(index = 34)]
                    ExpectPallet {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        module_name: ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        crate_major: ::core::primitive::u32,
                        #[codec(compact)]
                        min_crate_minor: ::core::primitive::u32,
                    },
                    #[codec(index = 35)]
                    ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 36)]
                    ClearTransactStatus,
                    #[codec(index = 37)]
                    UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
                    #[codec(index = 38)]
                    ExportMessage {
                        network: runtime_types::xcm::v3::junction::NetworkId,
                        destination: runtime_types::xcm::v3::junctions::Junctions,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 39)]
                    LockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 40)]
                    UnlockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        target: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 41)]
                    NoteUnlockable {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        owner: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 42)]
                    RequestUnlock {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        locker: runtime_types::xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 43)]
                    SetFeesMode {
                        jit_withdraw: ::core::primitive::bool,
                    },
                    #[codec(index = 44)]
                    SetTopic([::core::primitive::u8; 32usize]),
                    #[codec(index = 45)]
                    ClearTopic,
                    #[codec(index = 46)]
                    AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
                    #[codec(index = 47)]
                    UnpaidExecution {
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                        check_origin: ::core::option::Option<
                            runtime_types::xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum MaybeErrorCode {
                    #[codec(index = 0)]
                    Success,
                    #[codec(index = 1)]
                    Error(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    TruncatedError(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PalletInfo {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub major: ::core::primitive::u32,
                    #[codec(compact)]
                    pub minor: ::core::primitive::u32,
                    #[codec(compact)]
                    pub patch: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryResponseInfo {
                    pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    #[codec(compact)]
                    pub query_id: ::core::primitive::u64,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                    #[codec(index = 4)]
                    PalletsInfo(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::xcm::v3::PalletInfo,
                        >,
                    ),
                    #[codec(index = 5)]
                    DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(runtime_types::sp_weights::weight_v2::Weight),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedAssetId {
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::multiasset::AssetId),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedMultiAssets {
                #[codec(index = 1)]
                V2(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::multiasset::MultiAssets),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedMultiLocation {
                #[codec(index = 1)]
                V2(runtime_types::xcm::v2::multilocation::MultiLocation),
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::multilocation::MultiLocation),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedResponse {
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Response),
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::Response),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedXcm {
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm),
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::Xcm),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionedXcm2 {
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm2),
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::Xcm2),
            }
        }
    }
}
