use crate::pb::eth::v2::{Call, Log};
use crate::{pb::eth::v2 as pb, Event};

impl pb::Block {
    /// Iterates over successful transactions.
    pub fn transactions(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        self.transaction_traces.iter().filter(|tx| tx.status == 1)
    }

    /// Iterates over transaction receipts of successful transactions.
    pub fn receipts(&self) -> impl Iterator<Item = ReceiptView> {
        self.transactions().map(|transaction| transaction.receipt())
    }

    /// Iterates over logs in receipts of succesful transactions.
    pub fn logs(&self) -> impl Iterator<Item = LogView> {
        self.receipts().map(|receipt| receipt.logs()).flatten()
    }

    /// Iterates over calls of successful transactions.
    pub fn calls(&self) -> impl Iterator<Item = CallView> {
        self.transactions().map(|trx| trx.calls()).flatten()
    }

    /// A convenience for handlers that process a single type of event. Returns an iterator over
    /// pairs of `(event, log)`.
    ///
    /// If you need to process multiple event types in a single handler, try something like:
    /// ```ignore
    /// for log in block.logs() {
    ///     if !addresses.contains(&log.address()) {
    ///        continue;
    ///     }
    ///
    ///     if let Some(event) = E1::match_and_decode(log) {
    ///         // Process events of type E1
    ///     } else if let Some(event) = E2::match_and_decode(log) {
    ///         // Process events of type E2
    ///     }
    /// }
    /// ```
    pub fn events<'a, E: Event>(
        &'a self,
        addresses: &'a [&[u8]],
    ) -> impl Iterator<Item = (E, LogView)> {
        self.logs().filter_map(|log| {
            if !addresses.contains(&log.address()) {
                return None;
            }

            E::match_and_decode(log).map(|e| (e, log))
        })
    }

    pub fn timestamp_seconds(&self) -> u64 {
        self.header
            .as_ref()
            .unwrap()
            .timestamp
            .as_ref()
            .unwrap()
            .seconds as u64
    }
}

#[derive(Copy, Clone)]
pub struct ReceiptView<'a> {
    pub transaction: &'a pb::TransactionTrace,
    pub receipt: &'a pb::TransactionReceipt,
}

#[derive(Copy, Clone)]
pub struct LogView<'a> {
    pub receipt: ReceiptView<'a>,
    pub log: &'a pb::Log,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CallView<'a> {
    pub transaction: &'a pb::TransactionTrace,
    pub call: &'a pb::Call,
}

impl CallView<'_> {
    pub fn parent(&self) -> Option<&Call> {
        return self
            .transaction
            .calls
            .iter()
            .find(|call| call.index == self.call.parent_index);
    }
}

impl AsRef<pb::Call> for CallView<'_> {
    fn as_ref(&self) -> &pb::Call {
        self.call
    }
}

impl pb::TransactionTrace {
    pub fn calls(&self) -> impl Iterator<Item = CallView> {
        self.calls.iter().map(move |call| CallView {
            transaction: self,
            call,
        })
    }

    pub fn receipt(&self) -> ReceiptView {
        ReceiptView {
            transaction: self,
            receipt: &self.receipt.as_ref().unwrap(),
        }
    }

    /// Iterates over all logs in the transaction, excluding those from calls that were not
    /// recorded to the chain's state.
    ///
    /// The logs are sorted by their ordinal and returned as pairs of `(log, call)` where `call`
    /// is the call that produced the log.
    pub fn logs_with_calls(&self) -> impl Iterator<Item = (&Log, CallView)> {
        let mut res: Vec<(&Log, CallView)> = Vec::with_capacity(
            self.calls
                .iter()
                .filter(|call| !call.state_reverted)
                .map(|call| call.logs.len())
                .sum(),
        );

        for call in self.calls.iter() {
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                res.push((
                    &log,
                    CallView {
                        transaction: self,
                        call,
                    },
                ));
            }
        }

        res.sort_by(|x, y| x.0.ordinal.cmp(&y.0.ordinal));
        res.into_iter()
    }

    // TODO: Call view, filtering out failed calls
    // pub fn calls: Vec<CallView> { }
}

impl<'a> ReceiptView<'a> {
    pub fn state_root(self) -> &'a [u8] {
        &self.receipt.state_root
    }

    pub fn cumulative_gas_used(self) -> u64 {
        self.receipt.cumulative_gas_used
    }

    pub fn logs_bloom(self) -> &'a [u8] {
        &self.receipt.logs_bloom
    }

    pub fn logs(self) -> impl Iterator<Item = LogView<'a>> {
        self.receipt
            .logs
            .iter()
            .map(move |log| LogView { receipt: self, log })
    }
}

impl<'a> LogView<'a> {
    pub fn address(self) -> &'a [u8] {
        &self.log.address
    }

    pub fn topics(self) -> &'a Vec<Vec<u8>> {
        &self.log.topics
    }

    pub fn data(self) -> &'a [u8] {
        &self.log.data
    }

    pub fn index(self) -> u32 {
        self.log.index
    }

    pub fn block_index(self) -> u32 {
        self.log.block_index
    }

    pub fn ordinal(self) -> u64 {
        self.log.ordinal
    }
}

impl AsRef<pb::Log> for LogView<'_> {
    fn as_ref(&self) -> &pb::Log {
        self.log
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        block_view::CallView,
        pb::eth::v2::{Call, Log, TransactionTrace},
    };

    #[test]
    fn logs_with_calls() {
        let call = |to: &str, state_reverted, logs| Call {
            address: to.to_string().into_bytes(),
            state_reverted,
            logs,
            ..Default::default()
        };

        let log = |ordinal| Log {
            ordinal,
            ..Default::default()
        };

        let trace = TransactionTrace {
            calls: vec![
                call("1", true, vec![log(0)]),
                call("2", false, vec![log(8), log(2)]),
                call("3", false, vec![log(4)]),
                call("4", true, vec![log(1), log(3)]),
            ],
            ..Default::default()
        };

        let call_at = |call_index: usize| CallView {
            call: trace.calls.get(call_index).unwrap(),
            transaction: &trace,
        };

        let log_at = |call_index: usize, log_index: usize| {
            call_at(call_index).call.logs.get(log_index).unwrap()
        };

        assert_eq!(
            Vec::from_iter(trace.logs_with_calls()),
            vec![
                (log_at(1, 1), call_at(1)),
                (log_at(2, 0), call_at(2)),
                (log_at(1, 0), call_at(1)),
            ]
        );
    }
}
