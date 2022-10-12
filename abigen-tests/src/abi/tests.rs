    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub mod functions {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct FixedArrayAddressArrayUint256ReturnsUint256String {
            pub param0: [Vec<u8>; 2usize],
            pub param1: Vec<ethabi::Uint>,
        }
        impl FixedArrayAddressArrayUint256ReturnsUint256String {
            const METHOD_ID: [u8; 4] = [136u8, 229u8, 164u8, 109u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                2usize,
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                inner
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec()
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Uint(inner.clone()))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(ethabi::Uint, String), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<(ethabi::Uint, String), String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<(ethabi::Uint, String)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function
        for FixedArrayAddressArrayUint256ReturnsUint256String {
            const NAME: &'static str = "FixedArrayAddressArrayUint256ReturnsUint256String";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(ethabi::Uint, String)>
        for FixedArrayAddressArrayUint256ReturnsUint256String {
            fn output(data: &[u8]) -> Result<(ethabi::Uint, String), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FixedArrayAddressArrayAddressReturnsUint256String {
            pub param0: [Vec<u8>; 2usize],
            pub param1: Vec<Vec<u8>>,
        }
        impl FixedArrayAddressArrayAddressReturnsUint256String {
            const METHOD_ID: [u8; 4] = [222u8, 196u8, 49u8, 26u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                2usize,
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                inner
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec()
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(ethabi::Uint, String), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<(ethabi::Uint, String), String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR),
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<(ethabi::Uint, String)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function
        for FixedArrayAddressArrayAddressReturnsUint256String {
            const NAME: &'static str = "fixedArrayAddressArrayAddressReturnsUint256String";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(ethabi::Uint, String)>
        for FixedArrayAddressArrayAddressReturnsUint256String {
            fn output(data: &[u8]) -> Result<(ethabi::Uint, String), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunAll {
            pub param0: Vec<u8>,
            pub param1: Vec<u8>,
            pub param2: [u8; 8usize],
            pub param3: [u8; 32usize],
            pub param4: EthBigInt,
            pub param5: ethabi::Uint,
            pub param6: bool,
            pub param7: String,
            pub param8: [Vec<u8>; 2usize],
            pub param9: Vec<Vec<u8>>,
        }
        impl FunAll {
            const METHOD_ID: [u8; 4] = [26u8, 249u8, 60u8, 49u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::FixedBytes(8usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Int(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bool,
                            ethabi::ParamType::String,
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                2usize,
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    param2: {
                        let mut result = [0u8; 8];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param3: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param4: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                    param5: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    param6: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    param7: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    param8: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                inner
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec()
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                    param9: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param0),
                        ),
                        ethabi::Token::Bytes(self.param1.clone()),
                        ethabi::Token::FixedBytes(self.param2.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.param3.as_ref().to_vec()),
                        {
                            let non_full_signed_bytes = self
                                .param4
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                        ethabi::Token::Uint(self.param5),
                        ethabi::Token::Bool(self.param6),
                        ethabi::Token::String(self.param7.clone()),
                        {
                            let v = self
                                .param8
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::FixedArray(v)
                        },
                        {
                            let v = self
                                .param9
                                .iter()
                                .map(|inner| ethabi::Token::Address(
                                    ethabi::Address::from_slice(&inner),
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunAll {
            const NAME: &'static str = "funAll";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunInt256 {
            pub param0: EthBigInt,
        }
        impl FunInt256 {
            const METHOD_ID: [u8; 4] = [247u8, 10u8, 247u8, 59u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Int(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let non_full_signed_bytes = self
                                .param0
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunInt256 {
            const NAME: &'static str = "funInt256";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunInt32 {
            pub param0: EthBigInt,
        }
        impl FunInt32 {
            const METHOD_ID: [u8; 4] = [215u8, 140u8, 170u8, 179u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Int(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let non_full_signed_bytes = self
                                .param0
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunInt32 {
            const NAME: &'static str = "funInt32";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunInt8 {
            pub param0: EthBigInt,
        }
        impl FunInt8 {
            const METHOD_ID: [u8; 4] = [48u8, 54u8, 230u8, 135u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Int(8usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let non_full_signed_bytes = self
                                .param0
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunInt8 {
            const NAME: &'static str = "funInt8";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunInt8Int32Int64Int256 {
            pub param0: EthBigInt,
            pub param1: EthBigInt,
            pub param2: EthBigInt,
            pub param3: EthBigInt,
        }
        impl FunInt8Int32Int64Int256 {
            const METHOD_ID: [u8; 4] = [219u8, 97u8, 126u8, 143u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Int(8usize),
                            ethabi::ParamType::Int(32usize),
                            ethabi::ParamType::Int(64usize),
                            ethabi::ParamType::Int(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                    param1: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                    param2: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                    param3: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let non_full_signed_bytes = self
                                .param0
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                        {
                            let non_full_signed_bytes = self
                                .param1
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                        {
                            let non_full_signed_bytes = self
                                .param2
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                        {
                            let non_full_signed_bytes = self
                                .param3
                                .get_big_int()
                                .to_signed_bytes_be();
                            let mut full_signed_bytes = [0xff as u8; 32];
                            non_full_signed_bytes
                                .into_iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);
                            ethabi::Token::Int(
                                ethabi::Int::from_big_endian(full_signed_bytes.as_ref()),
                            )
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunInt8Int32Int64Int256 {
            const NAME: &'static str = "funInt8Int32Int64Int256";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunReturnsString1 {}
        impl FunReturnsString1 {
            const METHOD_ID: [u8; 4] = [122u8, 55u8, 25u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FunReturnsString1 {
            const NAME: &'static str = "funReturnsString1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for FunReturnsString1 {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunReturnsString2 {}
        impl FunReturnsString2 {
            const METHOD_ID: [u8; 4] = [122u8, 55u8, 25u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FunReturnsString2 {
            const NAME: &'static str = "funReturnsString2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for FunReturnsString2 {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunReturnsStringString {}
        impl FunReturnsStringString {
            const METHOD_ID: [u8; 4] = [133u8, 3u8, 47u8, 124u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(String, String), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<(String, String), String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<(String, String)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FunReturnsStringString {
            const NAME: &'static str = "funReturnsStringString";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(String, String)>
        for FunReturnsStringString {
            fn output(data: &[u8]) -> Result<(String, String), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunString {
            pub first: String,
        }
        impl FunString {
            const METHOD_ID: [u8; 4] = [176u8, 217u8, 68u8, 25u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    first: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[ethabi::Token::String(self.first.clone())]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunString {
            const NAME: &'static str = "funString";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FunStringString {
            pub first: String,
            pub second: String,
        }
        impl FunStringString {
            const METHOD_ID: [u8; 4] = [16u8, 173u8, 235u8, 27u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::String],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    first: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.first.clone()),
                        ethabi::Token::String(self.second.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for FunStringString {
            const NAME: &'static str = "funStringString";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use substreams_ethereum::scalar::EthBigInt;
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventAddressIdxString {
            pub first: Vec<u8>,
            pub second: String,
        }
        impl EventAddressIdxString {
            const TOPIC_ID: [u8; 32] = [
                60u8,
                219u8,
                49u8,
                1u8,
                113u8,
                239u8,
                164u8,
                192u8,
                134u8,
                23u8,
                83u8,
                80u8,
                68u8,
                1u8,
                111u8,
                184u8,
                30u8,
                192u8,
                169u8,
                219u8,
                70u8,
                140u8,
                6u8,
                176u8,
                8u8,
                210u8,
                244u8,
                70u8,
                238u8,
                147u8,
                70u8,
                168u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventAddressIdxString {
            const NAME: &'static str = "EventAddressIdxString";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventAddressIdxStringUint256IdxBytes {
            pub first: Vec<u8>,
            pub second: String,
            pub third: ethabi::Uint,
            pub fourth: Vec<u8>,
        }
        impl EventAddressIdxStringUint256IdxBytes {
            const TOPIC_ID: [u8; 32] = [
                19u8,
                200u8,
                39u8,
                200u8,
                175u8,
                246u8,
                156u8,
                140u8,
                81u8,
                164u8,
                6u8,
                130u8,
                90u8,
                34u8,
                49u8,
                60u8,
                55u8,
                176u8,
                29u8,
                164u8,
                184u8,
                232u8,
                204u8,
                26u8,
                185u8,
                95u8,
                249u8,
                229u8,
                171u8,
                212u8,
                51u8,
                169u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Bytes],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'third' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    fourth: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventAddressIdxStringUint256IdxBytes {
            const NAME: &'static str = "EventAddressIdxStringUint256IdxBytes";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventAddressIdxUint256Uint256AddressIdx {
            pub first: Vec<u8>,
            pub second: ethabi::Uint,
            pub third: ethabi::Uint,
            pub fourth: Vec<u8>,
        }
        impl EventAddressIdxUint256Uint256AddressIdx {
            const TOPIC_ID: [u8; 32] = [
                186u8,
                209u8,
                95u8,
                244u8,
                23u8,
                243u8,
                118u8,
                49u8,
                29u8,
                220u8,
                111u8,
                61u8,
                204u8,
                72u8,
                76u8,
                184u8,
                184u8,
                147u8,
                202u8,
                121u8,
                27u8,
                217u8,
                39u8,
                222u8,
                98u8,
                106u8,
                220u8,
                155u8,
                216u8,
                247u8,
                217u8,
                125u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    fourth: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'fourth' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    third: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventAddressIdxUint256Uint256AddressIdx {
            const NAME: &'static str = "EventAddressIdxUint256Uint256AddressIdx";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventBytes20UintAddressIdx {
            pub first: [u8; 20usize],
            pub second: ethabi::Uint,
            pub third: Vec<u8>,
        }
        impl EventBytes20UintAddressIdx {
            const TOPIC_ID: [u8; 32] = [
                130u8,
                252u8,
                100u8,
                31u8,
                27u8,
                89u8,
                229u8,
                170u8,
                29u8,
                114u8,
                181u8,
                106u8,
                121u8,
                91u8,
                106u8,
                55u8,
                182u8,
                124u8,
                76u8,
                74u8,
                112u8,
                156u8,
                148u8,
                128u8,
                139u8,
                142u8,
                18u8,
                200u8,
                60u8,
                188u8,
                147u8,
                225u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(20usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    third: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'third' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    first: {
                        let mut result = [0u8; 20];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventBytes20UintAddressIdx {
            const NAME: &'static str = "EventBytes20UintAddressIdx";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventBytes32UintAddressIdx {
            pub first: [u8; 32usize],
            pub second: ethabi::Uint,
            pub third: Vec<u8>,
        }
        impl EventBytes32UintAddressIdx {
            const TOPIC_ID: [u8; 32] = [
                168u8,
                98u8,
                190u8,
                18u8,
                161u8,
                177u8,
                122u8,
                105u8,
                123u8,
                83u8,
                68u8,
                67u8,
                62u8,
                60u8,
                188u8,
                116u8,
                76u8,
                127u8,
                158u8,
                43u8,
                11u8,
                195u8,
                155u8,
                175u8,
                77u8,
                196u8,
                9u8,
                165u8,
                168u8,
                198u8,
                176u8,
                179u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    third: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'third' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    first: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventBytes32UintAddressIdx {
            const NAME: &'static str = "EventBytes32UintAddressIdx";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventInt256 {
            pub param0: EthBigInt,
        }
        impl EventInt256 {
            const TOPIC_ID: [u8; 32] = [
                160u8,
                188u8,
                122u8,
                85u8,
                50u8,
                156u8,
                194u8,
                159u8,
                153u8,
                11u8,
                124u8,
                72u8,
                217u8,
                244u8,
                98u8,
                78u8,
                76u8,
                12u8,
                53u8,
                235u8,
                149u8,
                90u8,
                238u8,
                53u8,
                143u8,
                123u8,
                22u8,
                68u8,
                29u8,
                185u8,
                237u8,
                69u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Int(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        EthBigInt::new(v.into())
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventInt256 {
            const NAME: &'static str = "EventInt256";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventInt256Idx {
            pub param0: EthBigInt,
        }
        impl EventInt256Idx {
            const TOPIC_ID: [u8; 32] = [
                8u8,
                77u8,
                106u8,
                162u8,
                162u8,
                72u8,
                65u8,
                251u8,
                164u8,
                190u8,
                44u8,
                39u8,
                243u8,
                190u8,
                3u8,
                225u8,
                156u8,
                49u8,
                34u8,
                101u8,
                253u8,
                62u8,
                106u8,
                115u8,
                233u8,
                44u8,
                229u8,
                140u8,
                32u8,
                42u8,
                71u8,
                39u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    param0: EthBigInt::new(
                        substreams::scalar::BigInt::from_signed_bytes_be(
                            log.topics[1usize].as_ref(),
                        ),
                    ),
                })
            }
        }
        impl substreams_ethereum::Event for EventInt256Idx {
            const NAME: &'static str = "EventInt256Idx";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventUBytes8UBytes16UBytes24UBytes32 {
            pub param0: [u8; 8usize],
            pub param1: [u8; 16usize],
            pub param2: [u8; 24usize],
            pub param3: [u8; 32usize],
        }
        impl EventUBytes8UBytes16UBytes24UBytes32 {
            const TOPIC_ID: [u8; 32] = [
                117u8,
                163u8,
                183u8,
                105u8,
                165u8,
                81u8,
                172u8,
                34u8,
                102u8,
                86u8,
                223u8,
                144u8,
                28u8,
                150u8,
                58u8,
                227u8,
                241u8,
                114u8,
                6u8,
                108u8,
                111u8,
                135u8,
                51u8,
                238u8,
                216u8,
                185u8,
                110u8,
                7u8,
                16u8,
                185u8,
                176u8,
                196u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(8usize),
                            ethabi::ParamType::FixedBytes(16usize),
                            ethabi::ParamType::FixedBytes(24usize),
                            ethabi::ParamType::FixedBytes(32usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut result = [0u8; 8];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param1: {
                        let mut result = [0u8; 16];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param2: {
                        let mut result = [0u8; 24];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param3: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventUBytes8UBytes16UBytes24UBytes32 {
            const NAME: &'static str = "EventUBytes8UBytes16UBytes24UBytes32";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventUFixedArraySubDynamic {
            pub param0: [Vec<u8>; 2usize],
        }
        impl EventUFixedArraySubDynamic {
            const TOPIC_ID: [u8; 32] = [
                214u8,
                61u8,
                69u8,
                230u8,
                205u8,
                245u8,
                228u8,
                18u8,
                225u8,
                196u8,
                5u8,
                126u8,
                186u8,
                108u8,
                181u8,
                247u8,
                102u8,
                97u8,
                138u8,
                231u8,
                48u8,
                109u8,
                12u8,
                175u8,
                109u8,
                171u8,
                126u8,
                55u8,
                97u8,
                182u8,
                140u8,
                216u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Bytes),
                                2usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| inner.into_bytes().expect(INTERNAL_ERR));
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventUFixedArraySubDynamic {
            const NAME: &'static str = "EventUFixedArraySubDynamic";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventUFixedArraySubFixed {
            pub param0: [Vec<u8>; 2usize],
        }
        impl EventUFixedArraySubFixed {
            const TOPIC_ID: [u8; 32] = [
                22u8,
                94u8,
                52u8,
                167u8,
                38u8,
                186u8,
                221u8,
                105u8,
                133u8,
                181u8,
                69u8,
                163u8,
                4u8,
                1u8,
                135u8,
                60u8,
                189u8,
                40u8,
                248u8,
                164u8,
                143u8,
                120u8,
                73u8,
                131u8,
                239u8,
                158u8,
                186u8,
                238u8,
                40u8,
                225u8,
                171u8,
                178u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedArray(
                                Box::new(ethabi::ParamType::Address),
                                2usize,
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let mut iter = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                inner
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec()
                            });
                        [
                            iter.next().expect(INTERNAL_ERR),
                            iter.next().expect(INTERNAL_ERR),
                        ]
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventUFixedArraySubFixed {
            const NAME: &'static str = "EventUFixedArraySubFixed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventWithOverloads1 {
            pub first: Vec<u8>,
        }
        impl EventWithOverloads1 {
            const TOPIC_ID: [u8; 32] = [
                160u8,
                232u8,
                134u8,
                105u8,
                115u8,
                6u8,
                86u8,
                80u8,
                35u8,
                114u8,
                67u8,
                175u8,
                26u8,
                126u8,
                149u8,
                252u8,
                6u8,
                106u8,
                26u8,
                73u8,
                223u8,
                243u8,
                135u8,
                140u8,
                168u8,
                239u8,
                202u8,
                85u8,
                141u8,
                39u8,
                28u8,
                21u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads1 {
            const NAME: &'static str = "EventWithOverloads1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventWithOverloads2 {
            pub second: String,
        }
        impl EventWithOverloads2 {
            const TOPIC_ID: [u8; 32] = [
                145u8,
                118u8,
                46u8,
                207u8,
                115u8,
                54u8,
                221u8,
                72u8,
                60u8,
                196u8,
                163u8,
                135u8,
                96u8,
                124u8,
                102u8,
                200u8,
                23u8,
                92u8,
                204u8,
                138u8,
                26u8,
                202u8,
                74u8,
                78u8,
                144u8,
                18u8,
                132u8,
                75u8,
                11u8,
                147u8,
                35u8,
                150u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    second: ethabi::decode(
                            &[ethabi::ParamType::String],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'second' from topic of type 'string': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads2 {
            const NAME: &'static str = "EventWithOverloads2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventWithOverloads3 {
            pub third: ethabi::Uint,
        }
        impl EventWithOverloads3 {
            const TOPIC_ID: [u8; 32] = [
                2u8,
                227u8,
                188u8,
                100u8,
                110u8,
                72u8,
                64u8,
                66u8,
                173u8,
                42u8,
                220u8,
                51u8,
                91u8,
                78u8,
                119u8,
                162u8,
                240u8,
                131u8,
                178u8,
                30u8,
                179u8,
                110u8,
                9u8,
                69u8,
                110u8,
                117u8,
                232u8,
                227u8,
                123u8,
                96u8,
                73u8,
                118u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    third: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'third' from topic of type 'uint256': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads3 {
            const NAME: &'static str = "EventWithOverloads3";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }