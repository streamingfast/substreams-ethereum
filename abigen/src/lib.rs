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
use quote::quote;
use std::{
    borrow::Cow,
    env, fs,
    path::{Path, PathBuf},
};
use syn::Index;

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

pub fn generate_abi_code_from_bytes(
    bytes: &[u8],
) -> Result<proc_macro2::TokenStream, anyhow::Error> {
    let contract = Contract::load(bytes)?;
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

fn rust_type_indexed(input: &ParamType) -> proc_macro2::TokenStream {
    match input.is_dynamic() {
        true => {
            let t = rust_type(input);
            return quote! { substreams_ethereum::IndexedDynamicValue<#t> };
        }
        false => rust_type(input),
    }
}

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

/// The largest data section a generated guard will describe.
///
/// A `count` in a fixed array comes from the ABI file, so a size computed from it
/// can exceed what a `usize` holds on the `wasm32` target the generated code runs
/// on, and the literal baked into a guard has to stay representable there. No log
/// approaches this, so clamping only affects inputs that could not decode anyway.
const MAX_DATA_SIZE: usize = u32::MAX as usize;

/// The most elements a fixed array may declare before `abigen` refuses it.
///
/// Reads are emitted one per element, so a large count is a build that never
/// finishes rather than a decoder that misbehaves.
const MAX_FIXED_ARRAY_ELEMENTS: usize = 4096;

fn fixed_data_size(input: &ParamType) -> Option<usize> {
    match input {
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_) => Some(32),
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => None,
        // The element is itself sized here rather than assumed to have a size: a
        // non-dynamic element can still be an array whose own size is past what a
        // guard may describe, and that answer has to travel outwards.
        ParamType::FixedArray(ref sub_type, count) => match sub_type.is_dynamic() {
            true => None,
            false => count
                .checked_mul(fixed_data_size(sub_type)?)
                .filter(|size| *size <= MAX_DATA_SIZE),
        },
        ParamType::Tuple(ref types) => {
            if types.iter().any(ParamType::is_dynamic) {
                return None;
            }

            // A field can be a fixed array whose own size is past what a guard may
            // describe, and a tuple of those sums past it too, so neither the field
            // sizes nor their total are taken for granted here.
            types
                .iter()
                .try_fold(0usize, |total, kind| {
                    total.checked_add(fixed_data_size(kind)?)
                })
                .filter(|size| *size <= MAX_DATA_SIZE)
        }
    }
}

fn min_data_size(input: &ParamType) -> usize {
    match input {
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_) => {
            fixed_data_size(input).expect("not dynamic, will always be Some(_)")
        }
        // FixedArray with dynamic element becomes "dynamic" so we have
        // an initial data offset (32) plus the minimal data size of the sub type multipled
        // by number of element in the fixed array (count * min_data_size(sub_type)).
        //
        // If the sub type is not dynamic, we use its fixed data size.
        ParamType::FixedArray(ref sub_type, count) => match sub_type.is_dynamic() {
            true => {
                32 + count
                    .saturating_mul(min_data_size(sub_type))
                    .min(MAX_DATA_SIZE)
            }
            false => fixed_data_size(input).unwrap_or(MAX_DATA_SIZE),
        },
        // Those are dynamic type meaning there is first an offset where to find the data written (32 bytes)
        // and then minimally a length (32 bytes) so minimum size is `size(offset) + size(length)` which is
        // `32 + 32`.
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => 32 + 32,
        ParamType::Tuple(ref types) => types.iter().map(min_data_size).sum(),
    }
}

/// Emits a write of `value` into the head and tail buffers named by `heads` and
/// `tails`.
///
/// A fixed-size type writes itself into the head. A dynamic one writes a head
/// word holding the distance from the start of the head section to its own tail,
/// which is the head's full width plus whatever tails precede it, then appends
/// its content to the tail buffer. The head width is known from the ABI, so the
/// offset is computed rather than read back.
fn write_at(
    kind: &ParamType,
    value: &proc_macro2::TokenStream,
    out: &syn::Ident,
    base: &syn::Ident,
    slot: &proc_macro2::TokenStream,
    depth: usize,
) -> proc_macro2::TokenStream {
    if !kind.is_dynamic() {
        return write_fixed(kind, value, out, base, slot);
    }

    let tail = write_tail(kind, value, out, depth);

    quote! {
        {
            let at = substreams_ethereum::abi::tail_offset(&#out, #base);
            #tail;
            substreams_ethereum::abi::backfill_offset(&mut #out, #base + #slot, at);
        }
    }
}

/// Emits the write of a fixed-size type into the head slot at `base + slot`.
///
/// The slot was reserved before any tail was appended, so the value is written
/// into bytes the buffer already holds rather than pushed onto its end.
fn write_fixed(
    kind: &ParamType,
    value: &proc_macro2::TokenStream,
    out: &syn::Ident,
    base: &syn::Ident,
    slot: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match kind {
        ParamType::Address => quote! {
            substreams_ethereum::abi::write_address_at(&mut #out, #base + #slot, &#value)
        },
        ParamType::Uint(_) => quote! {
            substreams_ethereum::abi::write_uint_at(&mut #out, #base + #slot, &#value)
        },
        ParamType::Int(_) => quote! {
            substreams_ethereum::abi::write_int_at(&mut #out, #base + #slot, &#value)
        },
        ParamType::Bool => quote! {
            substreams_ethereum::abi::write_bool_at(&mut #out, #base + #slot, &#value)
        },
        ParamType::FixedBytes(_) => quote! {
            substreams_ethereum::abi::write_fixed_bytes_at(
                &mut #out,
                #base + #slot,
                #value.as_ref(),
            )
        },
        // A fixed array or tuple of fixed-size elements spans several slots of
        // the head, one after another, with no offset word of its own.
        ParamType::FixedArray(inner, size) => {
            assert!(
                *size <= MAX_FIXED_ARRAY_ELEMENTS,
                "the ABI declares a fixed array of {} elements, above the {} `abigen` will \
                 generate writes for",
                size,
                MAX_FIXED_ARRAY_ELEMENTS
            );

            let stride = element_stride(inner);
            let element = write_fixed(
                inner,
                &quote! { element },
                out,
                base,
                &quote! { #slot + index * #stride },
            );

            quote! {
                for (index, element) in #value.iter().enumerate() {
                    #element;
                }
            }
        }
        ParamType::Tuple(fields) => {
            let mut at = 0usize;
            let writes: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let index = Index::from(index);
                    let within = syn::Index::from(at);
                    at += element_stride(field);

                    write_fixed(
                        field,
                        &quote! { #value.#index },
                        out,
                        base,
                        &quote! { #slot + #within },
                    )
                })
                .collect();

            quote! {
                {
                    #(#writes;)*
                }
            }
        }
        ParamType::Bytes | ParamType::String | ParamType::Array(_) => {
            unreachable!("dynamic types are written through their tail")
        }
    }
}

/// Emits the write of a dynamic value's tail.
///
/// An array's elements form a head section of their own, so a dynamic element
/// nests the same two-buffer construction against that section rather than the
/// enclosing one.
fn write_tail(
    kind: &ParamType,
    value: &proc_macro2::TokenStream,
    out: &syn::Ident,
    depth: usize,
) -> proc_macro2::TokenStream {
    // A tail can hold another tail, and each level measures its own offsets from
    // where its head section starts, so every level names that position after its
    // own depth rather than shadowing the level above.
    let base = buffer_ident("base", depth);
    let element = element_ident(depth);

    match kind {
        ParamType::Bytes => quote! {
            substreams_ethereum::abi::write_bytes_tail(&mut #out, &#value)
        },
        ParamType::String => quote! {
            substreams_ethereum::abi::write_bytes_tail(&mut #out, #value.as_bytes())
        },
        // An array's elements form a head section of their own, one stride per
        // element, so a dynamic element's offset is measured from there rather
        // than from the enclosing list.
        ParamType::Array(inner) => {
            let stride_of_inner = element_stride(inner);
            let write_one = write_element(inner, out, &base, depth);

            quote! {
                {
                    let count = #value.len();
                    substreams_ethereum::abi::write_offset(&mut #out, count);

                    let #base = substreams_ethereum::abi::reserve_head(
                        &mut #out,
                        count * #stride_of_inner,
                    );

                    for (index, #element) in #value.iter().enumerate() {
                        let slot = index * #stride_of_inner;
                        #write_one;
                    }
                }
            }
        }
        // A fixed array or tuple is dynamic only because one of its elements is,
        // so it has no length word; its elements are laid out head-then-tail the
        // way a parameter list is.
        ParamType::FixedArray(inner, size) => {
            let stride_of_inner = element_stride(inner);
            let head_width = size.saturating_mul(stride_of_inner);
            let write_one = write_element(inner, out, &base, depth);

            quote! {
                {
                    let #base = substreams_ethereum::abi::reserve_head(&mut #out, #head_width);

                    for (index, #element) in #value.iter().enumerate() {
                        let slot = index * #stride_of_inner;
                        #write_one;
                    }
                }
            }
        }
        ParamType::Tuple(fields) => {
            let head_width: usize = fields.iter().map(element_stride).sum();

            let mut slot = 0usize;
            let writes: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let index = Index::from(index);
                    let at = syn::Index::from(slot);
                    slot += element_stride(field);

                    write_at(
                        field,
                        &quote! { #value.#index },
                        out,
                        &base,
                        &quote! { #at },
                        depth + 1,
                    )
                })
                .collect();

            quote! {
                {
                    let #base = substreams_ethereum::abi::reserve_head(&mut #out, #head_width);
                    #(#writes;)*
                }
            }
        }
        _ => unreachable!("a fixed-size type is written into the head"),
    }
}

/// Emits the write of one element of an array into the buffer its enclosing tail
/// reserved a head section in.
fn write_element(
    inner: &ParamType,
    out: &syn::Ident,
    base: &syn::Ident,
    depth: usize,
) -> proc_macro2::TokenStream {
    let element = element_ident(depth);

    write_at(
        inner,
        &quote! { #element },
        out,
        base,
        &quote! { slot },
        depth + 1,
    )
}

/// A position a nesting level measures its own offsets from.
fn buffer_ident(role: &str, depth: usize) -> syn::Ident {
    syn::Ident::new(&format!("{}_{}", role, depth), Span::call_site())
}

/// The loop variable a nesting level binds each element to.
fn element_ident(depth: usize) -> syn::Ident {
    syn::Ident::new(&format!("element_{}", depth), Span::call_site())
}

/// Check if the given ParamType (recursively navigating through the types if necessary)
/// is a long tuple (i.e. has more than 12 elements). Those indeed cannot have Debug/PartialEq
/// defined.
fn is_long_tuple(input: &ParamType) -> bool {
    match input {
        ParamType::Address
        | ParamType::Int(_)
        | ParamType::Uint(_)
        | ParamType::Bool
        | ParamType::FixedBytes(_)
        | ParamType::Bytes
        | ParamType::String => false,
        ParamType::Array(sub_type) => is_long_tuple(sub_type),
        ParamType::FixedArray(ref sub_type, _) => is_long_tuple(sub_type),
        ParamType::Tuple(ref types) => {
            if types.len() > 12 {
                return true;
            }

            types.iter().any(is_long_tuple)
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

/// Emits a read of `kind` at a byte offset known at generation time, for the
/// parameters of an event whose data section is entirely fixed-size.
///
/// Returns `None` for a dynamic type, which has no such offset; the caller
/// falls back to `ethabi::decode` for the whole event.
fn read_fixed_at(
    kind: &ParamType,
    name: &str,
    data: &proc_macro2::TokenStream,
    offset: &proc_macro2::TokenStream,
) -> Option<proc_macro2::TokenStream> {
    Some(match kind {
        ParamType::Address => quote! {
            substreams_ethereum::abi::read_address(#data, #offset, #name)?
        },
        ParamType::Uint(_) => quote! {
            substreams_ethereum::abi::read_uint(#data, #offset, #name)?
        },
        ParamType::Int(_) => quote! {
            substreams_ethereum::abi::read_int(#data, #offset, #name)?
        },
        ParamType::Bool => quote! {
            substreams_ethereum::abi::read_bool(#data, #offset, #name)?
        },
        ParamType::FixedBytes(size) => {
            let size = syn::Index::from(*size);
            quote! {
                substreams_ethereum::abi::read_fixed_bytes::<#size>(#data, #offset, #name)?
            }
        }
        _ => return None,
    })
}

/// Emits a read of any parameter at `offset` words into `data`.
///
/// A head word sits at a known offset whatever the type; a dynamic type spends it
/// on a pointer to its tail and is read from there, which is why the recursive
/// calls start again at offset zero against a different slice.
fn read_at(
    kind: &ParamType,
    name: &str,
    data: &proc_macro2::TokenStream,
    offset: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if let Some(fixed) = read_fixed_at(kind, name, data, offset) {
        return fixed;
    }

    match kind {
        ParamType::Bytes => quote! {
            substreams_ethereum::abi::read_bytes(#data, #offset, #name)?
        },
        ParamType::String => quote! {
            substreams_ethereum::abi::read_string(#data, #offset, #name)?
        },
        ParamType::Array(inner) => {
            // A dynamic element carries a head word holding an offset measured from
            // the start of the array's tail, so every element is read against that
            // one slice at an advancing offset rather than against a slice of its
            // own. Re-slicing per element would move what those offsets resolve
            // against.
            let element = read_at(inner, name, &quote! { tail }, &quote! { at });
            let step = element_stride(inner);

            quote! {
                {
                    let (tail, count) = substreams_ethereum::abi::read_array_tail(
                        #data, #offset, #name
                    )?;

                    let mut out = Vec::with_capacity(count.min(1024));
                    let mut at = 0usize;
                    for _ in 0..count {
                        out.push(#element);
                        at += #step;
                    }
                    out
                }
            }
        }
        ParamType::FixedArray(inner, size) => {
            assert!(
                *size <= MAX_FIXED_ARRAY_ELEMENTS,
                "the ABI declares a fixed array of {} elements for param '{}', above the {} \
                 `abigen` will generate reads for",
                size,
                name,
                MAX_FIXED_ARRAY_ELEMENTS
            );

            let base = base_slice(kind, data, offset, name);
            let step = element_stride(inner);
            let reads: Vec<_> = (0..*size)
                .map(|index| {
                    let at = syn::Index::from(index.saturating_mul(step));
                    read_at(inner, name, &quote! { base }, &quote! { #at })
                })
                .collect();

            quote! {
                {
                    let base = #base;
                    [#(#reads),*]
                }
            }
        }
        ParamType::Tuple(fields) => {
            let base = base_slice(kind, data, offset, name);
            let mut at = 0usize;
            let reads: Vec<_> = fields
                .iter()
                .map(|field| {
                    let index = syn::Index::from(at);
                    let read = read_at(field, name, &quote! { base }, &quote! { #index });
                    at += element_stride(field);
                    read
                })
                .collect();

            quote! {
                {
                    let base = #base;
                    (#(#reads,)*)
                }
            }
        }
        _ => unreachable!("read_fixed_at covers every remaining type"),
    }
}

/// The slice a fixed array or tuple reads its elements from: its own tail when it
/// is dynamic, otherwise the parameter list it sits in.
fn base_slice(
    kind: &ParamType,
    data: &proc_macro2::TokenStream,
    offset: &proc_macro2::TokenStream,
    name: &str,
) -> proc_macro2::TokenStream {
    match kind.is_dynamic() {
        true => quote! {
            substreams_ethereum::abi::read_dynamic_tail(#data, #offset, #name)?
        },
        false => quote! {
            #data.get(#offset..).ok_or_else(|| format!(
                "unable to decode param '{}': need bytes at offset {}", #name, #offset
            ))?
        },
    }
}

/// Bytes a parameter occupies in the list it belongs to: one word, unless it is a
/// fixed-size aggregate written out in place.
///
/// An aggregate whose size is past what a guard may describe has no stride worth
/// computing, and the reads emitted after it would be past the buffer regardless,
/// so it takes the width of a single word.
fn element_stride(kind: &ParamType) -> usize {
    match kind.is_dynamic() {
        true => 32,
        false => fixed_data_size(kind).unwrap_or(32),
    }
}

fn decode_topic(
    name: &String,
    kind: &ParamType,
    data_token: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match kind {
        ParamType::Int(_) => {
            quote! {
                substreams::scalar::BigInt::from_signed_bytes_be(#data_token)
            }
        }
        // An indexed parameter of a dynamic type carries the hash of the value
        // rather than the value, so the topic is kept as its 32 bytes.
        _ if kind.is_dynamic() => {
            quote! {
                substreams_ethereum::abi::read_fixed_bytes::<32>(#data_token, 0, #name)?
                    .to_vec()
                    .into()
            }
        }
        // A topic is exactly one word, so an indexed parameter of a fixed type is
        // the same read as the first word of a data section.
        _ => read_at(kind, name, data_token, &quote! { 0 }),
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
    use ethabi::ParamType;

    use crate::{fixed_data_size, min_data_size};

    #[test]
    fn from_firehose_types_to_ethabi_token() {
        use substreams::hex;

        let firehose_address = hex!("0000000000000000000000000000000000000000").to_vec();

        // Compilation is enough for those tests
        ethabi::Token::Address(ethabi::Address::from_slice(firehose_address.as_ref()));
    }

    #[test]
    fn it_fixed_data_size_works() {
        let inputs: Vec<(&str, ParamType, Option<usize>)> = vec![
            (
                "tuple(address)",
                ParamType::Tuple(vec![ParamType::Address]),
                Some(32),
            ),
            (
                "bool[2]",
                ParamType::FixedArray(Box::new(ParamType::Bool), 2),
                Some(64),
            ),
            (
                "string[2]",
                ParamType::FixedArray(Box::new(ParamType::String), 2),
                None,
            ),
        ];

        for (name, actual, expected) in inputs {
            assert_eq!(fixed_data_size(&actual), expected, "test case {}", name);
        }
    }

    #[test]
    fn it_min_data_size_works() {
        let inputs: Vec<(&str, ParamType, usize)> = vec![
            (
                "tuple(address)",
                ParamType::Tuple(vec![ParamType::Address]),
                32,
            ),
            (
                "bool[2]",
                ParamType::FixedArray(Box::new(ParamType::Bool), 2),
                2 * 32,
            ),
            (
                "string[2]",
                ParamType::FixedArray(Box::new(ParamType::String), 2),
                32 + (2 * 32) + (2 * 32),
            ),
        ];

        for (name, actual, expected) in inputs {
            assert_eq!(min_data_size(&actual), expected, "test case {}", name);
        }
    }
}
