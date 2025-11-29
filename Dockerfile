FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release -p dtt-api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dtt-api /usr/local/bin/dtt-api

EXPOSE 3000

CMD ["dtt-api"]