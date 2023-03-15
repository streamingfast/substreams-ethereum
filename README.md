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
substreams-ethereum = "0.6.0"
```

## Development

We manually keep in sync the rendered Rust Firehose Block models with the actual Protocol Buffer definitions file found in [sf-ethereum](https://github.com/streamingfast/sf-ethereum/tree/develop/proto) and we commit them to Git.

This means changes to Protobuf files must be manually re-generated and commit, see below for how to do it.

### Regenerate Rust Firehose Block from Protobuf

```
./gen.sh
```

## Caveats

### ABI with Tuple

Internally, we are using https://github.com/rust-ethereum/ethabi library for the ABI generator. Sadly, tuple are not supported in the library today that means that ABI containing `tuple` fails compiling with the following message:

```
Caused by:
  process didn't exit successfully: `/home/acme/target/debug/build/substreams-template-109c396339f1e9a0/build-script-build` (exit status: 101)
  --- stderr
  thread 'main' panicked at 'not implemented: Tuples are not supported. https://github.com/openethereum/ethabi/issues/175', /home/acme/.cargo/registry/src/github.com-1ecc6299db9ec823/substreams-ethereum-abigen-0.9.0/src/lib.rs:122:13
```

You can expand the collapsed [Instructions] section below to get detailed instructions how you can "manually" resolve that.

<details>
<summary>Instructions</summary>
Right now, the workaround for that is to create manually the decoding code that is required to decode the ABI. The rest of this section will give the instructions needed to do so.

First step will be to modify a bit the ABI so decoding code is generated correctly. This initial generated code will then be copied over and modified to decode the tuple correctly.

The idea will be to "explode" the inner `tuple` into it's own "event" in the ABI, this will generate some code for the `struct` representing the tuple as well as the decoding code for the `struct` itself. We will then tweak this generated to wire everything together.

From:

```json
[
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      },
      {
        "components": [
          {
            "internalType": "enum ItemType",
            "name": "itemType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "amount",
            "type": "uint256"
          }
        ],
        "indexed": false,
        "internalType": "struct SpentItem[]",
        "name": "offer",
        "type": "tuple[]"
      }
    ],
    "name": "OrderFulfilled",
    "type": "event"
  }
]
```

We explode `SpentItem` to it's own type and replace the `offer` type `tuple[]` to `address[]` to make it compile correctly:

```json
[
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      },
      {
        "components": [
          {
            "internalType": "enum ItemType",
            "name": "itemType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "amount",
            "type": "uint256"
          }
        ],
        "indexed": false,
        "internalType": "struct SpentItem[]",
        "name": "offer",
        "type": "address[]"
      }
    ],
    "name": "OrderFulfilled",
    "type": "event"
  },
  {
    "anonymous": false,
    "name": "SpentItem",
    "type": "event",
    "inputs": [
      {
        "internalType": "enum ItemType",
        "name": "itemType",
        "type": "uint8"
      },
      {
        "internalType": "uint256",
        "name": "amount",
        "type": "uint256"
      }
    ]
  }
]
```

> **Note** No need to remove the `components` or change the `internalType` value, they are ignored.

Perform a `cargo build` using this modified ABI so that code is generated in `src/abi/<file>.rs`, it's wrong right now but we are going to copy it somewhere else and make it work.

Find the generated code for the `OrderFulfilled` event within the `src/abi/<file>.rs` and copy over to a new file `src/events.rs`. You should copy over the `pub struct OrderFulfilled` block, the `impl OrderFulfilled` block and `impl substreams_ethereum::Event for OrderFulfilled` block.

Copy also the `SpentItem` structure and the `impl SpentItem` block:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct OrderFulfilled {
    pub order_hash: [u8; 32usize],
    pub offer: Vec<Vec<u8>>,
}
impl OrderFulfilled {
    const TOPIC_ID: [u8; 32] = [
        227u8,
        56u8,
        222u8,
        32u8,
        39u8,
        120u8,
        0u8,
        226u8,
        120u8,
        84u8,
        168u8,
        160u8,
        171u8,
        38u8,
        80u8,
        66u8,
        198u8,
        237u8,
        193u8,
        186u8,
        154u8,
        14u8,
        209u8,
        73u8,
        102u8,
        185u8,
        47u8,
        163u8,
        179u8,
        98u8,
        194u8,
        244u8,
    ];
    pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        if log.topics.len() != 1usize {
            return false;
        }
        if log.data.len() < 96usize {
            return false;
        }
        return log.topics.get(0).expect("bounds already checked").as_ref()
            == Self::TOPIC_ID;
    }
    pub fn decode(
        log: &substreams_ethereum::pb::eth::v2::Log,
    ) -> Result<Self, String> {
        let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Array(
                        Box::new(ethabi::ParamType::Address),
                    ),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
        values.reverse();
        Ok(Self {
            order_hash: {
                let mut result = [0u8; 32];
                let v = values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                result.copy_from_slice(&v);
                result
            },
            offer: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_array()
                .expect(INTERNAL_ERR)
                .into_iter()
                .map(|inner| {
                    inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                })
                .collect(),
        })
    }
}
impl substreams_ethereum::Event for OrderFulfilled {
    const NAME: &'static str = "OrderFulfilled";
    fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        Self::match_log(log)
    }
    fn decode(
        log: &substreams_ethereum::pb::eth::v2::Log,
    ) -> Result<Self, String> {
        Self::decode(log)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SpentItem {
    pub item_type: substreams::scalar::BigInt,
    pub amount: substreams::scalar::BigInt,
}
impl SpentItem {
    const TOPIC_ID: [u8; 32] = [
        18u8,
        7u8,
        103u8,
        62u8,
        30u8,
        101u8,
        94u8,
        85u8,
        209u8,
        209u8,
        166u8,
        82u8,
        139u8,
        137u8,
        197u8,
        45u8,
        11u8,
        224u8,
        230u8,
        74u8,
        27u8,
        234u8,
        238u8,
        52u8,
        150u8,
        245u8,
        214u8,
        202u8,
        230u8,
        104u8,
        138u8,
        22u8,
    ];
    pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        if log.topics.len() != 1usize {
            return false;
        }
        if log.data.len() != 64usize {
            return false;
        }
        return log.topics.get(0).expect("bounds already checked").as_ref()
            == Self::TOPIC_ID;
    }
    pub fn decode(
        log: &substreams_ethereum::pb::eth::v2::Log,
    ) -> Result<Self, String> {
        let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Uint(8usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
        values.reverse();
        Ok(Self {
            item_type: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            },
            amount: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            },
        })
    }
}
impl substreams_ethereum::Event for SpentItem {
    const NAME: &'static str = "SpentItem";
    fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        Self::match_log(log)
    }
    fn decode(
        log: &substreams_ethereum::pb::eth::v2::Log,
    ) -> Result<Self, String> {
        Self::decode(log)
    }
}
```

In Rust, this will ignore until it's "imported" somewhere, so let's define the module for it. In `src/lib.rs`, at the top of the file, add:

```rust
mod events
```

Let's start to modify our incorrect generated code to make it correct. First define `INTERNAL_ERR` constant because it's used normally, put it at top of the `src/events.rs` file:

```
const INTERNAL_ERR: &str = "decode event internal error";
```

Now a tricky part, we need to update the `TOPIC_ID` constant because it's wrong right now. If you already know the event ID (which is the `topics #0`), perfect. If you don't, you can compute it by taking the `keccak256` hash of the event definition. You need the event name and its types to have it, in our case it's `OrderFulfilled(bytes32,(uint8,uint256)[])`

You can use `jq -r '.[] | select(.name == "OrderFulfilled") | .inputs[].type' | tr "\n" ","` and `jq -r '.[] | select(.name == "SpentItem") | .inputs[].type' | tr "\n" ","` on the "modified" ABI file we just did to get the correct ordered types. Then assemble it correctly because `OrderFulfilled` second field is wrong.

> **Warning** It must be **without** space nor extra punctuation, a single wrong character will make the whole event ID wrong.

Now that we have our event ID definition, we can compute the keccak256 hash. We use a CLI tool to do it ([see](https://gist.github.com/miguelmota/60259aed8ce95477131c0a1f4f31e0da)):

```bash
$ printf 'OrderFulfilled(bytes32,(uint8,uint256)[])' | keccak-256sum
e86f4727db138d4b9cb776888b1d2239562eafaa38dd110b7d5def7698ccfd41  -
```

So our event topic 0 is `e86f4727db138d4b9cb776888b1d2239562eafaa38dd110b7d5def7698ccfd41`. Now we just need to change the `OrderFulfilled` definition to define be correct:

```rust
const TOPIC_ID: [u8; 32] = hex_literal::hex!("e86f4727db138d4b9cb776888b1d2239562eafaa38dd110b7d5def7698ccfd41");
```

Now, within `pub struct OrderFulfilled`, change `offer` field (defined right now as `offer: Vec<Vec<u8>>`) which normally holds an array of tuple to it's correct value `offer: Vec<Vec<SpentItem>>`.

Find the event "token" definition within the `pub fn decode(log: <type>)` which is:

```rust
let mut values = ethabi::decode(
    &[
        ethabi::ParamType::FixedBytes(32usize),
        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
    ],
    log.data.as_ref(),
)
```

Determine the field that is a tuple, in our case the second field and change it to be define as a `ethabi::ParamType::Tuple` type.

```rust
let mut values = ethabi::decode(
    &[
        ethabi::ParamType::FixedBytes(32usize),
        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
            ethabi::ParamType::Uint(8usize),
            ethabi::ParamType::Uint(256usize),
        ]))),
    ],
    log.data.as_ref(),
```

> **Info** The `SpentItem::decode` function has a `let mut values` definition variable right at the beginning of the function that list the correct element to put for the tuple.

Now still within `OrderFulfilled` function `pub fn decode(log: <type>)`, find the `offer:` piece of code that does the actual decoding work, it looks right now like:

```rust
 ...
 },
 offer: values
            .pop()
            .expect(INTERNAL_ERR)
            .into_array()
            .expect(INTERNAL_ERR)
            .into_iter()
            .map(|inner| {
                inner
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec()
            })
            .collect(),
```

Change it so it forwards its decoding to `SpentItem` structure:

```rust
 ...
 },
 offer: values
            .pop()
            .expect(INTERNAL_ERR)
            .into_array()
            .expect(INTERNAL_ERR)
            .into_iter()
            .map(|inner| {
                let fields = inner.into_tuple().expect(INTERNAL_ERR);
                SpentItem::decode(fields).expect(INTERNAL_ERR);
            })
            .collect(),
```

Now we have to modify a bit the `SpentItem` implementation. Within the `impl SpentItem` block, delete:

- `TOPIC_ID` constant
- `match_log` function

And remove fully the `impl substreams_ethereum::Event for SpentItem` block. Last thing to do is to tweak the `SpentItem::decode` function.

Change the signature:

```rust
pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String>
```

So that it accepts a `Vec<ethabi::Token>`, rename the variable to `values` and make it mutable:

```rust
pub fn decode(mut values: Vec<ethabi::Token>) -> Result<Self, String>
```

And finally, remove the previous `let mut values` definition that we have previously which looks like:

```rust
let mut values = ethabi::decode(
    &[
        ethabi::ParamType::Uint(8usize),
        ethabi::ParamType::Uint(256usize),
    ],
    log.data.as_ref(),
)
.map_err(|e| format!("unable to decode log.data: {:?}",
```

Now everything is done and you can use `OrderFulfilled` to decode your event that contains a `tuple`. The final code looks like that:

```rust
const INTERNAL_ERR: &str = "decode event internal error";
#[derive(Debug, Clone, PartialEq)]
pub struct OrderFulfilled {
    pub order_hash: [u8; 32usize],
    pub offer: Vec<SpentItem>,
}
impl OrderFulfilled {
    const TOPIC_ID: [u8; 32] =
        hex_literal::hex!("e86f4727db138d4b9cb776888b1d2239562eafaa38dd110b7d5def7698ccfd41");
    pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        if log.topics.len() != 1usize {
            return false;
        }
        if log.data.len() < 96usize {
            return false;
        }
        return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
    }
    pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        let mut values = ethabi::decode(
            &[
                ethabi::ParamType::FixedBytes(32usize),
                ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
            ],
            log.data.as_ref(),
        )
        .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
        values.reverse();
        Ok(Self {
            order_hash: {
                let mut result = [0u8; 32];
                let v = values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                result.copy_from_slice(&v);
                result
            },
            offer: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_array()
                .expect(INTERNAL_ERR)
                .into_iter()
                .map(|inner| {
                    let fields = inner.into_tuple().expect(INTERNAL_ERR);
                    SpentItem::decode(fields).unwrap()
                })
                .collect(),
        })
    }
}
impl substreams_ethereum::Event for OrderFulfilled {
    const NAME: &'static str = "OrderFulfilled";
    fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        Self::match_log(log)
    }
    fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        Self::decode(log)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SpentItem {
    pub item_type: substreams::scalar::BigInt,
    pub amount: substreams::scalar::BigInt,
}
impl SpentItem {
    pub fn decode(mut values: Vec<ethabi::Token>) -> Result<Self, String> {
        values.reverse();
        Ok(Self {
            item_type: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            },
            amount: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            },
        })
    }
}
```

If you struggle with something, reach out to us on Discord and we are going to help you out.
