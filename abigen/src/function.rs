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

use crate::{element_stride, is_long_tuple, read_at, write_at};

use super::{get_output_kinds, param_names, rust_type};

struct Inputs {
    writes: Vec<TokenStream>,
    head_width: usize,
    decoded_values: TokenStream,
    decoded_fields: Vec<TokenStream>,
    fields: Vec<TokenStream>,
    has_any_long_tuple: bool,
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
    /// Name of the function, de-duped and sanitized for Rust
    pub(crate) name: String,
    /// Original name of the function as defined in the ABI
    original_name: String,

    short_signature: [u8; 4],
    /// Function input params.
    inputs: Inputs,
    /// Function output params.
    outputs: Outputs,
}

impl<'a> From<(String, &'a ethabi::Function)> for Function {
    fn from((name, f): (String, &'a ethabi::Function)) -> Self {
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

        // The four selector bytes sit ahead of the parameter list, so every head
        // offset is measured from the input past them.
        let input_ethabi_param_types = if !f.inputs.is_empty() {
            quote! {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let data = maybe_data.unwrap();
            }
        } else {
            quote! {}
        };

        let data_token = quote! { data };
        let mut head_offset = 0usize;
        let input_struct_decoded_fields = f
            .inputs
            .iter()
            .zip(input_names.iter())
            .map(|(param, name)| {
                let offset = syn::Index::from(head_offset);
                let decode_input = read_at(
                    &param.kind,
                    &name.to_string(),
                    &data_token,
                    &quote! { #offset },
                );
                head_offset += element_stride(&param.kind);

                quote! {
                   #name: #decode_input
                }
            })
            .collect();

        // The head section holds one entry per parameter, so a dynamic parameter's
        // tail offset is measured from the end of all of them.
        let head_width: usize = f
            .inputs
            .iter()
            .map(|param| element_stride(&param.kind))
            .sum();

        let mut slot = 0usize;
        let writes: Vec<_> = input_names
            .iter()
            .zip(f.inputs.iter())
            .map(|(param_name, param)| {
                let at = syn::Index::from(slot);
                slot += element_stride(&param.kind);

                write_at(
                    &param.kind,
                    &quote! { self.#param_name },
                    &syn::Ident::new("out", Span::call_site()),
                    &syn::Ident::new("base", Span::call_site()),
                    &quote! { #at },
                    0,
                )
            })
            .collect();

        let output_result = get_output_kinds(&f.outputs);

        let output_implementation = match f.outputs.len() {
            0 => quote! {},
            1 => {
                let decode_input = read_at(
                    &f.outputs[0].kind,
                    "output",
                    &quote! { data },
                    &quote! { 0 },
                );

                quote! {
                    pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<#output_result, String> {
                        Self::output(call.return_data.as_ref())
                    }

                    pub fn output(data: &[u8]) -> Result<#output_result, String> {
                        Ok(#decode_input)
                    }
                }
            }
            _ => {
                let mut head_offset = 0usize;
                let output_tuple_decoded_fields: Vec<TokenStream> = f
                    .outputs
                    .iter()
                    .map(|param| {
                        let offset = syn::Index::from(head_offset);
                        let decode_input =
                            read_at(&param.kind, "output", &quote! { data }, &quote! { #offset });
                        head_offset += element_stride(&param.kind);

                        decode_input
                    })
                    .collect();

                quote! {
                    pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<#output_result, String> {
                        Self::output(call.return_data.as_ref())
                    }

                    pub fn output(data: &[u8]) -> Result<#output_result, String> {
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
            name,
            original_name: f.name.clone(),
            short_signature: f.short_signature(),
            inputs: Inputs {
                writes,
                head_width,
                decoded_values: input_ethabi_param_types,
                decoded_fields: input_struct_decoded_fields,
                fields: input_struct_fields,
                has_any_long_tuple: f.inputs.iter().any(|param| is_long_tuple(&param.kind)),
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
        let original_name = &self.original_name;
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());

        let signature_hash_bytes: Vec<_> = self
            .short_signature
            .iter()
            .map(|value| quote! { #value })
            .collect();

        let function_fields = &self.inputs.fields;
        let writes = &self.inputs.writes;
        let head_width = self.inputs.head_width;
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

        let rpc_decodable_implementation = match self.outputs.count {
            0 => quote! {},
            _ => quote! {
                impl substreams_ethereum::rpc::RPCDecodable<#outputs_result> for #camel_name {
                    fn output(data: &[u8]) -> Result<#outputs_result, String> {
                    Self::output(data)
                    }
                }
            },
        };

        let struct_header = if self.inputs.has_any_long_tuple {
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
                    let mut out: Vec<u8> = Vec::with_capacity(4 + #head_width);
                    out.extend(Self::METHOD_ID);

                    let base = substreams_ethereum::abi::reserve_head(&mut out, #head_width);
                    #(#writes;)*

                    out
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
                const NAME: &'static str = #original_name;
                fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                    Self::match_call(call)
                }
                fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
                    Self::decode(call)
                }
                fn encode(&self) -> Vec<u8> {
                    self.encode()
                }
            }

            #rpc_decodable_implementation
        }
    }
}
