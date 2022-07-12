pub mod pb;
pub mod rpc;

/// Helpers to deal with block sources.
pub mod block_view;

mod abi;
mod externs;

/// Represents the null address static array in bytes (20 bytes) which in hex is equivalent
/// to:
///
/// ```text
/// 0000000000000000000000000000000000000000
/// ```
pub const NULL_ADDRESS: [u8; 20] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8,
];

pub trait Event: Sized {
    const NAME: &'static str;

    fn match_log(log: &crate::pb::eth::v1::Log) -> bool;
    fn decode(log: &crate::pb::eth::v1::Log) -> Result<Self, String>;
}
