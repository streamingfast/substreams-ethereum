// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;
use quote::quote;

// use crate::{constructor::Constructor, event::Event, function::Function};
use crate::event::Event;

/// Structure used to generate rust interface for solidity contract.
pub struct Contract {
    // constructor: Option<Constructor>,
    // functions: Vec<Function>,
    events: Vec<Event>,
}

impl<'a> From<&'a ethabi::Contract> for Contract {
    fn from(c: &'a ethabi::Contract) -> Self {
        let mut events: Vec<(String, ethabi::Event)> = vec![];

        for overloads in c.events.values() {
            for (index, event) in overloads.iter().enumerate() {
                let name = match index {
                    0 => event.name.clone(),
                    _ => format!("{}{}", event.name, index),
                };

                events.push((name, event.clone()));
            }
        }

        Contract {
            // constructor: c.constructor.as_ref().map(Into::into),
            // functions: c.functions().map(Into::into).collect(),
            events: events.iter().map(Into::into).collect(),
        }
    }
}

impl Contract {
    /// Generates rust interface for a contract.
    pub fn generate(&self) -> TokenStream {
        // let constructor = self.constructor.as_ref().map(Constructor::generate);
        // let functions: Vec<_> = self.functions.iter().map(Function::generate).collect();
        let events: Vec<_> = self
            .events
            .iter()
            .map(|event| event.generate_event())
            .collect();
        // let logs: Vec<_> = self.events.iter().map(Event::generate_log).collect();
        quote! {
            const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

            // #constructor

            // Contract's functions.
            // pub mod functions {
            //     use super::INTERNAL_ERR;
            //     #(#functions)*
            // }

            /// Contract's events.
            #[allow(dead_code)]
            pub mod events {
                use super::INTERNAL_ERR;
                #(#events)*
            }
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;

    use crate::assertions::assert_ast_eq;

    use super::Contract;

    #[test]
    fn test_no_body() {
        let ethabi_contract = ethabi::Contract {
            constructor: None,
            functions: Default::default(),
            events: Default::default(),
            errors: Default::default(),
            receive: false,
            fallback: false,
        };

        let c = Contract::from(&ethabi_contract);

        assert_ast_eq(
            c.generate(),
            quote! {
                const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

                /// Contract's events.
                #[allow(dead_code)]
                pub mod events {
                    use super::INTERNAL_ERR;
                }
            },
        );
    }
}
