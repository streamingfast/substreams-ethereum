use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{decode_topic, fixed_data_size, min_data_size};

use super::{from_token, rust_type, to_syntax_string};

/// Structure used to generate contract's event interface.
pub struct Event {
    name: String,
    topic_hash: [u8; 32],
    topic_count: usize,
    min_data_size: usize,
    fixed_data_size: Option<usize>,
    log_fields: Vec<TokenStream>,
    decode_indexed_fields: Vec<TokenStream>,
    decode_unindexed_fields: Vec<TokenStream>,
    decode_data: TokenStream,
}

impl<'a> From<(&'a String, &'a ethabi::Event)> for Event {
    fn from((name, e): (&'a String, &'a ethabi::Event)) -> Self {
        let names: Vec<_> = e
            .inputs
            .iter()
            .enumerate()
            .map(|(index, param)| {
                if param.name.is_empty() {
                    if param.indexed {
                        syn::Ident::new(&format!("topic{}", index), Span::call_site())
                    } else {
                        syn::Ident::new(&format!("param{}", index), Span::call_site())
                    }
                } else {
                    syn::Ident::new(&param.name.to_snake_case(), Span::call_site())
                }
            })
            .collect();

        let topic_count = e.inputs.iter().filter(|param| param.indexed).count() + 1;

        let fixed_data_size = e.inputs.iter().filter(|param| !param.indexed).fold(
            Some(0usize),
            |size, param| -> Option<usize> {
                match size {
                    Some(count) => fixed_data_size(&param.kind)
                        .map(|param_fixed_size| count + param_fixed_size),
                    None => None,
                }
            },
        );

        let min_data_size = e
            .inputs
            .iter()
            .filter(|param| !param.indexed)
            .map(|param| min_data_size(&param.kind))
            .sum();

        let kinds: Vec<_> = e
            .inputs
            .iter()
            .map(|param| rust_type(&param.kind))
            .collect();

        let log_fields = names
            .iter()
            .zip(kinds.iter())
            .map(|(param_name, kind)| quote! { pub #param_name: #kind })
            .collect();

        let decode_indexed_fields = e
            .inputs
            .iter()
            .zip(names.iter())
            .filter(|(param, _)| param.indexed)
            .enumerate()
            .map(|(index, (param, name))| {
                let topic_index = index + 1;
                let topic_access = quote! { log.topics[#topic_index].as_ref() };
                let decode_topic = decode_topic(&name.to_string(), &param.kind, &topic_access);

                quote! {
                    #name: #decode_topic
                }
            })
            .collect();

        let decode_data = if e.inputs.iter().any(|input| !input.indexed) {
            let params: Vec<_> = e
                .inputs
                .iter()
                .filter(|input| !input.indexed)
                .map(|input| to_syntax_string(&input.kind))
                .collect();

            quote! {
                let mut values = ethabi::decode(&[#(#params),*], log.data.as_ref()).map_err(|e| format!("unable to decode log.data: {}", e))?;
            }
        } else {
            TokenStream::new()
        };

        // We go reverse in the iteration because we use a series of `.pop()` to correctly
        // extract elements.
        let decode_unindexed_fields = e
            .inputs
            .iter()
            .rev()
            .zip(names.iter().rev())
            .filter(|(param, _)| !param.indexed)
            .map(|(param, name)| {
                let data_access = quote! { values.pop().expect(INTERNAL_ERR) };
                let decode_topic = from_token(&param.kind, &data_access);

                quote! {
                    #name: #decode_topic
                }
            })
            .collect();

        Event {
            name: name.clone(),
            topic_hash: e.signature().to_fixed_bytes(),
            topic_count,
            fixed_data_size,
            min_data_size,
            log_fields,
            decode_indexed_fields,
            decode_unindexed_fields,
            decode_data,
        }
    }
}

impl Event {
    /// Generates rust interface for contract's event.
    pub fn generate_event(&self) -> TokenStream {
        let name = &self.name;
        let topic_count = &self.topic_count;
        let topic_hash_bytes: Vec<_> = self
            .topic_hash
            .iter()
            .map(|value| quote! { #value })
            .collect();
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());
        let log_fields = &self.log_fields;

        let decode_data = &self.decode_data;
        let mut decode_fields = Vec::with_capacity(
            self.decode_indexed_fields.len() + self.decode_unindexed_fields.len(),
        );
        decode_fields.extend(self.decode_indexed_fields.iter());
        decode_fields.extend(self.decode_unindexed_fields.iter());

        let must_decode_error_msg = format!(
            "Unable to decode logs.{} event: {{:#}}",
            self.name.to_upper_camel_case()
        );

        let min_data_size = &self.min_data_size;
        let log_match_data = match &self.fixed_data_size {
            Some(fixed_data_size) => {
                quote! {
                    if log.data.len() != #fixed_data_size {
                        return false;
                    }
                }
            }
            None => {
                quote! {
                    if log.data.len() < #min_data_size {
                        return false;
                    }
                }
            }
        };

        quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct #camel_name {
                #(#log_fields),*
            }

            impl #camel_name {
                // FIXME: We should generate the [u8; 32] directly and avoid hex_literal crate
                const TOPIC_ID: [u8; 32] = [#(#topic_hash_bytes),*];

                pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                    if log.topics.len() != #topic_count {
                        return false;
                    }

                    #log_match_data

                    return log.topics.get(0).expect("bounds already checked").as_ref()
                        == Self::TOPIC_ID;
                }

                pub fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                    #decode_data

                    Ok(Self {
                        #(#decode_fields),*
                    })
                }

                pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                    match Self::decode(log) {
                        Ok(v) => v,
                        Err(e) => panic!(#must_decode_error_msg, e),
                    }
                }
            }

            impl substreams_ethereum::Event for #camel_name {
                const NAME: &'static str = #name;
                fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                    Self::match_log(log)
                }
                fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                    Self::decode(log)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::assert_ast_eq;

    use super::Event;
    use quote::quote;

    #[test]
    fn test_empty_event() {
        let ethabi_event = ethabi::Event {
            name: "hello".into(),
            inputs: vec![],
            anonymous: false,
        };

        let e = Event::from((&ethabi_event.name, &ethabi_event));

        assert_ast_eq(
            e.generate_event(),
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct Hello {}
                impl Hello {
                    const TOPIC_ID: [u8; 32] = [
                        25u8,
                        255u8,
                        29u8,
                        33u8,
                        14u8,
                        6u8,
                        165u8,
                        62u8,
                        229u8,
                        14u8,
                        91u8,
                        173u8,
                        37u8,
                        250u8,
                        80u8,
                        154u8,
                        107u8,
                        0u8,
                        237u8,
                        57u8,
                        86u8,
                        149u8,
                        247u8,
                        217u8,
                        184u8,
                        43u8,
                        104u8,
                        21u8,
                        93u8,
                        158u8,
                        16u8,
                        101u8
                    ];
                    pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        if log.topics.len() != 1usize {
                            return false;
                        }
                        if log.data.len() != 0usize {
                            return false;
                        }
                        return log.topics.get(0).expect("bounds already checked").as_ref()
                            == Self::TOPIC_ID;
                    }
                    pub fn decode(
                        log: &substreams_ethereum::pb::eth::v1::Log
                    ) -> Result<Self, String> {
                        Ok(Self {})
                    }
                    pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                        match Self::decode(log) {
                            Ok(v) => v,
                            Err(e) => panic!("Unable to decode logs.Hello event: {:#}", e),
                        }
                    }
                }
                impl substreams_ethereum::Event for Hello {
                    const NAME: &'static str = "hello";
                    fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        Self::match_log(log)
                    }
                    fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                        Self::decode(log)
                    }
                }
            },
        );
    }

    #[test]
    fn test_event_with_one_input() {
        let ethabi_event = ethabi::Event {
            name: "one".into(),
            inputs: vec![ethabi::EventParam {
                name: "foo".into(),
                kind: ethabi::ParamType::Address,
                indexed: true,
            }],
            anonymous: false,
        };

        let e = Event::from((&ethabi_event.name, &ethabi_event));

        assert_ast_eq(
            e.generate_event(),
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct One {
                    pub foo: Vec<u8>
                }
                impl One {
                    const TOPIC_ID: [u8; 32] = [
                        242u8,
                        136u8,
                        154u8,
                        196u8,
                        193u8,
                        137u8,
                        107u8,
                        13u8,
                        185u8,
                        251u8,
                        115u8,
                        123u8,
                        176u8,
                        143u8,
                        246u8,
                        233u8,
                        171u8,
                        71u8,
                        223u8,
                        216u8,
                        191u8,
                        53u8,
                        192u8,
                        221u8,
                        120u8,
                        140u8,
                        192u8,
                        19u8,
                        121u8,
                        40u8,
                        22u8,
                        66u8
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
                        log: &substreams_ethereum::pb::eth::v1::Log
                    ) -> Result<Self, String> {
                        Ok(Self {
                            foo: ethabi::decode(
                                    &[ethabi::ParamType::Address],
                                    log.topics[1usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'foo' from topic of type 'address': {}",
                                    e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec()
                        })
                    }
                    pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                        match Self::decode(log) {
                            Ok(v) => v,
                            Err(e) => panic!("Unable to decode logs.One event: {:#}", e),
                        }
                    }
                }
                impl substreams_ethereum::Event for One {
                    const NAME: &'static str = "one";
                    fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        Self::match_log(log)
                    }
                    fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                        Self::decode(log)
                    }
                }
            },
        );
    }

    #[test]
    fn test_event_erc20_transfer() {
        let ethabi_event = ethabi::Event {
            name: "Transfer".into(),
            inputs: vec![
                ethabi::EventParam {
                    name: "from".into(),
                    kind: ethabi::ParamType::Address,
                    indexed: true,
                },
                ethabi::EventParam {
                    name: "to".into(),
                    kind: ethabi::ParamType::Address,
                    indexed: true,
                },
                ethabi::EventParam {
                    name: "quantity".into(),
                    kind: ethabi::ParamType::Uint(256),
                    indexed: false,
                },
            ],
            anonymous: false,
        };

        let e = Event::from((&ethabi_event.name, &ethabi_event));

        assert_ast_eq(
            e.generate_event(),
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct Transfer {
                    pub from: Vec<u8>,
                    pub to: Vec<u8>,
                    pub quantity: ethabi::Uint
                }
                impl Transfer {
                    const TOPIC_ID: [u8; 32] = [
                        221u8,
                        242u8,
                        82u8,
                        173u8,
                        27u8,
                        226u8,
                        200u8,
                        155u8,
                        105u8,
                        194u8,
                        176u8,
                        104u8,
                        252u8,
                        55u8,
                        141u8,
                        170u8,
                        149u8,
                        43u8,
                        167u8,
                        241u8,
                        99u8,
                        196u8,
                        161u8,
                        22u8,
                        40u8,
                        245u8,
                        90u8,
                        77u8,
                        245u8,
                        35u8,
                        179u8,
                        239u8
                    ];
                    pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        if log.topics.len() != 3usize {
                            return false;
                        }
                        if log.data.len() != 32usize {
                            return false;
                        }
                        return log.topics.get(0).expect("bounds already checked").as_ref()
                            == Self::TOPIC_ID;
                    }
                    pub fn decode(
                        log: &substreams_ethereum::pb::eth::v1::Log
                    ) -> Result<Self, String> {
                        let mut values = ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.data.as_ref()
                            )
                            .map_err(|e| format!("unable to decode log.data: {}", e))?;
                        Ok(Self {
                            from: ethabi::decode(
                                    &[ethabi::ParamType::Address],
                                    log.topics[1usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'from' from topic of type 'address': {}",
                                    e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            to: ethabi::decode(
                                    &[ethabi::ParamType::Address],
                                    log.topics[2usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'to' from topic of type 'address': {}", e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            quantity: values
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_uint()
                                .expect(INTERNAL_ERR)
                        })
                    }
                    pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                        match Self::decode(log) {
                            Ok(v) => v,
                            Err(e) => panic!("Unable to decode logs.Transfer event: {:#}", e),
                        }
                    }
                }
                impl substreams_ethereum::Event for Transfer {
                    const NAME: &'static str = "Transfer";
                    fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        Self::match_log(log)
                    }
                    fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                        Self::decode(log)
                    }
                }
            },
        );
    }

    #[test]
    fn test_event_erc721_transfer() {
        let ethabi_event = ethabi::Event {
            name: "Transfer".into(),
            inputs: vec![
                ethabi::EventParam {
                    name: "from".into(),
                    kind: ethabi::ParamType::Address,
                    indexed: true,
                },
                ethabi::EventParam {
                    name: "to".into(),
                    kind: ethabi::ParamType::Address,
                    indexed: true,
                },
                ethabi::EventParam {
                    name: "token_id".into(),
                    kind: ethabi::ParamType::Uint(256),
                    indexed: true,
                },
            ],
            anonymous: false,
        };

        let e = Event::from((&ethabi_event.name, &ethabi_event));

        assert_ast_eq(
            e.generate_event(),
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct Transfer {
                    pub from: Vec<u8>,
                    pub to: Vec<u8>,
                    pub token_id: ethabi::Uint
                }
                impl Transfer {
                    const TOPIC_ID: [u8; 32] = [
                        221u8,
                        242u8,
                        82u8,
                        173u8,
                        27u8,
                        226u8,
                        200u8,
                        155u8,
                        105u8,
                        194u8,
                        176u8,
                        104u8,
                        252u8,
                        55u8,
                        141u8,
                        170u8,
                        149u8,
                        43u8,
                        167u8,
                        241u8,
                        99u8,
                        196u8,
                        161u8,
                        22u8,
                        40u8,
                        245u8,
                        90u8,
                        77u8,
                        245u8,
                        35u8,
                        179u8,
                        239u8
                    ];
                    pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        if log.topics.len() != 4usize {
                            return false;
                        }
                        if log.data.len() != 0usize {
                            return false;
                        }
                        return log.topics.get(0).expect("bounds already checked").as_ref()
                            == Self::TOPIC_ID;
                    }
                    pub fn decode(
                        log: &substreams_ethereum::pb::eth::v1::Log
                    ) -> Result<Self, String> {
                        Ok(Self {
                            from: ethabi::decode(
                                    &[ethabi::ParamType::Address],
                                    log.topics[1usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'from' from topic of type 'address': {}",
                                    e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            to: ethabi::decode(
                                    &[ethabi::ParamType::Address],
                                    log.topics[2usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'to' from topic of type 'address': {}", e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            token_id: ethabi::decode(
                                    &[ethabi::ParamType::Uint(256usize)],
                                    log.topics[3usize].as_ref()
                                )
                                .map_err(|e| format!(
                                    "unable to decode param 'token_id' from topic of type 'uint256': {}",
                                    e
                                ))?
                                .pop()
                                .expect(INTERNAL_ERR)
                                .into_uint()
                                .expect(INTERNAL_ERR)
                        })
                    }
                    pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Self {
                        match Self::decode(log) {
                            Ok(v) => v,
                            Err(e) => panic!("Unable to decode logs.Transfer event: {:#}", e),
                        }
                    }
                }
                impl substreams_ethereum::Event for Transfer {
                    const NAME: &'static str = "Transfer";
                    fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                        Self::match_log(log)
                    }
                    fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Self, String> {
                        Self::decode(log)
                    }
                }
            },
        );
    }
}
