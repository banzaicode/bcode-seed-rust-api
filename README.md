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

## Docker

Build the image:

```bash
docker build -t bcode-seed-rust-api .
```

Run the container:

```bash
docker run --rm -p 8000:8000 bcode-seed-rust-api
```

