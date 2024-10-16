# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.10.0)

* Bumped dependencies to `substreams` to 0.6 and `prost` to 0.13 (see [Upgrade notes](https://github.com/streamingfast/substreams-rs/releases/tag/v0.6.0))

## [0.9.13](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.13)

- Fixed AbiGen when in presence of functions that was has leading underscores or casing differences.

  This was generating multiple Rust struct with the same name leading to compilation errors. Now, those cases will be de-duped and you will end up with N Rust struct all suffixed from 1 to N, like `TotalSupply1` and `TotalSupply2`.

- Fixed AbiGen generated `Event#NAME` and `Function#Name` static const in presence of multiple overloads.

  This was previously using the de-duped name but this was wrong as the intention was always to be the ABI's defined named.

## [0.9.12](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.12)

- Re-generated the Rust Ethereum bindings with latest Firehose Ethereum Protobuf definitions.

## [0.9.11](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.11)

- Added conversion from `&pb::BigInt` to `substreams::scalar::BigInt`.

- Added conversion from `&pb::BigInt` to `substreams::scalar::BigDecimal`.

## [0.9.10](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.10)

- Re-generated the Rust Ethereum bindings with latest Firehose Ethereum Protobuf definitions.

## [0.9.9](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.9)

- Fixed generated ABI code and decoding when dealing with indexed dynamic event like `event ContractDeployed(string indexed value)`. We introduced `substreams_ethereum::IndexedDynamicValue<T>` to hold the hash value which is equivalent to topic.

## [0.9.8](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.8)

- Fix bug where Int was not encoded properly in ABI generator

## [0.9.7](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.7)

- Blocks with `DetailLevel` set to `Base` now have transaction receipt information. Transaction successfulness is now supported.

## [0.9.6](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.6)

- Update `block_view` with updated ethereum block at `https://github.com/streamingfast/firehose-ethereum/releases/download/v2.0.0/ethereum-v1.1.0.spkg` with added field `DetailLevel`
  > **_IMPORTANT:_**: Using blocks with `DetailLevel` set to `Extended` (the default level), `block.transactions()` returns only successful transactions.
  > Blocks with `DetailLevel` set to `Base` does not have information about transaction successfulness. Note that a failed transaction will never have logs attached to it.

## [0.9.5](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.5)

- Generate ABI from bytes with method `from_bytes` of `Abigen`
  Example usage:

```rust
Abigen::from_bytes("Contract", content_bytes))?
            .generate()?
            .write_to_file("src/abi/contract.rs")?;
```

## [0.9.4](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.4)

- Fixed ABI code generator generating invalid code `encode` code when a function has a parameter of type `tuple`.

## [0.9.3](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.3)

- Added `timestamp` on `Block` type, returns a reference to the header's timestamp.

- Re-generated Protobuf `Block` type using latest `firehose-ethereum` source, this brings in some missing `BalanceChange#Reason` enum values.

## [0.9.2](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.2)

- Added `parent` on `CallView` to retrieve the parent call of the current `Call` pointed to by `CallView`

- Added `logs_with_calls` on `TransactionTrace` which iterates over the logs of the transaction (excluding those from call that were reverted) and yields each log as a tuple `(&Log, CallView)`.

## [0.9.1](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.9.1)

- Fixed encoding of function when dealing with a `bool` referenced as a sub type (like in a function that accepts `bool[]`).

- Added support for tuple in event in ABI generated code.

- Fixed wrong handling of `FixedArray` when the array element's type is dynamic itself in ABI generated code.

## [0.9.0](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.9.0)

- Re-exporting `block_view` from `substreams-ethereum-core`
- Fixed bug in ABI generation where `uint` was not properly handled

## [0.8.0](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.8.0)

- Bump `substreams` crate
- Changing `into()` call when converting to `BigInt` in abi to call the proper `signed` or `unsigned` encoding
- Adding `into()` method from `BigInt` to use `unsigned_bytes_be`

## [0.7.0](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.7.0)

- Bump `substreams` crate

## [0.6.2](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.6.2)

- Replacing `EthBigInt` and `ethabi::Uint` to only use `substreams-rs` scalar `BigInt` struct.

## [0.6.1](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.6.1)

- Made Windows target(s) able to run tests when depending on `substreams-ethereum` crate.

- Adding `allow` attribute(s) to remove warnings for code generated by ABI generator (should be good now).

## [0.6.0](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.6.0)

- Bump `substreams` to 0.3.0.

- Adding `allow` attribute(s) to remove warnings for code generated by ABI generator.

## [0.5.0](https://github.com/streaminfast/substreams-ethereum/releases/tag/v0.5.0)

- Bump `substreams`.

## [0.4.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.4.0)

- Fixed decoding of event where some indexed fields were `intN` (`int8` to `int256` by increment of 8).

- Fixed decoding of event where some unindexed fields were fixed array with sub element being dynamic types (`bytes[2]`).

- Fixed decoding of event where some unindexed fields were fixed bytes (`bytes8` to `bytes32`).

- Added RPC batching functionality and the RPCDecodable trait to allow decoding of RPC responses.

## [0.3.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.3.0)

- Breaking change
  - Replaced `num_bigint` with `EthBigInt` and `BigInt` from `substreams-rs` crate

## [0.2.2-rc.2](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.2.2-rc.2)

- Fixed issue when type of field was Vec of Uint and the underlying passed in value wasn't cloned. The clone only occurs when it is a Vec<Uint> and not when the type is only a Uint.

## [0.2.2-rc.1](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.2.2-rc.1)

- Fixed issue with dependency with api generation. Semver reference for proper way of importing dependencies.

## [0.2.1-rc](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.2.1-rc)

- Adding ABI function generation first draft

## [0.2.1](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.2.1)

- Adding tests for decoding of different value types

## [0.2.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/v0.2.0)

- **Breaking** Replaced `substreams-ethereum/pb::eth::v1` to `substreams-ethereum/pb::eth::v2` (perform a global replace of any references of `substreams-ethereum/pb::eth::v1` to `substreams-ethereum/pb::eth::v2` and don't forget to re-generate ABI bindings also which depends on `substreams-ethereum/pb::eth::v1`).

- **Breaking** `substreams-ethereum/pb::eth::rpc::RpcCall#method_signature` is now named `data`.

- **Breaking** Bumped `prost` to `^0.11.0` (will requires you to bump `substreams = "~0.0.19"`).

- **Breaking** Removed `must_decode` on ABI generated Event code. Instead, use `if let Some(event) = <EventType>::match_and_decode(&log)`. For example, if you had the following code:

  ```
  if !abi::erc721::events::Transfer::match_log(log) {
    return None;
  }

  let transfer = abi::erc721::events::Transfer::must_decode(log);
  ```

  Replace it with

  ```
  if let Some(transfer) = abi::erc721::events::Transfer::match_and_decode(log) {
    // Do something
  }
  ```

## [0.1.9](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.9)

- Fixed packaging issue.

> Skipped `0.1.8` by mistake

## [0.1.7](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.7)

- Bumped `substreams` version to `0.0.17`

## [0.1.6](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.6)

- Bump versions in release and bump substreams create dependency

## [0.1.5](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.5)

- Adding helper functions for block handling (transactions, receipts and logs)
- Rework of crate structure
- Fix bug where underlying `ethabi` library was incorrectly decoding an `int24` to an un-overflowing value. This resulted in always having a positive number. Switching to using `BigInt`, proper overflowing of int24 value makes it that we can have negative values.
- Changed crate structure to separate `substreams-ethereum-core` from `substreams-ethereum`.
- Added helpers to `pb::Block` and `pb::TransactionTrace` to facilitate iterating over events.
- Removed `Event::must_decode`, use the new `Event::match_and_decode` instead.

## [0.1.4](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.4)

- Fixed bug when an ABI has multiple times the same Event's name but with a different signature.

## [0.1.3](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.3)

- Fixed `bytes` usage in Abigen (was generating `address` type).

- Fixed bug on Abigen when event contains unindexed/indexed fields on certain ordering of them.

- Improved generated code for ABI generator for events to `Self` wherever possible.

## [0.1.2](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.2)

- Bumped to Substreams [0.0.11](https://github.com/streamingfast/substreams/releases/tag/v0.0.11).

- `substreams::init` is now always defined and the actual `getrandom` custom registration is conditional based on the platform you compile to. This will enable non `wasm32-unknown-unknown` target to compile which is required for unit testing

## [0.1.1](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.1)

- Nothing

## [0.1.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.0)

- Added codegen API for ABI bindings that can be used instead of the macro for those who prefer that.

- Added ABI bindings macro `use_contract` (restricted to Events for now).

- Added `NULL_ADDRESS` constant that can be re-used by dependent projects.

- StreamingFast Firehose Block generated Rust code is now included in this library directly.
