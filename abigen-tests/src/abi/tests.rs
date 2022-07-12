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
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {}",
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
        impl substreams_ethereum::Event for EventAddressIdxString {
            const NAME: &'static str = "EventAddressIdxString";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
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
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {}",
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
                                "unable to decode param 'third' from topic of type 'uint256': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    fourth: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
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
                            "Unable to decode logs.EventAddressIdxStringUint256IdxBytes event: {:#}",
                            e
                        )
                    }
                }
            }
        }
        impl substreams_ethereum::Event for EventAddressIdxStringUint256IdxBytes {
            const NAME: &'static str = "EventAddressIdxStringUint256IdxBytes";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {}",
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
                                "unable to decode param 'fourth' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    third: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    second: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "Unable to decode logs.EventAddressIdxUint256Uint256AddressIdx event: {:#}",
                            e
                        )
                    }
                }
            }
        }
        impl substreams_ethereum::Event for EventAddressIdxUint256Uint256AddressIdx {
            const NAME: &'static str = "EventAddressIdxUint256Uint256AddressIdx";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    first: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'first' from topic of type 'address': {}",
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
            pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "Unable to decode logs.EventWithOverloads1 event: {:#}", e
                        )
                    }
                }
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads1 {
            const NAME: &'static str = "EventWithOverloads1";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    second: ethabi::decode(
                            &[ethabi::ParamType::String],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'second' from topic of type 'string': {}",
                                e
                            )
                        })?
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
                            "Unable to decode logs.EventWithOverloads2 event: {:#}", e
                        )
                    }
                }
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads2 {
            const NAME: &'static str = "EventWithOverloads2";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
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
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
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
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    third: ethabi::decode(
                            &[ethabi::ParamType::Uint(256usize)],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'third' from topic of type 'uint256': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                match Self::decode(log) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "Unable to decode logs.EventWithOverloads3 event: {:#}", e
                        )
                    }
                }
            }
        }
        impl substreams_ethereum::Event for EventWithOverloads3 {
            const NAME: &'static str = "EventWithOverloads3";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }