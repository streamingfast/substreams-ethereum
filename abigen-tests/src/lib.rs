mod abi;

#[cfg(test)]
mod tests {
    use crate::abi::tests;
    use ethabi::ethereum_types::U256;
    use pretty_assertions::assert_eq;
    use substreams::hex;
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
}
