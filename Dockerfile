ARG RUST_VERSION=1.73.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --locked --release && \
  cp ./target/release/colatiger /app

FROM debian:bookworm-slim AS final
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /app/colatiger /usr/local/bin
RUN chown appuser /usr/local/bin/colatiger
COPY --from=builder /app/config /opt/colatiger/config
RUN chown -R appuser /opt/colatiger/config
USER appuser
ENV RUST_LOG="colatiger=debug,tower_http=debug,info"
WORKDIR /opt/colatiger
ENTRYPOINT ["colatiger"]
EXPOSE 8080/tcp