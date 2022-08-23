#!/bin/bash

ETH_SPKG="${ETH_SPKG:-https://github.com/streamingfast/sf-ethereum/releases/download/v0.10.2/ethereum-v0.10.3.spkg}"

echo "Generating Ethereum Protobuf using $ETH_SPKG"
substreams protogen "$ETH_SPKG" --exclude-paths="sf/substreams/v1,google/" --output-path="./core/src/pb"
