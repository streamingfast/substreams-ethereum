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
substreams-ethereum = "0.1.0"
```

## Development

We manually keep in sync the rendered Rust Firehose Block models with the actual Protocol Buffer definitions file found in [sf-ethereum](https://github.com/streamingfast/sf-ethereum/tree/develop/proto) and we commit them to Git.

This means changes to Protobuf files must be manually re-generated and commit, see below for how to do it.

### Regenerate Rust Firehose Block from Protobuf

```
./gen.sh
```

### Release

> *Important* Don't forget to replace `${version}` by your real version like `0.1.3`!

- Ensure build and tests
- Ensure you are in a clean and pushed Git state
- Find & replace all occurrences of Regex `^version = "[^"]+"` in all `Cargo.toml` files to `version = "${version}"`
- Find & replace all occurrences of Regex `^substreams-ethereum(-[^ =]+)\s*=\s*\{\s*version\s*=\s*"[^"]+"` in all `Cargo.toml` files to `substreams-ethereum$1 = { version = "${version}"`
- Update the `CHANGELOG.md` to update the `## Unreleased` header to become `## [v${version}](https://github.com/streamingfast/substreams-ethereum/releases/tag/${version})`
- Ensure that Keybase is running and logged in
- Ensure that `cargo login` has been done in your terminal
- Commit everything with message `Preparing release of ${version}`.
- `./bin/release.sh v${version}` (Add `-f` before `v${version}` to doing the real non-dry mode)
- If everything went well, `git tag ${version}`
- Git push everything (`git push origin develop v${version}`)

You can then update the https://github.com/streamingfast/substreams-template with the latest.

## Community

Need any help? Reach out!

* [StreamingFast Discord](https://discord.gg/jZwqxJAvRs)
* [The Graph Discord](https://discord.gg/vtvv7FP)

## License

[Apache 2.0](LICENSE)
