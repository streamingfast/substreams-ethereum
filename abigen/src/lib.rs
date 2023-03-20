// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "256"]

extern crate proc_macro;

mod assertions;
pub mod build;
// mod constructor;
mod contract;
mod event;
mod function;

use anyhow::format_err;
// use ethabi::{Contract, Error, Param, ParamType, Result};
use ethabi::{Contract, Error, Param, ParamType};
use heck::ToSnakeCase;
use proc_macro2::Span;
// use heck::ToSnakeCase;
use quote::{quote, ToTokens};
use syn::Index;
use std::{
    borrow::Cow,
    env, fs,
    path::{Path, PathBuf},
};

pub fn generate_abi_code<S: AsRef<str>>(
    path: S,
) -> Result<proc_macro2::TokenStream, anyhow::Error> {
    let normalized_path = normalize_path(path.as_ref())?;
    let source_file = fs::File::open(&normalized_path).map_err(|_| {
        Error::Other(Cow::Owned(format!(
            "Cannot load contract abi from `{}`",
            normalized_path.display()
        )))
    })?;
    let contract = Contract::load(source_file)?;
    let c = contract::Contract::from(&contract);
    Ok(c.generate())
}

fn normalize_path<S: AsRef<Path>>(relative_path: S) -> Result<PathBuf, anyhow::Error> {
    // workaround for https://github.com/rust-lang/rust/issues/43860
    let cargo_toml_directory =
        env::var("CARGO_MANIFEST_DIR").map_err(|_| format_err!("Cannot find manifest file"))?;
    let mut path: PathBuf = cargo_toml_directory.into();
    path.push(relative_path);
    Ok(path)
}

fn to_syntax_string(param_type: &ethabi::ParamType) -> proc_macro2::TokenStream {
    match *param_type {
        ParamType::Address => quote! { ethabi::ParamType::Address },
        ParamType::Bytes => quote! { ethabi::ParamType::Bytes },
        ParamType::Int(x) => quote! { ethabi::ParamType::Int(#x) },
        ParamType::Uint(x) => quote! { ethabi::ParamType::Uint(#x) },
        ParamType::Bool => quote! { ethabi::ParamType::Bool },
        ParamType::String => quote! { ethabi::ParamType::String },
        ParamType::Array(ref param_type) => {
            let param_type_quote = to_syntax_string(param_type);
            quote! { ethabi::ParamType::Array(Box::new(#param_type_quote)) }
        }
        ParamType::FixedBytes(x) => quote! { ethabi::ParamType::FixedBytes(#x) },
        ParamType::FixedArray(ref param_type, ref x) => {
            let param_type_quote = to_syntax_string(param_type);
            quote! { ethabi::ParamType::FixedArray(Box::new(#param_type_quote), #x) }
        }
        ParamType::Tuple(ref v) => {
            let param_type_quotes = v.iter().map(|x| to_syntax_string(x));
            quote! { ethabi::ParamType::Tuple(vec![#(#param_type_quotes),*]) }
        }
    }
}

// fn to_ethabi_param_vec<'a, P: 'a>(params: P) -> proc_macro2::TokenStream
// where
//     P: IntoIterator<Item = &'a Param>,
// {
//     let p = params
//         .into_iter()
//         .map(|x| {
//             let name = &x.name;
//             let kind = to_syntax_string(&x.kind);
//             quote! {
//                 ethabi::Param {
//                     name: #name.to_owned(),
//                     kind: #kind,
//                     internal_type: None
//                 }
//             }
//         })
//         .collect::<Vec<_>>();

//     quote! { vec![ #(#p),* ] }
// }

fn rust_type(input: &ParamType) -> proc_macro2::TokenStream {
    match *input {
        ParamType::Address => quote! { Vec<u8> },
        ParamType::Bytes => quote! { Vec<u8> },
        ParamType::FixedBytes(size) => quote! { [u8; #size] },
        ParamType::Int(_) => quote! { substreams::scalar::BigInt },
        ParamType::Uint(_) => quote! { substreams::scalar::BigInt },
        ParamType::Bool => quote! { bool },
        ParamType::String => quote! { String },
        ParamType::Array(ref kind) => {
            let t = rust_type(&*kind);
            quote! { Vec<#t> }
        }
        ParamType::FixedArray(ref kind, size) => {
            let t = rust_type(&*kind);
            quote! { [#t; #size] }
        }
        ParamType::Tuple(ref types) => {
            let tuple_elements = types.iter().map(rust_type);
            quote! { (#(#tuple_elements,)*) }
        }
    }
}

fn fixed_data_size(input: &ParamType) -> Option<usize> {
    match *input {
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_) => Some(32),
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => None,
        ParamType::FixedArray(_, _) if input.is_dynamic() => None,
        ParamType::FixedArray(ref sub_type, count) => {
            Some(count * fixed_data_size(sub_type).expect("not dynamic, will always be Some(_)"))
        }
        ParamType::Tuple(ref types) => {
            let sizes = types.iter().map(fixed_data_size);
            if sizes.clone().any(|x| x.is_none()) {
                None
            } else {
                Some(sizes.map(Option::unwrap).sum())
            }
        }
    }
}

fn min_data_size(input: &ParamType) -> usize {
    match *input {
        ParamType::FixedArray(ref sub_type, count) if sub_type.is_dynamic() => {
            count * min_data_size(sub_type)
        }
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_)
        | ParamType::FixedArray(_, _) => {
            fixed_data_size(input).expect("not dynamic, will always be Some(_)")
        }
        // Those are dynamic type meaning there is first an offset where to find the data written (32 bytes)
        // and then minimally a length (32 bytes) so minimum size is `size(offset) + size(length)` which is
        // `32 + 32`.
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => 32 + 32,
        ParamType::Tuple(ref types) => types.iter().map(min_data_size).sum(),
    }
}

// fn template_param_type(input: &ParamType, index: usize) -> proc_macro2::TokenStream {
//     let t_ident = syn::Ident::new(&format!("T{}", index), Span::call_site());
//     let u_ident = syn::Ident::new(&format!("U{}", index), Span::call_site());
//     match *input {
//         ParamType::Address => quote! { #t_ident: Into<ethabi::Address> },
//         ParamType::Bytes => quote! { #t_ident: Into<ethabi::Bytes> },
//         ParamType::FixedBytes(32) => quote! { #t_ident: Into<ethabi::Hash> },
//         ParamType::FixedBytes(size) => quote! { #t_ident: Into<[u8; #size]> },
//         ParamType::Int(_) => quote! { #t_ident: Into<ethabi::Int> },
//         ParamType::Uint(_) => quote! { #t_ident: Into<ethabi::Uint> },
//         ParamType::Bool => quote! { #t_ident: Into<bool> },
//         ParamType::String => quote! { #t_ident: Into<String> },
//         ParamType::Array(ref kind) => {
//             let t = rust_type(&*kind);
//             quote! {
//                 #t_ident: IntoIterator<Item = #u_ident>, #u_ident: Into<#t>
//             }
//         }
//         ParamType::FixedArray(ref kind, size) => {
//             let t = rust_type(&*kind);
//             quote! {
//                 #t_ident: Into<[#u_ident; #size]>, #u_ident: Into<#t>
//             }
//         }
//         ParamType::Tuple(_) => {
//             unimplemented!(
//                 "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
//             )
//         }
//     }
// }

// fn from_template_param(input: &ParamType, name: &syn::Ident) -> proc_macro2::TokenStream {
//     match *input {
//         ParamType::Array(_) => {
//             quote! { self.#name.into_iter().map(Into::into).collect::<Vec<_>>() }
//         }
//         ParamType::FixedArray(_, _) => {
//             quote! { (Box::new(self.#name.into()) as Box<[_]>).into_vec().into_iter().map(Into::into).collect::<Vec<_>>() }
//         }
//         ParamType::Address => quote! { ethabi::Address::from_slice(self.#name.as_ref() ) },
//         _ => firehose_into_ethabi_type(input, quote! { self.#name }),
//     }
// }

// fn firehose_into_ethabi_type(
//     input: &ParamType,
//     variable: proc_macro2::TokenStream,
// ) -> proc_macro2::TokenStream {
//     match *input {
//         ParamType::Address => quote! { ethabi::Address::from_slice(#variable) },
//         ParamType::String => quote! { #variable.clone() },
//         _ => quote! {#variable.into() },
//     }
// }

fn to_token(name: &proc_macro2::TokenStream, kind: &ParamType) -> proc_macro2::TokenStream {
    match *kind {
        ParamType::Address => {
            quote! { ethabi::Token::Address(ethabi::Address::from_slice(&#name)) }
        }
        ParamType::Bytes => quote! { ethabi::Token::Bytes(#name.clone()) },
        ParamType::FixedBytes(_) => quote! { ethabi::Token::FixedBytes(#name.as_ref().to_vec()) },
        ParamType::Int(_) => {
            quote! {
                {
                    let non_full_signed_bytes = #name.to_signed_bytes_be();
                    let mut full_signed_bytes = [0xff as u8; 32];
                    non_full_signed_bytes.into_iter().rev().enumerate().for_each(|(i, byte)| full_signed_bytes[31 - i] = byte);

                    ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes.as_ref()))
                }
            }
        }
        ParamType::Uint(_) => {
            quote! {
                ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match #name.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    },
                                }.as_slice(),
                            ),
                        )
            }
        }
        ParamType::Bool => quote! { ethabi::Token::Bool(#name.clone()) },
        ParamType::String => quote! { ethabi::Token::String(#name.clone()) },
        ParamType::Array(ref kind) => {
            let inner_name = quote! { inner };
            let inner_loop = to_token(&inner_name, kind);
            quote! {
                // note the double {{
                {
                    let v = #name.iter().map(|#inner_name| #inner_loop).collect();
                    ethabi::Token::Array(v)
                }
            }
        }
        ParamType::FixedArray(ref kind, _) => {
            let inner_name = quote! { inner };
            let inner_loop = to_token(&inner_name, kind);
            quote! {
                // note the double {{
                {
                    let v = #name.iter().map(|#inner_name| #inner_loop).collect();
                    ethabi::Token::FixedArray(v)
                }
            }
        }
        ParamType::Tuple(ref types) => {
        
            let inner_names = (0..types.len())
                .map(|i| {
                    let i = Index::from(i);
                    quote! { inner.#i }
                })
                .collect::<Vec<_>>();

            let inner_tokens = types
                .iter()
                .zip(&inner_names)
                .map(|(kind, inner_name)| to_token(&inner_name.to_token_stream(), kind))
                .collect::<Vec<_>>();

            quote! {
                ethabi::Token::Tuple(vec![
                    #(#inner_tokens),*
                ])
            }
        }
    }
}

fn from_token(kind: &ParamType, token: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match *kind {
        ParamType::Address => {
            quote! { #token.into_address().expect(INTERNAL_ERR).as_bytes().to_vec() }
        }
        ParamType::Bytes => {
            quote! { #token.into_bytes().expect(INTERNAL_ERR) }
        }
        ParamType::FixedBytes(size) => {
            let size: syn::Index = size.into();
            quote! {
                {
                    let mut result = [0u8; #size];
                    let v = #token.into_fixed_bytes().expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                }
            }
        }
        ParamType::Int(_) => quote! {
            {
                let mut v = [0 as u8; 32];
                #token.into_int().expect(INTERNAL_ERR).to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_signed_bytes_be(&v)
            }
        },
        ParamType::Uint(_) => quote! {
                {
                    let mut v = [0 as u8; 32];
                    #token.into_uint().expect(INTERNAL_ERR).to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                }
        },
        ParamType::Bool => quote! { #token.into_bool().expect(INTERNAL_ERR) },
        ParamType::String => quote! { #token.into_string().expect(INTERNAL_ERR) },
        ParamType::Array(ref kind) => {
            let inner = quote! { inner };
            let inner_loop = from_token(kind, &inner);
            quote! {
                #token.into_array().expect(INTERNAL_ERR).into_iter()
                    .map(|#inner| #inner_loop)
                    .collect()
            }
        }
        ParamType::FixedArray(ref kind, size) => {
            let inner = quote! { inner };
            let inner_loop = from_token(kind, &inner);
            let to_array = vec![quote! { iter.next().expect(INTERNAL_ERR) }; size];
            quote! {
                {
                    let mut iter = #token.into_fixed_array().expect(INTERNAL_ERR).into_iter()
                        .map(|#inner| #inner_loop);
                    [#(#to_array),*]
                }
            }
        }
        ParamType::Tuple(ref types) => {
            let conversion = types.iter().enumerate().map(|(i, t)| {
                let inner = quote! { tuple_elements[#i].clone() };
                let inner_conversion = from_token(t, &inner);
                quote! { #inner_conversion }
            });

            quote! {
                {
                    let tuple_elements = #token.into_tuple().expect(INTERNAL_ERR);
                    (#(#conversion,)*)
                }
            }
        }
    }
}

fn decode_topic(
    name: &String,
    kind: &ParamType,
    data_token: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let syntax_type = to_syntax_string(kind);
    let error_msg = format!(
        "unable to decode param '{}' from topic of type '{}': {{:?}}",
        name, kind
    );

    match kind {
        ParamType::Int(_) => {
            quote! {
                substreams::scalar::BigInt::from_signed_bytes_be(#data_token)
            }
        }
        _ => {
            let decode_topic = quote! {
                        ethabi::decode(&[#syntax_type], #data_token)
                        .map_err(|e| format!(#error_msg, e))?
                        .pop()
                        .expect(INTERNAL_ERR)
            };

            from_token(kind, &decode_topic)
        }
    }
}

fn param_names(inputs: &[Param]) -> Vec<syn::Ident> {
    inputs
        .iter()
        .enumerate()
        .map(|(index, param)| {
            if param.name.is_empty() {
                syn::Ident::new(&format!("param{}", index), Span::call_site())
            } else {
                syn::Ident::new(&rust_variable(&param.name), Span::call_site())
            }
        })
        .collect()
}

// fn get_template_names(kinds: &[proc_macro2::TokenStream]) -> Vec<syn::Ident> {
//     kinds
//         .iter()
//         .enumerate()
//         .map(|(index, _)| syn::Ident::new(&format!("T{}", index), Span::call_site()))
//         .collect()
// }

fn get_output_kinds(outputs: &[Param]) -> proc_macro2::TokenStream {
    match outputs.len() {
        0 => quote! {()},
        1 => {
            let t = rust_type(&outputs[0].kind);
            quote! { #t }
        }
        _ => {
            let outs: Vec<_> = outputs.iter().map(|param| rust_type(&param.kind)).collect();
            quote! { (#(#outs),*) }
        }
    }
}

/// Convert input into a rust variable name.
///
/// Avoid using keywords by escaping them.
fn rust_variable(name: &str) -> String {
    // avoid keyword parameters
    match name {
        "self" => "_self".to_string(),
        other => other.to_snake_case(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_firehose_types_to_ethabi_token() {
        use substreams::hex;

        let firehose_address = hex!("0000000000000000000000000000000000000000").to_vec();

        // Compilation is enough for those tests
        ethabi::Token::Address(ethabi::Address::from_slice(firehose_address.as_ref()));
    }
}
