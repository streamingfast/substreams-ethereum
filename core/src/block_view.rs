use buffa_types::google::protobuf::Timestamp;

use crate::pb::eth::v2::{Call, Log};
use crate::{pb::eth::v2 as pb, Event};

impl pb::Block {
    /// Iterates over successful transactions
    pub fn transactions(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        self.transaction_traces
            .iter()
            .filter(|tx| tx.status == pb::TransactionTraceStatus::Succeeded)
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
    ) -> impl Iterator<Item = (E, LogView<'a>)> {
        self.logs().filter_map(|log| {
            if !addresses.contains(&log.address()) {
                return None;
            }

            E::match_and_decode(log).map(|e| (e, log))
        })
    }

    /// Timestamp returns a reference to the block's header timestamp.
    ///
    /// A block with no header, or a header with no timestamp, yields the default
    /// `Timestamp` (the Unix epoch) rather than panicking.
    pub fn timestamp(&self) -> &Timestamp {
        &self.header.timestamp
    }

    /// Timestamp returns block's header timestamp in seconds.
    ///
    /// A block with no header, or a header with no timestamp, yields `0` rather than
    /// panicking.
    pub fn timestamp_seconds(&self) -> u64 {
        self.header.timestamp.seconds as u64
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

    /// A transaction with no receipt yields a default `ReceiptView`, which reports no
    /// logs, rather than panicking.
    pub fn receipt(&self) -> ReceiptView {
        ReceiptView {
            transaction: self,
            receipt: &self.receipt,
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

        res.sort_by_key(|(log, _)| log.ordinal);
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

    pub fn ordinal(self) -> u64 {
        self.log.ordinal
    }

    /// The log's index within the block.
    pub fn block_index(self) -> u32 {
        self.log.block_index
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

/// Block accessors over buffa's lazy views, mirroring the `pb::Block` methods above.
///
/// A deferred field is only decoded when read, so these yield values rather than references
/// and can fail. Each has a `try_` twin that surfaces the error; the plain one skips it.
mod lazy {
    use crate::pb::eth::v2::__buffa::lazy_view::{
        BlockLazyView, CallLazyView, LogLazyView, TransactionReceiptLazyView,
        TransactionTraceLazyView,
    };
    use buffa::DecodeError;

    /// A log together with the call that emitted it.
    pub struct LazyLogWithCall<'a> {
        pub log: LogLazyView<'a>,
        pub call: CallLazyView<'a>,
    }

    impl<'a> BlockLazyView<'a> {
        /// Iterates over successful transactions, skipping any that fail to decode.
        pub fn transactions(&self) -> impl Iterator<Item = TransactionTraceLazyView<'a>> + '_ {
            self.try_transactions().filter_map(Result::ok)
        }

        /// Iterates over successful transactions, surfacing decode errors.
        pub fn try_transactions(
            &self,
        ) -> impl Iterator<Item = Result<TransactionTraceLazyView<'a>, DecodeError>> + '_ {
            // Keep undecodable entries so the error reaches the caller.
            self.transaction_traces.iter().filter(|transaction| {
                transaction
                    .as_ref()
                    .map(|transaction| {
                        transaction.status == crate::pb::eth::v2::TransactionTraceStatus::Succeeded
                    })
                    .unwrap_or(true)
            })
        }

        /// Iterates over transaction receipts of successful transactions, skipping any that
        /// fail to decode.
        pub fn receipts(&self) -> impl Iterator<Item = TransactionReceiptLazyView<'a>> + '_ {
            self.try_receipts().filter_map(Result::ok)
        }

        /// Iterates over transaction receipts of successful transactions, surfacing decode
        /// errors. A transaction with no receipt yields a default one, as on the owned block.
        pub fn try_receipts(
            &self,
        ) -> impl Iterator<Item = Result<TransactionReceiptLazyView<'a>, DecodeError>> + '_
        {
            self.try_transactions().map(|transaction| {
                transaction
                    .and_then(|transaction| transaction.receipt().map(Option::unwrap_or_default))
            })
        }

        /// Iterates over logs in receipts of successful transactions, skipping any that fail
        /// to decode.
        ///
        /// A corrupt log is dropped silently, so a malformed block yields a short list rather
        /// than an error. Use [`try_logs`](Self::try_logs) where that matters.
        pub fn logs(&self) -> impl Iterator<Item = LogLazyView<'a>> + '_ {
            self.try_logs().filter_map(Result::ok)
        }

        /// Iterates over logs in receipts of successful transactions, surfacing decode errors.
        pub fn try_logs(&self) -> impl Iterator<Item = Result<LogLazyView<'a>, DecodeError>> + '_ {
            self.try_receipts().flat_map(|receipt| match receipt {
                Ok(receipt) => receipt.logs.iter().collect::<Vec<_>>(),
                Err(err) => vec![Err(err)],
            })
        }

        /// Iterates over calls of successful transactions, skipping any that fail to decode.
        pub fn calls(&self) -> impl Iterator<Item = CallLazyView<'a>> + '_ {
            self.try_calls().filter_map(Result::ok)
        }

        /// Iterates over calls of successful transactions, surfacing decode errors.
        pub fn try_calls(
            &self,
        ) -> impl Iterator<Item = Result<CallLazyView<'a>, DecodeError>> + '_ {
            self.try_transactions()
                .flat_map(|transaction| match transaction {
                    Ok(transaction) => transaction.calls.iter().collect::<Vec<_>>(),
                    Err(err) => vec![Err(err)],
                })
        }
    }

    impl<'a> TransactionTraceLazyView<'a> {
        pub fn receipt(&self) -> Result<Option<TransactionReceiptLazyView<'a>>, DecodeError> {
            self.receipt.get()
        }

        /// All logs in the transaction, excluding those from calls that were not recorded to
        /// the chain's state, sorted by ordinal and paired with the call that produced them.
        pub fn logs_with_calls(&self) -> Result<Vec<LazyLogWithCall<'a>>, DecodeError> {
            let mut out = Vec::with_capacity(self.calls.len());

            for call in self.calls.iter() {
                let call = call?;
                if call.state_reverted {
                    continue;
                }

                for log in call.logs.iter() {
                    out.push(LazyLogWithCall {
                        log: log?,
                        call: call.clone(),
                    });
                }
            }

            out.sort_by_key(|entry| entry.log.ordinal);

            Ok(out)
        }
    }
}

pub use lazy::LazyLogWithCall;

#[cfg(test)]
mod lazy_tests {
    use crate::pb::eth::v2::{self as pb, __buffa::lazy_view::BlockLazyView};
    use buffa::view::LazyMessageView;
    use buffa::Message;

    fn block_with_statuses(statuses: &[i32]) -> pb::Block {
        pb::Block {
            number: 12_345,
            hash: vec![0xaa; 32],
            transaction_traces: statuses
                .iter()
                .enumerate()
                .map(|(index, status)| pb::TransactionTrace {
                    hash: vec![index as u8; 32],
                    status: (*status).into(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn it_selects_the_same_transactions_as_the_owned_block() {
        let block = block_with_statuses(&[1, 2, 1, 3, 1]);
        let bytes = block.encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        assert_eq!(view.number, block.number);
        assert_eq!(view.hash, &block.hash[..]);

        let expected: Vec<_> = block.transactions().map(|tx| tx.hash.clone()).collect();
        let actual: Vec<_> = view.transactions().map(|tx| tx.hash.to_vec()).collect();

        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 3, "only the status=1 transactions are kept");
    }

    #[test]
    fn it_yields_nothing_for_a_block_without_successful_transactions() {
        let bytes = block_with_statuses(&[2, 3]).encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        assert_eq!(view.transactions().count(), 0);
    }

    #[test]
    fn it_surfaces_decode_errors_through_every_try_accessor() {
        let mut bytes = block_with_statuses(&[1]).encode_to_vec();
        let last = bytes.len() - 1;
        bytes[last] = 0xff;

        let view = BlockLazyView::decode_lazy(&bytes).expect("the block's own fields are valid");

        assert!(
            view.try_transactions().any(|entry| entry.is_err()),
            "try_transactions must report a malformed transaction"
        );
        assert!(
            view.try_receipts().any(|entry| entry.is_err()),
            "try_receipts must report a malformed transaction"
        );
        assert!(
            view.try_logs().any(|entry| entry.is_err()),
            "try_logs must report a malformed transaction"
        );
        assert!(
            view.try_calls().any(|entry| entry.is_err()),
            "try_calls must report a malformed transaction"
        );

        assert_eq!(view.transactions().count(), 0, "the lossy twin skips it");
    }
}

#[cfg(test)]
mod lazy_view_parity_tests {
    use crate::pb::eth::v2::{
        self as pb, Call, Log, TransactionReceipt, TransactionTrace,
        __buffa::lazy_view::BlockLazyView,
    };
    use buffa::view::LazyMessageView;
    use buffa::Message;

    fn log(ordinal: u64, index: u32) -> Log {
        Log {
            address: vec![index as u8; 20],
            data: vec![index as u8; 4],
            index,
            ordinal,
            ..Default::default()
        }
    }

    fn block() -> pb::Block {
        pb::Block {
            number: 1,
            transaction_traces: vec![
                TransactionTrace {
                    hash: vec![1; 32],
                    status: pb::TransactionTraceStatus::Succeeded.into(),
                    calls: vec![
                        Call {
                            index: 0,
                            logs: vec![log(20, 0), log(10, 1)],
                            ..Default::default()
                        },
                        Call {
                            index: 1,
                            state_reverted: true,
                            logs: vec![log(30, 2)],
                            ..Default::default()
                        },
                    ],
                    receipt: TransactionReceipt {
                        logs: vec![log(20, 0), log(10, 1)],
                        cumulative_gas_used: 21_000,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                TransactionTrace {
                    hash: vec![2; 32],
                    status: pb::TransactionTraceStatus::Failed.into(),
                    receipt: TransactionReceipt {
                        logs: vec![log(40, 3)],
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn it_yields_receipts_of_successful_transactions_only() {
        let bytes = block().encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        let gas: Vec<_> = view.receipts().map(|r| r.cumulative_gas_used).collect();

        assert_eq!(gas, vec![21_000], "the status=2 transaction is excluded");
    }

    #[test]
    fn it_yields_a_default_receipt_for_a_transaction_without_one() {
        let block = pb::Block {
            transaction_traces: vec![TransactionTrace {
                hash: vec![1; 32],
                status: pb::TransactionTraceStatus::Succeeded.into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let bytes = block.encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        assert_eq!(
            block.receipts().count(),
            1,
            "owned block yields one receipt"
        );
        assert_eq!(view.receipts().count(), 1, "lazy view must agree");
    }

    #[test]
    fn it_yields_logs_of_successful_transactions_only() {
        let bytes = block().encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        let indexes: Vec<_> = view.logs().map(|l| l.index).collect();

        assert_eq!(
            indexes,
            vec![0, 1],
            "log 3 belongs to the failed transaction"
        );
    }

    #[test]
    fn it_yields_calls_of_successful_transactions_only() {
        let bytes = block().encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        assert_eq!(view.calls().count(), 2);
    }

    #[test]
    fn it_pairs_logs_with_their_calls_skipping_reverted() {
        let bytes = block().encode_to_vec();
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        let transaction = view.transactions().next().expect("one successful trx");
        let pairs = transaction.logs_with_calls().expect("decodes");

        let ordinals: Vec<_> = pairs.iter().map(|entry| entry.log.ordinal).collect();
        assert_eq!(ordinals, vec![10, 20], "sorted by ordinal");

        assert!(
            pairs.iter().all(|entry| entry.call.index == 0),
            "the reverted call's log is excluded"
        );
    }
}
