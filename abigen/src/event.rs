use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
    decode_topic, element_stride, fixed_data_size, is_long_tuple, min_data_size, read_at,
    rust_type_indexed,
};

use super::rust_type;

/// Structure used to generate contract's event interface.
pub struct Event {
    /// Name of the event, de-duped and sanitized for Rust
    pub(crate) name: String,
    /// Original name of the event as defined in the ABI
    original_name: String,
    topic_hash: [u8; 32],
    topic_count: usize,
    min_data_size: usize,
    has_any_long_tuple: bool,
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
            .map(|param| match param.indexed {
                true => rust_type_indexed(&param.kind),
                false => rust_type(&param.kind),
            })
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
                let topic_access =
                    quote! { log.topic(#topic_index).expect("bounds already checked") };
                let decode_topic = decode_topic(&name.to_string(), &param.kind, &topic_access);

                quote! {
                    #name: #decode_topic
                }
            })
            .collect();

        // Pair each unindexed parameter with its own name: `names` covers every
        // input, so an indexed parameter ahead of an unindexed one would otherwise
        // shift the pairing.
        let unindexed: Vec<_> = e
            .inputs
            .iter()
            .zip(names.iter())
            .filter(|(param, _)| !param.indexed)
            .collect();

        // Each parameter's head word sits at an offset known once the ABI is read, so
        // every read is emitted against `log.data()` directly. A dynamic parameter
        // spends its head word on a pointer and is followed from there.
        let data_token = quote! { log.data() };

        let mut head_offset = 0usize;
        let decode_unindexed_fields: Vec<TokenStream> = unindexed
            .iter()
            .map(|(param, name)| {
                let offset = syn::Index::from(head_offset);
                let read = read_at(
                    &param.kind,
                    &name.to_string(),
                    &data_token,
                    &quote! { #offset },
                );
                head_offset += element_stride(&param.kind);

                quote! { #name: #read }
            })
            .collect();

        let decode_data = TokenStream::new();

        Event {
            name: name.clone(),
            original_name: e.name.clone(),
            topic_hash: e.signature().to_fixed_bytes(),
            topic_count,
            fixed_data_size,
            min_data_size,
            has_any_long_tuple: e.inputs.iter().any(|input| is_long_tuple(&input.kind)),
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
        let original_name = &self.original_name;
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

        let min_data_size = &self.min_data_size;
        let log_match_data = match &self.fixed_data_size {
            Some(fixed_data_size) => {
                quote! {
                    if log.data().len() != #fixed_data_size {
                        return false;
                    }
                }
            }
            None => {
                quote! {
                    if log.data().len() < #min_data_size {
                        return false;
                    }
                }
            }
        };

        // Every read below sits at an offset inside the first `fixed_data_size` bytes,
        // so a buffer at least that long makes all of them in-bounds. `ethabi` reads
        // the parameters it was given and ignores whatever follows, so a longer buffer
        // is accepted here too.
        let decode_match_data = match &self.fixed_data_size {
            // An event with no unindexed parameter reads nothing out of the data
            // section, so there is no length to require.
            Some(0) => TokenStream::new(),
            Some(fixed_data_size) => {
                quote! {
                    if log.data().len() < #fixed_data_size {
                        return Err(format!(
                            "data too short, expected at least {}, got {}",
                            #fixed_data_size,
                            log.data().len()
                        ));
                    }
                }
            }
            None => {
                quote! {
                    if log.data().len() < #min_data_size {
                        return Err(format!(
                            "data too short, expected at least {}, got {}",
                            #min_data_size,
                            log.data().len()
                        ));
                    }
                }
            }
        };

        let struct_header = if self.has_any_long_tuple {
            quote! {
                #[derive(Clone)]
            }
        } else {
            quote! {
                #[derive(Debug, Clone, PartialEq)]
            }
        };

        quote! {
            #struct_header
            pub struct #camel_name {
                #(#log_fields),*
            }

            impl #camel_name {
                const TOPIC_ID: [u8; 32] = [#(#topic_hash_bytes),*];

                pub fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                    if log.topic_count() != #topic_count {
                        return false;
                    }

                    #log_match_data

                    return log.topic(0).expect("bounds already checked")
                        == Self::TOPIC_ID;
                }

                pub fn decode<L: substreams_ethereum::LogLike>(log: &L) -> Result<Self, String> {
                    // Reading an indexed parameter indexes into the topics, so the count
                    // is checked here too: `decode` is public and callable without
                    // `match_log` having run first.
                    if log.topic_count() != #topic_count {
                        return Err(format!(
                            "unexpected topic count, expected {}, got {}",
                            #topic_count,
                            log.topic_count()
                        ));
                    }

                    #decode_match_data

                    #decode_data

                    Ok(Self {
                        #(#decode_fields),*
                    })
                }
            }

            impl substreams_ethereum::Event for #camel_name {
                const NAME: &'static str = #original_name;
                fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                    Self::match_log(log)
                }
                fn decode<L: substreams_ethereum::LogLike>(log: &L) -> Result<Self, String> {
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
                    pub fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        if log.topic_count() != 1usize {
                            return false;
                        }
                        if log.data().len() != 0usize {
                            return false;
                        }
                        return log.topic(0).expect("bounds already checked")
                            == Self::TOPIC_ID;
                    }
                    pub fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
                        if log.topic_count() != 1usize {
                            return Err(
                                format!(
                                    "unexpected topic count, expected {}, got {}", 1usize, log
                                    .topic_count()
                                )
                            );
                        }
                        Ok(Self {})
                    }
                }
                impl substreams_ethereum::Event for Hello {
                    const NAME: &'static str = "hello";
                    fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        Self::match_log(log)
                    }
                    fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
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
                    pub fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        if log.topic_count() != 2usize {
                            return false;
                        }
                        if log.data().len() != 0usize {
                            return false;
                        }
                        return log.topic(0).expect("bounds already checked")
                            == Self::TOPIC_ID;
                    }
                    pub fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
                        if log.topic_count() != 2usize {
                            return Err(
                                format!(
                                    "unexpected topic count, expected {}, got {}", 2usize, log
                                    .topic_count()
                                )
                            );
                        }
                        Ok(Self {
                            foo: substreams_ethereum::abi::read_address(
                                log.topic(1usize).expect("bounds already checked"),
                                0,
                                "foo"
                            )?
                        })
                    }
                }
                impl substreams_ethereum::Event for One {
                    const NAME: &'static str = "one";
                    fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        Self::match_log(log)
                    }
                    fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
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
                    pub quantity: substreams::scalar::BigInt
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
                    pub fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        if log.topic_count() != 3usize {
                            return false;
                        }
                        if log.data().len() != 32usize {
                            return false;
                        }
                        return log.topic(0).expect("bounds already checked")
                            == Self::TOPIC_ID;
                    }
                    pub fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
                        if log.topic_count() != 3usize {
                            return Err(
                                format!(
                                    "unexpected topic count, expected {}, got {}", 3usize, log
                                    .topic_count()
                                )
                            );
                        }
                        if log.data().len() < 32usize {
                            return Err(
                                format!(
                                    "data too short, expected at least {}, got {}", 32usize, log
                                    .data().len()
                                )
                            );
                        }
                        Ok(Self {
                            from: substreams_ethereum::abi::read_address(
                                log.topic(1usize).expect("bounds already checked"),
                                0,
                                "from"
                            )?,
                            to: substreams_ethereum::abi::read_address(
                                log.topic(2usize).expect("bounds already checked"),
                                0,
                                "to"
                            )?,
                            quantity: substreams_ethereum::abi::read_uint(
                                log.data(),
                                0,
                                "quantity"
                            )?
                        })
                    }
                }
                impl substreams_ethereum::Event for Transfer {
                    const NAME: &'static str = "Transfer";
                    fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        Self::match_log(log)
                    }
                    fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
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
                    pub token_id: substreams::scalar::BigInt
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
                    pub fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        if log.topic_count() != 4usize {
                            return false;
                        }
                        if log.data().len() != 0usize {
                            return false;
                        }
                        return log.topic(0).expect("bounds already checked")
                            == Self::TOPIC_ID;
                    }
                    pub fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
                        if log.topic_count() != 4usize {
                            return Err(
                                format!(
                                    "unexpected topic count, expected {}, got {}", 4usize, log
                                    .topic_count()
                                )
                            );
                        }
                        Ok(Self {
                            from: substreams_ethereum::abi::read_address(
                                log.topic(1usize).expect("bounds already checked"),
                                0,
                                "from"
                            )?,
                            to: substreams_ethereum::abi::read_address(
                                log.topic(2usize).expect("bounds already checked"),
                                0,
                                "to"
                            )?,
                            token_id: substreams_ethereum::abi::read_uint(
                                log.topic(3usize).expect("bounds already checked"),
                                0,
                                "token_id"
                            )?
                        })
                    }
                }
                impl substreams_ethereum::Event for Transfer {
                    const NAME: &'static str = "Transfer";
                    fn match_log<L: substreams_ethereum::LogLike>(log: &L) -> bool {
                        Self::match_log(log)
                    }
                    fn decode<L: substreams_ethereum::LogLike>(
                        log: &L
                    ) -> Result<Self, String> {
                        Self::decode(log)
                    }
                }
            },
        );
    }
}
