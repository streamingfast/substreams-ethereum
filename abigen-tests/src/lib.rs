mod abi;

#[cfg(test)]
mod tests {
    use crate::abi::tests;
    use ethabi::ethereum_types::U256;
    use pretty_assertions::assert_eq;
    use substreams::hex;
    use substreams_ethereum::pb;

    #[test]
    fn it_decode_event_int256_idx() {
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
                param0: num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),
            }),
        );
    }

    #[test]
    fn it_decode_event_int256() {
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
                param0: num_bigint::ToBigInt::to_bigint(&-9809887317731i64).unwrap(),
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
                second: U256::from_str_radix("0x1000000000", 16).unwrap(),
                third: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec()
            }),
        );
    }

    #[test]
    fn it_decode_event_bytes_20_uint_address_idx() {
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
                second: U256::from_str_radix("0x1000000000", 16).unwrap(),
                third: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec()
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
}
