#!/bin/bash

set -e

input="${BUF_MODULE_REF:-buf.build/streamingfast/firehose-ethereum}"
temp_dir="$(mktemp -d)"
trap "rm -rf $temp_dir" EXIT

echo "Generating Ethereum Protobuf using $input"
buf generate "$input" --exclude-path "sf/ethereum/transform"

echo "Generating Quick Protobuf code from $input"
echo "Temporary directory: $temp_dir"

echo "Exporting proto files..."
buf export "$input" -o "$temp_dir" \
    --exclude-path "sf/ethereum/transform" \
    --exclude-path "sf/ethereum/substreams"

echo "Exporting google protobuf Well-Known Types..."
buf export buf.build/protocolbuffers/wellknowntypes -o "$temp_dir"

echo "Generating and organizing Quick Protobuf code via build.rs..."
GENERATE_QUICK_PB=1 PROTO_DIR="$temp_dir" cargo build --manifest-path core/Cargo.toml

echo "Quick Protobuf generation complete!"
