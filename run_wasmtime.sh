#!/bin/sh
export CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime run --tcplisten 127.0.0.1:8443 --env FD_COUNT=1"
exec cargo +nightly run --release --target wasm32-wasi

