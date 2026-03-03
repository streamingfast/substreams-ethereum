// Automatically generated rust module for 'type.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::super::super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransactionTraceStatus {
    UNKNOWN = 0,
    SUCCEEDED = 1,
    FAILED = 2,
    REVERTED = 3,
}

impl Default for TransactionTraceStatus {
    fn default() -> Self {
        TransactionTraceStatus::UNKNOWN
    }
}

impl From<i32> for TransactionTraceStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => TransactionTraceStatus::UNKNOWN,
            1 => TransactionTraceStatus::SUCCEEDED,
            2 => TransactionTraceStatus::FAILED,
            3 => TransactionTraceStatus::REVERTED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TransactionTraceStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => TransactionTraceStatus::UNKNOWN,
            "SUCCEEDED" => TransactionTraceStatus::SUCCEEDED,
            "FAILED" => TransactionTraceStatus::FAILED,
            "REVERTED" => TransactionTraceStatus::REVERTED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CallType {
    UNSPECIFIED = 0,
    CALL = 1,
    CALLCODE = 2,
    DELEGATE = 3,
    STATIC = 4,
    CREATE = 5,
}

impl Default for CallType {
    fn default() -> Self {
        CallType::UNSPECIFIED
    }
}

impl From<i32> for CallType {
    fn from(i: i32) -> Self {
        match i {
            0 => CallType::UNSPECIFIED,
            1 => CallType::CALL,
            2 => CallType::CALLCODE,
            3 => CallType::DELEGATE,
            4 => CallType::STATIC,
            5 => CallType::CREATE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CallType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNSPECIFIED" => CallType::UNSPECIFIED,
            "CALL" => CallType::CALL,
            "CALLCODE" => CallType::CALLCODE,
            "DELEGATE" => CallType::DELEGATE,
            "STATIC" => CallType::STATIC,
            "CREATE" => CallType::CREATE,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Block<'a> {
    pub hash: Cow<'a, [u8]>,
    pub number: u64,
    pub size: u64,
    pub header: Option<sf::ethereum::r#type::v2::BlockHeader<'a>>,
    pub uncles: Vec<sf::ethereum::r#type::v2::BlockHeader<'a>>,
    pub transaction_traces: Vec<sf::ethereum::r#type::v2::TransactionTrace<'a>>,
    pub balance_changes: Vec<sf::ethereum::r#type::v2::BalanceChange<'a>>,
    pub detail_level: sf::ethereum::r#type::v2::mod_Block::DetailLevel,
    pub code_changes: Vec<sf::ethereum::r#type::v2::CodeChange<'a>>,
    pub system_calls: Vec<sf::ethereum::r#type::v2::Call<'a>>,
    pub ver: i32,
}

impl<'a> MessageRead<'a> for Block<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.number = r.read_uint64(bytes)?,
                Ok(32) => msg.size = r.read_uint64(bytes)?,
                Ok(42) => msg.header = Some(r.read_message::<sf::ethereum::r#type::v2::BlockHeader>(bytes)?),
                Ok(50) => msg.uncles.push(r.read_message::<sf::ethereum::r#type::v2::BlockHeader>(bytes)?),
                Ok(82) => msg.transaction_traces.push(r.read_message::<sf::ethereum::r#type::v2::TransactionTrace>(bytes)?),
                Ok(90) => msg.balance_changes.push(r.read_message::<sf::ethereum::r#type::v2::BalanceChange>(bytes)?),
                Ok(96) => msg.detail_level = r.read_enum(bytes)?,
                Ok(162) => msg.code_changes.push(r.read_message::<sf::ethereum::r#type::v2::CodeChange>(bytes)?),
                Ok(170) => msg.system_calls.push(r.read_message::<sf::ethereum::r#type::v2::Call>(bytes)?),
                Ok(8) => msg.ver = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Block<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.number == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.number) as u64) }
        + if self.size == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.uncles.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.transaction_traces.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.balance_changes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.detail_level == sf::ethereum::r#type::v2::mod_Block::DetailLevel::DETAILLEVEL_EXTENDED { 0 } else { 1 + sizeof_varint(*(&self.detail_level) as u64) }
        + self.code_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.system_calls.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.ver == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.ver) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.hash))?; }
        if self.number != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.number))?; }
        if self.size != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.size))?; }
        if let Some(ref s) = self.header { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.uncles { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.transaction_traces { w.write_with_tag(82, |w| w.write_message(s))?; }
        for s in &self.balance_changes { w.write_with_tag(90, |w| w.write_message(s))?; }
        if self.detail_level != sf::ethereum::r#type::v2::mod_Block::DetailLevel::DETAILLEVEL_EXTENDED { w.write_with_tag(96, |w| w.write_enum(*&self.detail_level as i32))?; }
        for s in &self.code_changes { w.write_with_tag(162, |w| w.write_message(s))?; }
        for s in &self.system_calls { w.write_with_tag(170, |w| w.write_message(s))?; }
        if self.ver != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.ver))?; }
        Ok(())
    }
}

pub mod mod_Block {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DetailLevel {
    DETAILLEVEL_EXTENDED = 0,
    DETAILLEVEL_BASE = 2,
}

impl Default for DetailLevel {
    fn default() -> Self {
        DetailLevel::DETAILLEVEL_EXTENDED
    }
}

impl From<i32> for DetailLevel {
    fn from(i: i32) -> Self {
        match i {
            0 => DetailLevel::DETAILLEVEL_EXTENDED,
            2 => DetailLevel::DETAILLEVEL_BASE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DetailLevel {
    fn from(s: &'a str) -> Self {
        match s {
            "DETAILLEVEL_EXTENDED" => DetailLevel::DETAILLEVEL_EXTENDED,
            "DETAILLEVEL_BASE" => DetailLevel::DETAILLEVEL_BASE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockHeader<'a> {
    pub parent_hash: Cow<'a, [u8]>,
    pub uncle_hash: Cow<'a, [u8]>,
    pub coinbase: Cow<'a, [u8]>,
    pub state_root: Cow<'a, [u8]>,
    pub transactions_root: Cow<'a, [u8]>,
    pub receipt_root: Cow<'a, [u8]>,
    pub logs_bloom: Cow<'a, [u8]>,
    pub difficulty: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: Option<google::protobuf::Timestamp>,
    pub extra_data: Cow<'a, [u8]>,
    pub mix_hash: Cow<'a, [u8]>,
    pub nonce: u64,
    pub hash: Cow<'a, [u8]>,
    pub base_fee_per_gas: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub withdrawals_root: Cow<'a, [u8]>,
    pub tx_dependency: Option<sf::ethereum::r#type::v2::Uint64NestedArray>,
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
    pub parent_beacon_root: Cow<'a, [u8]>,
    pub requests_hash: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for BlockHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.parent_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.uncle_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.coinbase = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.state_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.transactions_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.receipt_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.logs_bloom = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.difficulty = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(72) => msg.number = r.read_uint64(bytes)?,
                Ok(80) => msg.gas_limit = r.read_uint64(bytes)?,
                Ok(88) => msg.gas_used = r.read_uint64(bytes)?,
                Ok(98) => msg.timestamp = Some(r.read_message::<google::protobuf::Timestamp>(bytes)?),
                Ok(106) => msg.extra_data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.mix_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(120) => msg.nonce = r.read_uint64(bytes)?,
                Ok(130) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(146) => msg.base_fee_per_gas = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(154) => msg.withdrawals_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(162) => msg.tx_dependency = Some(r.read_message::<sf::ethereum::r#type::v2::Uint64NestedArray>(bytes)?),
                Ok(176) => msg.blob_gas_used = r.read_uint64(bytes)?,
                Ok(184) => msg.excess_blob_gas = r.read_uint64(bytes)?,
                Ok(194) => msg.parent_beacon_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(202) => msg.requests_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.parent_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.parent_hash).len()) }
        + if self.uncle_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.uncle_hash).len()) }
        + if self.coinbase == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.coinbase).len()) }
        + if self.state_root == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.state_root).len()) }
        + if self.transactions_root == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.transactions_root).len()) }
        + if self.receipt_root == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.receipt_root).len()) }
        + if self.logs_bloom == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.logs_bloom).len()) }
        + self.difficulty.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.number == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.number) as u64) }
        + if self.gas_limit == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_limit) as u64) }
        + if self.gas_used == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_used) as u64) }
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.extra_data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.extra_data).len()) }
        + if self.mix_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.mix_hash).len()) }
        + if self.nonce == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.nonce) as u64) }
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.hash).len()) }
        + self.base_fee_per_gas.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.withdrawals_root == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.withdrawals_root).len()) }
        + self.tx_dependency.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + if self.blob_gas_used == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.blob_gas_used) as u64) }
        + if self.excess_blob_gas == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.excess_blob_gas) as u64) }
        + if self.parent_beacon_root == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.parent_beacon_root).len()) }
        + if self.requests_hash == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.requests_hash).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.parent_hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.parent_hash))?; }
        if self.uncle_hash != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.uncle_hash))?; }
        if self.coinbase != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.coinbase))?; }
        if self.state_root != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.state_root))?; }
        if self.transactions_root != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.transactions_root))?; }
        if self.receipt_root != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.receipt_root))?; }
        if self.logs_bloom != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.logs_bloom))?; }
        if let Some(ref s) = self.difficulty { w.write_with_tag(66, |w| w.write_message(s))?; }
        if self.number != 0u64 { w.write_with_tag(72, |w| w.write_uint64(*&self.number))?; }
        if self.gas_limit != 0u64 { w.write_with_tag(80, |w| w.write_uint64(*&self.gas_limit))?; }
        if self.gas_used != 0u64 { w.write_with_tag(88, |w| w.write_uint64(*&self.gas_used))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(98, |w| w.write_message(s))?; }
        if self.extra_data != Cow::Borrowed(b"") { w.write_with_tag(106, |w| w.write_bytes(&**&self.extra_data))?; }
        if self.mix_hash != Cow::Borrowed(b"") { w.write_with_tag(114, |w| w.write_bytes(&**&self.mix_hash))?; }
        if self.nonce != 0u64 { w.write_with_tag(120, |w| w.write_uint64(*&self.nonce))?; }
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(130, |w| w.write_bytes(&**&self.hash))?; }
        if let Some(ref s) = self.base_fee_per_gas { w.write_with_tag(146, |w| w.write_message(s))?; }
        if self.withdrawals_root != Cow::Borrowed(b"") { w.write_with_tag(154, |w| w.write_bytes(&**&self.withdrawals_root))?; }
        if let Some(ref s) = self.tx_dependency { w.write_with_tag(162, |w| w.write_message(s))?; }
        if self.blob_gas_used != 0u64 { w.write_with_tag(176, |w| w.write_uint64(*&self.blob_gas_used))?; }
        if self.excess_blob_gas != 0u64 { w.write_with_tag(184, |w| w.write_uint64(*&self.excess_blob_gas))?; }
        if self.parent_beacon_root != Cow::Borrowed(b"") { w.write_with_tag(194, |w| w.write_bytes(&**&self.parent_beacon_root))?; }
        if self.requests_hash != Cow::Borrowed(b"") { w.write_with_tag(202, |w| w.write_bytes(&**&self.requests_hash))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Uint64NestedArray {
    pub val: Vec<sf::ethereum::r#type::v2::Uint64Array>,
}

impl<'a> MessageRead<'a> for Uint64NestedArray {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.val.push(r.read_message::<sf::ethereum::r#type::v2::Uint64Array>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Uint64NestedArray {
    fn get_size(&self) -> usize {
        0
        + self.val.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.val { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Uint64Array {
    pub val: Vec<u64>,
}

impl<'a> MessageRead<'a> for Uint64Array {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.val = r.read_packed(bytes, |r, bytes| Ok(r.read_uint64(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Uint64Array {
    fn get_size(&self) -> usize {
        0
        + if self.val.is_empty() { 0 } else { 1 + sizeof_len(self.val.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.val, |w, m| w.write_uint64(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BigInt<'a> {
    pub bytes: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for BigInt<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bytes = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BigInt<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.bytes == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bytes).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.bytes != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.bytes))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransactionTrace<'a> {
    pub to: Cow<'a, [u8]>,
    pub nonce: u64,
    pub gas_price: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub gas_limit: u64,
    pub value: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub input: Cow<'a, [u8]>,
    pub v: Cow<'a, [u8]>,
    pub r: Cow<'a, [u8]>,
    pub s: Cow<'a, [u8]>,
    pub gas_used: u64,
    pub type_pb: sf::ethereum::r#type::v2::mod_TransactionTrace::Type,
    pub access_list: Vec<sf::ethereum::r#type::v2::AccessTuple<'a>>,
    pub max_fee_per_gas: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub max_priority_fee_per_gas: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub index: u32,
    pub hash: Cow<'a, [u8]>,
    pub from: Cow<'a, [u8]>,
    pub return_data: Cow<'a, [u8]>,
    pub public_key: Cow<'a, [u8]>,
    pub begin_ordinal: u64,
    pub end_ordinal: u64,
    pub status: sf::ethereum::r#type::v2::TransactionTraceStatus,
    pub receipt: Option<sf::ethereum::r#type::v2::TransactionReceipt<'a>>,
    pub calls: Vec<sf::ethereum::r#type::v2::Call<'a>>,
    pub blob_gas: u64,
    pub blob_gas_fee_cap: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub blob_hashes: Vec<Cow<'a, [u8]>>,
    pub set_code_authorizations: Vec<sf::ethereum::r#type::v2::SetCodeAuthorization<'a>>,
}

impl<'a> MessageRead<'a> for TransactionTrace<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.nonce = r.read_uint64(bytes)?,
                Ok(26) => msg.gas_price = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(32) => msg.gas_limit = r.read_uint64(bytes)?,
                Ok(42) => msg.value = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(50) => msg.input = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.v = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.r = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.s = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.gas_used = r.read_uint64(bytes)?,
                Ok(96) => msg.type_pb = r.read_enum(bytes)?,
                Ok(114) => msg.access_list.push(r.read_message::<sf::ethereum::r#type::v2::AccessTuple>(bytes)?),
                Ok(90) => msg.max_fee_per_gas = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(106) => msg.max_priority_fee_per_gas = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(160) => msg.index = r.read_uint32(bytes)?,
                Ok(170) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(178) => msg.from = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(186) => msg.return_data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(194) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(200) => msg.begin_ordinal = r.read_uint64(bytes)?,
                Ok(208) => msg.end_ordinal = r.read_uint64(bytes)?,
                Ok(240) => msg.status = r.read_enum(bytes)?,
                Ok(250) => msg.receipt = Some(r.read_message::<sf::ethereum::r#type::v2::TransactionReceipt>(bytes)?),
                Ok(258) => msg.calls.push(r.read_message::<sf::ethereum::r#type::v2::Call>(bytes)?),
                Ok(264) => msg.blob_gas = r.read_uint64(bytes)?,
                Ok(274) => msg.blob_gas_fee_cap = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(282) => msg.blob_hashes.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(290) => msg.set_code_authorizations.push(r.read_message::<sf::ethereum::r#type::v2::SetCodeAuthorization>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransactionTrace<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.to).len()) }
        + if self.nonce == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.nonce) as u64) }
        + self.gas_price.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.gas_limit == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_limit) as u64) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.input == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.input).len()) }
        + if self.v == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.v).len()) }
        + if self.r == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.r).len()) }
        + if self.s == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.s).len()) }
        + if self.gas_used == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_used) as u64) }
        + if self.type_pb == sf::ethereum::r#type::v2::mod_TransactionTrace::Type::TRX_TYPE_LEGACY { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.access_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.max_fee_per_gas.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_priority_fee_per_gas.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.index == 0u32 { 0 } else { 2 + sizeof_varint(*(&self.index) as u64) }
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.hash).len()) }
        + if self.from == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.from).len()) }
        + if self.return_data == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.return_data).len()) }
        + if self.public_key == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.public_key).len()) }
        + if self.begin_ordinal == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.begin_ordinal) as u64) }
        + if self.end_ordinal == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.end_ordinal) as u64) }
        + if self.status == sf::ethereum::r#type::v2::TransactionTraceStatus::UNKNOWN { 0 } else { 2 + sizeof_varint(*(&self.status) as u64) }
        + self.receipt.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.calls.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.blob_gas == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.blob_gas) as u64) }
        + self.blob_gas_fee_cap.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.blob_hashes.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.set_code_authorizations.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.to != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.to))?; }
        if self.nonce != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.nonce))?; }
        if let Some(ref s) = self.gas_price { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.gas_limit != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.gas_limit))?; }
        if let Some(ref s) = self.value { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.input != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.input))?; }
        if self.v != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.v))?; }
        if self.r != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.r))?; }
        if self.s != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.s))?; }
        if self.gas_used != 0u64 { w.write_with_tag(80, |w| w.write_uint64(*&self.gas_used))?; }
        if self.type_pb != sf::ethereum::r#type::v2::mod_TransactionTrace::Type::TRX_TYPE_LEGACY { w.write_with_tag(96, |w| w.write_enum(*&self.type_pb as i32))?; }
        for s in &self.access_list { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_fee_per_gas { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_priority_fee_per_gas { w.write_with_tag(106, |w| w.write_message(s))?; }
        if self.index != 0u32 { w.write_with_tag(160, |w| w.write_uint32(*&self.index))?; }
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(170, |w| w.write_bytes(&**&self.hash))?; }
        if self.from != Cow::Borrowed(b"") { w.write_with_tag(178, |w| w.write_bytes(&**&self.from))?; }
        if self.return_data != Cow::Borrowed(b"") { w.write_with_tag(186, |w| w.write_bytes(&**&self.return_data))?; }
        if self.public_key != Cow::Borrowed(b"") { w.write_with_tag(194, |w| w.write_bytes(&**&self.public_key))?; }
        if self.begin_ordinal != 0u64 { w.write_with_tag(200, |w| w.write_uint64(*&self.begin_ordinal))?; }
        if self.end_ordinal != 0u64 { w.write_with_tag(208, |w| w.write_uint64(*&self.end_ordinal))?; }
        if self.status != sf::ethereum::r#type::v2::TransactionTraceStatus::UNKNOWN { w.write_with_tag(240, |w| w.write_enum(*&self.status as i32))?; }
        if let Some(ref s) = self.receipt { w.write_with_tag(250, |w| w.write_message(s))?; }
        for s in &self.calls { w.write_with_tag(258, |w| w.write_message(s))?; }
        if self.blob_gas != 0u64 { w.write_with_tag(264, |w| w.write_uint64(*&self.blob_gas))?; }
        if let Some(ref s) = self.blob_gas_fee_cap { w.write_with_tag(274, |w| w.write_message(s))?; }
        for s in &self.blob_hashes { w.write_with_tag(282, |w| w.write_bytes(&**s))?; }
        for s in &self.set_code_authorizations { w.write_with_tag(290, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TransactionTrace {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    TRX_TYPE_LEGACY = 0,
    TRX_TYPE_ACCESS_LIST = 1,
    TRX_TYPE_DYNAMIC_FEE = 2,
    TRX_TYPE_BLOB = 3,
    TRX_TYPE_SET_CODE = 4,
    TRX_TYPE_ARBITRUM_DEPOSIT = 100,
    TRX_TYPE_ARBITRUM_UNSIGNED = 101,
    TRX_TYPE_ARBITRUM_CONTRACT = 102,
    TRX_TYPE_ARBITRUM_RETRY = 104,
    TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE = 105,
    TRX_TYPE_ARBITRUM_INTERNAL = 106,
    TRX_TYPE_ARBITRUM_LEGACY = 120,
    TRX_TYPE_OPTIMISM_DEPOSIT = 126,
}

impl Default for Type {
    fn default() -> Self {
        Type::TRX_TYPE_LEGACY
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::TRX_TYPE_LEGACY,
            1 => Type::TRX_TYPE_ACCESS_LIST,
            2 => Type::TRX_TYPE_DYNAMIC_FEE,
            3 => Type::TRX_TYPE_BLOB,
            4 => Type::TRX_TYPE_SET_CODE,
            100 => Type::TRX_TYPE_ARBITRUM_DEPOSIT,
            101 => Type::TRX_TYPE_ARBITRUM_UNSIGNED,
            102 => Type::TRX_TYPE_ARBITRUM_CONTRACT,
            104 => Type::TRX_TYPE_ARBITRUM_RETRY,
            105 => Type::TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE,
            106 => Type::TRX_TYPE_ARBITRUM_INTERNAL,
            120 => Type::TRX_TYPE_ARBITRUM_LEGACY,
            126 => Type::TRX_TYPE_OPTIMISM_DEPOSIT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "TRX_TYPE_LEGACY" => Type::TRX_TYPE_LEGACY,
            "TRX_TYPE_ACCESS_LIST" => Type::TRX_TYPE_ACCESS_LIST,
            "TRX_TYPE_DYNAMIC_FEE" => Type::TRX_TYPE_DYNAMIC_FEE,
            "TRX_TYPE_BLOB" => Type::TRX_TYPE_BLOB,
            "TRX_TYPE_SET_CODE" => Type::TRX_TYPE_SET_CODE,
            "TRX_TYPE_ARBITRUM_DEPOSIT" => Type::TRX_TYPE_ARBITRUM_DEPOSIT,
            "TRX_TYPE_ARBITRUM_UNSIGNED" => Type::TRX_TYPE_ARBITRUM_UNSIGNED,
            "TRX_TYPE_ARBITRUM_CONTRACT" => Type::TRX_TYPE_ARBITRUM_CONTRACT,
            "TRX_TYPE_ARBITRUM_RETRY" => Type::TRX_TYPE_ARBITRUM_RETRY,
            "TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE" => Type::TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE,
            "TRX_TYPE_ARBITRUM_INTERNAL" => Type::TRX_TYPE_ARBITRUM_INTERNAL,
            "TRX_TYPE_ARBITRUM_LEGACY" => Type::TRX_TYPE_ARBITRUM_LEGACY,
            "TRX_TYPE_OPTIMISM_DEPOSIT" => Type::TRX_TYPE_OPTIMISM_DEPOSIT,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccessTuple<'a> {
    pub address: Cow<'a, [u8]>,
    pub storage_keys: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for AccessTuple<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.storage_keys.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccessTuple<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + self.storage_keys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        for s in &self.storage_keys { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetCodeAuthorization<'a> {
    pub discarded: bool,
    pub chain_id: Cow<'a, [u8]>,
    pub address: Cow<'a, [u8]>,
    pub nonce: u64,
    pub v: u32,
    pub r: Cow<'a, [u8]>,
    pub s: Cow<'a, [u8]>,
    pub authority: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for SetCodeAuthorization<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.discarded = r.read_bool(bytes)?,
                Ok(18) => msg.chain_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.nonce = r.read_uint64(bytes)?,
                Ok(32) => msg.v = r.read_uint32(bytes)?,
                Ok(42) => msg.r = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.s = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.authority = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetCodeAuthorization<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.discarded == false { 0 } else { 1 + sizeof_varint(*(&self.discarded) as u64) }
        + if self.chain_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.chain_id).len()) }
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.nonce == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.nonce) as u64) }
        + if self.v == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.v) as u64) }
        + if self.r == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.r).len()) }
        + if self.s == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.s).len()) }
        + if self.authority == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.authority).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.discarded != false { w.write_with_tag(8, |w| w.write_bool(*&self.discarded))?; }
        if self.chain_id != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.chain_id))?; }
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.address))?; }
        if self.nonce != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.nonce))?; }
        if self.v != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.v))?; }
        if self.r != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.r))?; }
        if self.s != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.s))?; }
        if self.authority != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.authority))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransactionReceipt<'a> {
    pub state_root: Cow<'a, [u8]>,
    pub cumulative_gas_used: u64,
    pub logs_bloom: Cow<'a, [u8]>,
    pub logs: Vec<sf::ethereum::r#type::v2::Log<'a>>,
    pub blob_gas_used: u64,
    pub blob_gas_price: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
}

impl<'a> MessageRead<'a> for TransactionReceipt<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.state_root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.cumulative_gas_used = r.read_uint64(bytes)?,
                Ok(26) => msg.logs_bloom = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.logs.push(r.read_message::<sf::ethereum::r#type::v2::Log>(bytes)?),
                Ok(40) => msg.blob_gas_used = r.read_uint64(bytes)?,
                Ok(50) => msg.blob_gas_price = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransactionReceipt<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.state_root == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.state_root).len()) }
        + if self.cumulative_gas_used == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.cumulative_gas_used) as u64) }
        + if self.logs_bloom == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.logs_bloom).len()) }
        + self.logs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.blob_gas_used == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.blob_gas_used) as u64) }
        + self.blob_gas_price.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.state_root != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.state_root))?; }
        if self.cumulative_gas_used != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.cumulative_gas_used))?; }
        if self.logs_bloom != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.logs_bloom))?; }
        for s in &self.logs { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.blob_gas_used != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.blob_gas_used))?; }
        if let Some(ref s) = self.blob_gas_price { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Log<'a> {
    pub address: Cow<'a, [u8]>,
    pub topics: Vec<Cow<'a, [u8]>>,
    pub data: Cow<'a, [u8]>,
    pub index: u32,
    pub blockIndex: u32,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for Log<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.topics.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.index = r.read_uint32(bytes)?,
                Ok(48) => msg.blockIndex = r.read_uint32(bytes)?,
                Ok(56) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Log<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + self.topics.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.data).len()) }
        + if self.index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + if self.blockIndex == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.blockIndex) as u64) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        for s in &self.topics { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if self.data != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.data))?; }
        if self.index != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.index))?; }
        if self.blockIndex != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.blockIndex))?; }
        if self.ordinal != 0u64 { w.write_with_tag(56, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Call<'a> {
    pub index: u32,
    pub parent_index: u32,
    pub depth: u32,
    pub call_type: sf::ethereum::r#type::v2::CallType,
    pub caller: Cow<'a, [u8]>,
    pub address: Cow<'a, [u8]>,
    pub address_delegates_to: Cow<'a, [u8]>,
    pub value: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub gas_limit: u64,
    pub gas_consumed: u64,
    pub return_data: Cow<'a, [u8]>,
    pub input: Cow<'a, [u8]>,
    pub executed_code: bool,
    pub suicide: bool,
    pub keccak_preimages: KVMap<Cow<'a, str>, Cow<'a, str>>,
    pub storage_changes: Vec<sf::ethereum::r#type::v2::StorageChange<'a>>,
    pub balance_changes: Vec<sf::ethereum::r#type::v2::BalanceChange<'a>>,
    pub nonce_changes: Vec<sf::ethereum::r#type::v2::NonceChange<'a>>,
    pub logs: Vec<sf::ethereum::r#type::v2::Log<'a>>,
    pub code_changes: Vec<sf::ethereum::r#type::v2::CodeChange<'a>>,
    pub gas_changes: Vec<sf::ethereum::r#type::v2::GasChange>,
    pub status_failed: bool,
    pub status_reverted: bool,
    pub failure_reason: Cow<'a, str>,
    pub state_reverted: bool,
    pub begin_ordinal: u64,
    pub end_ordinal: u64,
}

impl<'a> MessageRead<'a> for Call<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = r.read_uint32(bytes)?,
                Ok(16) => msg.parent_index = r.read_uint32(bytes)?,
                Ok(24) => msg.depth = r.read_uint32(bytes)?,
                Ok(32) => msg.call_type = r.read_enum(bytes)?,
                Ok(42) => msg.caller = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(274) => msg.address_delegates_to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.value = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(64) => msg.gas_limit = r.read_uint64(bytes)?,
                Ok(72) => msg.gas_consumed = r.read_uint64(bytes)?,
                Ok(106) => msg.return_data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.input = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(120) => msg.executed_code = r.read_bool(bytes)?,
                Ok(128) => msg.suicide = r.read_bool(bytes)?,
                Ok(162) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?))?;
                    msg.keccak_preimages.insert(key, value);
                }
                Ok(170) => msg.storage_changes.push(r.read_message::<sf::ethereum::r#type::v2::StorageChange>(bytes)?),
                Ok(178) => msg.balance_changes.push(r.read_message::<sf::ethereum::r#type::v2::BalanceChange>(bytes)?),
                Ok(194) => msg.nonce_changes.push(r.read_message::<sf::ethereum::r#type::v2::NonceChange>(bytes)?),
                Ok(202) => msg.logs.push(r.read_message::<sf::ethereum::r#type::v2::Log>(bytes)?),
                Ok(210) => msg.code_changes.push(r.read_message::<sf::ethereum::r#type::v2::CodeChange>(bytes)?),
                Ok(226) => msg.gas_changes.push(r.read_message::<sf::ethereum::r#type::v2::GasChange>(bytes)?),
                Ok(80) => msg.status_failed = r.read_bool(bytes)?,
                Ok(96) => msg.status_reverted = r.read_bool(bytes)?,
                Ok(90) => msg.failure_reason = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(240) => msg.state_reverted = r.read_bool(bytes)?,
                Ok(248) => msg.begin_ordinal = r.read_uint64(bytes)?,
                Ok(256) => msg.end_ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Call<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + if self.parent_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.parent_index) as u64) }
        + if self.depth == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.depth) as u64) }
        + if self.call_type == sf::ethereum::r#type::v2::CallType::UNSPECIFIED { 0 } else { 1 + sizeof_varint(*(&self.call_type) as u64) }
        + if self.caller == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.caller).len()) }
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.address_delegates_to == Cow::Borrowed(b"") { 0 } else { 2 + sizeof_len((&self.address_delegates_to).len()) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.gas_limit == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_limit) as u64) }
        + if self.gas_consumed == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.gas_consumed) as u64) }
        + if self.return_data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.return_data).len()) }
        + if self.input == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.input).len()) }
        + if self.executed_code == false { 0 } else { 1 + sizeof_varint(*(&self.executed_code) as u64) }
        + if self.suicide == false { 0 } else { 2 + sizeof_varint(*(&self.suicide) as u64) }
        + self.keccak_preimages.iter().map(|(k, v)| 2 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
        + self.storage_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.balance_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.nonce_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.logs.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.code_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.gas_changes.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.status_failed == false { 0 } else { 1 + sizeof_varint(*(&self.status_failed) as u64) }
        + if self.status_reverted == false { 0 } else { 1 + sizeof_varint(*(&self.status_reverted) as u64) }
        + if self.failure_reason == "" { 0 } else { 1 + sizeof_len((&self.failure_reason).len()) }
        + if self.state_reverted == false { 0 } else { 2 + sizeof_varint(*(&self.state_reverted) as u64) }
        + if self.begin_ordinal == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.begin_ordinal) as u64) }
        + if self.end_ordinal == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.end_ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.index != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.index))?; }
        if self.parent_index != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.parent_index))?; }
        if self.depth != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.depth))?; }
        if self.call_type != sf::ethereum::r#type::v2::CallType::UNSPECIFIED { w.write_with_tag(32, |w| w.write_enum(*&self.call_type as i32))?; }
        if self.caller != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.caller))?; }
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.address))?; }
        if self.address_delegates_to != Cow::Borrowed(b"") { w.write_with_tag(274, |w| w.write_bytes(&**&self.address_delegates_to))?; }
        if let Some(ref s) = self.value { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.gas_limit != 0u64 { w.write_with_tag(64, |w| w.write_uint64(*&self.gas_limit))?; }
        if self.gas_consumed != 0u64 { w.write_with_tag(72, |w| w.write_uint64(*&self.gas_consumed))?; }
        if self.return_data != Cow::Borrowed(b"") { w.write_with_tag(106, |w| w.write_bytes(&**&self.return_data))?; }
        if self.input != Cow::Borrowed(b"") { w.write_with_tag(114, |w| w.write_bytes(&**&self.input))?; }
        if self.executed_code != false { w.write_with_tag(120, |w| w.write_bool(*&self.executed_code))?; }
        if self.suicide != false { w.write_with_tag(128, |w| w.write_bool(*&self.suicide))?; }
        for (k, v) in self.keccak_preimages.iter() { w.write_with_tag(162, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        for s in &self.storage_changes { w.write_with_tag(170, |w| w.write_message(s))?; }
        for s in &self.balance_changes { w.write_with_tag(178, |w| w.write_message(s))?; }
        for s in &self.nonce_changes { w.write_with_tag(194, |w| w.write_message(s))?; }
        for s in &self.logs { w.write_with_tag(202, |w| w.write_message(s))?; }
        for s in &self.code_changes { w.write_with_tag(210, |w| w.write_message(s))?; }
        for s in &self.gas_changes { w.write_with_tag(226, |w| w.write_message(s))?; }
        if self.status_failed != false { w.write_with_tag(80, |w| w.write_bool(*&self.status_failed))?; }
        if self.status_reverted != false { w.write_with_tag(96, |w| w.write_bool(*&self.status_reverted))?; }
        if self.failure_reason != "" { w.write_with_tag(90, |w| w.write_string(&**&self.failure_reason))?; }
        if self.state_reverted != false { w.write_with_tag(240, |w| w.write_bool(*&self.state_reverted))?; }
        if self.begin_ordinal != 0u64 { w.write_with_tag(248, |w| w.write_uint64(*&self.begin_ordinal))?; }
        if self.end_ordinal != 0u64 { w.write_with_tag(256, |w| w.write_uint64(*&self.end_ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StorageChange<'a> {
    pub address: Cow<'a, [u8]>,
    pub key: Cow<'a, [u8]>,
    pub old_value: Cow<'a, [u8]>,
    pub new_value: Cow<'a, [u8]>,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for StorageChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.old_value = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.new_value = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StorageChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.key == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.old_value == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.old_value).len()) }
        + if self.new_value == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.new_value).len()) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        if self.key != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.key))?; }
        if self.old_value != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.old_value))?; }
        if self.new_value != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.new_value))?; }
        if self.ordinal != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BalanceChange<'a> {
    pub address: Cow<'a, [u8]>,
    pub old_value: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub new_value: Option<sf::ethereum::r#type::v2::BigInt<'a>>,
    pub reason: sf::ethereum::r#type::v2::mod_BalanceChange::Reason,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for BalanceChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.old_value = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(26) => msg.new_value = Some(r.read_message::<sf::ethereum::r#type::v2::BigInt>(bytes)?),
                Ok(32) => msg.reason = r.read_enum(bytes)?,
                Ok(40) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BalanceChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + self.old_value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.new_value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.reason == sf::ethereum::r#type::v2::mod_BalanceChange::Reason::REASON_UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.reason) as u64) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        if let Some(ref s) = self.old_value { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.new_value { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.reason != sf::ethereum::r#type::v2::mod_BalanceChange::Reason::REASON_UNKNOWN { w.write_with_tag(32, |w| w.write_enum(*&self.reason as i32))?; }
        if self.ordinal != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

pub mod mod_BalanceChange {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reason {
    REASON_UNKNOWN = 0,
    REASON_REWARD_MINE_UNCLE = 1,
    REASON_REWARD_MINE_BLOCK = 2,
    REASON_DAO_REFUND_CONTRACT = 3,
    REASON_DAO_ADJUST_BALANCE = 4,
    REASON_TRANSFER = 5,
    REASON_GENESIS_BALANCE = 6,
    REASON_GAS_BUY = 7,
    REASON_REWARD_TRANSACTION_FEE = 8,
    REASON_REWARD_FEE_RESET = 14,
    REASON_GAS_REFUND = 9,
    REASON_TOUCH_ACCOUNT = 10,
    REASON_SUICIDE_REFUND = 11,
    REASON_SUICIDE_WITHDRAW = 13,
    REASON_CALL_BALANCE_OVERRIDE = 12,
    REASON_BURN = 15,
    REASON_WITHDRAWAL = 16,
    REASON_REWARD_BLOB_FEE = 17,
    REASON_INCREASE_MINT = 18,
    REASON_REVERT = 19,
}

impl Default for Reason {
    fn default() -> Self {
        Reason::REASON_UNKNOWN
    }
}

impl From<i32> for Reason {
    fn from(i: i32) -> Self {
        match i {
            0 => Reason::REASON_UNKNOWN,
            1 => Reason::REASON_REWARD_MINE_UNCLE,
            2 => Reason::REASON_REWARD_MINE_BLOCK,
            3 => Reason::REASON_DAO_REFUND_CONTRACT,
            4 => Reason::REASON_DAO_ADJUST_BALANCE,
            5 => Reason::REASON_TRANSFER,
            6 => Reason::REASON_GENESIS_BALANCE,
            7 => Reason::REASON_GAS_BUY,
            8 => Reason::REASON_REWARD_TRANSACTION_FEE,
            14 => Reason::REASON_REWARD_FEE_RESET,
            9 => Reason::REASON_GAS_REFUND,
            10 => Reason::REASON_TOUCH_ACCOUNT,
            11 => Reason::REASON_SUICIDE_REFUND,
            13 => Reason::REASON_SUICIDE_WITHDRAW,
            12 => Reason::REASON_CALL_BALANCE_OVERRIDE,
            15 => Reason::REASON_BURN,
            16 => Reason::REASON_WITHDRAWAL,
            17 => Reason::REASON_REWARD_BLOB_FEE,
            18 => Reason::REASON_INCREASE_MINT,
            19 => Reason::REASON_REVERT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Reason {
    fn from(s: &'a str) -> Self {
        match s {
            "REASON_UNKNOWN" => Reason::REASON_UNKNOWN,
            "REASON_REWARD_MINE_UNCLE" => Reason::REASON_REWARD_MINE_UNCLE,
            "REASON_REWARD_MINE_BLOCK" => Reason::REASON_REWARD_MINE_BLOCK,
            "REASON_DAO_REFUND_CONTRACT" => Reason::REASON_DAO_REFUND_CONTRACT,
            "REASON_DAO_ADJUST_BALANCE" => Reason::REASON_DAO_ADJUST_BALANCE,
            "REASON_TRANSFER" => Reason::REASON_TRANSFER,
            "REASON_GENESIS_BALANCE" => Reason::REASON_GENESIS_BALANCE,
            "REASON_GAS_BUY" => Reason::REASON_GAS_BUY,
            "REASON_REWARD_TRANSACTION_FEE" => Reason::REASON_REWARD_TRANSACTION_FEE,
            "REASON_REWARD_FEE_RESET" => Reason::REASON_REWARD_FEE_RESET,
            "REASON_GAS_REFUND" => Reason::REASON_GAS_REFUND,
            "REASON_TOUCH_ACCOUNT" => Reason::REASON_TOUCH_ACCOUNT,
            "REASON_SUICIDE_REFUND" => Reason::REASON_SUICIDE_REFUND,
            "REASON_SUICIDE_WITHDRAW" => Reason::REASON_SUICIDE_WITHDRAW,
            "REASON_CALL_BALANCE_OVERRIDE" => Reason::REASON_CALL_BALANCE_OVERRIDE,
            "REASON_BURN" => Reason::REASON_BURN,
            "REASON_WITHDRAWAL" => Reason::REASON_WITHDRAWAL,
            "REASON_REWARD_BLOB_FEE" => Reason::REASON_REWARD_BLOB_FEE,
            "REASON_INCREASE_MINT" => Reason::REASON_INCREASE_MINT,
            "REASON_REVERT" => Reason::REASON_REVERT,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NonceChange<'a> {
    pub address: Cow<'a, [u8]>,
    pub old_value: u64,
    pub new_value: u64,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for NonceChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.old_value = r.read_uint64(bytes)?,
                Ok(24) => msg.new_value = r.read_uint64(bytes)?,
                Ok(32) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NonceChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.old_value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.old_value) as u64) }
        + if self.new_value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.new_value) as u64) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        if self.old_value != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.old_value))?; }
        if self.new_value != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.new_value))?; }
        if self.ordinal != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountCreation<'a> {
    pub account: Cow<'a, [u8]>,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for AccountCreation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AccountCreation<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.account == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.account).len()) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.account != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.account))?; }
        if self.ordinal != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CodeChange<'a> {
    pub address: Cow<'a, [u8]>,
    pub old_hash: Cow<'a, [u8]>,
    pub old_code: Cow<'a, [u8]>,
    pub new_hash: Cow<'a, [u8]>,
    pub new_code: Cow<'a, [u8]>,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for CodeChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.old_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.old_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.new_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.new_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CodeChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.old_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.old_hash).len()) }
        + if self.old_code == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.old_code).len()) }
        + if self.new_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.new_hash).len()) }
        + if self.new_code == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.new_code).len()) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.address != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.address))?; }
        if self.old_hash != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.old_hash))?; }
        if self.old_code != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.old_code))?; }
        if self.new_hash != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.new_hash))?; }
        if self.new_code != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.new_code))?; }
        if self.ordinal != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GasChange {
    pub old_value: u64,
    pub new_value: u64,
    pub reason: sf::ethereum::r#type::v2::mod_GasChange::Reason,
    pub ordinal: u64,
}

impl<'a> MessageRead<'a> for GasChange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.old_value = r.read_uint64(bytes)?,
                Ok(16) => msg.new_value = r.read_uint64(bytes)?,
                Ok(24) => msg.reason = r.read_enum(bytes)?,
                Ok(32) => msg.ordinal = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GasChange {
    fn get_size(&self) -> usize {
        0
        + if self.old_value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.old_value) as u64) }
        + if self.new_value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.new_value) as u64) }
        + if self.reason == sf::ethereum::r#type::v2::mod_GasChange::Reason::REASON_UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.reason) as u64) }
        + if self.ordinal == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ordinal) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.old_value != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.old_value))?; }
        if self.new_value != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.new_value))?; }
        if self.reason != sf::ethereum::r#type::v2::mod_GasChange::Reason::REASON_UNKNOWN { w.write_with_tag(24, |w| w.write_enum(*&self.reason as i32))?; }
        if self.ordinal != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.ordinal))?; }
        Ok(())
    }
}

pub mod mod_GasChange {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reason {
    REASON_UNKNOWN = 0,
    REASON_CALL = 1,
    REASON_CALL_CODE = 2,
    REASON_CALL_DATA_COPY = 3,
    REASON_CODE_COPY = 4,
    REASON_CODE_STORAGE = 5,
    REASON_CONTRACT_CREATION = 6,
    REASON_CONTRACT_CREATION2 = 7,
    REASON_DELEGATE_CALL = 8,
    REASON_EVENT_LOG = 9,
    REASON_EXT_CODE_COPY = 10,
    REASON_FAILED_EXECUTION = 11,
    REASON_INTRINSIC_GAS = 12,
    REASON_PRECOMPILED_CONTRACT = 13,
    REASON_REFUND_AFTER_EXECUTION = 14,
    REASON_RETURN = 15,
    REASON_RETURN_DATA_COPY = 16,
    REASON_REVERT = 17,
    REASON_SELF_DESTRUCT = 18,
    REASON_STATIC_CALL = 19,
    REASON_STATE_COLD_ACCESS = 20,
    REASON_TX_INITIAL_BALANCE = 21,
    REASON_TX_REFUNDS = 22,
    REASON_TX_LEFT_OVER_RETURNED = 23,
    REASON_CALL_INITIAL_BALANCE = 24,
    REASON_CALL_LEFT_OVER_RETURNED = 25,
    REASON_WITNESS_CONTRACT_INIT = 26,
    REASON_WITNESS_CONTRACT_CREATION = 27,
    REASON_WITNESS_CODE_CHUNK = 28,
    REASON_WITNESS_CONTRACT_COLLISION_CHECK = 29,
    REASON_TX_DATA_FLOOR = 30,
}

impl Default for Reason {
    fn default() -> Self {
        Reason::REASON_UNKNOWN
    }
}

impl From<i32> for Reason {
    fn from(i: i32) -> Self {
        match i {
            0 => Reason::REASON_UNKNOWN,
            1 => Reason::REASON_CALL,
            2 => Reason::REASON_CALL_CODE,
            3 => Reason::REASON_CALL_DATA_COPY,
            4 => Reason::REASON_CODE_COPY,
            5 => Reason::REASON_CODE_STORAGE,
            6 => Reason::REASON_CONTRACT_CREATION,
            7 => Reason::REASON_CONTRACT_CREATION2,
            8 => Reason::REASON_DELEGATE_CALL,
            9 => Reason::REASON_EVENT_LOG,
            10 => Reason::REASON_EXT_CODE_COPY,
            11 => Reason::REASON_FAILED_EXECUTION,
            12 => Reason::REASON_INTRINSIC_GAS,
            13 => Reason::REASON_PRECOMPILED_CONTRACT,
            14 => Reason::REASON_REFUND_AFTER_EXECUTION,
            15 => Reason::REASON_RETURN,
            16 => Reason::REASON_RETURN_DATA_COPY,
            17 => Reason::REASON_REVERT,
            18 => Reason::REASON_SELF_DESTRUCT,
            19 => Reason::REASON_STATIC_CALL,
            20 => Reason::REASON_STATE_COLD_ACCESS,
            21 => Reason::REASON_TX_INITIAL_BALANCE,
            22 => Reason::REASON_TX_REFUNDS,
            23 => Reason::REASON_TX_LEFT_OVER_RETURNED,
            24 => Reason::REASON_CALL_INITIAL_BALANCE,
            25 => Reason::REASON_CALL_LEFT_OVER_RETURNED,
            26 => Reason::REASON_WITNESS_CONTRACT_INIT,
            27 => Reason::REASON_WITNESS_CONTRACT_CREATION,
            28 => Reason::REASON_WITNESS_CODE_CHUNK,
            29 => Reason::REASON_WITNESS_CONTRACT_COLLISION_CHECK,
            30 => Reason::REASON_TX_DATA_FLOOR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Reason {
    fn from(s: &'a str) -> Self {
        match s {
            "REASON_UNKNOWN" => Reason::REASON_UNKNOWN,
            "REASON_CALL" => Reason::REASON_CALL,
            "REASON_CALL_CODE" => Reason::REASON_CALL_CODE,
            "REASON_CALL_DATA_COPY" => Reason::REASON_CALL_DATA_COPY,
            "REASON_CODE_COPY" => Reason::REASON_CODE_COPY,
            "REASON_CODE_STORAGE" => Reason::REASON_CODE_STORAGE,
            "REASON_CONTRACT_CREATION" => Reason::REASON_CONTRACT_CREATION,
            "REASON_CONTRACT_CREATION2" => Reason::REASON_CONTRACT_CREATION2,
            "REASON_DELEGATE_CALL" => Reason::REASON_DELEGATE_CALL,
            "REASON_EVENT_LOG" => Reason::REASON_EVENT_LOG,
            "REASON_EXT_CODE_COPY" => Reason::REASON_EXT_CODE_COPY,
            "REASON_FAILED_EXECUTION" => Reason::REASON_FAILED_EXECUTION,
            "REASON_INTRINSIC_GAS" => Reason::REASON_INTRINSIC_GAS,
            "REASON_PRECOMPILED_CONTRACT" => Reason::REASON_PRECOMPILED_CONTRACT,
            "REASON_REFUND_AFTER_EXECUTION" => Reason::REASON_REFUND_AFTER_EXECUTION,
            "REASON_RETURN" => Reason::REASON_RETURN,
            "REASON_RETURN_DATA_COPY" => Reason::REASON_RETURN_DATA_COPY,
            "REASON_REVERT" => Reason::REASON_REVERT,
            "REASON_SELF_DESTRUCT" => Reason::REASON_SELF_DESTRUCT,
            "REASON_STATIC_CALL" => Reason::REASON_STATIC_CALL,
            "REASON_STATE_COLD_ACCESS" => Reason::REASON_STATE_COLD_ACCESS,
            "REASON_TX_INITIAL_BALANCE" => Reason::REASON_TX_INITIAL_BALANCE,
            "REASON_TX_REFUNDS" => Reason::REASON_TX_REFUNDS,
            "REASON_TX_LEFT_OVER_RETURNED" => Reason::REASON_TX_LEFT_OVER_RETURNED,
            "REASON_CALL_INITIAL_BALANCE" => Reason::REASON_CALL_INITIAL_BALANCE,
            "REASON_CALL_LEFT_OVER_RETURNED" => Reason::REASON_CALL_LEFT_OVER_RETURNED,
            "REASON_WITNESS_CONTRACT_INIT" => Reason::REASON_WITNESS_CONTRACT_INIT,
            "REASON_WITNESS_CONTRACT_CREATION" => Reason::REASON_WITNESS_CONTRACT_CREATION,
            "REASON_WITNESS_CODE_CHUNK" => Reason::REASON_WITNESS_CODE_CHUNK,
            "REASON_WITNESS_CONTRACT_COLLISION_CHECK" => Reason::REASON_WITNESS_CONTRACT_COLLISION_CHECK,
            "REASON_TX_DATA_FLOOR" => Reason::REASON_TX_DATA_FLOOR,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeaderOnlyBlock<'a> {
    pub header: Option<sf::ethereum::r#type::v2::BlockHeader<'a>>,
}

impl<'a> MessageRead<'a> for HeaderOnlyBlock<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(42) => msg.header = Some(r.read_message::<sf::ethereum::r#type::v2::BlockHeader>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HeaderOnlyBlock<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockWithRefs<'a> {
    pub id: Cow<'a, str>,
    pub block: Option<sf::ethereum::r#type::v2::Block<'a>>,
    pub transaction_trace_refs: Option<sf::ethereum::r#type::v2::TransactionRefs<'a>>,
    pub irreversible: bool,
}

impl<'a> MessageRead<'a> for BlockWithRefs<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.block = Some(r.read_message::<sf::ethereum::r#type::v2::Block>(bytes)?),
                Ok(26) => msg.transaction_trace_refs = Some(r.read_message::<sf::ethereum::r#type::v2::TransactionRefs>(bytes)?),
                Ok(32) => msg.irreversible = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockWithRefs<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.block.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.transaction_trace_refs.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.irreversible == false { 0 } else { 1 + sizeof_varint(*(&self.irreversible) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.block { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.transaction_trace_refs { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.irreversible != false { w.write_with_tag(32, |w| w.write_bool(*&self.irreversible))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransactionTraceWithBlockRef<'a> {
    pub trace: Option<sf::ethereum::r#type::v2::TransactionTrace<'a>>,
    pub block_ref: Option<sf::ethereum::r#type::v2::BlockRef<'a>>,
}

impl<'a> MessageRead<'a> for TransactionTraceWithBlockRef<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trace = Some(r.read_message::<sf::ethereum::r#type::v2::TransactionTrace>(bytes)?),
                Ok(18) => msg.block_ref = Some(r.read_message::<sf::ethereum::r#type::v2::BlockRef>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransactionTraceWithBlockRef<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trace.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.block_ref.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.trace { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.block_ref { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransactionRefs<'a> {
    pub hashes: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for TransactionRefs<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hashes.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransactionRefs<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hashes.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.hashes { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockRef<'a> {
    pub hash: Cow<'a, [u8]>,
    pub number: u64,
}

impl<'a> MessageRead<'a> for BlockRef<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.number = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockRef<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.number == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.number) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.hash))?; }
        if self.number != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.number))?; }
        Ok(())
    }
}

