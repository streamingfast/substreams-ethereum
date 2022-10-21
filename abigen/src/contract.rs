// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proc_macro2::TokenStream;
use quote::quote;

// use crate::{constructor::Constructor,};
use crate::{event::Event, function::Function};

/// Structure used to generate rust interface for solidity contract.
pub struct Contract {
    // constructor: Option<Constructor>,
    functions: Vec<Function>,
    events: Vec<Event>,
}

impl<'a> From<&'a ethabi::Contract> for Contract {
    fn from(c: &'a ethabi::Contract) -> Self {
        let mut events: Vec<_> = c
            .events
            .values()
            .flat_map(|events| {
                let count = events.len();

                events.iter().enumerate().map(move |(index, event)| {
                    if count <= 1 {
                        (&event.name, event).into()
                    } else {
                        (&format!("{}{}", event.name, index + 1), event).into()
                    }
                })
            })
            .collect();

        // Since some people will actually commit this code, we use a "stable" generation order
        events.sort_by(|left: &Event, right: &Event| left.name.cmp(&right.name));

        let mut functions: Vec<_> = c
            .functions
            .values()
            .flat_map(|functions| {
                let count = functions.len();

                functions.iter().enumerate().map(move |(index, function)| {
                    if count <= 1 {
                        (&function.name, function).into()
                    } else {
                        (&format!("{}{}", function.name, index + 1), function).into()
                    }
                })
            })
            .collect();

        // Since some people will actually commit this code, we use a "stable" generation order
        functions.sort_by(|left: &Function, right: &Function| left.name.cmp(&right.name));

        Contract {
            // constructor: c.constructor.as_ref().map(Into::into),
            functions,
            events,
        }
    }
}

impl Contract {
    /// Generates rust interface for a contract.
    pub fn generate(&self) -> TokenStream {
        // let constructor = self.constructor.as_ref().map(Constructor::generate);
        let functions: Vec<_> = self.functions.iter().map(Function::generate).collect();
        let events: Vec<_> = self
            .events
            .iter()
            .map(|event| event.generate_event())
            .collect();
        // let logs: Vec<_> = self.events.iter().map(Event::generate_log).collect();
        quote! {
            const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

            // #constructor

            /// Contract's functions.
            #[allow(dead_code, unused_imports, unused_variables)]
            pub mod functions {
                use super::INTERNAL_ERR;
                #(#functions)*
            }

            /// Contract's events.
            #[allow(dead_code, unused_imports, unused_variables)]
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

                /// Contract's functions.
                #[allow(dead_code, unused_imports, unused_variables)]
                pub mod functions {
                    use super::INTERNAL_ERR;
                }

                /// Contract's events.
                #[allow(dead_code, unused_imports, unused_variables)]
                pub mod events {
                    use super::INTERNAL_ERR;
                }
            },
        );
    }
}
