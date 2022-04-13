FROM ghcr.io/chipp/build.rust.x86_64_musl:1.59.0_1 AS builder

WORKDIR /home/rust/src
RUN USER=rust \
  cargo init --bin /home/rust/src

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

ENV RUSTFLAGS="-C relocation-model=static"

RUN cargo build --release -p wot_api && \
  cargo clean --release -p wot_api \
  --target x86_64-unknown-linux-musl && \
  rm ./src/*.rs

COPY ./src ./src

RUN cargo build --release -p wot_api && \
  mv target/x86_64-unknown-linux-musl/release/wot_api ./ && \
  rm -rf target/x86_64-unknown-linux-musl/release/ target/release/

FROM alpine:3.15
RUN apk --no-cache add ca-certificates && update-ca-certificates

WORKDIR /root/
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
ENV RUST_BACKTRACE=full

COPY --from=0 /home/rust/src/wot_api .
