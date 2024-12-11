FROM rust:1.83.0-bookworm AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bookworm

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rust-api /app/rust-api

CMD ["rust-api"]
