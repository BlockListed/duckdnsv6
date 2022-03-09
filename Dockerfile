FROM rust:1.59 as builder
WORKDIR /usr/src/duckdnsv6
COPY . .
RUN cargo install --path .

FROM s6on/debian
COPY --from=builder /usr/src/duckdnsv6/doc/duckdnsv6.toml /usr/doc/duckdnsv6/
COPY --from=builder /usr/local/cargo/bin/duckdnsv6 /usr/local/bin/duckdnsv6
COPY rootfs/ /
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*