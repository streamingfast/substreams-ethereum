/// Prelude module that exports all the common types and traits for working with Ethereum data.
///
/// This module is designed to be imported with a glob import:
///
/// ```rust
/// use substreams_ethereum::prelude::*;
/// ```
///
/// # Examples
///
/// Converting between `scalar::BigInt` and `pb::eth::v2::BigInt`:
///
/// ```rust
/// use substreams_ethereum::prelude::*;
///
/// // Convert from scalar::BigInt to pb::eth::v2::BigInt
/// let scalar_bigint = substreams::scalar::BigInt::from(42);
/// let pb_bigint: pb::eth::v2::BigInt = scalar_bigint.into();
///
/// // Convert from pb::eth::v2::BigInt to scalar::BigInt
/// let pb_bigint = pb::eth::v2::BigInt { bytes: vec![42] };
/// let scalar_bigint: substreams::scalar::BigInt = pb_bigint.into();
/// ```

// Re-export the scalar types from substreams
pub use substreams::scalar::{BigDecimal, BigInt};

// Re-export the Ethereum protobuf types
pub use crate::pb::eth::v2 as pb;

// Re-export the conversion traits
pub use std::convert::{From, Into};

// Re-export utility functions for working with BigInt and BigDecimal
pub use crate::scalar::{to_option_bigint, to_option_decimal, to_option_decimal_with_decimal};

// Re-export common Ethereum types and utilities
pub use crate::{block_view, Event, Function, NULL_ADDRESS};

