use ethabi::ethereum_types::{U128, U256, U64};
use num_bigint::ParseBigIntError;
pub use num_bigint::Sign as BigIntSign;
use std::str::FromStr;
use substreams::scalar::{BigDecimal, BigInt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EthBigInt {
    i: BigInt,
}

impl EthBigInt {
    pub fn new(i: BigInt) -> EthBigInt {
        EthBigInt { i }
    }

    pub fn get_big_int(&self) -> BigInt {
        return self.i.clone();
    }

    pub fn to_decimal(&self, decimals: u64) -> BigDecimal {
        self.get_big_int().to_decimal(decimals)
    }
}

impl ToString for EthBigInt {
    fn to_string(&self) -> String {
        self.get_big_int().to_string()
    }
}

impl AsRef<BigInt> for EthBigInt {
    fn as_ref(&self) -> &BigInt {
        &self.i
    }
}

impl TryFrom<U256> for EthBigInt {
    type Error = ParseBigIntError;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        let big_int: Result<BigInt, <num_bigint::BigInt as FromStr>::Err> =
            BigInt::from_str(value.to_string().as_str());
        match big_int {
            Ok(i) => Ok(EthBigInt { i }),
            Err(err) => Err(err),
        }
    }
}

impl From<U64> for EthBigInt {
    /// This implementation assumes that U64 represents an unsigned U64,
    /// and not a signed U64 (aka int64 in Solidity). Right now, this is
    /// all we need (for block numbers). If it ever becomes necessary to
    /// handle signed U64s, we should add the same
    /// `{to,from}_{signed,unsigned}_u64` methods that we have for U64.
    fn from(value: U64) -> EthBigInt {
        EthBigInt {
            i: BigInt::from(value.as_u64()),
        }
    }
}

impl From<U128> for EthBigInt {
    /// This implementation assumes that U128 represents an unsigned U128,
    /// and not a signed U128 (aka int128 in Solidity). Right now, this is
    /// all we need (for block numbers). If it ever becomes necessary to
    /// handle signed U128s, we should add the same
    /// `{to,from}_{signed,unsigned}_u128` methods that we have for U256.
    fn from(value: U128) -> EthBigInt {
        let mut bytes: [u8; 16] = [0; 16];
        value.to_little_endian(&mut bytes);
        EthBigInt {
            i: BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes),
        }
    }
}

impl Into<BigInt> for EthBigInt {
    fn into(self) -> BigInt {
        self.get_big_int()
    }
}
