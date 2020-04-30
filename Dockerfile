# Build step
FROM rust:1.43 as builder
WORKDIR /usr/src/papergraph
COPY . .
RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo install --path .

# Serving container
FROM debian:buster-slim
WORKDIR /usr/src/papergraph
RUN apt-get update && apt-get install -y libpq-dev wget
COPY . .
COPY --from=builder /usr/local/cargo/bin/papergraph /usr/local/bin/papergraph
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
CMD ["papergraph"]
