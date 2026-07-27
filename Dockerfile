FROM rust:1.97.0-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends build-essential clang curl perl pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos --version 0.3.7 --locked
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 app \
    && useradd --uid 10001 --gid app --no-create-home --shell /usr/sbin/nologin app

COPY --from=builder --chown=app:app /app/target/release/zero2prod /usr/local/bin/zero2prod
COPY --from=builder --chown=app:app /app/target/site ./site
COPY --chown=app:app ./configuration ./configuration
ENV APP_ENVIRONMENT=production
ENV LEPTOS_SITE_ROOT=/app/site
EXPOSE 8000
USER app
HEALTHCHECK --interval=10s --timeout=3s --start-period=15s --retries=5 \
    CMD ["curl", "--fail", "--silent", "http://127.0.0.1:8000/healthz"]
ENTRYPOINT ["/usr/local/bin/zero2prod"]
