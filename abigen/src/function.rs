// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::to_syntax_string;

use super::{from_token, get_output_kinds, param_names, rust_type, to_token};

struct Inputs {
    tokenize: Vec<TokenStream>,
    decoded_values: TokenStream,
    decoded_fields: Vec<TokenStream>,
    fields: Vec<TokenStream>,
}

struct Outputs {
    /// Decoding implementation.
    implementation: TokenStream,
    /// Decode result.
    result: TokenStream,

    count: usize,
}

/// Structure used to generate contract's function interface.
pub struct Function {
    /// Function name.
    pub(crate) name: String,

    short_signature: [u8; 4],
    /// Function input params.
    inputs: Inputs,
    /// Function output params.
    outputs: Outputs,
}

impl<'a> From<(&'a String, &'a ethabi::Function)> for Function {
    fn from((name, f): (&'a String, &'a ethabi::Function)) -> Self {
        // [param0, hello_world, param2]
        let input_names = param_names(&f.inputs);

        // [Uint, Bytes, Vec<Uint>]
        let input_kinds: Vec<_> = f
            .inputs
            .iter()
            .map(|param| rust_type(&param.kind))
            .collect();

        let input_struct_fields = input_names
            .iter()
            .zip(input_kinds.iter())
            .map(|(param_name, kind)| quote! { pub #param_name: #kind })
            .collect();

        let input_ethabi_param_types = if !f.inputs.is_empty() {
            let params: Vec<_> = f
                .inputs
                .iter()
                .map(|input| to_syntax_string(&input.kind))
                .collect();

            quote! {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }

                let mut values = ethabi::decode(&[#(#params),*], maybe_data.unwrap())
                        .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
            }
        } else {
            quote! {}
        };

        // We go reverse in the iteration because we use a series of `.pop()` to correctly
        // extract elements and put them in the good fields.
        let input_struct_decoded_fields = f
            .inputs
            .iter()
            .zip(input_names.iter())
            .map(|(param, name)| {
                let data_access = quote! { values.pop().expect(INTERNAL_ERR) };
                let decode_input = from_token(&param.kind, &data_access);
                quote! {
                   #name: #decode_input
                }
            })
            .collect();

        // [Token::Uint(param0.into()), Token::Bytes(hello_world.into()), Token::Array(param2.into_iter().map(Into::into).collect())]
        let tokenize: Vec<_> = input_names
            .iter()
            .zip(f.inputs.iter())
            .map(|(param_name, param)| to_token(&quote! { self.#param_name }, &param.kind, false))
            .collect();

        let output_result = get_output_kinds(&f.outputs);

        let output_param_types: Vec<_> = f
            .outputs
            .iter()
            .map(|output| to_syntax_string(&output.kind))
            .collect();

        let output_implementation = match f.outputs.len() {
            0 => quote! {},
            1 => {
                let decode_param_type = &output_param_types[0];
                let data_access =
                    quote! { values.pop().expect("one output data should have existed") };
                let decode_input = from_token(&f.outputs[0].kind, &data_access);

                quote! {
                    pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<#output_result, String> {
                        Self::output(call.return_data.as_ref())
                    }

                    pub fn output(data: &[u8]) -> Result<#output_result, String> {
                        let mut values = ethabi::decode(&[#decode_param_type], data.as_ref())
                        .map_err(|e| format!("unable to decode output data: {:?}", e))?;

                        Ok(#decode_input)
                    }
                }
            }
            _ => {
                let output_tuple_fields: Vec<_> = f
                    .outputs
                    .iter()
                    .map(|input| to_syntax_string(&input.kind))
                    .collect();

                let output_ethabi_decoded_values = quote! {
                    let mut values = ethabi::decode(&[#(#output_tuple_fields),*], data.as_ref())
                            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                    values.reverse();
                };

                // We go reverse in the iteration because we use a series of `.pop()` to correctly
                // extract elements and put them in the good fields.
                let output_tuple_decoded_fields: Vec<TokenStream> = f
                    .outputs
                    .iter()
                    .map(|param| {
                        let data_access = quote! { values.pop().expect(INTERNAL_ERR) };
                        let decode_input = from_token(&param.kind, &data_access);
                        quote! {
                           #decode_input
                        }
                    })
                    .collect();

                quote! {
                    pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<#output_result, String> {
                        Self::output(call.return_data.as_ref())
                    }

                    pub fn output(data: &[u8]) -> Result<#output_result, String> {
                        #output_ethabi_decoded_values

                        Ok((#(#output_tuple_decoded_fields),*))
                    }
                }
            }
        };

        // The allow deprecated only applies to the field 'constant', but
        // due to this issue: https://github.com/rust-lang/rust/issues/60681
        // it must go on the entire struct
        #[allow(deprecated)]
        Function {
            name: name.clone(),
            short_signature: f.short_signature(),
            inputs: Inputs {
                tokenize,
                decoded_values: input_ethabi_param_types,
                decoded_fields: input_struct_decoded_fields,
                fields: input_struct_fields,
            },
            outputs: Outputs {
                implementation: output_implementation,
                result: output_result,
                count: f.outputs.len(),
            },
        }
    }
}

impl Function {
    /// Generates the interface for contract's function.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());

        let signature_hash_bytes: Vec<_> = self
            .short_signature
            .iter()
            .map(|value| quote! { #value })
            .collect();

        let function_fields = &self.inputs.fields;
        let tokenize = &self.inputs.tokenize;
        let decoded_input_values = &self.inputs.decoded_values;
        let decoded_input_fields = &self.inputs.decoded_fields;

        let output_implementation = &self.outputs.implementation;
        let outputs_result = &self.outputs.result;

        let call_implementation = match self.outputs.count {
            0 => quote! {},
            _ => quote! {
                pub fn call(&self, address: Vec<u8>) -> Option<#outputs_result> {
                    use substreams_ethereum::pb::eth::rpc;

                    let rpc_calls = rpc::RpcCalls {
                        calls: vec![rpc::RpcCall {
                            to_addr: address,
                            data: self.encode(),
                        }],
                    };

                    let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                    let response = responses.get(0).expect("one response should have existed");

                    if response.failed {
                        return None;
                    }

                    match Self::output(response.raw.as_ref()) {
                        Ok(data) => Some(data),
                        Err(err) => {
                            use substreams_ethereum::Function;

                            substreams::log::info!(
                                "Call output for function `{}` failed to decode with error: {}",
                                Self::NAME,
                                err
                            );
                            None
                        }
                    }
                }
            },
        };

        quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct #camel_name {
                #(#function_fields),*
            }

            impl #camel_name {
                const METHOD_ID: [u8; 4] = [#(#signature_hash_bytes),*];

                pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
                    #decoded_input_values

                    Ok(Self {
                        #(#decoded_input_fields),*
                    })
                }

                pub fn encode(&self) -> Vec<u8> {
                    let data = ethabi::encode(&[#(#tokenize),*]);

                    let mut encoded = Vec::with_capacity(4 + data.len());
                    encoded.extend(Self::METHOD_ID);
                    encoded.extend(data);

                    encoded
                }

                #output_implementation

                pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                    match call.input.get(0..4) {
                        Some(signature) => Self::METHOD_ID == signature,
                        None => false
                    }
                }

                #call_implementation
            }

            impl substreams_ethereum::Function for #camel_name {
                const NAME: &'static str = #name;
                fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                    Self::match_call(call)
                }
                fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
                    Self::decode(call)
                }
            }
        }
    }
}
