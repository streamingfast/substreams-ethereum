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
            Err(err) => panic!(
                "Log for event `{}` at index {} matched but failed to decode with error: {}",
                Self::NAME,
                log.block_index,
                err
            ),
        }
    }
}

impl AsRef<Log> for Log {
    fn as_ref(&self) -> &Self {
        self
    }
}
