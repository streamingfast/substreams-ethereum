//! The ABI type model the generator reads an ABI file into.

use std::fmt;

/// A parameter's type, as declared in an ABI file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamType {
    Address,
    Bytes,
    Int(usize),
    Uint(usize),
    Bool,
    String,
    Array(Box<ParamType>),
    FixedBytes(usize),
    FixedArray(Box<ParamType>, usize),
    Tuple(Vec<ParamType>),
}

impl ParamType {
    /// Whether the value is written through an offset word rather than in place.
    ///
    /// A fixed array is dynamic when its element is, and a tuple when any field
    /// is, so the answer travels outwards through the wrappers.
    pub fn is_dynamic(&self) -> bool {
        match self {
            ParamType::Bytes | ParamType::String | ParamType::Array(_) => true,
            ParamType::FixedArray(element, _) => element.is_dynamic(),
            ParamType::Tuple(fields) => fields.iter().any(ParamType::is_dynamic),
            _ => false,
        }
    }

    /// The canonical form, as it appears in the signature a selector or topic
    /// hash is taken over.
    ///
    /// A tuple renders as its fields in parentheses rather than the `tuple`
    /// keyword the ABI file spells it with.
    pub fn canonical(&self) -> String {
        match self {
            ParamType::Address => "address".to_owned(),
            ParamType::Bytes => "bytes".to_owned(),
            ParamType::FixedBytes(len) => format!("bytes{}", len),
            ParamType::Int(len) => format!("int{}", len),
            ParamType::Uint(len) => format!("uint{}", len),
            ParamType::Bool => "bool".to_owned(),
            ParamType::String => "string".to_owned(),
            ParamType::Array(element) => format!("{}[]", element.canonical()),
            ParamType::FixedArray(element, len) => format!("{}[{}]", element.canonical(), len),
            ParamType::Tuple(fields) => {
                let fields: Vec<_> = fields.iter().map(ParamType::canonical).collect();
                format!("({})", fields.join(","))
            }
        }
    }

    /// The innermost tuple's field list, reached through any array wrappers.
    ///
    /// An ABI file spells a tuple as the `tuple` keyword and a separate list of
    /// components, so the fields are filled in after the type itself is read,
    /// and `tuple[]` has to be descended into to reach them.
    pub(crate) fn inner_tuple_mut(&mut self) -> Option<&mut Vec<ParamType>> {
        let mut at = self;
        loop {
            match at {
                ParamType::Array(element) => at = element.as_mut(),
                ParamType::FixedArray(element, _) => at = element.as_mut(),
                ParamType::Tuple(fields) => return Some(fields),
                _ => return None,
            }
        }
    }
}

impl fmt::Display for ParamType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.canonical())
    }
}

/// Reads the type string an ABI file declares a parameter with.
///
/// A trailing `[]` or `[N]` wraps whatever precedes it. Anything that is not a
/// known type is a Solidity `enum`, which an ABI file names after the enum
/// rather than its underlying type, and which is encoded as a `uint8`.
pub fn read_type(name: &str) -> ParamType {
    if let Some(open) = name.rfind('[') {
        if name.ends_with(']') {
            let count = &name[open + 1..name.len() - 1];
            let element = read_type(&name[..open]);

            return match count.is_empty() {
                true => ParamType::Array(Box::new(element)),
                false => match count.parse() {
                    Ok(count) => ParamType::FixedArray(Box::new(element), count),
                    // A count that is not a number has no encoding to generate,
                    // and the enum fallback below cannot describe an array.
                    Err(_) => ParamType::Array(Box::new(element)),
                },
            };
        }
    }

    match name {
        "address" => ParamType::Address,
        "bytes" => ParamType::Bytes,
        "bool" => ParamType::Bool,
        "string" => ParamType::String,
        "int" => ParamType::Int(256),
        "uint" => ParamType::Uint(256),
        "tuple" => ParamType::Tuple(vec![]),
        _ => read_sized(name).unwrap_or(ParamType::Uint(8)),
    }
}

/// Reads `intN`, `uintN` and `bytesN`, returning `None` when the width is not a
/// number so the caller can fall back to the `enum` reading.
fn read_sized(name: &str) -> Option<ParamType> {
    if let Some(width) = name.strip_prefix("uint") {
        return width.parse().ok().map(ParamType::Uint);
    }

    if let Some(width) = name.strip_prefix("int") {
        return width.parse().ok().map(ParamType::Int);
    }

    if let Some(width) = name.strip_prefix("bytes") {
        return width.parse().ok().map(ParamType::FixedBytes);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_scalar_types() {
        assert_eq!(read_type("address"), ParamType::Address);
        assert_eq!(read_type("bytes"), ParamType::Bytes);
        assert_eq!(read_type("bool"), ParamType::Bool);
        assert_eq!(read_type("string"), ParamType::String);
        assert_eq!(read_type("int"), ParamType::Int(256));
        assert_eq!(read_type("uint"), ParamType::Uint(256));
        assert_eq!(read_type("int32"), ParamType::Int(32));
        assert_eq!(read_type("uint64"), ParamType::Uint(64));
        assert_eq!(read_type("bytes32"), ParamType::FixedBytes(32));
    }

    #[test]
    fn it_reads_arrays() {
        assert_eq!(
            read_type("address[]"),
            ParamType::Array(Box::new(ParamType::Address))
        );
        assert_eq!(
            read_type("address[2]"),
            ParamType::FixedArray(Box::new(ParamType::Address), 2)
        );
        assert_eq!(
            read_type("uint256[][]"),
            ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Uint(256)))))
        );
        assert_eq!(
            read_type("uint256[2][]"),
            ParamType::Array(Box::new(ParamType::FixedArray(
                Box::new(ParamType::Uint(256)),
                2
            )))
        );
    }

    #[test]
    fn it_reads_a_tuple_as_an_empty_field_list() {
        assert_eq!(read_type("tuple"), ParamType::Tuple(vec![]));
        assert_eq!(
            read_type("tuple[]"),
            ParamType::Array(Box::new(ParamType::Tuple(vec![])))
        );
    }

    #[test]
    fn it_reads_an_unknown_type_as_the_enum_it_is() {
        // Solidity names an `enum` parameter after the enum, and encodes it as
        // a `uint8`.
        assert_eq!(read_type("MyEnum"), ParamType::Uint(8));
        assert_eq!(read_type("contract IERC20"), ParamType::Uint(8));
    }

    #[test]
    fn it_reaches_a_tuple_through_its_array_wrappers() {
        let mut kind = read_type("tuple[]");
        kind.inner_tuple_mut()
            .expect("a tuple is in there")
            .push(ParamType::Address);

        assert_eq!(
            kind,
            ParamType::Array(Box::new(ParamType::Tuple(vec![ParamType::Address])))
        );
    }

    #[test]
    fn it_reports_which_types_are_dynamic() {
        assert!(!read_type("uint256").is_dynamic());
        assert!(!read_type("address[2]").is_dynamic());
        assert!(read_type("bytes").is_dynamic());
        assert!(read_type("string").is_dynamic());
        assert!(read_type("address[]").is_dynamic());
        assert!(read_type("string[2]").is_dynamic());

        assert!(!ParamType::Tuple(vec![ParamType::Address]).is_dynamic());
        assert!(ParamType::Tuple(vec![ParamType::String]).is_dynamic());
    }

    #[test]
    fn it_renders_the_canonical_form() {
        assert_eq!(read_type("uint256").canonical(), "uint256");
        assert_eq!(read_type("uint").canonical(), "uint256");
        assert_eq!(read_type("address[2]").canonical(), "address[2]");
        assert_eq!(read_type("uint256[][]").canonical(), "uint256[][]");

        let tuple = ParamType::Tuple(vec![ParamType::Address, ParamType::Uint(256)]);
        assert_eq!(tuple.canonical(), "(address,uint256)");
        assert_eq!(
            ParamType::Array(Box::new(tuple)).canonical(),
            "(address,uint256)[]"
        );
    }
}
