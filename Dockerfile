# Stage 1: Build the Rust app
FROM rust:1.75 as builder

RUN apt-get update && \
	apt-get install -y pkg-config libudev-dev libssl-dev build-essential && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

RUN apt-get update && \
	apt-get install -y ca-certificates libssl-dev libudev-dev && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/sanctum-cranker-update-api .

EXPOSE 5555
CMD ["./sanctum-cranker-update-api"]
