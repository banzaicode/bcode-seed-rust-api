# bcode-seed-rust-api

This is a minimal Actix Web example.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain
- `cargo` (installed with Rust)

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

The server listens on `127.0.0.1:8000`.

## Test

```bash
cargo test
```

## Sample request

After the server is running you can hit the root endpoint:

```bash
curl -i http://127.0.0.1:8000/
```

## Docker (optional)

There is no Dockerfile in this repository, but you can run the app with a Rust
Docker image if you prefer:

```bash
docker run --rm -it -p 8000:8000 -v "$PWD":/usr/src/app -w /usr/src/app rust:latest cargo run
```
