# syntax = docker/dockerfile:1.3

FROM rust
COPY . feed-bouncer/
WORKDIR /feed-bouncer
RUN --mount=type=cache,target=/feed-bouncer/target cargo build -p feed-bouncer-server --release

VOLUME [ "/storage" ]
CMD target/release/feed-bouncer-server --storage-path /storage
