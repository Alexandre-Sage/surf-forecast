FROM  rust:alpine AS builder
WORKDIR /opt/builder
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM alpine
WORKDIR /opt/server
COPY --from=builder /opt/builder/target/release/ bin
ENV RUST_LOG=error
CMD ["./bin"]
