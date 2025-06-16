# Project Overview

This repository contains a minimal REST API written in [Actix Web](https://actix.rs/). The goal is to provide a small starting point for building services in Rust.

## Directory Structure

```
/ (project root)
├── Cargo.toml          # Rust package configuration
├── Dockerfile          # Container build instructions
├── README.md           # Basic usage instructions
├── src/                # Application source code
│   ├── main.rs         # Application entry point
│   ├── handlers.rs     # HTTP handler implementations
│   └── test.rs         # Integration tests
└── doc/
    └── README-project.md  # This document
```

## Source Code Overview

- **main.rs** sets up the Actix Web server. It defines a simple `/` endpoint (`index`) and a `/health` route pointing to `handlers::health_check`. The server binds to the `HOST` and `PORT` environment variables or defaults to `127.0.0.1:8000`.
- **handlers.rs** contains the `health_check` function returning a JSON payload indicating the service status.
- **test.rs** uses `actix_web::test` utilities to verify the index and health endpoints.

## Building and Running

Standard `cargo build`, `cargo run`, and `cargo test` commands are used. A `Dockerfile` is included to build a containerized release version.

