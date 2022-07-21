# Cryptle: a secure multi-party Wordle clone with Enarx

Wordle is a popular web-based game, where a single player has to guess a five-letter word in six attempts, with yellow/green colored titles shown as hints in each round, indicating letters that match with the secret word.

We’ve created an open source clone of Wordle called Cryptle, with the goal of demonstrating data encryption in use, where the processing of the data is done in a Trusted Execution Environment (TEE), and only accessible to the Cryptle application.

Cryptle is similar to Wordle but one important difference is that it is multi-party and the secret words are suggested by the players themselves. Each player proposes words that are most likely to match those sent by others. The words are sent to the Cryptle application deployed and running in an Enarx Keep (a specific TEE instance) and are only revealed to the players when there’s a match between the secret words.

The standard way to engage with the game is for players to guess the secret words by playing Cryptle from the client side. However, we will also be allowing an alternative: players may write an application which runs on the host side and attempts to derive or otherwise guess the secret words. The mechanism chosen is up to the implementer, but the application must be available as open source (under an OSI license) in a public repository in GitHub or GitLab. It may be written in any programming language, but should not be intentionally obfuscated. The application may run with root privileges. Documentation should be provided to show how the application is able to “guess” or derive the secret words. The host will be running a modern Linux kernel and applications will be run as ELF binaries. No physical access to the host will be allowed or provided.

# Trying it out

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
