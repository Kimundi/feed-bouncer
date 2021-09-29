# syntax = docker/dockerfile:1.3

FROM rust
ENV CARGO_TARGET_DIR=/feed-bouncer/target
COPY . feed-bouncer/
WORKDIR /feed-bouncer
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/feed-bouncer/target \
    cargo install --path feed-bouncer-server --locked

VOLUME [ "/storage" ]
CMD feed-bouncer-server --storage-path /storage
