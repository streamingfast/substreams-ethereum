mod abi;

#[cfg(test)]
mod tests {
    use crate::abi::tests;
    use substreams::hex;
    use substreams_ethereum::pb;

    #[test]
    fn it_decode_indexed_and_unindexed_event_correctly() {
        use tests::events::EventAddressIdxString;

        let log = pb::eth::v1::Log{
            address: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            topics: vec![
                hex!("3cdb310171efa4c08617535044016fb81ec0a9db468c06b008d2f446ee9346a8").to_vec(),
                hex!("000000000000000000000000ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d7365636f6e6420737472696e6700000000000000000000000000000000000000").to_vec(),
            index: 0,
            block_index: 0,
            ordinal:0,
        };

        assert_eq!(EventAddressIdxString::match_log(&log), true);

        let event = EventAddressIdxString::decode(&log);

        assert_eq!(
            event,
            Ok(EventAddressIdxString {
                indexed: hex!("ab07a50ad459b41fe065f7bbab866d5390e9f705").to_vec(),
                unindexed: "second string".to_string(),
            }),
        );
    }
}
