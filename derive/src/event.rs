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

impl<'a> From<&'a ethabi::Event> for Event {
    fn from(e: &'a ethabi::Event) -> Self {
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
            .filter(|param| param.indexed)
            .zip(names.iter())
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
            .filter(|param| !param.indexed)
            .zip(names.iter().rev())
            .map(|(param, name)| {
                let data_access = quote! { values.pop().expect(INTERNAL_ERR) };
                let decode_topic = from_token(&param.kind, &data_access);

                quote! {
                    #name: #decode_topic
                }
            })
            .collect();

        Event {
            name: e.name.clone(),
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

                pub fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<#camel_name, String> {
                    #decode_data

                    Ok(Self {
                        #(#decode_fields),*
                    })
                }

                pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> #camel_name {
                    match Self::decode(log) {
                        Ok(v) => v,
                        Err(e) => panic!(#must_decode_error_msg, e),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Event;
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn test_empty_event() {
        let ethabi_event = ethabi::Event {
            name: "hello".into(),
            inputs: vec![],
            anonymous: false,
        };

        let e = Event::from(&ethabi_event);

        let expected = quote! {
            pub mod hello {
                use ethabi;
                use super::INTERNAL_ERR;

                pub fn event() -> ethabi::Event {
                    ethabi::Event {
                        name: "Hello".into(),
                        inputs: vec![],
                        anonymous: false,
                    }
                }

                pub fn parse_log(log: ethabi::RawLog) -> ethabi::Result<super::super::logs::Hello> {
                    let e = event();
                    let mut log = e.parse_log(log)?.params.into_iter();
                    let result = super::super::logs::Hello {};
                    Ok(result)
                }
            }
        };

        assert_eq!(
            expected.to_string(),
            e.generate_event().to_string(),
            "\n\nActual:\n {}",
            pretty_print_item(e.generate_event())
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

        let e = Event::from(&ethabi_event);

        let expected = quote! {
            pub mod one {
                use ethabi;
                use super::INTERNAL_ERR;

                pub fn event() -> ethabi::Event {
                    ethabi::Event {
                        name: "One".into(),
                        inputs: vec![ethabi::EventParam {
                            name: "foo".to_owned(),
                            kind: ethabi::ParamType::Address,
                            indexed: false
                        }],
                        anonymous: false,
                    }
                }

                pub fn parse_log(log: ethabi::RawLog) -> ethabi::Result<super::super::logs::One> {
                    let e = event();
                    let mut log = e.parse_log(log)?.params.into_iter();
                    let result = super::super::logs::One {
                        foo: log.next().expect(INTERNAL_ERR).value.into_address().expect(INTERNAL_ERR)
                    };
                    Ok(result)
                }
            }
        };

        assert_eq!(
            expected.to_string(),
            e.generate_event().to_string(),
            "\n\nActual:\n {}",
            pretty_print_item(e.generate_event())
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

        let e = Event::from(&ethabi_event);

        let expected = quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Transfer {
                pub from: Vec<u8>,
                pub to: Vec<u8>,
                pub quantity: ethabi::Uint
            }
            impl Transfer {
                const TOPIC_ID: [u8; 32] = [221u8 , 242u8 , 82u8 , 173u8 , 27u8 , 226u8 , 200u8 , 155u8 , 105u8 , 194u8 , 176u8 , 104u8 , 252u8 , 55u8 , 141u8 , 170u8 , 149u8 , 43u8 , 167u8 , 241u8 , 99u8 , 196u8 , 161u8 , 22u8 , 40u8 , 245u8 , 90u8 , 77u8 , 245u8 , 35u8 , 179u8 , 239u8];

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
                ) -> Result<Transfer, String> {
                    let values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], log.data)?;
                    Self {
                        from: ethabi::decode(&[ethabi::ParamType::Address], log.topics[1usize].as_ref())?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        to: ethabi::decode(&[ethabi::ParamType::Address], log.topics[2usize].as_ref())?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        quantity: values.pop().expect(INTERNAL_ERR).into_uint().expect(INTERNAL_ERR)
                    }
                }
            }
        };

        assert_eq!(
            expected.to_string(),
            e.generate_event().to_string(),
            "\n\nActual:\n {}",
            pretty_print_item(e.generate_event())
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

        let e = Event::from(&ethabi_event);

        let expected = quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Transfer {
                pub from: Vec<u8>,
                pub to: Vec<u8>,
                pub token_id: ethabi::Uint
            }
            impl Transfer {
                const TOPIC_ID: [u8; 32] = [221u8 , 242u8 , 82u8 , 173u8 , 27u8 , 226u8 , 200u8 , 155u8 , 105u8 , 194u8 , 176u8 , 104u8 , 252u8 , 55u8 , 141u8 , 170u8 , 149u8 , 43u8 , 167u8 , 241u8 , 99u8 , 196u8 , 161u8 , 22u8 , 40u8 , 245u8 , 90u8 , 77u8 , 245u8 , 35u8 , 179u8 , 239u8];

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
                ) -> Result<Transfer, String> {
                    Self {
                        from: ethabi::decode(&[ethabi::ParamType::Address], log.topics[1usize].as_ref())?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        to: ethabi::decode(&[ethabi::ParamType::Address], log.topics[2usize].as_ref())?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        token_id: ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[3usize].as_ref()
                            )?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                    }
                }
            }
        };

        assert_eq!(
            expected.to_string(),
            e.generate_event().to_string(),
            "\n\nActual:\n {}",
            pretty_print_item(e.generate_event())
        );
    }

    fn pretty_print_item(item: proc_macro2::TokenStream) -> String {
        // Maybe just if it actually fails?
        let mod_wrap = quote! {
            mod pp {
                #item
            }
        };

        let as_string = mod_wrap.to_string();

        let item = match syn::parse2(mod_wrap) {
            Ok(item) => item,
            Err(err) => return format!("unable to parse AST (due to {}): {}", err, as_string),
        };

        let file = syn::File {
            attrs: vec![],
            items: vec![item],
            shebang: None,
        };

        prettyplease::unparse(&file)
    }
}
