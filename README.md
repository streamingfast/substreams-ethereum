# Substreams Ethereum

Substreams development kit for Ethereum chains, contains Rust Firehose Block model and helpers as well as utilities for Ethereum ABI encoding/decoding.

## Usage

```toml
[package]
name = "substreams-acme"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
substreams-ethereum = "1.0.0"
```



## Development

We manually keep in sync the rendered Rust Firehose Block models with the actual Protocol Buffer definitions file found in [sf-ethereum](https://github.com/streamingfast/sf-ethereum/tree/develop/proto) and we commit them to Git.

This means changes to Protobuf files must be manually re-generated and commit, see below for how to do it.

### Regenerate Rust Firehose Block from Protobuf

Clone repository https://github.com/streamingfast/sf-ethereum somewhere and checkout the correct
reference you want to re-generate from.

Then export environment variable `SUBTREAMS_ETHEREUM_PROTO_PATH` and make it point where you cloned
the [sf-ethereum](https://github.com/streamingfast/sf-ethereum) repository.

Then simply do:

```
SUBTREAMS_ETHEREUM_REGENERATE_PROTO=true SUBTREAMS_ETHEREUM_PROTO_PATH=<path> cargo build --target wasm32-unknown-unknown --release
```

## Community

Need any help? Reach out!

* [StreamingFast Discord](https://discord.gg/jZwqxJAvRs)
* [The Graph Discord](https://discord.gg/vtvv7FP)

## License

[Apache 2.0](LICENSE)
