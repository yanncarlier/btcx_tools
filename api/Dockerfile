# Build stage
FROM rust:1.81 AS builder

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . /app

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/bitcoin_tx_api /app/bitcoin_tx_api

# Expose port 8080
EXPOSE 8080

# Run the binary
CMD ["/app/bitcoin_tx_api"]