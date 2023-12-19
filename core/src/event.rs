use std::marker::PhantomData;

use crate::pb::eth::v2::Log;

pub trait Event: Sized {
    const NAME: &'static str;

    fn match_log(log: &Log) -> bool;
    fn decode(log: &Log) -> Result<Self, String>;

    /// Attempts to match and decode the log.
    /// If `Self::match_log(log)` is `false`, returns `None`.
    /// If it matches, but decoding fails, logs the decoding error and returns `None`.
    fn match_and_decode(log: impl AsRef<Log>) -> Option<Self> {
        let log = log.as_ref();
        if !Self::match_log(log) {
            return None;
        }

        match Self::decode(&log) {
            Ok(event) => Some(event),
            Err(err) => {
                substreams::log::info!(
                    "Log for event `{}` at index {} matched but failed to decode with error: {}",
                    Self::NAME,
                    log.block_index,
                    err
                );
                None
            }
        }
    }
}

impl AsRef<Log> for Log {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Ethereum events with indexed parameters that are of dynamic types like a 'string',
/// 'bytes' or array of value do not contain the actual value in the log. Instead, they
/// contain a hash of the value. This struct is used to represent such values in the
/// decoded event.
///
/// The hash value read can be retrieved from the `hash` field, the original value
/// cannot be retrieved (unless you know it already, in which case you can validate
/// it fits the current hash).
///
/// You can access the hash (also equivalent to the topic in this case) directly
/// on the struct:
///
/// ```ignore
/// # use substreams_ethereum::IndexedDynamicValue;
/// let value = IndexedDynamicValue::<String>::new("0x1234".into());
/// assert_eq!(value.hash, "0x1234".into());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct IndexedDynamicValue<T> {
    phantom: PhantomData<T>,

    /// The hash of the value that was indexed, **not** the real value
    /// that was actually indexed. The original real value cannot be
    /// retrieved.
    pub hash: Vec<u8>,
}

impl<T> IndexedDynamicValue<T> {
    pub fn new(topic: Vec<u8>) -> Self {
        Self {
            phantom: PhantomData,
            hash: topic,
        }
    }
}

impl<T> From<Vec<u8>> for IndexedDynamicValue<T> {
    fn from(topic: Vec<u8>) -> Self {
        Self::new(topic)
    }
}
