#[path = "sf.ethereum.r#type.v1.rs"]
mod sf_ethereum_type_v1;

pub mod eth {
    pub mod v1 {
        pub use crate::pb::sf_ethereum_type_v1::*;
    }
}
