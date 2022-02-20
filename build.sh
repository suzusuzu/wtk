#!/bin/bash
cd sample-wasm
cargo build --target=wasm32-unknown-unknown
cd ..
cargo run --example sample