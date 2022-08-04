
## Next

* Changed crate structure to separate `substreams-ethereum-core` from `substreams-ethereum`.
* Added helpers to `pb::Block` and `pb::TransactionTrace` to facilitate iterating over events.
* Removed `Event::must_decode`, use the new `Event::match_and_decode` instead.

## [v0.1.6](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.6)

* Bump versions in release and bump substreams create dependency

## [v0.1.5](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.5)

* Adding helper functions for block handling (transactions, receipts and logs)
* Rework of crate structure
* Fix bug where underlying `ethabi` library was incorrectly decoding an `int24` to an un-overflowing value. This resulted  in always having a positive number. Switching to using `BigInt`, proper overflowing of int24 value makes it that we can have negative values.

## [v0.1.4](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.4)

* Fixed bug when an ABI has multiple times the same Event's name but with a different signature.

## [v0.1.3](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.3)

* Fixed `bytes` usage in Abigen (was generating `address` type).

* Fixed bug on Abigen when event contains unindexed/indexed fields on certain ordering of them.

* Improved generated code for ABI generator for events to `Self` wherever possible.

## [v0.1.2](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.2)

* Bumped to Substreams [0.0.11](https://github.com/streamingfast/substreams/releases/tag/v0.0.11).

* `substreams::init` is now always defined and the actual `getrandom` custom registration is conditional based on the platform you compile to. This will enable non `wasm32-unknown-unknown` target to compile which is required for unit testing

## [v0.1.1](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.1)

* Nothing

## [v0.1.0](https://github.com/streamingfast/substreams-ethereum/releases/tag/0.1.0)

* Added codegen API for ABI bindings that can be used instead of the macro for those who prefer that.

* Added ABI bindings macro `use_contract` (restricted to Events for now).

* Added `NULL_ADDRESS` constant that can be re-used by dependent projects.

* StreamingFast Firehose Block generated Rust code is now included in this library directly.