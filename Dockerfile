# Stage 1: Build the Rust app
FROM rust:1.75 as builder

# Install system dependencies needed for building crates like hidapi
RUN apt-get update && \
    apt-get install -y pkg-config libudev-dev libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
# COPY /app/target/release/sanctum-clanker-update .

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["./app/target/release/sanctum-clanker-update"]