use crate::{pb::eth::v1 as pb, Event};

impl pb::Block {
    /// Iterates over succesful transactions.
    pub fn transactions(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        self.transaction_traces.iter().filter(|tx| tx.status == 1)
    }

    /// Iterates over transaction receipts of succesful transactions.
    pub fn receipts(&self) -> impl Iterator<Item = ReceiptView> {
        self.transactions().map(|transaction| ReceiptView {
            transaction,
            receipt: transaction.receipt.as_ref().unwrap(),
        })
    }

    /// Iterates over logs in receipts of succesful transactions.
    pub fn logs(&self) -> impl Iterator<Item = LogView> {
        self.receipts().map(|receipt| receipt.logs()).flatten()
    }

    /// Filters logs returned by `self.logs()` for events of type `E` matching any address in
    /// `addresses`.
    ///
    /// Returns an iteratos over pairs of `(event, log)`.
    pub fn events<'a, E: Event>(
        &'a self,
        addresses: &'a [&[u8]],
    ) -> impl Iterator<Item = (E, LogView)> {
        self.logs().filter_map(|log| {
            if !addresses.contains(&log.address()) || !E::match_log(log.log) {
                return None;
            }

            match E::decode(&log.log) {
                Ok(event) => Some((event, log)),
                Err(err) => {
                    substreams::log::info!(
                    "Log for event `{}` at index {} matched but failed to decode with error: {}",
                    E::NAME,
                    log.block_index(),
                    err
                );
                    None
                }
            }
        })
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

impl pb::TransactionTrace {
    pub fn receipt(&self) -> ReceiptView {
        ReceiptView {
            transaction: self,
            receipt: &self.receipt.as_ref().unwrap(),
        }
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
