use crate::{pb::eth::v1 as pb, Event};

impl pb::Block {
    /// Iterates over succesful transactions.
    pub fn transactions(&self) -> impl Iterator<Item = TransactionView> {
        self.transaction_traces
            .iter()
            .filter(|tx| tx.status == 1)
            .map(|transaction| TransactionView {
                block: self,
                transaction,
            })
    }

    /// Iterates over transaction receipts of succesful transactions.
    pub fn receipts(&self) -> impl Iterator<Item = ReceiptView> {
        self.transactions().map(|transaction| ReceiptView {
            transaction,
            receipt: transaction.transaction.receipt.as_ref().unwrap(),
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
pub struct TransactionView<'a> {
    pub block: &'a pb::Block,
    pub transaction: &'a pb::TransactionTrace,
}

#[derive(Copy, Clone)]
pub struct ReceiptView<'a> {
    pub transaction: TransactionView<'a>,
    pub receipt: &'a pb::TransactionReceipt,
}

#[derive(Copy, Clone)]
pub struct LogView<'a> {
    pub receipt: ReceiptView<'a>,
    pub log: &'a pb::Log,
}

impl<'a> TransactionView<'a> {
    pub fn to(self) -> &'a [u8] {
        &self.transaction.to
    }

    pub fn nonce(self) -> u64 {
        self.transaction.nonce
    }

    pub fn gas_price(self) -> &'a pb::BigInt {
        self.transaction.gas_price.as_ref().unwrap()
    }

    pub fn gas_limit(self) -> u64 {
        self.transaction.gas_limit
    }
    pub fn value(self) -> &'a pb::BigInt {
        self.transaction.value.as_ref().unwrap()
    }
    pub fn input(self) -> &'a [u8] {
        &self.transaction.input
    }
    pub fn v(self) -> &'a [u8] {
        &self.transaction.v
    }
    pub fn r(self) -> &'a [u8] {
        &self.transaction.r
    }
    pub fn s(self) -> &'a [u8] {
        &self.transaction.s
    }
    pub fn gas_used(self) -> u64 {
        self.transaction.gas_used
    }
    pub fn r#type(self) -> i32 {
        self.transaction.r#type
    }
    pub fn access_list(self) -> &'a Vec<pb::AccessTuple> {
        &self.transaction.access_list
    }
    pub fn max_fee_per_gas(self) -> Option<&'a pb::BigInt> {
        self.transaction.max_fee_per_gas.as_ref()
    }
    pub fn max_priority_fee_per_gas(self) -> Option<&'a pb::BigInt> {
        self.transaction.max_priority_fee_per_gas.as_ref()
    }
    pub fn index(self) -> u32 {
        self.transaction.index
    }
    pub fn hash(self) -> &'a [u8] {
        &self.transaction.hash
    }
    pub fn from(self) -> &'a [u8] {
        &self.transaction.from
    }
    pub fn return_data(self) -> &'a [u8] {
        &self.transaction.return_data
    }
    pub fn public_key(self) -> &'a [u8] {
        &self.transaction.public_key
    }
    pub fn begin_ordinal(self) -> u64 {
        self.transaction.begin_ordinal
    }
    pub fn end_ordinal(self) -> u64 {
        self.transaction.end_ordinal
    }
    pub fn status(self) -> i32 {
        self.transaction.status
    }
    pub fn receipt(self) -> ReceiptView<'a> {
        ReceiptView {
            transaction: self,
            receipt: &self.transaction.receipt.as_ref().unwrap(),
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
