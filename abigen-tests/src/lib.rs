mod abi;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::abi::tests;
    use pretty_assertions::assert_eq;
    use substreams::scalar::BigInt;
    use substreams::{hex, Hex};
    use substreams_ethereum::{pb, Event, Function, IndexedDynamicValue};

    #[macro_export]
    macro_rules! assert_bytes {
        ($left:expr, $right:expr$(,)?) => {{
            let (left, right) = (&$left, &$right);
            if !(*left == *right) {
                // Delegate to `assert_eq!` for diff
                assert_eq!(Hex(left).to_string(), Hex(right).to_string())
            }
        }};
    }

    #[test]
    fn it_decode_event_tuple() {
        use tests::events::EventUTupleAddress as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("adb25b4ab5d8f04dc5e8073124d207a0974cb9aecac69a6197dbd5cf8dce87d3").to_vec(),
            ],
            data: hex!("000000000000000000000000db0de9288cf0713de91371969efcc9969dd94117").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(
            event,
            Ok(Event {
                param0: (hex!("db0de9288cf0713de91371969efcc9969dd94117").to_vec(),),
            }),
        );
    }

    #[test]
    fn it_decode_event_tuple_bool() {
        use tests::events::EventUTupleBool as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("e46e0615228a85d593cefeae9bb5f9d1b6698858b635d549b40492afb258ff23").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000001").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(event, Ok(Event { param0: (true,) }),);
    }

    #[test]
    fn it_decode_event_int256_idx() {
        use substreams::scalar::BigInt;
        use tests::events::EventInt256Idx as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("084d6aa2a24841fba4be2c27f3be03e19c312265fd3e6a73e92ce58c202a4727").to_vec(),
                hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d").to_vec(),
            ],
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap()),
            }),
        );
    }

    #[test]
    fn it_renders_original_event_name_on_dedup() {
        use tests::events::EventWithOverloads1 as Event1;
        use tests::events::EventWithOverloads2 as Event2;
        use tests::events::EventWithOverloads3 as Event3;

        assert!(Event1::NAME == "EventWithOverloads");
        assert!(Event2::NAME == "EventWithOverloads");
        assert!(Event3::NAME == "EventWithOverloads");
    }

    #[test]
    fn it_decode_event_string_idx() {
        use tests::events::EventStringIdx as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("b6e8616369603c14126f2f830d422b55910c71d2bda5145db145e33db8cb51dd").to_vec(),
                hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d").to_vec(),
            ],
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: IndexedDynamicValue::<String>::new(
                    hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d")
                        .to_vec(),
                ),
            }),
        );
    }

    #[test]
    fn it_decode_event_array_bool() {
        use tests::events::EventUArrayBool as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("ee0cd0e55d575e4e32db712d239532b1104938ed2971f10d8b63e4aa4c17afb6").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: vec![true, false],
            }),
        );
    }

    #[test]
    fn it_decode_event_fixed_array_string() {
        use tests::events::EventUFixedArrayString as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("2f66d1a00558d55ced0f61b550ca490f9718523b5181b89c06b24ed7752e137c").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000005666972737400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067365636f6e640000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: ["first".to_string(), "second".to_string()],
            }),
        );
    }

    #[test]
    fn it_decode_event_int256() {
        use substreams::scalar::BigInt;
        use tests::events::EventInt256 as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("a0bc7a55329cc29f990b7c48d9f4624e4c0c35eb955aee358f7b16441db9ed45").to_vec(),
            ],
            data: hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),),
            }),
        );
    }

    #[test]
    fn it_decode_event_bytes8_bytes16_bytes24_bytes32() {
        use tests::events::EventUBytes8UBytes16UBytes24UBytes32 as Event;

        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("75a3b769a551ac226656df901c963ae3f172066c6f8733eed8b96e0710b9b0c4").to_vec(),
            ],
            data: hex!("c5abac1e99944b1d00000000000000000000000000000000000000000000000057dbc30b9acfebfb86bcc5f9e2fe3fa00000000000000000000000000000000004a81d8d5c3958b07e558ff8e58e1edf1871c14b34ecdc1c0000000000000000f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca184").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: hex!("c5abac1e99944b1d"),
                param1: hex!("57dbc30b9acfebfb86bcc5f9e2fe3fa0"),
                param2: hex!("04a81d8d5c3958b07e558ff8e58e1edf1871c14b34ecdc1c"),
                param3: hex!("f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca184"),
            }),
        );
    }

    #[test]
    fn it_decode_event_fixed_array_sub_fixed() {
        use tests::events::EventUFixedArraySubFixed as Event;

        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("165e34a726badd6985b545a30401873cbd28f8a48f784983ef9ebaee28e1abb2").to_vec(),
            ],
            data: hex!("000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000000000000000000000000ffffffffffffffffffffffffffffffffffffffff").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: [
                    hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").to_vec(),
                    hex!("ffffffffffffffffffffffffffffffffffffffff").to_vec()
                ],
            }),
        );
    }

    #[test]
    fn it_decode_event_fixed_array_sub_dynamic() {
        use tests::events::EventUFixedArraySubDynamic as Event;

        let log = pb::eth::v2::Log{
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("d63d45e6cdf5e412e1c4057eba6cb5f766618ae7306d0caf6dab7e3761b68cd8").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000005aaaaaaaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005ffffffffff000000000000000000000000000000000000000000000000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);
        assert_eq!(
            event,
            Ok(Event {
                param0: [hex!("aaaaaaaaaa").to_vec(), hex!("ffffffffff").to_vec()],
            }),
        );
    }

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
    fn it_decode_event_bytes_32_uint_address_idx() {
        use num_traits::Num;
        use tests::events::EventBytes32UintAddressIdx as Event;

        // ethc tools encode --abi ./abigen-tests/abi/tests.json event 'EventBytes32UintAddressIdx' "0x245414afb5b0fd4cd1285d0ff67e7d40218df67e1426c7e37c835cf2b5090cd2" "0x1000000000" "0xab07a50AD459B41Fe065f7BBAb866D5390e9f705"
        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("a862be12a1b17a697b5344433e3cbc744c7f9e2b0bc39baf4dc409a5a8c6b0b3").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            ],
            data: hex!("245414afb5b0fd4cd1285d0ff67e7d40218df67e1426c7e37c835cf2b5090cd20000000000000000000000000000000000000000000000000000001000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(
            event,
            Ok(Event {
                first: hex!("245414afb5b0fd4cd1285d0ff67e7d40218df67e1426c7e37c835cf2b5090cd2"),
                second: BigInt::from(num_bigint::BigInt::from_str_radix("1000000000", 16).unwrap()),
                third: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec()
            }),
        );
    }

    #[test]
    fn it_decode_event_bytes_20_uint_address_idx() {
        use num_traits::Num;
        use tests::events::EventBytes20UintAddressIdx as Event;

        // ethc tools encode --abi ./abigen-tests/abi/tests.json event 'EventBytes20UintAddressIdx' "0xab07a50ad459b41fe065f7bbab866d5390e9f705" "0x1000000000" "0xab07a50AD459B41Fe065f7BBAb866D5390e9f705"
        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("82fc641f1b59e5aa1d72b56a795b6a37b67c4c4a709c94808b8e12c83cbc93e1").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            ],
            data: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f7050000000000000000000000000000000000000000000000000000000000000000000000000000001000000000").to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log);

        assert_eq!(
            event,
            Ok(Event {
                first: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705"),
                second: BigInt::from(num_bigint::BigInt::from_str_radix("1000000000", 16).unwrap()),
                third: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec()
            }),
        );
    }

    #[test]
    fn it_decode_event_address_idx_string_uint256_idx_bytes() {
        use num_traits::Num;
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
                third: BigInt::from(num_bigint::BigInt::from_str_radix("1000000000", 16).unwrap()),
                fourth: hex!("abdeff90").to_vec(),
            }),
        );
    }

    #[test]
    fn it_decode_event_address_uint256_uint256_address_idx_bytes() {
        use num_traits::Num;
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
                second: BigInt::from(
                    num_bigint::BigInt::from_str_radix("1000000000000000", 16).unwrap()
                ),
                third: BigInt::from(
                    num_bigint::BigInt::from_str_radix("2000000000000000000", 16).unwrap()
                ),
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
    fn it_encode_fun_input_fixed_array_address_array_address_returns_uint256_string() {
        use tests::functions::FixedArrayAddressArrayAddressReturnsUint256String as Function;

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

        assert_eq!(fun.encode(), hex!("dec4311a000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000fffdb7377345371817f2b4dd490319755f5899eb00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000003000000000000000000000000affdb7377345371817f2b4dd490319755f5899ec000000000000000000000000bffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000cffdb7377345371817f2b4dd490319755f5899ec").to_vec());
    }

    #[test]
    fn it_encode_fun_uint256() {
        use tests::functions::FunUint256 as Function;

        // Under Int256
        let fun = Function {
            param0: BigInt::from_str("10").unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("2b15216f000000000000000000000000000000000000000000000000000000000000000a")
                .to_vec()
        );

        // Over Int256
        let fun = Function {
            param0: BigInt::from_str(
                "115792089237316195423570985008687907837957278154198333183605726673483560124417",
            )
            .unwrap(),
        };

        assert_eq!(
            fun.encode(),
            hex!("2b15216fffffffffffffffffffffffffffffffd300000000000000000000000000000001")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_dynamic_bool_array() {
        use tests::functions::FunDynamicBoolArray as Function;

        let fun = Function {
            param0: vec![true, false],
        };

        assert_eq!(
            fun.encode(),
            hex!("b0e615780000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000")
                .to_vec()
        );
    }

    #[test]
    fn it_decode_fun_input_fixed_array_address_array_address_returns_uint256_string() {
        use tests::functions::FixedArrayAddressArrayAddressReturnsUint256String as Function;

        // Generated through Solidity in https://github.com/streamingfast/eth-go/blob/4d23b26dcf6bbe91fad82aabf162fe1f2622f4b4/tests/src/test/Codec.sol#L24-L25
        let call = pb::eth::v2::Call {
            input: hex!("dec4311a000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000fffdb7377345371817f2b4dd490319755f5899eb00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000003000000000000000000000000affdb7377345371817f2b4dd490319755f5899ec000000000000000000000000bffdb7377345371817f2b4dd490319755f5899ec000000000000000000000000cffdb7377345371817f2b4dd490319755f5899ec").to_vec(),
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
        use substreams::scalar::BigInt;
        use tests::functions::FunInt8 as Function;

        let fun = Function {
            param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-127).unwrap()),
        };

        assert_eq!(
            fun.encode(),
            hex!("3036e687ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int32() {
        use substreams::scalar::BigInt;
        use tests::functions::FunInt32 as Function;

        let fun = Function {
            param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-898877731).unwrap()),
        };

        assert_eq!(
            fun.encode(),
            hex!("d78caab3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36dd")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int256() {
        use substreams::scalar::BigInt;
        use tests::functions::FunInt256 as Function;

        let fun = Function {
            param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap()),
        };

        assert_eq!(
            fun.encode(),
            hex!("f70af73bfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11d")
                .to_vec()
        );
    }

    #[test]
    fn it_encode_fun_input_int8_int32_int64_int256() {
        use substreams::scalar::BigInt;
        use tests::functions::FunInt8Int32Int64Int256 as Function;

        let fun = Function {
            param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-127).unwrap()),
            param1: BigInt::from(num_bigint::ToBigInt::to_bigint(&-898877731).unwrap()),
            param2: BigInt::from(
                num_bigint::ToBigInt::to_bigint(&(-9809887317731 as i64)).unwrap(),
            ),
            param3: BigInt::from(
                num_bigint::ToBigInt::to_bigint(&(-223372036854775808 as i64)).unwrap(),
            ),
        };

        assert_eq!(
            fun.encode(),
            hex!("db617e8fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36ddfffffffffffffffffffffffffffffffffffffffffffffffffffff713f526b11dfffffffffffffffffffffffffffffffffffffffffffffffffce66c50e2840000")
                .to_vec()
        );
    }

    #[test]
    fn it_decode_fun_input_int8_int32_int64_int256() {
        use substreams::scalar::BigInt;
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
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-127).unwrap(),),
                param1: BigInt::from(num_bigint::ToBigInt::to_bigint(&-898877731).unwrap(),),
                param2: BigInt::from(
                    num_bigint::ToBigInt::to_bigint(&(-9809887317731 as i64)).unwrap(),
                ),
                param3: BigInt::from(
                    num_bigint::ToBigInt::to_bigint(&(-223372036854775808 as i64)).unwrap(),
                ),
            }),
        );
    }

    #[test]
    fn it_manual_encode_num_bigint_signed_bytes() {
        use substreams::scalar::BigInt;

        let num = BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap());
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
        use substreams::scalar::BigInt;
        use tests::functions::FunAll as Function;

        let fun = Function {
            param0: hex!("FffDB7377345371817F2b4dD490319755F5899eC").to_vec(),
            param1: hex!("b2").to_vec(),
            param2: hex!("cf36ac4f97dc10d9"),
            param3: hex!("cf36ac4f97dc10d91fc2cbb20d718e94a8cbfe0f82eaedc6a4aa38946fb797cd"),
            param4: BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap()),
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
        use substreams::scalar::BigInt;
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
                param4: BigInt::from(num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap()),
                param5: BigInt::from(1827641804u64),
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

    #[test]
    fn it_encode_fun_tuple_address() {
        use tests::functions::FunTupleAddress as Function;

        let fun = Function {
            param0: (hex!("FffDB7377345371817F2b4dD490319755F5899eC").to_vec(),),
        };

        assert_eq!(
            fun.encode(),
            hex!("a369a3c9000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec")
                .to_vec()
        );
    }

    #[test]
    fn it_decode_fun_tuple_address() {
        use tests::functions::FunTupleAddress as Function;

        let call = pb::eth::v2::Call {
            input: hex!("a369a3c9000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec")
                .to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                param0: (hex!("FffDB7377345371817F2b4dD490319755F5899eC").to_vec(),),
            }),
        );
    }

    #[test]
    fn it_decode_fun_int128() {
        use tests::functions::FunInt128 as Function;

        let call = pb::eth::v2::Call {
            input: hex!("5b3357ff0000000000000000000000000000000000000000000000000000000000000000")
                .to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call);
        assert_eq!(
            fun,
            Ok(Function {
                arg0: BigInt::zero(),
            }),
        );
    }

    #[test]
    fn it_encode_fun_fun_int128() {
        use substreams::scalar::BigInt;
        use tests::functions::FunInt128 as Function;

        let fun = Function {
            arg0: BigInt::zero(),
        };

        assert_bytes!(
            fun.encode(),
            hex!("5b3357ff0000000000000000000000000000000000000000000000000000000000000000")
                .to_vec()
        );
    }

    #[test]
    fn it_renders_original_function_name_on_dedup() {
        use tests::functions::FunWithOverloads1 as Fun1;
        use tests::functions::FunWithOverloads2 as Fun2;

        assert!(Fun1::NAME == "funWithOverloads");
        assert!(Fun2::NAME == "funWithOverloads");
    }

    #[test]
    fn it_dedup_function_with_leading_underscore_difference() {
        use tests::functions::FunWithOverloadsLeadingUnderscore1 as Fun1;
        use tests::functions::FunWithOverloadsLeadingUnderscore2 as Fun2;

        assert!(Fun1::NAME == "_funWithOverloadsLeadingUnderscore");
        assert!(Fun2::NAME == "funWithOverloadsLeadingUnderscore");
    }

    #[test]
    fn it_dedup_function_with_casing_difference() {
        use tests::functions::FunWithOverloadsCasing1 as Fun1;
        use tests::functions::FunWithOverloadsCasing2 as Fun2;

        assert!(Fun1::NAME == "FunWithOverloadsCasing");
        assert!(Fun2::NAME == "funWithOverloadsCasing");
    }

    #[test]
    fn it_decode_event_tuple_with_more_than_twelve_fields() {
        use tests::events::EventTupleMoreThanTwelveFields as Event;

        let log = pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("10a22da5741e1b76c50c9f1af0201301a9710b806cf43843a8f2fef789fea7e4").to_vec(),
            ],
            data: hex!(
                "
                0000000000000000000000000000000000000000000000000000000000000001
                0000000000000000000000000000000000000000000000000000000000000002
                0000000000000000000000000000000000000000000000000000000000000003
                0000000000000000000000000000000000000000000000000000000000000004
                0000000000000000000000000000000000000000000000000000000000000005
                0000000000000000000000000000000000000000000000000000000000000006
                0000000000000000000000000000000000000000000000000000000000000007
                0000000000000000000000000000000000000000000000000000000000000008
                0000000000000000000000000000000000000000000000000000000000000009
                0000000000000000000000000000000000000000000000000000000000000010
                0000000000000000000000000000000000000000000000000000000000000011
                0000000000000000000000000000000000000000000000000000000000000012
                0000000000000000000000000000000000000000000000000000000000000013
            "
            )
            .to_vec(),
            ..Default::default()
        };

        assert_eq!(Event::match_log(&log), true);

        let event = Event::decode(&log).unwrap();

        // We check only a few fields, no need to check all 13 fields
        assert_eq!(
            event.param0.0,
            hex!("0000000000000000000000000000000000000001").to_vec(),
        );
        assert_eq!(
            event.param0.6,
            hex!("0000000000000000000000000000000000000007").to_vec(),
        );
        assert_eq!(
            event.param0.12,
            hex!("0000000000000000000000000000000000000013").to_vec(),
        );
    }

    #[test]
    fn it_decode_function_tuple_with_more_than_twelve_fields() {
        use tests::functions::FunTupleMoreThanTwelveFields as Function;

        let call = pb::eth::v2::Call {
            input: hex!(
                "
                02b0cf64
                0000000000000000000000000000000000000000000000000000000000000001
                0000000000000000000000000000000000000000000000000000000000000002
                0000000000000000000000000000000000000000000000000000000000000003
                0000000000000000000000000000000000000000000000000000000000000004
                0000000000000000000000000000000000000000000000000000000000000005
                0000000000000000000000000000000000000000000000000000000000000006
                0000000000000000000000000000000000000000000000000000000000000007
                0000000000000000000000000000000000000000000000000000000000000008
                0000000000000000000000000000000000000000000000000000000000000009
                0000000000000000000000000000000000000000000000000000000000000010
                0000000000000000000000000000000000000000000000000000000000000011
                0000000000000000000000000000000000000000000000000000000000000012
                0000000000000000000000000000000000000000000000000000000000000013
            "
            )
            .to_vec(),
            ..Default::default()
        };

        assert_eq!(Function::match_call(&call), true);

        let fun = Function::decode(&call).unwrap();
        // We check only a few fields, no need to check all 13 fields
        assert_eq!(
            fun.arg0.0,
            hex!("0000000000000000000000000000000000000001").to_vec(),
        );
        assert_eq!(
            fun.arg0.6,
            hex!("0000000000000000000000000000000000000007").to_vec(),
        );
        assert_eq!(
            fun.arg0.12,
            hex!("0000000000000000000000000000000000000013").to_vec(),
        );
    }
}

/// Events whose unindexed parameters are all fixed-size, one per shape the data
/// section can take. Every expected encoding here comes from `cast abi-encode`
/// rather than from `ethabi`, so the fixture does not check the library against
/// itself.
#[cfg(test)]
mod fixed_path_events {
    use crate::abi::tests;
    use pretty_assertions::assert_eq;
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_ethereum::pb;

    fn log(topics: Vec<Vec<u8>>, data: Vec<u8>) -> pb::eth::v2::Log {
        pb::eth::v2::Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics,
            data,
            ..Default::default()
        }
    }

    #[test]
    fn it_reads_a_uint8_over_the_whole_word() {
        use tests::events::EventUint8 as Event;

        let log = log(
            vec![hex!("4827c7e926cb6bed5d63b31ba81bd2bfd592b6c7d977c9183634428169f77dff").to_vec()],
            hex!("00000000000000000000000000000000000000000000000000000000000000ff").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(255u64)
            })
        );
    }

    #[test]
    fn it_reads_a_uint32_at_its_maximum() {
        use tests::events::EventUint32 as Event;

        let log = log(
            vec![hex!("4947d76ac7119dc4e859d7b5a439c3abbdbf8de3c99ee6aa85414c3b19d53cf7").to_vec()],
            hex!("00000000000000000000000000000000000000000000000000000000ffffffff").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(4294967295u64)
            })
        );
    }

    #[test]
    fn it_reads_a_uint64_at_its_maximum() {
        use tests::events::EventUint64 as Event;

        let log = log(
            vec![hex!("eea619cf76dcf9d64f9478c94106d872a03531cefe59b925ee2f80f89d324449").to_vec()],
            hex!("000000000000000000000000000000000000000000000000ffffffffffffffff").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(18446744073709551615u64)
            })
        );
    }

    #[test]
    fn it_sign_extends_a_negative_int8() {
        use tests::events::EventInt8 as Event;

        // -1 fills the whole word, so a reader that stopped at the low byte would
        // still be right here; `it_reads_the_minimum_int8` is the one that catches it.
        let log = log(
            vec![hex!("6fb5061a4f99f19040cb9e84831965da35d53f71f58788a3a150b7c4a8ebd61d").to_vec()],
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-1i64).unwrap())
            })
        );
    }

    #[test]
    fn it_reads_the_minimum_int8() {
        use tests::events::EventInt8 as Event;

        let log = log(
            vec![hex!("6fb5061a4f99f19040cb9e84831965da35d53f71f58788a3a150b7c4a8ebd61d").to_vec()],
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80").to_vec(),
        );

        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-128i64).unwrap())
            })
        );
    }

    #[test]
    fn it_reads_a_negative_int32() {
        use tests::events::EventInt32 as Event;

        let log = log(
            vec![hex!("b00d3d0dfd077270dea963a3171892adfe3438be439b08d1e0667d21887745a2").to_vec()],
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffca6c36dd").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-898877731i64).unwrap())
            })
        );
    }

    #[test]
    fn it_reads_bytes1_from_the_front_of_the_word() {
        use tests::events::EventBytes1 as Event;

        let log = log(
            vec![hex!("c9cccfabe1fd530fbd32fa8c2e9d5864efedcfcbf1079a9bf7e611a42c22799b").to_vec()],
            hex!("ff00000000000000000000000000000000000000000000000000000000000000").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(Event::decode(&log), Ok(Event { param0: hex!("ff") }));
    }

    #[test]
    fn it_reads_bytes4_from_the_front_of_the_word() {
        use tests::events::EventBytes4 as Event;

        let log = log(
            vec![hex!("77fe9f33afe7d3670ce3e758040336824dc40a1455a29df09ab0f7fdf365660b").to_vec()],
            hex!("deadbeef00000000000000000000000000000000000000000000000000000000").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: hex!("deadbeef")
            })
        );
    }

    #[test]
    fn it_reads_a_bool_from_the_low_byte() {
        use tests::events::EventBool as Event;

        let topic =
            hex!("333dd89d9c702f4d468fd02e664620fa34a8145656d32ac75091b105d8bec331").to_vec();

        let yes = log(
            vec![topic.clone()],
            hex!("0000000000000000000000000000000000000000000000000000000000000001").to_vec(),
        );
        let no = log(
            vec![topic],
            hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
        );

        assert_eq!(Event::decode(&yes), Ok(Event { param0: true }));
        assert_eq!(Event::decode(&no), Ok(Event { param0: false }));
    }

    #[test]
    fn it_reads_two_bools_at_their_own_offsets() {
        use tests::events::EventBoolBool as Event;

        let log = log(
            vec![hex!("2af9848907b889469f1bf9be9a9a2d4e304d8f64722e612cdbde19a760781bbd").to_vec()],
            hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: true,
                param1: false
            })
        );
    }

    #[test]
    fn it_reads_a_mix_of_every_fixed_type() {
        use tests::events::EventAddressUint64BoolBytes4 as Event;

        let log = log(
            vec![hex!("5ea7f20001dfa820ca9b1caf89d706a286302726362f66c27888f71079944528").to_vec()],
            hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705000000000000000000000000000000000000000000000000ffffffffffffffff0000000000000000000000000000000000000000000000000000000000000001deadbeef00000000000000000000000000000000000000000000000000000000").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                param1: BigInt::from(18446744073709551615u64),
                param2: true,
                param3: hex!("deadbeef"),
            })
        );
    }

    #[test]
    fn it_reads_four_unsigned_widths_in_sequence() {
        use tests::events::EventUint8Uint16Uint32Uint64 as Event;

        let log = log(
            vec![hex!("fab45ed66679bf20ef278bb65e16b4bc3618266a6d62f08c77c0ca50644357d1").to_vec()],
            hex!("00000000000000000000000000000000000000000000000000000000000000ff000000000000000000000000000000000000000000000000000000000000ffff00000000000000000000000000000000000000000000000000000000ffffffff000000000000000000000000000000000000000000000000ffffffffffffffff").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                param0: BigInt::from(255u64),
                param1: BigInt::from(65535u64),
                param2: BigInt::from(4294967295u64),
                param3: BigInt::from(18446744073709551615u64),
            })
        );
    }

    #[test]
    fn it_keeps_field_names_aligned_when_an_indexed_param_comes_first() {
        use tests::events::EventAddressIdxUint256Bool as Event;

        let log = log(
            vec![
                hex!("0ab8c761503e2a479f09380fed9c33c75e20ccfc65eb0b06731937a37dc698ed").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            ],
            hex!("00000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000000000000000001").to_vec(),
        );

        assert_eq!(Event::match_log(&log), true);
        assert_eq!(
            Event::decode(&log),
            Ok(Event {
                first: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                second: BigInt::from(1000000u64),
                third: true,
            })
        );
    }
}

/// Events that carry a fixed-size parameter the straight-line reads do not cover,
/// and the widest fixed layouts. The first two decode through `ethabi` while still
/// being length-checked as fixed, which is the one place the two sides of that
/// decision have to agree.
#[cfg(test)]
mod fixed_size_fallback_events {
    use crate::abi::tests;
    use pretty_assertions::assert_eq;
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_ethereum::pb;

    fn log(topic: [u8; 32], data: Vec<u8>) -> pb::eth::v2::Log {
        pb::eth::v2::Log {
            topics: vec![topic.to_vec()],
            data,
            ..Default::default()
        }
    }

    const FIXED_ARRAY: [u8; 32] =
        hex!("77c8495e931c56c8f8ff11ec395d63fa9e864a5b2a3ff51828cffce6387d54f7");
    const ALL_FIXED_TUPLE: [u8; 32] =
        hex!("8a546754d28475bda2f3494dee524ac9ce10de967edb8a1b2c9d6d6e97307fe9");
    const SIX_WORDS: [u8; 32] =
        hex!("bbe32b3dbb75fa6c07a6fa458651dbefb8c86ccb626f0e9e6cdab58fafe69734");
    const BYTES32: [u8; 32] =
        hex!("f7c7d865a0a8afdbc9b102818a852adfe4af9b7d80b8b6496b9725528b6d5a72");
    const INT128: [u8; 32] =
        hex!("6be05b21b2009192aecdd4262292fc9d09411af9aa1e9dfdd73030c58e370e26");

    #[test]
    fn it_decodes_a_fixed_array_of_uint256() {
        use tests::events::EventUFixedArrayUint2563 as Event;

        let data = hex!("000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003").to_vec();

        assert_eq!(Event::match_log(&log(FIXED_ARRAY, data.clone())), true);
        assert_eq!(
            Event::decode(&log(FIXED_ARRAY, data)),
            Ok(Event {
                param0: [BigInt::from(1u64), BigInt::from(2u64), BigInt::from(3u64)]
            })
        );
    }

    #[test]
    fn it_decodes_a_tuple_whose_fields_are_all_fixed() {
        use tests::events::EventUTupleAllFixed as Event;

        let data = hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f70500000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000000000000000001").to_vec();

        assert_eq!(Event::match_log(&log(ALL_FIXED_TUPLE, data.clone())), true);
        assert_eq!(
            Event::decode(&log(ALL_FIXED_TUPLE, data)),
            Ok(Event {
                param0: (
                    hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                    BigInt::from(1000000u64),
                    true,
                )
            })
        );
    }

    #[test]
    fn it_reads_six_fixed_words_at_their_own_offsets() {
        use tests::events::EventSixWords as Event;

        let data = hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f70500000000000000000000000000000000000000000000000000000000000f4240ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000001f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca18400000000000000000000000000000000000000000000000000000000000000ff").to_vec();

        assert_eq!(Event::match_log(&log(SIX_WORDS, data.clone())), true);
        assert_eq!(
            Event::decode(&log(SIX_WORDS, data)),
            Ok(Event {
                p0: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                p1: BigInt::from(1000000u64),
                p2: BigInt::from(num_bigint::ToBigInt::to_bigint(&-1i64).unwrap()),
                p3: true,
                p4: hex!("f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca184"),
                p5: BigInt::from(255u64),
            })
        );
    }

    #[test]
    fn it_reads_a_full_word_of_fixed_bytes() {
        use tests::events::EventBytes32 as Event;

        let data =
            hex!("f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca184").to_vec();

        assert_eq!(
            Event::decode(&log(BYTES32, data)),
            Ok(Event {
                param0: hex!("f154bf9817019c089414b85e6c5a19fd5d1ea04c103fcd039314132b354ca184")
            })
        );
    }

    #[test]
    fn it_reads_an_int128_that_the_encoder_sign_extended() {
        use tests::events::EventInt128 as Event;

        // `cast abi-encode "f(int128)" -170141183460469231731687303715884105728`, which
        // pads the 128-bit two's complement up to the word with sign bytes.
        let data =
            hex!("ffffffffffffffffffffffffffffffff80000000000000000000000000000000").to_vec();

        assert_eq!(
            Event::decode(&log(INT128, data)),
            Ok(Event {
                param0: BigInt::from(
                    num_bigint::BigInt::parse_bytes(
                        b"-170141183460469231731687303715884105728",
                        10
                    )
                    .unwrap()
                )
            })
        );
    }

    #[test]
    fn it_reads_an_int128_of_minus_one() {
        use tests::events::EventInt128 as Event;

        let data =
            hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").to_vec();

        assert_eq!(
            Event::decode(&log(INT128, data)),
            Ok(Event {
                param0: BigInt::from(num_bigint::ToBigInt::to_bigint(&-1i64).unwrap())
            })
        );
    }

    #[test]
    fn it_requires_the_full_length_of_every_fixed_layout() {
        use tests::events::EventSixWords as Event6;
        use tests::events::EventUFixedArrayUint2563 as EventArr;
        use tests::events::EventUTupleAllFixed as EventTup;

        // One byte short of the guard, on the straight-line path and on both
        // `ethabi` fallbacks that are nonetheless length-checked as fixed.
        assert!(Event6::decode(&log(SIX_WORDS, vec![0u8; 191])).is_err());
        assert!(EventArr::decode(&log(FIXED_ARRAY, vec![0u8; 95])).is_err());
        assert!(EventTup::decode(&log(ALL_FIXED_TUPLE, vec![0u8; 95])).is_err());

        // Exactly the guard length decodes.
        assert!(Event6::decode(&log(SIX_WORDS, vec![0u8; 192])).is_ok());
        assert!(EventArr::decode(&log(FIXED_ARRAY, vec![0u8; 96])).is_ok());
        assert!(EventTup::decode(&log(ALL_FIXED_TUPLE, vec![0u8; 96])).is_ok());

        // One word past it is ignored rather than rejected.
        assert!(Event6::decode(&log(SIX_WORDS, vec![0u8; 224])).is_ok());
        assert!(EventArr::decode(&log(FIXED_ARRAY, vec![0u8; 128])).is_ok());
        assert!(EventTup::decode(&log(ALL_FIXED_TUPLE, vec![0u8; 128])).is_ok());
    }

    #[test]
    fn match_log_still_requires_an_exact_length() {
        use tests::events::EventSixWords as Event;

        // `decode` accepts a longer buffer but `match_log` does not, which is the
        // behaviour every released version has had.
        assert_eq!(Event::match_log(&log(SIX_WORDS, vec![0u8; 192])), true);
        assert_eq!(Event::match_log(&log(SIX_WORDS, vec![0u8; 224])), false);
        assert_eq!(Event::match_log(&log(SIX_WORDS, vec![0u8; 191])), false);
    }
}

/// Reads of a single value checked against `ethabi` on the same bytes.
///
/// Each case decodes one parameter through both implementations and requires the
/// same verdict and, where both succeed, the same value. `ethabi` is the reference
/// for what an encoded parameter means, so these pin the reads against it directly
/// rather than against expectations written by hand.
#[cfg(test)]
mod ethabi_parity {
    use substreams::scalar::BigInt;
    use substreams_ethereum::abi;

    fn word_of(value: u64) -> Vec<u8> {
        let mut out = vec![0u8; 32];
        out[24..32].copy_from_slice(&value.to_be_bytes());
        out
    }

    fn concat(words: &[Vec<u8>]) -> Vec<u8> {
        words.iter().flat_map(|word| word.iter().copied()).collect()
    }

    fn encode_hex(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    /// `ethabi`'s answer for one parameter, rendered so it can be compared with ours.
    fn theirs(kind: ethabi::ParamType, data: &[u8]) -> Result<String, ()> {
        match ethabi::decode(&[kind], data) {
            Ok(tokens) => Ok(render(&tokens[0])),
            Err(_) => Err(()),
        }
    }

    fn render(token: &ethabi::Token) -> String {
        match token {
            ethabi::Token::Address(value) => encode_hex(value.as_bytes()),
            ethabi::Token::Bool(value) => value.to_string(),
            ethabi::Token::Bytes(value) | ethabi::Token::FixedBytes(value) => encode_hex(value),
            ethabi::Token::String(value) => value.clone(),
            ethabi::Token::Uint(value) => {
                let mut bytes = [0u8; 32];
                value.to_big_endian(&mut bytes);
                BigInt::from_unsigned_bytes_be(&bytes).to_string()
            }
            ethabi::Token::Int(value) => {
                let mut bytes = [0u8; 32];
                value.to_big_endian(&mut bytes);
                BigInt::from_signed_bytes_be(&bytes).to_string()
            }
            ethabi::Token::Array(items) | ethabi::Token::FixedArray(items) => {
                format!(
                    "[{}]",
                    items.iter().map(render).collect::<Vec<_>>().join(",")
                )
            }
            ethabi::Token::Tuple(items) => format!(
                "({})",
                items.iter().map(render).collect::<Vec<_>>().join(",")
            ),
        }
    }

    fn assert_same(
        label: &str,
        kind: ethabi::ParamType,
        data: &[u8],
        ours: Result<String, String>,
    ) {
        let ours = ours.map_err(|_| ());
        let theirs = theirs(kind, data);

        assert_eq!(
            ours.is_ok(),
            theirs.is_ok(),
            "{label}: ours={ours:?} ethabi={theirs:?}"
        );

        if let (Ok(ours), Ok(theirs)) = (&ours, &theirs) {
            assert_eq!(ours, theirs, "{label}: decoded values differ");
        }
    }

    #[test]
    fn it_reads_a_uint_as_ethabi_does() {
        for data in [word_of(0), word_of(1), word_of(u64::MAX), vec![0xff; 32]] {
            assert_same(
                "uint256",
                ethabi::ParamType::Uint(256),
                &data,
                abi::read_uint(&data, 0, "p").map(|value| value.to_string()),
            );
        }
    }

    #[test]
    fn it_reads_an_int_as_ethabi_does() {
        let minus_one = vec![0xff; 32];
        let mut min = vec![0u8; 32];
        min[0] = 0x80;

        for data in [word_of(0), word_of(5), minus_one, min] {
            assert_same(
                "int256",
                ethabi::ParamType::Int(256),
                &data,
                abi::read_int(&data, 0, "p").map(|value| value.to_string()),
            );
        }
    }

    #[test]
    fn it_reads_an_address_as_ethabi_does() {
        let mut clean = vec![0u8; 12];
        clean.extend_from_slice(&[0xaa; 20]);

        // A dirty upper twelve bytes is truncated by `ethabi` rather than rejected.
        let mut dirty = vec![0xff; 12];
        dirty.extend_from_slice(&[0xaa; 20]);

        for data in [clean, dirty, vec![0u8; 32], vec![0u8; 31]] {
            assert_same(
                "address",
                ethabi::ParamType::Address,
                &data,
                abi::read_address(&data, 0, "p").map(encode_hex),
            );
        }
    }

    #[test]
    fn it_reads_a_bool_as_ethabi_does() {
        let mut one = vec![0u8; 32];
        one[31] = 1;
        let mut two = vec![0u8; 32];
        two[31] = 2;
        let mut dirty = vec![0x11u8; 32];
        dirty[31] = 0;

        for data in [vec![0u8; 32], one, two, dirty, vec![0u8; 31]] {
            assert_same(
                "bool",
                ethabi::ParamType::Bool,
                &data,
                abi::read_bool(&data, 0, "p").map(|value| value.to_string()),
            );
        }
    }

    #[test]
    fn it_reads_fixed_bytes_as_ethabi_does() {
        let word = {
            let mut out = vec![0u8; 32];
            out[..4].copy_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
            out
        };

        assert_same(
            "bytes4 from a full word",
            ethabi::ParamType::FixedBytes(4),
            &word,
            abi::read_fixed_bytes::<4>(&word, 0, "p").map(encode_hex),
        );
        assert_same(
            "bytes32 from a full word",
            ethabi::ParamType::FixedBytes(32),
            &word,
            abi::read_fixed_bytes::<32>(&word, 0, "p").map(encode_hex),
        );

        // A `bytesN` spans only its leading `N` bytes, so a buffer holding those is
        // enough even where it stops short of the word.
        let short = vec![0xde, 0xad, 0xbe, 0xef];
        assert_same(
            "bytes4 from a four byte buffer",
            ethabi::ParamType::FixedBytes(4),
            &short,
            abi::read_fixed_bytes::<4>(&short, 0, "p").map(encode_hex),
        );
        assert_same(
            "bytes0 from an empty buffer",
            ethabi::ParamType::FixedBytes(0),
            &[],
            abi::read_fixed_bytes::<0>(&[], 0, "p").map(encode_hex),
        );
    }

    #[test]
    fn it_reports_fixed_bytes_wider_than_a_word() {
        // `bytes33` has no valid encoding. Reading past the word would trap on
        // `wasm32`, where a slice out of bounds is not a catchable panic.
        let word = vec![0u8; 32];

        assert!(abi::read_fixed_bytes::<33>(&word, 0, "p").is_err());
        assert!(ethabi::decode(&[ethabi::ParamType::FixedBytes(33)], &word).is_err());
    }

    #[test]
    fn it_reads_bytes_as_ethabi_does() {
        let mut well_formed = concat(&[word_of(32), word_of(4)]);
        well_formed.extend_from_slice(&[0xab, 0xde, 0xff, 0x90]);
        well_formed.extend_from_slice(&[0u8; 28]);

        let cases = vec![
            ("well formed", well_formed),
            ("empty", concat(&[word_of(32), word_of(0)])),
            (
                "length past the buffer",
                concat(&[word_of(32), word_of(0xffff_ffff)]),
            ),
            ("offset past the buffer", concat(&[word_of(224)])),
            ("empty buffer", vec![]),
        ];

        for (label, data) in cases {
            assert_same(
                label,
                ethabi::ParamType::Bytes,
                &data,
                abi::read_bytes(&data, 0, "p").map(encode_hex),
            );
        }
    }

    #[test]
    fn it_reads_a_string_as_ethabi_does() {
        let mut well_formed = concat(&[word_of(32), word_of(13)]);
        well_formed.extend_from_slice(b"second string");
        well_formed.extend_from_slice(&[0u8; 19]);

        // Bytes that are not valid UTF-8 are read lossily rather than rejected.
        let mut broken = concat(&[word_of(32), word_of(4)]);
        broken.extend_from_slice(&[0xe4, 0xb8, 0x8d, 0xe5]);
        broken.extend_from_slice(&[0u8; 28]);

        for (label, data) in [
            ("well formed", well_formed),
            ("not utf8", broken),
            ("empty", concat(&[word_of(32), word_of(0)])),
        ] {
            assert_same(
                label,
                ethabi::ParamType::String,
                &data,
                abi::read_string(&data, 0, "p"),
            );
        }
    }

    #[test]
    fn it_walks_an_array_as_ethabi_does() {
        let read = |data: &[u8]| -> Result<String, String> {
            let (tail, count) = abi::read_array_tail(data, 0, "p")?;
            let mut parts = Vec::with_capacity(count);
            for index in 0..count {
                parts.push(abi::read_uint(tail, index * 32, "e")?.to_string());
            }
            Ok(format!("[{}]", parts.join(",")))
        };

        let cases = vec![
            (
                "two elements",
                concat(&[word_of(32), word_of(2), word_of(7), word_of(8)]),
            ),
            ("empty", concat(&[word_of(32), word_of(0)])),
            (
                "length beyond the tail",
                concat(&[word_of(32), word_of(3), word_of(1), word_of(2)]),
            ),
            (
                "length word past the buffer",
                concat(&[word_of(0xffff_fff0)]),
            ),
            ("self referential offset", concat(&[word_of(0), word_of(1)])),
            ("empty buffer", vec![]),
        ];

        for (label, data) in cases {
            assert_same(
                label,
                ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256))),
                &data,
                read(&data),
            );
        }
    }
}

/// How decoding answers input that is not what the ABI describes.
///
/// The suite otherwise only covers well-formed logs, so these pin the current
/// behaviour: which malformed inputs are rejected, which are accepted, and which
/// panic. Each expectation was taken from `ethabi` directly, so a replacement
/// decoder has a contract to meet rather than a guess.
#[cfg(test)]
mod decode_error_behaviour {
    use crate::abi::tests;
    use substreams::hex;
    use substreams_ethereum::pb;

    const EVENT_BOOL: [u8; 32] =
        hex!("333dd89d9c702f4d468fd02e664620fa34a8145656d32ac75091b105d8bec331");
    const EVENT_UINT8: [u8; 32] =
        hex!("4827c7e926cb6bed5d63b31ba81bd2bfd592b6c7d977c9183634428169f77dff");
    const EVENT_ADDRESS_IDX_STRING: [u8; 32] =
        hex!("3cdb310171efa4c08617535044016fb81ec0a9db468c06b008d2f446ee9346a8");

    fn log(topics: Vec<Vec<u8>>, data: Vec<u8>) -> pb::eth::v2::Log {
        pb::eth::v2::Log {
            topics,
            data,
            ..Default::default()
        }
    }

    #[test]
    fn it_rejects_a_bool_word_carrying_dirty_padding() {
        use tests::events::EventBool as Event;

        let mut data = vec![0x11u8; 32];
        data[31] = 0;

        assert!(Event::decode(&log(vec![EVENT_BOOL.to_vec()], data)).is_err());
    }

    #[test]
    fn it_reads_a_bool_low_byte_other_than_one_as_false() {
        use tests::events::EventBool as Event;

        let mut data = vec![0u8; 32];
        data[31] = 2;

        assert_eq!(
            Event::decode(&log(vec![EVENT_BOOL.to_vec()], data)),
            Ok(Event { param0: false })
        );
    }

    #[test]
    fn it_discards_the_high_order_padding_of_an_address() {
        use tests::events::EventAddressUint64BoolBytes4 as Event;

        // A dirty upper 12 bytes is truncated rather than rejected, and contracts
        // do emit these.
        let mut data = vec![0xffu8; 12];
        data.extend_from_slice(&hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705"));
        data.extend_from_slice(&hex!(
            "000000000000000000000000000000000000000000000000ffffffffffffffff"
        ));
        data.extend_from_slice(&hex!(
            "0000000000000000000000000000000000000000000000000000000000000001"
        ));
        data.extend_from_slice(&hex!(
            "deadbeef00000000000000000000000000000000000000000000000000000000"
        ));

        let decoded = Event::decode(&log(
            vec![hex!("5ea7f20001dfa820ca9b1caf89d706a286302726362f66c27888f71079944528").to_vec()],
            data,
        ))
        .expect("the dirty padding is discarded");

        assert_eq!(
            decoded.param0,
            hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec()
        );
    }

    #[test]
    fn it_accepts_trailing_bytes_past_the_parameters() {
        use tests::events::EventUint8 as Event;

        let mut data = vec![0u8; 32];
        data.extend_from_slice(&[0xde; 32]);

        assert!(
            Event::decode(&log(vec![EVENT_UINT8.to_vec()], data)).is_ok(),
            "ethabi ignores trailing bytes"
        );
    }

    #[test]
    fn it_rejects_a_data_section_one_byte_short() {
        use tests::events::EventUint8 as Event;

        assert!(Event::decode(&log(vec![EVENT_UINT8.to_vec()], vec![0u8; 31])).is_err());
    }

    #[test]
    fn it_rejects_a_data_section_holding_only_the_first_of_two_params() {
        use tests::events::EventBoolBool as Event;

        let topic = hex!("2af9848907b889469f1bf9be9a9a2d4e304d8f64722e612cdbde19a760781bbd");

        assert!(Event::decode(&log(vec![topic.to_vec()], vec![0u8; 32])).is_err());
    }

    #[test]
    fn it_rejects_an_empty_data_section() {
        use tests::events::EventUint8 as Event;

        assert!(Event::decode(&log(vec![EVENT_UINT8.to_vec()], vec![])).is_err());
    }

    #[test]
    fn match_log_rejects_every_malformed_shape_without_panicking() {
        use tests::events::EventUint8 as Event;

        assert_eq!(
            Event::match_log(&log(vec![], vec![0u8; 32])),
            false,
            "no topics at all"
        );
        assert_eq!(
            Event::match_log(&log(vec![EVENT_UINT8.to_vec()], vec![0u8; 31])),
            false,
            "data one byte short"
        );
        assert_eq!(
            Event::match_log(&log(vec![vec![0u8; 32]], vec![0u8; 32])),
            false,
            "wrong topic id"
        );
    }

    #[test]
    fn it_reports_an_indexed_param_with_no_topic_rather_than_panicking() {
        use tests::events::EventAddressIdxString as Event;

        // `decode` is public and callable without `match_log` having run, so it
        // checks the topic count itself before reading an indexed parameter.
        let log = log(
            vec![EVENT_ADDRESS_IDX_STRING.to_vec()],
            hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d7365636f6e6420737472696e6700000000000000000000000000000000000000").to_vec(),
        );

        let outcome = std::panic::catch_unwind(|| Event::decode(&log));

        match outcome {
            Ok(result) => assert!(result.is_err(), "a missing topic is an error"),
            Err(_) => panic!("decode must not panic on a missing topic"),
        }
    }

    #[test]
    fn it_reports_a_surplus_topic_rather_than_reading_the_wrong_one() {
        use tests::events::EventUint8 as Event;

        let log = log(vec![EVENT_UINT8.to_vec(), vec![0u8; 32]], vec![0u8; 32]);

        let outcome = std::panic::catch_unwind(|| Event::decode(&log));

        match outcome {
            Ok(result) => assert!(result.is_err(), "an unexpected extra topic is an error"),
            Err(_) => panic!("decode must not panic on a surplus topic"),
        }
    }
}

#[cfg(test)]
mod lazy_view_tests {
    use crate::abi::tests;
    use buffa::view::LazyMessageView;
    use buffa::Message;
    use substreams::hex;
    use substreams_ethereum::pb::eth::v2::{Log, __buffa::lazy_view::LogLazyView};
    use substreams_ethereum::Event;

    #[test]
    fn it_decodes_the_same_event_from_an_owned_log_and_a_lazy_view() {
        use tests::events::EventUTupleAddress as Event_;

        let log = Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("adb25b4ab5d8f04dc5e8073124d207a0974cb9aecac69a6197dbd5cf8dce87d3").to_vec(),
            ],
            data: hex!("000000000000000000000000db0de9288cf0713de91371969efcc9969dd94117").to_vec(),
            ..Default::default()
        };

        let bytes = log.encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        assert!(Event_::match_log(&log), "owned log must match");
        assert!(Event_::match_log(&lazy), "lazy view must match");

        assert_eq!(
            Event_::decode(&log),
            Event_::decode(&lazy),
            "both representations must decode to the same event"
        );
    }

    #[test]
    fn it_matches_and_decodes_through_a_lazy_view() {
        use tests::events::EventUTupleAddress as Event_;

        let log = Log {
            address: hex!("0000000000000000000000000000000000000000").to_vec(),
            topics: vec![
                hex!("adb25b4ab5d8f04dc5e8073124d207a0974cb9aecac69a6197dbd5cf8dce87d3").to_vec(),
            ],
            data: hex!("000000000000000000000000db0de9288cf0713de91371969efcc9969dd94117").to_vec(),
            ..Default::default()
        };

        let bytes = log.encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        let from_owned = Event_::match_and_decode(log);
        let from_lazy = Event_::match_and_decode(lazy);

        assert!(from_owned.is_some(), "owned log must decode");
        assert_eq!(from_owned, from_lazy);
    }

    #[test]
    fn it_rejects_a_non_matching_log_in_both_representations() {
        use tests::events::EventUTupleAddress as Event_;

        let log = Log {
            topics: vec![
                hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ],
            ..Default::default()
        };

        let bytes = log.encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        assert!(!Event_::match_log(&log));
        assert!(!Event_::match_log(&lazy));
    }
}

#[cfg(test)]
mod match_and_decode_shapes {
    use crate::abi::tests;
    use buffa::view::LazyMessageView;
    use buffa::Message;
    use substreams::hex;
    use substreams_ethereum::pb::eth::v2::{Log, __buffa::lazy_view::LogLazyView};
    use substreams_ethereum::Event;

    fn matching_log() -> Log {
        Log {
            topics: vec![
                hex!("adb25b4ab5d8f04dc5e8073124d207a0974cb9aecac69a6197dbd5cf8dce87d3").to_vec(),
            ],
            data: hex!("000000000000000000000000db0de9288cf0713de91371969efcc9969dd94117").to_vec(),
            ..Default::default()
        }
    }

    #[test]
    fn it_accepts_every_log_shape() {
        use tests::events::EventUTupleAddress as Event_;

        let log = matching_log();
        let bytes = log.encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        let by_ref = Event_::match_and_decode(&log);
        let by_ref_lazy = Event_::match_and_decode(&lazy);
        let owned_lazy = Event_::match_and_decode(lazy);
        let owned = Event_::match_and_decode(log);

        assert!(owned.is_some(), "owned log must decode");
        assert_eq!(owned, by_ref);
        assert_eq!(owned, by_ref_lazy);
        assert_eq!(owned, owned_lazy);
    }

    #[test]
    fn it_accepts_a_reference_from_an_iterator() {
        use tests::events::EventUTupleAddress as Event_;

        let logs = vec![matching_log()];
        let decoded: Vec<_> = logs.iter().filter_map(Event_::match_and_decode).collect();

        assert_eq!(decoded.len(), 1);
    }
}

#[cfg(test)]
mod encode_parity {
    use crate::abi::tests::functions;
    use ethabi::Token;
    use substreams::scalar::BigInt;

    /// The encoded parameters, past the four selector bytes. The selector comes
    /// from the signature rather than the encoder, so only what follows it is
    /// worth comparing.
    fn ours(encoded: Vec<u8>) -> Vec<u8> {
        encoded[4..].to_vec()
    }

    fn theirs(tokens: &[Token]) -> Vec<u8> {
        ethabi::encode(tokens)
    }

    fn uint(value: u64) -> BigInt {
        BigInt::from(value)
    }

    fn eth_uint(value: u64) -> Token {
        Token::Uint(ethabi::Uint::from(value))
    }

    fn text(value: &str) -> Token {
        Token::String(value.to_string())
    }

    #[test]
    fn it_encodes_a_string_array_as_ethabi_does() {
        let call = functions::FunStringArray {
            a: vec!["first".to_string(), "second".to_string()],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![text("first"), text("second")])])
        );
    }

    #[test]
    fn it_encodes_an_empty_string_array_as_ethabi_does() {
        let call = functions::FunStringArray { a: vec![] };

        assert_eq!(ours(call.encode()), theirs(&[Token::Array(vec![])]));
    }

    #[test]
    fn it_encodes_string_array_elements_of_uneven_length() {
        let long = "this one is longer than a single word of thirty two";
        let call = functions::FunStringArray {
            a: vec!["a".to_string(), long.to_string(), "z".to_string()],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![text("a"), text(long), text("z")])])
        );
    }

    #[test]
    fn it_encodes_a_bytes_array_as_ethabi_does() {
        let call = functions::FunBytesArray {
            a: vec![vec![0xde, 0xad], vec![0xbe, 0xef, 0x01, 0x02, 0x03]],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![
                Token::Bytes(vec![0xde, 0xad]),
                Token::Bytes(vec![0xbe, 0xef, 0x01, 0x02, 0x03]),
            ])])
        );
    }

    #[test]
    fn it_encodes_a_fixed_array_of_strings_without_a_length_word() {
        let call = functions::FunStringFixedArray {
            a: ["alpha".to_string(), "beta".to_string()],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::FixedArray(vec![text("alpha"), text("beta")])])
        );
    }

    #[test]
    fn it_encodes_an_array_whose_elements_are_arrays() {
        let call = functions::FunUintNestedArray {
            a: vec![vec![uint(1), uint(2)], vec![uint(3)]],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![
                Token::Array(vec![eth_uint(1), eth_uint(2)]),
                Token::Array(vec![eth_uint(3)]),
            ])])
        );
    }

    #[test]
    fn it_encodes_a_nested_array_holding_an_empty_inner() {
        let call = functions::FunUintNestedArray {
            a: vec![vec![], vec![uint(7)]],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![
                Token::Array(vec![]),
                Token::Array(vec![eth_uint(7)]),
            ])])
        );
    }

    #[test]
    fn it_encodes_a_tuple_whose_leading_field_is_dynamic() {
        let call = functions::FunTupleWithString {
            a: ("hello".to_string(), uint(42)),
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Tuple(vec![text("hello"), eth_uint(42)])])
        );
    }

    #[test]
    fn it_encodes_a_tuple_of_two_dynamic_fields() {
        let call = functions::FunTupleTwoDynamic {
            a: ("hello".to_string(), vec![0x01, 0x02, 0x03]),
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Tuple(vec![
                text("hello"),
                Token::Bytes(vec![0x01, 0x02, 0x03]),
            ])])
        );
    }

    #[test]
    fn it_measures_a_later_tail_past_the_one_before_it() {
        let call = functions::FunMixedLeadingDynamic {
            a: "leading".to_string(),
            b: uint(99),
            c: vec![0xaa; 40],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[text("leading"), eth_uint(99), Token::Bytes(vec![0xaa; 40]),])
        );
    }

    #[test]
    fn it_measures_an_array_tail_across_a_wider_head_section() {
        let call = functions::FunStringArrayThenUint {
            a: vec!["one".to_string(), "two".to_string()],
            b: uint(5),
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![text("one"), text("two")]), eth_uint(5),])
        );
    }

    #[test]
    fn it_encodes_an_array_of_dynamic_tuples() {
        let call = functions::FunTupleArray {
            a: vec![("x".to_string(),), ("yy".to_string(),)],
        };

        assert_eq!(
            ours(call.encode()),
            theirs(&[Token::Array(vec![
                Token::Tuple(vec![text("x")]),
                Token::Tuple(vec![text("yy")]),
            ])])
        );
    }
}
