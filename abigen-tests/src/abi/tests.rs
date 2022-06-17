    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'first' from topic of type 'address': {}",
                            e
                        ))?
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
            pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "Unable to decode logs.EventAddressIdxString event: {:#}", e
                        )
                    }
                }
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Bytes],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'first' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    second: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'second' from topic of type 'uint256': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    fourth: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "Unable to decode logs.EventAddressIdxStringUint256IdxBytes event: {:#}",
                            e
                        )
                    }
                }
            }
        }
    }