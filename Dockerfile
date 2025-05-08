# Stage 1: Build the Rust app
FROM rust:1.75 as builder
# Install system dependencies for building native crates
RUN apt-get update && \
    apt-get install -y pkg-config libudev-dev libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*
# Create app directory and copy source
WORKDIR /app
COPY . .
# Build release binary
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bullseye-slim
# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*
# Create app directory
WORKDIR /app
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/sanctum-clanker-update /app/sanctum-clanker-update
# Expose the port your app listens on
EXPOSE 8080
# Run the binary
CMD ["/app/sanctum-clanker-update"]
