#!/bin/bash

set -e

ETH_SPKG="${ETH_SPKG:-https://github.com/streamingfast/sf-ethereum/releases/download/v0.10.2/ethereum-v0.10.3.spkg}"

input="$ETH_SPKG"#format=bin

echo "Generating Ethereum Protobuf using $input"
buf generate "$input" --exclude-path sf/substreams/v1,sf/substreams/rpc,google/
