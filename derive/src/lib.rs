// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "256"]

extern crate proc_macro;

use ethabi::{Error, Result};
use std::borrow::Cow;

const ERROR_MSG: &str = "`derive(EthabiContract)` in substreams-ethereum failed";

#[proc_macro_derive(EthabiContract, attributes(ethabi_contract_options))]
pub fn ethabi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect(ERROR_MSG);
    let gen = impl_ethabi_derive(&ast).expect(ERROR_MSG);
    gen.into()
}

fn impl_ethabi_derive(ast: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let options = get_options(&ast.attrs, "ethabi_contract_options")?;
    let path = get_option(&options, "path")?;

    substreams_ethereum_abigen::generate_abi_code(path)
        .map_err(|e| Error::Other(Cow::Owned(format!("{}", e))))
}

fn get_options(attrs: &[syn::Attribute], name: &str) -> Result<Vec<syn::NestedMeta>> {
    let options = attrs
        .iter()
        .flat_map(syn::Attribute::parse_meta)
        .find(|meta| meta.path().is_ident(name));

    match options {
        Some(syn::Meta::List(list)) => Ok(list.nested.into_iter().collect()),
        _ => Err(Error::Other(Cow::Borrowed("Unexpected meta item"))),
    }
}

fn get_option(options: &[syn::NestedMeta], name: &str) -> Result<String> {
    let item = options
        .iter()
        .flat_map(|nested| match *nested {
            syn::NestedMeta::Meta(ref meta) => Some(meta),
            _ => None,
        })
        .find(|meta| meta.path().is_ident(name))
        .ok_or_else(|| Error::Other(Cow::Owned(format!("Expected to find option {}", name))))?;

    str_value_of_meta_item(item, name)
}

fn str_value_of_meta_item(item: &syn::Meta, name: &str) -> Result<String> {
    if let syn::Meta::NameValue(ref name_value) = *item {
        if let syn::Lit::Str(ref value) = name_value.lit {
            return Ok(value.value());
        }
    }

    Err(Error::Other(Cow::Owned(format!(
        r#"`{}` must be in the form `#[{}="something"]`"#,
        name, name
    ))))
}
