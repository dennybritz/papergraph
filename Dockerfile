# Build step
FROM rust:1.43 as builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

# Serving container
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /usr/local/cargo/bin/papergraph /usr/local/bin/papergraph
CMD ["papergraph"]
