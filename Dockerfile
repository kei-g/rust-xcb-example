FROM debian:stable-slim

ENV DEBCONF_NOWARNINGS=yes
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
apt-get upgrade -y && \
apt-get dist-upgrade -y && \
apt-get install -y curl gcc libxcb1-dev make upx && \
curl --proto '=https' --tlsv1.2 -fSs https://sh.rustup.rs | sh -s -- -y

COPY Cargo.toml ./
COPY Makefile ./
COPY src/ ./src

RUN env PATH="$HOME/.cargo/bin:$PATH" \
$HOME/.cargo/bin/cargo build --release && \
strip --strip-all target/release/xcb-example
