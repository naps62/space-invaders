#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
  --out-dir ./out \
  --out-name "bevy-space-invaders" \
  ./target/wasm32-unknown-unknown/release/bevy-space-invaders.wasm
