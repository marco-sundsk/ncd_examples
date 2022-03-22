#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --target wasm32-unknown-unknown --release
mkdir -p ../../res
cp ../../target/wasm32-unknown-unknown/release/mock_dex.wasm ../../res/