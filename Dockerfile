FROM rust:1.96.0-slim-bookworm AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod /usr/local/bin/zero2prod
COPY ./configuration ./configuration
ENV APP_ENVIRONMENT=production
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/zero2prod"]