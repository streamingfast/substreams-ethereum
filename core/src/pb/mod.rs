//! This module contains the protobuf generated code for the Substreams Ethereum block
//! model.
//!
//! This is the raw Protbuf code, types in here can be used without problem.

mod generated;

/// Re-export the protobuf generated code directly, at some point we might
/// deprecate the `eth` module so `pb::eth::v2` becomes `pb::sf::ethereum::r#type::v2`.
pub mod sf {
    pub use crate::pb::generated::sf::*;
}

pub mod eth {
    pub mod v2 {
        pub use crate::pb::generated::sf::ethereum::r#type::v2::*;
    }
    pub mod rpc {
        pub use crate::pb::generated::sf::ethereum::substreams::v1::*;
    }
}
