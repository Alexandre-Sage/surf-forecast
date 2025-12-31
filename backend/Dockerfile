ARG LINUX_VERSION=latest
ARG RUST_VERSION=latest
ARG APP_NAME=forecast_api

FROM rust:${RUST_VERSION} AS builder
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
	--mount=type=bind,source=./.env,target=./.env \
	--mount=type=bind,source=./sqlite-databases,target=./sqlite-databases \
	--mount=type=bind,source=Cargo.toml,target=Cargo.toml \
	--mount=type=bind,source=Cargo.lock,target=Cargo.lock \
	--mount=type=cache,target=/app/target/ \
	--mount=type=cache,target=/usr/local/cargo/git/db \
	--mount=type=cache,target=/usr/local/cargo/registry/ \
	cargo build --locked --release && \
	cp ./target/release/$APP_NAME /bin/server && \
	cp -r ./sqlite-databases /bin/sqlite-databases



FROM debian:bookworm-slim AS app

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
	libssl3 \
	ca-certificates \
	libsqlite3-0 && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /bin/app

COPY --from=builder /bin/server /bin/app/server
COPY --from=builder /bin/sqlite-databases /bin/app/sqlite-databases

RUN mkdir -p /bin/app/sqlite-databases && \
	chmod 755 /bin/app/sqlite-databases

CMD ["/bin/app/server"]

