FROM rust:1.75 as builder
RUN apt-get update && \
    apt-get install -y pkg-config libudev-dev libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/clanker-update /app/clanker-update
EXPOSE 8080
CMD ["/app/clanker-update"]
