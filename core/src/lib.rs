pub mod pb;

pub use substreams_ethereum_derive::EthabiContract;

/// This macro can be used to import an Ethereum ABI file in JSON format and generate all the
/// required bindings for ABI decoding/encoding in Rust, targetting `substreams` developer
/// experience.
///
/// ```no_run
/// use_contract!(path = "../abi/erc20.json")
/// ```
///
/// This invocation will generate the following code (signatures only for consiscness):
///
/// ```
/// mod erc20 {
///   mod logs {
///
///   }
///   mod events {
///
///   }
/// }
/// ```
#[macro_export]
macro_rules! use_contract {
    ($module: ident, $path: expr) => {
        #[allow(dead_code)]
        #[allow(missing_docs)]
        #[allow(unused_imports)]
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        pub mod $module {
            #[derive(substreams_ethereum::EthabiContract)]
            #[ethabi_contract_options(path = $path)]
            struct _Dummy;
        }
    };
}

const GETRANDOM_UNVAILABLE_IN_SUBSTREAMS: u32 = Error::CUSTOM_START + 42;

pub fn getrandom_unavailable(_buf: &mut [u8]) -> Result<(), Error> {
    let code = NonZeroU32::new(GETRANDOM_UNVAILABLE_IN_SUBSTREAMS).unwrap();
    Err(Error::from(code))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
