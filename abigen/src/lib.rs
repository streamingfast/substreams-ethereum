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
// mod function;

use anyhow::format_err;
// use ethabi::{Contract, Error, Param, ParamType, Result};
use ethabi::{Contract, Error, ParamType};
// use heck::ToSnakeCase;
use quote::quote;
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
        ParamType::Tuple(_) => {
            unimplemented!(
                "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
            )
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
        ParamType::Int(_) => quote! { num_bigint::BigInt },
        ParamType::Uint(_) => quote! { ethabi::Uint },
        ParamType::Bool => quote! { bool },
        ParamType::String => quote! { String },
        ParamType::Array(ref kind) => {
            let t = rust_type(&*kind);
            quote! { Vec<#t> }
        }
        ParamType::FixedArray(ref kind, size) => {
            let t = rust_type(&*kind);
            quote! { [#t, #size] }
        }
        ParamType::Tuple(_) => {
            unimplemented!(
                "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
            )
        }
    }
}

fn fixed_data_size(input: &ParamType) -> Option<usize> {
    match *input {
        ParamType::Address | ParamType::Int(_) | ParamType::Uint(_) | ParamType::Bool => Some(32),
        ParamType::FixedBytes(byte_count) => Some((byte_count / 32) + 1),
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => None,
        ParamType::FixedArray(_, _) if input.is_dynamic() => None,
        ParamType::FixedArray(ref sub_type, count) => {
            Some(count * fixed_data_size(sub_type).expect("not dynamic, will always be Some(_)"))
        }
        ParamType::Tuple(_) => {
            unimplemented!(
                "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
            )
        }
    }
}

fn min_data_size(input: &ParamType) -> usize {
    let is_dynamic = input.is_dynamic();

    match *input {
        ParamType::FixedArray(ref sub_type, count) if is_dynamic => count * min_data_size(sub_type),
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_)
        | ParamType::FixedArray(_, _) => {
            fixed_data_size(input).expect("not dynamic, will always be Some(_)")
        }
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => 32 + 32,
        ParamType::Tuple(_) => {
            unimplemented!(
                "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
            )
        }
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
//         ParamType::Array(_) => quote! { #name.into_iter().map(Into::into).collect::<Vec<_>>() },
//         ParamType::FixedArray(_, _) => {
//             quote! { (Box::new(#name.into()) as Box<[_]>).into_vec().into_iter().map(Into::into).collect::<Vec<_>>() }
//         }
//         _ => quote! {#name.into() },
//     }
// }

fn from_token(kind: &ParamType, token: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match *kind {
        ParamType::Address => {
            quote! { #token.into_address().unwrap().as_bytes().to_vec() }
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
        ParamType::Int(_) => quote! { #token.into_int().expect(INTERNAL_ERR) },
        ParamType::Uint(_) => quote! { #token.into_uint().expect(INTERNAL_ERR) },
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
            let to_array = vec![quote! { iter.next() }; size];
            quote! {
                {
                    let iter = #token.to_array().expect(INTERNAL_ERR).into_iter()
                        .map(|#inner| #inner_loop);
                    [#(#to_array),*]
                }
            }
        }
        ParamType::Tuple(_) => {
            unimplemented!(
                "Tuples are not supported. https://github.com/openethereum/ethabi/issues/175"
            )
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
        "unable to decode param '{}' from topic of type '{}': {{}}",
        name, kind
    );

    match kind {
        ParamType::Int(_) => {
            quote! {
                num_bigint::BigInt::from_signed_bytes_be(#data_token)
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
