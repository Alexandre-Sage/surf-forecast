FROM   archlinux:latest AS builder
RUN pacman -Sy --noconfirm
RUN pacman -S postgresql rustup base-devel sqlite --noconfirm
RUN rustup default stable
ARG PROJECT
WORKDIR /opt/builder
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release -p ${PROJECT}

FROM alpine
WORKDIR /opt/server
COPY --from=builder /opt/builder/target/release/${PROJECT} bin
ENV RUST_LOG=error
CMD ["./bin"]
