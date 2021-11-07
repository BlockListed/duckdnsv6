FROM rust:latest as builder
WORKDIR /usr/src/duckdnsv6
COPY . .
RUN make

FROM debian:bullseye-slim
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/cargo/bin/duckdnsv6 /usr/bin
COPY --from=builder /opt/cargo/duckdnsv6/duckdnsv6.toml /usr/share/doc/
CMD ['duckdnsv6']