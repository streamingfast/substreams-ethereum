#!/bin/bash

ETH_SPKG="https://github.com/streamingfast/sf-ethereum/releases/download/v0.10.2/ethereum-v0.10.2.spkg"
buf generate "$ETH_SPKG#format=bin" --exclude-path="sf/substreams/v1" --exclude-path="google/"