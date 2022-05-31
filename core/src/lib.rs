mod externs;
pub mod pb;
pub mod rpc;

pub use substreams_ethereum_derive::EthabiContract;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::num::NonZeroU32;

/// Builder struct for generating type-safe bindings from a contract's ABI
///
/// # Example
///
/// Running the code below will generate a file called `token.rs` containing the
/// bindings inside, which exports an `erc` struct, along with all its events. Put into a
/// `build.rs` file this will generate the bindings during `cargo build`.
///
/// ```no_run
///     use anyhow::{Ok, Result};
///     use substreams_ethereum::Abigen;
///
///     fn main() -> Result<(), anyhow::Error> {
///         Abigen::new("ERC721", "abi/erc721.json")?
///             .generate()?
///             .write_to_file("src/abi/erc721.rs")?;
///
///         Ok(())
///     }
/// ```
pub use substreams_ethereum_abigen::build::Abigen;

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

/// This macro can be used to import an Ethereum ABI file in JSON format and generate all the
/// required bindings for ABI decoding/encoding in Rust, targetting `substreams` developer
/// experience.
///
/// ```no_run
///     use substreams_ethereum::use_contract;
///
///     use_contract!(erc721, "./examples/abi/erc721.json");
/// ```
///
/// This invocation will generate the following code (signatures only for consiscness):
///
/// ```rust,ignore
/// mod erc721 {
///     pub mod events {
///         #[derive(Debug, Clone, PartialEq)]
///         pub struct Transfer {
///             pub from: Vec<u8>,
///             pub to: Vec<u8>,
///             pub token_id: ethabi::Uint,
///         }
///
///         impl Transfer {
///             pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
///                ...
///             }
///
///             pub fn decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Result<Transfer, String> {
///                ...
///             }
///
///             pub fn must_decode(log: &substreams_ethereum::pb::eth::v1::Log) -> Transfer {
///                ...
///             }
///         }
///
///         ... Other events ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! use_contract {
    ($module: ident, $path: expr) => {
        #[allow(dead_code)]
        #[allow(missing_docs)]
        #[allow(unused_imports)]
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        pub mod $module {
            #[derive(substreams_ethereum::EthabiContract)]
            #[ethabi_contract_options(path = $path)]
            struct _Dummy;
        }
    };
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
/// The `init` macro registers a custom get random function in the system which is required
/// because `ethabi` that we rely on for ABI decoding/encoding primitives use it somewhere in
/// its transitive set of dependencies and causes problem in `wasm32-unknown-unknown` target.
///
/// This macro must be invoked in the root crate so you must have the `substreams_ethereum::init!()`
/// call in your `lib.rs` of your Substreams.
///
/// In addition, you need to have `getrandom = { version = "0.2", features = ["custom"] }` dependency
/// in your Substreams `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// ...
/// # Required so that ethabi > ethereum-types build correctly under wasm32-unknown-unknown
/// getrandom = { version = "0.2", features = ["custom"] }
///```
#[macro_export]
macro_rules! init {
    () => {
        getrandom::register_custom_getrandom!(substreams_ethereum::getrandom_unavailable);
    };
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
const GETRANDOM_UNVAILABLE_IN_SUBSTREAMS: u32 = getrandom::Error::CUSTOM_START + 5545;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn getrandom_unavailable(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let code = NonZeroU32::new(GETRANDOM_UNVAILABLE_IN_SUBSTREAMS).unwrap();
    Err(getrandom::Error::from(code))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
