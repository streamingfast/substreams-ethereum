// Example demonstrating the bidirectional conversion between scalar::BigInt and pb::eth::v2::BigInt

use substreams_ethereum::prelude::*;

fn main() {
    // Create a scalar::BigInt
    let scalar_bigint = BigInt::from(42);
    println!("Original scalar::BigInt: {}", scalar_bigint);

    // Convert scalar::BigInt to pb::eth::v2::BigInt
    let pb_bigint: pb::BigInt = scalar_bigint.clone().into();
    println!("Converted to pb::BigInt: {:?}", pb_bigint.bytes);

    // Convert pb::eth::v2::BigInt back to scalar::BigInt
    let roundtrip_bigint: BigInt = pb_bigint.into();
    println!("Converted back to scalar::BigInt: {}", roundtrip_bigint);

    // Verify the roundtrip conversion
    assert_eq!(scalar_bigint, roundtrip_bigint);
    println!("Roundtrip conversion successful!");

    // Example with a larger number
    let large_scalar_bigint = BigInt::from(1_000_000);
    println!("\nLarge scalar::BigInt: {}", large_scalar_bigint);

    // Convert large scalar::BigInt to pb::eth::v2::BigInt
    let large_pb_bigint: pb::BigInt = large_scalar_bigint.clone().into();
    println!("Converted to pb::BigInt: {:?}", large_pb_bigint.bytes);

    // Convert large pb::eth::v2::BigInt back to scalar::BigInt
    let large_roundtrip_bigint: BigInt = large_pb_bigint.into();
    println!("Converted back to scalar::BigInt: {}", large_roundtrip_bigint);

    // Verify the roundtrip conversion for the large number
    assert_eq!(large_scalar_bigint, large_roundtrip_bigint);
    println!("Large number roundtrip conversion successful!");
}

