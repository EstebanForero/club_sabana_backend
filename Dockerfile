FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
COPY . .
RUN cargo chef cook --release
RUN cargo build --release --bin
# Assuming the binary is in the http_api crate; adjust if necessary
RUN mv ./target/release/club_sabana_backend ./app

FROM debian:stable-slim AS runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]
