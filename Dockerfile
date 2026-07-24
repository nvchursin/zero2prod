FROM rust:1.96.0-slim-bookworm AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

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
COPY --chown=app:app ./configuration ./configuration
ENV APP_ENVIRONMENT=production
EXPOSE 8000
USER app
HEALTHCHECK --interval=10s --timeout=3s --start-period=15s --retries=5 \
    CMD ["curl", "--fail", "--silent", "http://127.0.0.1:8000/healthz"]
ENTRYPOINT ["/usr/local/bin/zero2prod"]
