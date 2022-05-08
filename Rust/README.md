# cryptle

## wasmtime

```console
CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime run --tcplisten 127.0.0.1:8443 --env FD_COUNT=1"  cargo +nightly run --target wasm32-wasi
```

Server is running on [`http://127.0.0.1:8443`](http://127.0.0.1:8443).

## enarx

after installing enarx in `$PATH` with `cargo install`

```console
CARGO_TARGET_WASM32_WASI_RUNNER="enarx run --wasmcfgfile ../Enarx.toml"  cargo +nightly run --target wasm32-wasi
```

or simply with the help of `.cargo/config`:

```console
cargo +nightly run
```

Server is running on [`https://127.0.0.1:8443`](https://127.0.0.1:8443).
