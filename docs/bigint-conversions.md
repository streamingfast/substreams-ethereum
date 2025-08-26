# BigInt Conversions

This document explains how to convert between `scalar::BigInt` and `pb::eth::v2::BigInt` types in the Substreams Ethereum package.

## Overview

The Substreams Ethereum package provides bidirectional conversion between:
- `substreams::scalar::BigInt` (from the Substreams package)
- `substreams_ethereum::pb::eth::v2::BigInt` (from the Ethereum protobuf definitions)

These conversions are implemented using the `From` and `Into` traits, which allows for seamless conversion between the different types.

## Usage

The easiest way to use these conversions is through the prelude module:

```rust
use substreams_ethereum::prelude::*;
```

### Converting from scalar::BigInt to pb::eth::v2::BigInt

```rust
// Create a scalar::BigInt
let scalar_bigint = BigInt::from(42);

// Convert to pb::eth::v2::BigInt
let pb_bigint: pb::BigInt = scalar_bigint.into();
```

### Converting from pb::eth::v2::BigInt to scalar::BigInt

```rust
// Create a pb::eth::v2::BigInt
let pb_bigint = pb::BigInt { bytes: vec![42] };

// Convert to scalar::BigInt
let scalar_bigint: BigInt = pb_bigint.into();
```

### Roundtrip Conversion

```rust
// Start with a scalar::BigInt
let original = BigInt::from(12345);

// Convert to pb::eth::v2::BigInt
let pb_bigint: pb::BigInt = original.clone().into();

// Convert back to scalar::BigInt
let roundtrip: BigInt = pb_bigint.into();

// Verify the roundtrip conversion
assert_eq!(original, roundtrip);
```

## Implementation Details

The conversions are implemented as follows:

1. From `scalar::BigInt` to `pb::eth::v2::BigInt`:
   ```rust
   impl From<BigInt> for pb::BigInt {
       fn from(value: BigInt) -> Self {
           pb::BigInt {
               bytes: value.to_unsigned_bytes_be(),
           }
       }
   }
   ```

2. From `pb::eth::v2::BigInt` to `scalar::BigInt`:
   ```rust
   impl Into<BigInt> for pb::BigInt {
       fn into(self) -> BigInt {
           BigInt::from_unsigned_bytes_be(self.bytes.as_ref())
       }
   }
   ```

## Example

See the full example in [examples/bigint_conversion.rs](../examples/bigint_conversion.rs).

