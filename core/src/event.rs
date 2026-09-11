use std::marker::PhantomData;

use crate::pb::eth::v2::Log;

/// The fields ABI decoding reads off a log.
///
/// Implemented by the owned `Log`, the borrowed `LogView`, and buffa's `LogLazyView`, so
/// generated event code is written once and works with any of them.
pub trait LogLike {
    fn topic(&self, index: usize) -> Option<&[u8]>;
    fn topic_count(&self) -> usize;
    fn data(&self) -> &[u8];
    fn index(&self) -> u32;
}

impl<T: LogLike + ?Sized> LogLike for &T {
    fn topic(&self, index: usize) -> Option<&[u8]> {
        (**self).topic(index)
    }

    fn topic_count(&self) -> usize {
        (**self).topic_count()
    }

    fn data(&self) -> &[u8] {
        (**self).data()
    }

    fn index(&self) -> u32 {
        (**self).index()
    }
}

impl LogLike for Log {
    fn topic(&self, index: usize) -> Option<&[u8]> {
        self.topics.get(index).map(|topic| topic.as_ref())
    }

    fn topic_count(&self) -> usize {
        self.topics.len()
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn index(&self) -> u32 {
        self.index
    }
}

impl LogLike for crate::pb::eth::v2::__buffa::lazy_view::LogLazyView<'_> {
    fn topic(&self, index: usize) -> Option<&[u8]> {
        self.topics.get(index).copied()
    }

    fn topic_count(&self) -> usize {
        self.topics.len()
    }

    fn data(&self) -> &[u8] {
        self.data
    }

    fn index(&self) -> u32 {
        self.index
    }
}

pub trait Event: Sized {
    const NAME: &'static str;

    fn match_log<L: LogLike>(log: &L) -> bool;
    fn decode<L: LogLike>(log: &L) -> Result<Self, String>;

    /// Attempts to match and decode the log.
    /// If `Self::match_log(log)` is `false`, returns `None`.
    /// If it matches, but decoding fails, logs the decoding error and returns `None`.
    fn match_and_decode<L: LogLike>(log: L) -> Option<Self> {
        let log = &log;
        if !Self::match_log(log) {
            return None;
        }

        match Self::decode(log) {
            Ok(event) => Some(event),
            Err(err) => {
                substreams::log::info!(
                    "Log for event `{}` at index {} matched but failed to decode with error: {}",
                    Self::NAME,
                    log.index(),
                    err
                );
                None
            }
        }
    }
}

impl LogLike for crate::block_view::LogView<'_> {
    fn topic(&self, index: usize) -> Option<&[u8]> {
        self.log.topic(index)
    }

    fn topic_count(&self) -> usize {
        self.log.topic_count()
    }

    fn data(&self) -> &[u8] {
        self.log.data()
    }

    fn index(&self) -> u32 {
        self.log.index()
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

#[cfg(test)]
mod tests {
    use super::LogLike;
    use crate::pb::eth::v2::{Log, __buffa::lazy_view::LogLazyView};
    use buffa::view::LazyMessageView;
    use buffa::Message;

    #[test]
    fn it_reads_the_same_fields_from_both_representations() {
        let log = Log {
            topics: vec![vec![0xdd; 32], vec![0x01; 32], vec![0x02; 32]],
            data: vec![0x09; 64],
            ..Default::default()
        };

        let bytes = log.encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        assert_eq!(log.topic_count(), lazy.topic_count());
        assert_eq!(log.data(), lazy.data());
        for index in 0..log.topic_count() + 1 {
            assert_eq!(log.topic(index), lazy.topic(index), "topic {index}");
        }
    }

    #[test]
    fn it_reports_no_topics_for_an_empty_log() {
        let bytes = Log::default().encode_to_vec();
        let lazy = LogLazyView::decode_lazy(&bytes).expect("valid log");

        assert_eq!(lazy.topic_count(), 0);
        assert_eq!(lazy.topic(0), None);
        assert!(lazy.data().is_empty());
    }
}
