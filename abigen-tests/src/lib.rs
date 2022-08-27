mod abi;

#[cfg(test)]
mod tests {
    use crate::abi::tests;
    use ethabi::ethereum_types::U256;
    use pretty_assertions::assert_eq;
    use substreams::{hex, Hex};
    use substreams_ethereum::pb;

    #[test]
    fn it_decode_event_address_idx_string() {
        use tests::events::EventAddressIdxString as Event;

        // ethc tools encode --abi ./abigen-tests/abi/tests.json event 'EventAddressIdxString' "0xab07a50AD459B41Fe065f7BBAb866D5390e9f705"  "second string"
        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("3cdb310171efa4c08617535044016fb81ec0a9db468c06b008d2f446ee9346a8").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d7365636f6e6420737472696e6700000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                first: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                second: "second string".to_string(),
            }),
        );
    }

    #[test]
    fn it_decode_event_address_idx_string_uint256_idx_bytes() {
        use tests::events::EventAddressIdxStringUint256IdxBytes as Event;

        // ethc tools encode --abi ./abigen-tests/abi/tests.json event 'EventAddressIdxStringUint256IdxBytes' "0xab07a50AD459B41Fe065f7BBAb866D5390e9f705" "second string" "0x1000000000" "0xabdeff90"
        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("13c827c8aff69c8c51a406825a22313c37b01da4b8e8cc1ab95ff9e5abd433a9").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                hex!("0000000000000000000000000000000000000000000000000000001000000000").to_vec(),
            ],
            data: hex!("00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000d7365636f6e6420737472696e67000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004abdeff9000000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(
            event,
            Ok(Event {
                first: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                second: "second string".to_string(),
                third: U256::from_str_radix("0x1000000000", 16).unwrap(),
                fourth: hex!("abdeff90").to_vec(),
            }),
        );
    }

    #[test]
    fn it_decode_event_address_uint256_uint256_address_idx_bytes() {
        use tests::events::EventAddressIdxUint256Uint256AddressIdx as Event;

        // ethc tools encode --abi ./abigen-tests/abi/tests.json event 'EventAddressIdxUint256Uint256AddressIdx' "0xab07a50AD459B41Fe065f7BBAb866D5390e9f705"  "0x1000000000000000" "0x2000000000000000000" "0xcd91a50AD459B41Fe065f7BBAb866D5390e945fa"
        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("bad15ff417f376311ddc6f3dcc484cb8b893ca791bd927de626adc9bd8f7d97d").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                hex!("000000000000000000000000cd91a50ad459b41fe065f7bbab866d5390e945fa").to_vec(),
            ],
            data: hex!("00000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000002000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(
            event,
            Ok(Event {
                first: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                second: U256::from_str_radix("0x1000000000000000", 16).unwrap(),
                third: U256::from_str_radix("0x2000000000000000000", 16).unwrap(),
                fourth: hex!("cd91a50ad459b41fe065f7bbab866d5390e945fa").to_vec(),
            }),
        );
    }

    #[test]
    fn it_decode_fun_input_string() {
        use tests::functions::FunString as Function;

        // Generated through Solidity in https://github.com/streamingfast/eth-go/blob/4d23b26dcf6bbe91fad82aabf162fe1f2622f4b4/tests/src/test/Codec.sol#L24-L25
        let call = pb::eth::v2::Call {
            input: hex!("b0d94419000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000047465737400000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                first: "test".to_string(),
            }),
        );
    }

    #[test]
    fn it_encode_fun_input_string() {
        use tests::functions::FunString as Function;

        let fun = Function {
            first: "test".to_string(),
        };

        assert_eq!(fun.encode(), hex!("b0d94419000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000047465737400000000000000000000000000000000000000000000000000000000").to_vec());
    }

    #[test]
    fn it_decode_fun_output_string() {
        use tests::functions::FunReturnsString1 as Function;

        // Generated through Solidity in https://github.com/streamingfast/eth-go/blob/4d23b26dcf6bbe91fad82aabf162fe1f2622f4b4/tests/src/test/Codec.sol#L24-L25
        let call = pb::eth::v2::Call {
            input: hex!("7a3719f0").to_vec(),
            return_data: hex!("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000047465737400000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let output = Function::output_call(&call);
        assert_eq!(output, Ok("test".to_string()));
    }

    #[test]
    fn it_manual_decode_fun_output_string_string() {
        let decoded = ethabi::decode(
            &[ethabi::ParamType::String, ethabi::ParamType::String],
            hex!("000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000005746573743100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000057465737432000000000000000000000000000000000000000000000000000000").as_ref(),
        );

        assert_eq!(
            decoded.unwrap(),
            vec![
                ethabi::Token::String("test1".to_string()),
                ethabi::Token::String("test2".to_string())
            ]
        );
    }

    #[test]
    fn it_decode_fun_output_string_string() {
        use tests::functions::FunReturnsStringString as Function;

        // Generated through Solidity in https://github.com/streamingfast/eth-go/blob/4d23b26dcf6bbe91fad82aabf162fe1f2622f4b4/tests/src/test/Codec.sol#L24-L25
        let call = pb::eth::v2::Call {
            input: hex!("85032f7c").to_vec(),
            return_data: hex!("000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000005746573743100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000057465737432000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let output = Function::output_call(&call);
        assert_eq!(output, Ok(("test1".to_string(), "test2".to_string())));
    }

    #[test]
    fn it_encode_fun_input_fixed_array_address_array_uint256() {
        use tests::functions::FixedArrayAddressArrayUint256ReturnsUint256String as Function;

        let fun = Function {
            param0: [
                hex!("fffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                hex!("fffdb7377345371817f2b4dd490319755f5899eb").to_vec(),
            ],
            param1: vec![
                hex!("affdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                hex!("bffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                hex!("cffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
            ],
        };

        assert_eq!(fun.encode(), hex!("74ac01d1000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000fffdb7377345371817f2b4dd490319755f5899eb00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000003000000000000000000000000affdb7377345371817f2b4dd490319755f5899ec000000000000000000000000bffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000cffdb7377345371817f2b4dd490319755f5899ec").to_vec());
    }

    #[test]
    fn it_decode_fun_input_fixed_array_address_array_uint256() {
        use tests::functions::FixedArrayAddressArrayUint256ReturnsUint256String as Function;

        // Generated through Solidity in https://github.com/streamingfast/eth-go/blob/4d23b26dcf6bbe91fad82aabf162fe1f2622f4b4/tests/src/test/Codec.sol#L24-L25
        let call = pb::eth::v2::Call {
            input: hex!("74ac01d1000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000fffdb7377345371817f2b4dd490319755f5899eb00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000003000000000000000000000000affdb7377345371817f2b4dd490319755f5899ec000000000000000000000000bffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000cffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                param0: [
                    hex!("fffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                    hex!("fffdb7377345371817f2b4dd490319755f5899eb").to_vec()
                ],
                param1: vec![
                    hex!("affdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                    hex!("bffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                    hex!("cffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
                ],
            }),
        );
    }

    #[test]
    fn it_manual_decode_fun_input_fixed_array_address_array_uint256() {
        let decoded = ethabi::decode(
            &[
                ethabi::ParamType::FixedArray(
                    Box::new(ethabi::ParamType::Address),
                    2usize,
                ),
                ethabi::ParamType::Array(
                    Box::new(ethabi::ParamType::Address),
                ),
            ],
            hex!("000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000fffdb7377345371817f2b4dd490319755f5899eb00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000003000000000000000000000000affdb7377345371817f2b4dd490319755f5899ec000000000000000000000000bffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000cffdb7377345371817f2b4dd490319755f5899ec").as_ref(),
        );

        assert_eq!(
            decoded.unwrap(),
            vec![
                ethabi::Token::FixedArray(vec![
                    ethabi::Token::Address(hex!("fffdb7377345371817f2b4dd490319755f5899ec").into()),
                    ethabi::Token::Address(hex!("fffdb7377345371817f2b4dd490319755f5899eb").into())
                ]),
                ethabi::Token::Array(vec![
                    ethabi::Token::Address(hex!("affdb7377345371817f2b4dd490319755f5899ec").into()),
                    ethabi::Token::Address(hex!("bffdb7377345371817f2b4dd490319755f5899ec").into()),
                    ethabi::Token::Address(hex!("cffdb7377345371817f2b4dd490319755f5899ec").into()),
                ])
            ]
        );
    }

    #[test]
    fn it_encode_fun_input_int8() {
        use tests::functions::FunInt8 as Function;

        let fun = Function {
            param0: num_bigint::ToBigInt::to_bigint(&-127).unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("3036e687ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int32() {
        use tests::functions::FunInt32 as Function;

        let fun = Function {
            param0: num_bigint::ToBigInt::to_bigint(&-898877731).unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("d78caab3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36dd")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int256() {
        use tests::functions::FunInt256 as Function;

        let fun = Function {
            param0: num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("f70af73bfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int8_int32_int64_int256() {
        use tests::functions::FunInt8Int32Int64Int256 as Function;

        let fun = Function {
            param0: num_bigint::ToBigInt::to_bigint(&-127).unwrap(),
            param1: num_bigint::ToBigInt::to_bigint(&-898877731).unwrap(),
            param2: num_bigint::ToBigInt::to_bigint(&(-9809887317731 as i64)).unwrap(),
            param3: num_bigint::ToBigInt::to_bigint(&(-223372036854775808 as i64)).unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("db617e8fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36ddfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11dfffffffffffffffffffffffffffffffffffffffffffffffffce66c50e2840000")
                .to_vec()
        );
    }

    #[test]
    fn it_decode_fun_input_int8_int32_int64_int256() {
        use tests::functions::FunInt8Int32Int64Int256 as Function;

        let call = pb::eth::v2::Call {
            input: hex!("db617e8fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36ddfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11dfffffffffffffffffffffffffffffffffffffffffffffffffce66c50e2840000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                param0: num_bigint::ToBigInt::to_bigint(&-127).unwrap(),
                param1: num_bigint::ToBigInt::to_bigint(&-898877731).unwrap(),
                param2: num_bigint::ToBigInt::to_bigint(&(-9809887317731 as i64)).unwrap(),
                param3: num_bigint::ToBigInt::to_bigint(&(-223372036854775808 as i64)).unwrap(),
            }),
        );
    }

    #[test]
    fn it_manual_encode_num_bitint_signed_bytes() {
        let num = num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap();
        let as_hex = num.to_signed_bytes_be();

        let mut final_hex = [0xff as u8; 32];
        as_hex
            .into_iter()
            .rev()
            .enumerate()
            .for_each(|(i, byte)| final_hex[31 - i] = byte);

        assert_eq!(
            Hex(final_hex).to_string(),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d".to_string(),
        );
    }

    #[test]
    fn it_encode_fun_input_all() {
        use tests::functions::FunAll as Function;

        let fun = Function {
            param0: hex!("FffDB7377345371817F2b4dD490319755F5899eC").to_vec(),
            param1: hex!("b2").to_vec(),
            param2: hex!("cf36ac4f97dc10d9"),
            param3: hex!("cf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cd"),
            param4: num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),
            param5: 1827641804u64.into(),
            param6: true,
            param7: "test".to_string(),
            param8: [
                hex!("0000000000000000000000000000000000000000").to_vec(),
                hex!("0000000000000000000000000000000000000000").to_vec(),
            ],
            param9: vec![],
        };

        assert_eq!(fun.encode(), hex!("1af93c31000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec0000000000000000000000000000000000000000000000000000000000000160cf36ac4f97dc10d9000000000000000000000000000000000000000000000000cf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cdfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d000000000000000000000000000000000000000000000000000000006cef99cc000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000000000000000000000000000000000000000000001b200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000474657374000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec());
    }

    #[test]
    fn it_decode_fun_input_all() {
        use tests::functions::FunAll as Function;

        let call = pb::eth::v2::Call {
            input: hex!("1af93c31000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec0000000000000000000000000000000000000000000000000000000000000160cf36ac4f97dc10d9000000000000000000000000000000000000000000000000cf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cdfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d000000000000000000000000000000000000000000000000000000006cef99cc000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000000000000000000000000000000000000000000001b200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000474657374000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                param0: hex!("FffDB7377345371817F2b4dD490319755F5899eC").to_vec(),
                param1: hex!("b2").to_vec(),
                param2: hex!("cf36ac4f97dc10d9"),
                param3: hex!("cf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cd"),
                param4: num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),
                param5: 1827641804u64.into(),
                param6: true,
                param7: "test".to_string(),
                param8: [
                    hex!("0000000000000000000000000000000000000000").to_vec(),
                    hex!("0000000000000000000000000000000000000000").to_vec(),
                ],
                param9: vec![],
            }),
        );
    }
}
