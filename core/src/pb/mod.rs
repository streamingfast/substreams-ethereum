#[rustfmt::skip]
#[path = "sf.ethereum.type.v2.rs"]
mod sf_ethereum_type_v2;

#[rustfmt::skip]
#[path = "sf.ethereum.substreams.v1.rs"]
#[allow(dead_code)] // we added this since prost generates a FILE_DESCRIPTOR_SET const in the proto file that is not used
mod sf_ethereum_substreams_v1;

pub mod eth {
    pub mod v2 {
        pub use crate::pb::sf_ethereum_type_v2::*;
    }
    pub mod rpc {
        pub use crate::pb::sf_ethereum_substreams_v1::*;
    }
}
