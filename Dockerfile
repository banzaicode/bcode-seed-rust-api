FROM rust:1 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/bcode-seed-rust-api ./
ENV HOST=0.0.0.0
EXPOSE 8000
CMD ["./bcode-seed-rust-api"]
