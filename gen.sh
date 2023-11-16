#!/bin/bash

set -e

ETH_SPKG="${ETH_SPKG:-https://github.com/streamingfast/firehose-ethereum/releases/download/v2.0.0/ethereum-v1.1.0.spkg}"

input="$ETH_SPKG"#format=bin

echo "Generating Ethereum Protobuf using $input"
buf generate "$input" --exclude-path sf/substreams/v1,sf/substreams/rpc,google/,sf/substreams/sink,sf/substreams
