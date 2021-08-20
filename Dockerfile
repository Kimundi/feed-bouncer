FROM rust
COPY . feed-bouncer/
WORKDIR /feed-bouncer
RUN cargo build -p feed-bouncer-server --release

VOLUME [ "/storage" ]
CMD target/release/feed-bouncer-server --storage-path /storage
