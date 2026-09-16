# Substreams Ethereum

Substreams development kit for Ethereum chains, contains Rust Firehose Block model and helpers as well as utilities for Ethereum ABI encoding/decoding.

## Usage

```toml
[package]
name = "substreams-acme"
version = 0.1.2

[lib]
crate-type = ["cdylib"]

[dependencies]
substreams-ethereum = "0.10"
```

## Development

We manually keep in sync the rendered Rust Firehose Block models with the actual Protocol Buffer definitions file found in [firehose-ethereum](https://github.com/streamingfast/firehose-ethereum/blob/develop/proto/sf/ethereum/type/v2/type.proto#L51o) and we commit them to Git.

This means changes to Protobuf files must be manually re-generated and commit, see below for how to do it.

### Regenerate Rust Firehose Block from Protobuf

```
buf generate
```

The schema module and the paths to exclude are declared in `buf.gen.yaml`.

## Caveats

### ABI with Tuple

Tuples are supported by the generated code, as Rust unnamed tuples for the event
or function that declares them.

## Release

```bash
sfreleaser release
```

Follow instructions the CLI is asking, the process is now automatic and version bump and Substreams package building is now all done automatically.
