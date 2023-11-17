pub use value_incrementer::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod value_incrementer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getValue"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("incrementValue"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VALUEINCREMENTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x04\x8B8\x03\x80a\x04\x8B\x839\x81\x01`@\x81\x90Ra\0/\x91a\0]V[`\0a\0;\x82\x82a\x01\xB1V[PP`\0`\x01Ua\x02pV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\0pW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\0\x87W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\0\x9BW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\0\xADWa\0\xADa\0GV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\0\xD5Wa\0\xD5a\0GV[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\0\xEDW`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15a\x01\x0FW\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92a\0\xF2V[`\0\x86\x84\x83\x01\x01R\x80\x96PPPPPPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01ZWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xACW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x01\x89WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xA8W\x82\x81U`\x01\x01a\x01\x95V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xCAWa\x01\xCAa\0GV[a\x01\xDE\x81a\x01\xD8\x84Ta\x01&V[\x84a\x01`V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x02\x13W`\0\x84\x15a\x01\xFBWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x01\xA8V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x02BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x02#V[P\x85\x82\x10\x15a\x02`W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x02\x0C\x80a\x02\x7F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\0FW\x80c \x96RU\x14a\0dW\x80c;\xFD\x7F\xD3\x14a\0uW[`\0\x80\xFD[a\0Na\0\x7FV[`@Qa\0[\x91\x90a\x01&V[`@Q\x80\x91\x03\x90\xF3[`\x01T`@Q\x90\x81R` \x01a\0[V[a\0}a\x01\rV[\0[`\0\x80Ta\0\x8C\x90a\x01uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xB8\x90a\x01uV[\x80\x15a\x01\x05W\x80`\x1F\x10a\0\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x05V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\0\xE8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01\x80`\0\x82\x82Ta\x01\x1F\x91\x90a\x01\xAFV[\x90\x91UPPV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01TW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x018V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x89W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xA9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x01\xD0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x02\x91\xDF\xC3\xB9\xC1\xFC\xC1K\x08\x05\x1A\xDA7\x103(EM\xE6\"\xEE\x8BV\xF0F\xA4\x1E1b|\tdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static VALUEINCREMENTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\0FW\x80c \x96RU\x14a\0dW\x80c;\xFD\x7F\xD3\x14a\0uW[`\0\x80\xFD[a\0Na\0\x7FV[`@Qa\0[\x91\x90a\x01&V[`@Q\x80\x91\x03\x90\xF3[`\x01T`@Q\x90\x81R` \x01a\0[V[a\0}a\x01\rV[\0[`\0\x80Ta\0\x8C\x90a\x01uV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xB8\x90a\x01uV[\x80\x15a\x01\x05W\x80`\x1F\x10a\0\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x05V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\0\xE8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01\x80`\0\x82\x82Ta\x01\x1F\x91\x90a\x01\xAFV[\x90\x91UPPV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01TW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x018V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x89W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xA9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x01\xD0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x02\x91\xDF\xC3\xB9\xC1\xFC\xC1K\x08\x05\x1A\xDA7\x103(EM\xE6\"\xEE\x8BV\xF0F\xA4\x1E1b|\tdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static VALUEINCREMENTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ValueIncrementer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ValueIncrementer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ValueIncrementer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ValueIncrementer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ValueIncrementer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ValueIncrementer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ValueIncrementer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VALUEINCREMENTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                VALUEINCREMENTER_ABI.clone(),
                VALUEINCREMENTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getValue` (0x20965255) function
        pub fn get_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 150, 82, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementValue` (0x3bfd7fd3) function
        pub fn increment_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 253, 127, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ValueIncrementer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getValue` function with signature `getValue()` and selector `0x20965255`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getValue", abi = "getValue()")]
    pub struct GetValueCall;
    ///Container type for all input parameters for the `incrementValue` function with signature `incrementValue()` and selector `0x3bfd7fd3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "incrementValue", abi = "incrementValue()")]
    pub struct IncrementValueCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ValueIncrementerCalls {
        GetValue(GetValueCall),
        IncrementValue(IncrementValueCall),
        Name(NameCall),
    }
    impl ::ethers::core::abi::AbiDecode for ValueIncrementerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValue(decoded));
            }
            if let Ok(decoded) = <IncrementValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncrementValue(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ValueIncrementerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncrementValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ValueIncrementerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetValueCall> for ValueIncrementerCalls {
        fn from(value: GetValueCall) -> Self {
            Self::GetValue(value)
        }
    }
    impl ::core::convert::From<IncrementValueCall> for ValueIncrementerCalls {
        fn from(value: IncrementValueCall) -> Self {
            Self::IncrementValue(value)
        }
    }
    impl ::core::convert::From<NameCall> for ValueIncrementerCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    ///Container type for all return fields from the `getValue` function with signature `getValue()` and selector `0x20965255`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
}
