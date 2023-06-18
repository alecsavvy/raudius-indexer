#[allow(dead_code, clippy::type_complexity, clippy::large_enum_variant)]
mod entity_manager {
    # [rustfmt :: skip] use ethcontract as ethcontract ;
    #[doc = "Generated by `ethcontract`"]
    #[derive(Clone)]
    pub struct Contract {
        methods: Methods,
    }
    impl Contract {
        #[doc = r" Retrieves the raw contract instance used to generate the type safe"]
        #[doc = r" API for this contract."]
        pub fn raw_contract() -> &'static self::ethcontract::Contract {
            use self::ethcontract::common::artifact::truffle::TruffleLoader;
            use self::ethcontract::private::lazy_static;
            use self::ethcontract::Contract;
            lazy_static! {
                pub static ref CONTRACT: Contract = {
                    # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"EntityManager\",\"abi\":[{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_verifierAddress\",\"type\":\"address\"},{\"name\":\"_networkId\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"manageEntity\",\"inputs\":[{\"name\":\"_userId\",\"type\":\"uint256\"},{\"name\":\"_entityType\",\"type\":\"string\"},{\"name\":\"_entityId\",\"type\":\"uint256\"},{\"name\":\"_action\",\"type\":\"string\"},{\"name\":\"_metadata\",\"type\":\"string\"},{\"name\":\"_nonce\",\"type\":\"bytes32\"},{\"name\":\"_subjectSig\",\"type\":\"bytes\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"manageIsVerified\",\"inputs\":[{\"name\":\"_userId\",\"type\":\"uint256\"},{\"name\":\"_isVerified\",\"type\":\"bool\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"usedSignatures\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"ManageEntity\",\"inputs\":[{\"name\":\"_userId\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"_signer\",\"type\":\"address\",\"indexed\":false},{\"name\":\"_entityType\",\"type\":\"string\",\"indexed\":false},{\"name\":\"_entityId\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"_metadata\",\"type\":\"string\",\"indexed\":false},{\"name\":\"_action\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ManageIsVerified\",\"inputs\":[{\"name\":\"_userId\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"_isVerified\",\"type\":\"bool\",\"indexed\":false}],\"anonymous\":false}],\"bytecode\":\"\",\"deployedBytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
                    contract
                };
            }
            &CONTRACT
        }
        #[doc = r" Creates a new contract instance with the specified `web3`"]
        #[doc = r" provider at the given `Address`."]
        #[doc = r""]
        #[doc = r" Note that this does not verify that a contract with a matching"]
        #[doc = r" `Abi` is actually deployed at the given address."]
        pub fn at<F, B, T>(
            web3: &self::ethcontract::web3::api::Web3<T>,
            address: self::ethcontract::Address,
        ) -> Self
        where
            F: std::future::Future<
                    Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
                > + Send
                + 'static,
            B: std::future::Future<
                    Output = Result<
                        Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
                        self::ethcontract::web3::Error,
                    >,
                > + Send
                + 'static,
            T: self::ethcontract::web3::Transport<Out = F>
                + self::ethcontract::web3::BatchTransport<Batch = B>
                + Send
                + Sync
                + 'static,
        {
            Contract::with_deployment_info(web3, address, None)
        }
        #[doc = r" Creates a new contract instance with the specified `web3` provider with"]
        #[doc = r" the given `Abi` at the given `Address` and an optional transaction hash."]
        #[doc = r" This hash is used to retrieve contract related information such as the"]
        #[doc = r" creation block (which is useful for fetching all historic events)."]
        #[doc = r""]
        #[doc = r" Note that this does not verify that a contract with a matching `Abi` is"]
        #[doc = r" actually deployed at the given address nor that the transaction hash,"]
        #[doc = r" when provided, is actually for this contract deployment."]
        pub fn with_deployment_info<F, B, T>(
            web3: &self::ethcontract::web3::api::Web3<T>,
            address: self::ethcontract::Address,
            deployment_information: Option<ethcontract::common::DeploymentInformation>,
        ) -> Self
        where
            F: std::future::Future<
                    Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
                > + Send
                + 'static,
            B: std::future::Future<
                    Output = Result<
                        Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
                        self::ethcontract::web3::Error,
                    >,
                > + Send
                + 'static,
            T: self::ethcontract::web3::Transport<Out = F>
                + self::ethcontract::web3::BatchTransport<Batch = B>
                + Send
                + Sync
                + 'static,
        {
            use self::ethcontract::transport::DynTransport;
            use self::ethcontract::web3::api::Web3;
            use self::ethcontract::Instance;
            let transport = DynTransport::new(web3.transport().clone());
            let web3 = Web3::new(transport);
            let abi = Self::raw_contract().abi.clone();
            let instance =
                Instance::with_deployment_info(web3, abi, address, deployment_information);
            Contract::from_raw(instance)
        }
        #[doc = r" Creates a contract from a raw instance."]
        fn from_raw(instance: self::ethcontract::dyns::DynInstance) -> Self {
            let methods = Methods { instance };
            Contract { methods }
        }
        #[doc = r" Returns the contract address being used by this instance."]
        pub fn address(&self) -> self::ethcontract::Address {
            self.raw_instance().address()
        }
        #[doc = r" Returns the deployment information of the contract"]
        #[doc = r" if it is known, `None` otherwise."]
        pub fn deployment_information(&self) -> Option<ethcontract::common::DeploymentInformation> {
            self.raw_instance().deployment_information()
        }
        #[doc = r" Returns a reference to the default method options used by this"]
        #[doc = r" contract."]
        pub fn defaults(&self) -> &self::ethcontract::contract::MethodDefaults {
            &self.raw_instance().defaults
        }
        #[doc = r" Returns a mutable reference to the default method options used"]
        #[doc = r" by this contract."]
        pub fn defaults_mut(&mut self) -> &mut self::ethcontract::contract::MethodDefaults {
            &mut self.raw_instance_mut().defaults
        }
        #[doc = r" Returns a reference to the raw runtime instance used by this"]
        #[doc = r" contract."]
        pub fn raw_instance(&self) -> &self::ethcontract::dyns::DynInstance {
            &self.methods.instance
        }
        #[doc = r" Returns a mutable reference to the raw runtime instance used by"]
        #[doc = r" this contract."]
        fn raw_instance_mut(&mut self) -> &mut self::ethcontract::dyns::DynInstance {
            &mut self.methods.instance
        }
    }
    impl std::fmt::Debug for Contract {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(EntityManager))
                .field(&self.address())
                .finish()
        }
    }
    impl Contract {
        #[doc = r" Returns an object that allows accessing typed method signatures."]
        pub fn signatures() -> Signatures {
            Signatures
        }
        #[doc = r" Retrieves a reference to type containing all the generated"]
        #[doc = r" contract methods. This can be used for methods where the name"]
        #[doc = r" would collide with a common method (like `at` or `deployed`)."]
        pub fn methods(&self) -> &Methods {
            &self.methods
        }
    }
    #[doc = r" Type containing signatures for all methods for generated contract type."]
    #[derive(Clone, Copy)]
    pub struct Signatures;
    impl Signatures {
        #[doc = "Returns signature for method `initialize(address,uint256)`."]
        #[allow(clippy::type_complexity)]
        pub fn initialize(
            &self,
        ) -> self::ethcontract::contract::Signature<
            (self::ethcontract::Address, self::ethcontract::U256),
            (),
        > {
            self::ethcontract::contract::Signature::new([205, 109, 198, 135])
        }
        #[doc = "Returns signature for method `manageEntity(uint256,string,uint256,string,string,bytes32,bytes)`."]
        #[allow(clippy::type_complexity)]
        pub fn manage_entity(
            &self,
        ) -> self::ethcontract::contract::Signature<
            (
                self::ethcontract::U256,
                String,
                self::ethcontract::U256,
                String,
                String,
                self::ethcontract::tokens::Bytes<[u8; 32]>,
                self::ethcontract::tokens::Bytes<Vec<u8>>,
            ),
            (),
        > {
            self::ethcontract::contract::Signature::new([214, 34, 199, 45])
        }
        #[doc = "Returns signature for method `manageIsVerified(uint256,bool)`."]
        #[allow(clippy::type_complexity)]
        pub fn manage_is_verified(
            &self,
        ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256, bool), ()> {
            self::ethcontract::contract::Signature::new([128, 9, 163, 175])
        }
        #[doc = "Returns signature for method `usedSignatures(bytes32):(bool)`."]
        #[allow(clippy::type_complexity)]
        pub fn used_signatures(
            &self,
        ) -> self::ethcontract::contract::Signature<
            (self::ethcontract::tokens::Bytes<[u8; 32]>,),
            bool,
        > {
            self::ethcontract::contract::Signature::new([249, 120, 253, 97])
        }
    }
    #[doc = r" Type containing all contract methods for generated contract type."]
    #[derive(Clone)]
    pub struct Methods {
        instance: self::ethcontract::dyns::DynInstance,
    }
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    impl Methods {
        #[doc = "Generated by `ethcontract`"]
        pub fn initialize(
            &self,
            verifier_address: self::ethcontract::Address,
            network_id: self::ethcontract::U256,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([205, 109, 198, 135], (verifier_address, network_id))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn manage_entity(
            &self,
            user_id: self::ethcontract::U256,
            entity_type: String,
            entity_id: self::ethcontract::U256,
            action: String,
            metadata: String,
            nonce: self::ethcontract::tokens::Bytes<[u8; 32]>,
            subject_sig: self::ethcontract::tokens::Bytes<Vec<u8>>,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method(
                    [214, 34, 199, 45],
                    (
                        user_id,
                        entity_type,
                        entity_id,
                        action,
                        metadata,
                        nonce,
                        subject_sig,
                    ),
                )
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn manage_is_verified(
            &self,
            user_id: self::ethcontract::U256,
            is_verified: bool,
        ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
            self.instance
                .method([128, 9, 163, 175], (user_id, is_verified))
                .expect("generated call")
        }
        #[doc = "Generated by `ethcontract`"]
        pub fn used_signatures(
            &self,
            p0: self::ethcontract::tokens::Bytes<[u8; 32]>,
        ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
            self.instance
                .view_method([249, 120, 253, 97], (p0,))
                .expect("generated call")
        }
    }
    impl std::ops::Deref for Contract {
        type Target = Methods;
        fn deref(&self) -> &Self::Target {
            &self.methods
        }
    }
    #[doc = r" Module containing all generated data models for this contract's"]
    #[doc = r" events."]
    pub mod event_data {
        use super::ethcontract;
        #[derive(
            Clone, Debug, Default, Eq, PartialEq, serde :: Serialize, serde :: Deserialize,
        )]
        pub struct ManageEntity {
            pub user_id: self::ethcontract::U256,
            pub signer: self::ethcontract::Address,
            pub entity_type: String,
            pub entity_id: self::ethcontract::U256,
            pub metadata: String,
            pub action: String,
        }
        impl ManageEntity {
            #[doc = r" Retrieves the signature for the event this data corresponds to."]
            #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
            #[doc = r" this event."]
            pub fn signature() -> self::ethcontract::H256 {
                self::ethcontract::H256([
                    119, 45, 98, 210, 28, 200, 70, 122, 20, 18, 127, 17, 171, 44, 9, 77, 50, 229,
                    181, 33, 67, 60, 239, 186, 90, 115, 18, 252, 70, 77, 136, 180,
                ])
            }
            #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
            #[doc = r" to. For this event the value should always be:"]
            #[doc = r""]
            #[doc = "`ManageEntity(uint256,address,string,uint256,string,string)`"]
            pub fn abi_signature() -> &'static str {
                "ManageEntity(uint256,address,string,uint256,string,string)"
            }
        }
        impl self::ethcontract::tokens::Tokenize for ManageEntity {
            fn from_token(
                token: self::ethcontract::common::abi::Token,
            ) -> Result<Self, self::ethcontract::tokens::Error> {
                let (user_id, signer, entity_type, entity_id, metadata, action) =
                    self::ethcontract::tokens::Tokenize::from_token(token)?;
                Ok(ManageEntity {
                    user_id,
                    signer,
                    entity_type,
                    entity_id,
                    metadata,
                    action,
                })
            }
            fn into_token(self) -> self::ethcontract::common::abi::Token {
                unimplemented!("events are only decoded, not encoded")
            }
        }
        #[derive(
            Clone, Debug, Default, Eq, PartialEq, serde :: Serialize, serde :: Deserialize,
        )]
        pub struct ManageIsVerified {
            pub user_id: self::ethcontract::U256,
            pub is_verified: bool,
        }
        impl ManageIsVerified {
            #[doc = r" Retrieves the signature for the event this data corresponds to."]
            #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
            #[doc = r" this event."]
            pub fn signature() -> self::ethcontract::H256 {
                self::ethcontract::H256([
                    251, 36, 137, 18, 183, 119, 184, 94, 83, 173, 207, 63, 234, 232, 204, 171, 252,
                    50, 27, 126, 109, 142, 238, 251, 15, 170, 174, 228, 175, 11, 103, 112,
                ])
            }
            #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
            #[doc = r" to. For this event the value should always be:"]
            #[doc = r""]
            #[doc = "`ManageIsVerified(uint256,bool)`"]
            pub fn abi_signature() -> &'static str {
                "ManageIsVerified(uint256,bool)"
            }
        }
        impl self::ethcontract::tokens::Tokenize for ManageIsVerified {
            fn from_token(
                token: self::ethcontract::common::abi::Token,
            ) -> Result<Self, self::ethcontract::tokens::Error> {
                let (user_id, is_verified) =
                    self::ethcontract::tokens::Tokenize::from_token(token)?;
                Ok(ManageIsVerified {
                    user_id,
                    is_verified,
                })
            }
            fn into_token(self) -> self::ethcontract::common::abi::Token {
                unimplemented!("events are only decoded, not encoded")
            }
        }
    }
    impl Contract {
        #[doc = r" Retrieves a handle to a type containing for creating event"]
        #[doc = r" streams for all the contract events."]
        pub fn events(&self) -> Events<'_> {
            Events {
                instance: self.raw_instance(),
            }
        }
    }
    pub struct Events<'a> {
        instance: &'a self::ethcontract::dyns::DynInstance,
    }
    impl Events<'_> {
        #[doc = r" Generated by `ethcontract`."]
        pub fn manage_entity(&self) -> self::event_builders::ManageEntityBuilder {
            self::event_builders::ManageEntityBuilder(
                self.instance
                    .event(self::ethcontract::H256([
                        119, 45, 98, 210, 28, 200, 70, 122, 20, 18, 127, 17, 171, 44, 9, 77, 50,
                        229, 181, 33, 67, 60, 239, 186, 90, 115, 18, 252, 70, 77, 136, 180,
                    ]))
                    .expect("generated event filter"),
            )
        }
        #[doc = r" Generated by `ethcontract`."]
        pub fn manage_is_verified(&self) -> self::event_builders::ManageIsVerifiedBuilder {
            self::event_builders::ManageIsVerifiedBuilder(
                self.instance
                    .event(self::ethcontract::H256([
                        251, 36, 137, 18, 183, 119, 184, 94, 83, 173, 207, 63, 234, 232, 204, 171,
                        252, 50, 27, 126, 109, 142, 238, 251, 15, 170, 174, 228, 175, 11, 103, 112,
                    ]))
                    .expect("generated event filter"),
            )
        }
    }
    #[doc = r" Module containing the generated event stream builders with type safe"]
    #[doc = r" filter methods for this contract's events."]
    pub mod event_builders {
        use super::ethcontract;
        use super::event_data;
        #[doc = "A builder for creating a filtered stream of `ManageEntity` events."]
        pub struct ManageEntityBuilder(
            #[doc = r" The inner event builder."]
            pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::ManageEntity>,
        );
        impl ManageEntityBuilder {
            #[doc = r" Sets the starting block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the latest block."]
            #[allow(clippy::wrong_self_convention)]
            pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).from_block(block);
                self
            }
            #[doc = r" Sets the last block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the streaming until the end of days."]
            #[allow(clippy::wrong_self_convention)]
            pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).to_block(block);
                self
            }
            #[doc = r" Limits the number of events that can be retrieved by this filter."]
            #[doc = r""]
            #[doc = r" Note that this parameter is non-standard."]
            pub fn limit(mut self, value: usize) -> Self {
                self.0 = (self.0).limit(value);
                self
            }
            #[doc = r" Sets the polling interval. This is used as the interval between"]
            #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
            pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
                self.0 = (self.0).poll_interval(value);
                self
            }
            #[doc = r" Returns a future that resolves with a collection of all existing"]
            #[doc = r" logs matching the builder parameters."]
            pub async fn query(
                self,
            ) -> std::result::Result<
                std::vec::Vec<self::ethcontract::Event<self::event_data::ManageEntity>>,
                self::ethcontract::errors::EventError,
            > {
                (self.0).query().await
            }
            #[doc = r" Creates an event stream from the current event builder."]
            pub fn stream(
                self,
            ) -> impl self::ethcontract::futures::stream::Stream<
                Item = std::result::Result<
                    self::ethcontract::StreamEvent<self::event_data::ManageEntity>,
                    self::ethcontract::errors::EventError,
                >,
            > {
                (self.0).stream()
            }
        }
        #[doc = "A builder for creating a filtered stream of `ManageIsVerified` events."]
        pub struct ManageIsVerifiedBuilder(
            #[doc = r" The inner event builder."]
            pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::ManageIsVerified>,
        );
        impl ManageIsVerifiedBuilder {
            #[doc = r" Sets the starting block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the latest block."]
            #[allow(clippy::wrong_self_convention)]
            pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).from_block(block);
                self
            }
            #[doc = r" Sets the last block from which to stream logs for."]
            #[doc = r""]
            #[doc = r" If left unset defaults to the streaming until the end of days."]
            #[allow(clippy::wrong_self_convention)]
            pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
                self.0 = (self.0).to_block(block);
                self
            }
            #[doc = r" Limits the number of events that can be retrieved by this filter."]
            #[doc = r""]
            #[doc = r" Note that this parameter is non-standard."]
            pub fn limit(mut self, value: usize) -> Self {
                self.0 = (self.0).limit(value);
                self
            }
            #[doc = r" Sets the polling interval. This is used as the interval between"]
            #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
            pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
                self.0 = (self.0).poll_interval(value);
                self
            }
            #[doc = r" Returns a future that resolves with a collection of all existing"]
            #[doc = r" logs matching the builder parameters."]
            pub async fn query(
                self,
            ) -> std::result::Result<
                std::vec::Vec<self::ethcontract::Event<self::event_data::ManageIsVerified>>,
                self::ethcontract::errors::EventError,
            > {
                (self.0).query().await
            }
            #[doc = r" Creates an event stream from the current event builder."]
            pub fn stream(
                self,
            ) -> impl self::ethcontract::futures::stream::Stream<
                Item = std::result::Result<
                    self::ethcontract::StreamEvent<self::event_data::ManageIsVerified>,
                    self::ethcontract::errors::EventError,
                >,
            > {
                (self.0).stream()
            }
        }
    }
    impl Contract {
        #[doc = r" Returns a log stream with all events."]
        pub fn all_events(&self) -> self::ethcontract::dyns::DynAllEventsBuilder<Event> {
            self::ethcontract::dyns::DynAllEventsBuilder::new(
                self.raw_instance().web3(),
                self.address(),
                self.deployment_information(),
            )
        }
    }
    #[doc = r" A contract event."]
    #[derive(Clone, Debug, Eq, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum Event {
        ManageEntity(self::event_data::ManageEntity),
        ManageIsVerified(self::event_data::ManageIsVerified),
    }
    impl self::ethcontract::contract::ParseLog for Event {
        fn parse_log(
            log: self::ethcontract::RawLog,
        ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
            let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([119 , 45 , 98 , 210 , 28 , 200 , 70 , 122 , 20 , 18 , 127 , 17 , 171 , 44 , 9 , 77 , 50 , 229 , 181 , 33 , 67 , 60 , 239 , 186 , 90 , 115 , 18 , 252 , 70 , 77 , 136 , 180]) => Ok (Event :: ManageEntity (log . clone () . decode (Contract :: raw_contract () . abi . event ("ManageEntity") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([251 , 36 , 137 , 18 , 183 , 119 , 184 , 94 , 83 , 173 , 207 , 63 , 234 , 232 , 204 , 171 , 252 , 50 , 27 , 126 , 109 , 142 , 238 , 251 , 15 , 170 , 174 , 228 , 175 , 11 , 103 , 112]) => Ok (Event :: ManageIsVerified (log . clone () . decode (Contract :: raw_contract () . abi . event ("ManageIsVerified") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
            if let Some(Ok(data)) = standard_event {
                return Ok(data);
            }
            Err(self::ethcontract::errors::ExecutionError::from(
                self::ethcontract::common::abi::Error::InvalidData,
            ))
        }
    }
}
use self::entity_manager::Contract as EntityManager;
