# Build step
FROM rust:1.43 as builder
WORKDIR /usr/src/papergraph
COPY . .
RUN cargo install --path .

# Serving container
FROM debian:buster-slim
WORKDIR /usr/src/papergraph

RUN apt-get update && apt-get install -y wget tar
RUN wget https://github.com/dgraph-io/dgraph/releases/download/v20.03.1/dgraph-linux-amd64.tar.gz && \
  tar -C /usr/local/bin -xzf dgraph-linux-amd64.tar.gz && \
  rm dgraph-linux-amd64.tar.gz

COPY . .
COPY --from=builder /usr/local/cargo/bin/papergraph /usr/local/bin/papergraph
CMD ["papergraph"]