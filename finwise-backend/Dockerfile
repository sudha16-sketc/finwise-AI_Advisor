# Step 1: Build the binary
FROM rust:1.75-slim as builder

# Install system dependencies for OpenSSL (common for Stellar/Soroban)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the release binary
RUN cargo build --release

# Step 2: Run the binary
FROM debian:bookworm-slim

# Install OpenSSL in the runner image so the binary can talk to the network
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/finwise-backend .

# Expose the port your Actix server uses
EXPOSE 8080

# Start the app
CMD ["./finwise-backend"]