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
