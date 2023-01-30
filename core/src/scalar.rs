use std::ops::Mul;
use crate::{pb::eth::v2 as pb};
use substreams::scalar::{BigInt, BigDecimal};

impl Into<BigInt> for pb::BigInt {
    fn into(self) -> BigInt {
        BigInt::from_unsigned_bytes_be(self.bytes.as_ref())
    }
}

impl Into<BigDecimal> for pb::BigInt {
    fn into(self) -> BigDecimal {
        let v = BigInt::from_unsigned_bytes_be(self.bytes.as_ref());
        BigDecimal::new(v, 0)
    }
}

pub fn to_option_decimal(v: Option<pb::BigInt>) ->  Option<BigDecimal> {
    match  v {
        Some(v) => {
            let out : BigDecimal = v.into();
            Some(out)
        },
        None => None
    }
}

pub fn to_option_bigint(v: Option<pb::BigInt>) ->  Option<BigInt> {
    match  v {
        Some(v) => Some(v.into()),
        None => None
    }
}

pub fn to_option_decimal_with_decimal(v: Option<pb::BigInt>, decimal: u32) ->  Option<BigDecimal> {
    match  v {
        Some(v) => Some(v.with_decimal(decimal)),
        None => None
    }
}


impl pb::BigInt {
    pub fn with_decimal(self, decimal: u32) -> BigDecimal {
        let num : BigDecimal = self.into();
        let dem = BigInt::from(10 as u32).pow(decimal);
        return  num / BigDecimal::from(dem)
    }
}


#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use crate::{pb::eth::v2 as pb};
    use crate::scalar::{to_option_bigint, to_option_decimal, to_option_decimal_with_decimal};

    #[test]
    fn zero_into_bigint() {
        let v : substreams::scalar::BigInt = new_pb_bigint(0).into();
        assert_eq!(v.to_u64(), 0);
    }

    #[test]
    fn number_into_bigint() {
        let v : substreams::scalar::BigInt = new_pb_bigint(253).into();
        assert_eq!(v.to_u64(), 253);
    }

    #[test]
    fn zero_into_bigdecmal() {
        let v : substreams::scalar::BigDecimal = new_pb_bigint(0).into();
        assert_eq!(v.to_string(),"0");
    }

    #[test]
    fn number_into_bigdecmal() {
        let v : substreams::scalar::BigDecimal = new_pb_bigint(253).into();
        assert_eq!(v.to_string(),"253");
    }

    #[test]
    fn some_option_pb_to_decimal() {
        let v =  Some(new_pb_bigint(253));
        assert_eq!(to_option_decimal(v),Some(substreams::scalar::BigDecimal::from(253 as u32)));
    }

    #[test]
    fn none_option_pb_to_decimal() {
        let v : Option<pb::BigInt> =  None;
        assert_eq!(to_option_decimal(v), None);
    }

    #[test]
    fn some_option_pb_to_bigint() {
        let v =  Some(new_pb_bigint(253));
        assert_eq!(to_option_bigint(v), Some(substreams::scalar::BigInt::from(253 as u32)));
    }

    #[test]
    fn none_option_pb_to_bigint() {
        let v : Option<pb::BigInt> =  None;
        assert_eq!(to_option_bigint(v), None);
    }

    #[test]
    fn with_decimal() {
        let v = new_pb_bigint_bytes(vec![114,10,199,169,74,64,0].as_ref());
        assert_eq!(v.with_decimal(18).to_string(), "0.0321");
    }

    #[test]
    fn some_option_pb_to_bigdecimal_with_decimal() {
        let v =  Some(new_pb_bigint_bytes(vec![114,10,199,169,74,64,0].as_ref()));
        let out = substreams::scalar::BigDecimal::try_from("0.0321".to_owned()).unwrap();
        assert_eq!(to_option_decimal_with_decimal(v, 18),Some(out));
    }

    #[test]
    fn none_option_pb_to_bigdecimal_with_decimal() {
        let v : Option<pb::BigInt> =  None;
        assert_eq!(to_option_decimal_with_decimal(v, 18), None);
    }


    #[test]
    fn some_option_pb_to_bigdecimal() {
        let v =  Some(new_pb_bigint(253));
        assert_eq!(to_option_decimal(v),Some(substreams::scalar::BigDecimal::from(253 as u32)));
    }

    #[test]
    fn none_option_pb_to_bigdecimal() {
        let v : Option<pb::BigInt> =  None;
        assert_eq!(to_option_decimal(v), None);
    }

    pub fn new_pb_bigint(value: u32) -> pb::BigInt {
        let v = num_bigint::BigInt::new(num_bigint::Sign::Plus, vec![value]);
        let (_, bytes) = v.to_bytes_be();
        pb::BigInt{ bytes }
    }

    pub fn new_pb_bigint_bytes(bytes: &[u8]) -> pb::BigInt {
        let v = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes);
        let (_, bytes) = v.to_bytes_be();
        pb::BigInt{ bytes }
    }

}


// let old_value = match balance_change.old_value.as_ref() {
// Some(value) => {
// BigDecimal::from(BigInt::from_unsigned_bytes_be(&value.bytes))
// / BigDecimal::from(1e18 as i64)
// }
// None => BigDecimal::zero(),
// };