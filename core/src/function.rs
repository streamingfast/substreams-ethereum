use crate::pb::eth::v2::Call;

pub trait Function: Sized {
    const NAME: &'static str;

    fn match_call(log: &Call) -> bool;
    fn decode(log: &Call) -> Result<Self, String>;
    fn encode(&self) -> Vec<u8>;

    /// Attempts to match and decode the call.
    /// If `Self::match_call(log)` is `false`, returns `None`.
    /// If it matches, but decoding fails, logs the decoding error and returns `None`.
    fn match_and_decode(call: impl AsRef<Call>) -> Option<Self> {
        let call = call.as_ref();
        if !Self::match_call(call) {
            return None;
        }

        match Self::decode(&call) {
            Ok(function) => Some(function),
            Err(err) => {
                substreams::log::info!(
                    "Call for function `{}` at index {} matched but failed to decode with error: {}",
                    Self::NAME,
                    call.index,
                    err
                );
                None
            }
        }
    }
}

impl AsRef<Call> for Call {
    fn as_ref(&self) -> &Self {
        self
    }
}
