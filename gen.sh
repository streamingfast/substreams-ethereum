#!/bin/bash

set -e

input="${BUF_MODULE_REF:-buf.build/streamingfast/firehose-ethereum}"

echo "Generating Ethereum Protobuf using $input"
buf generate "$input" --exclude-path "sf/ethereum/transform,sf/ethereum/trxstream"
