# Stage 1: Build the Rust application
FROM rust:1.79 as builder

# Create app directory
WORKDIR /usr/src/app

# Copy Cargo files first for caching dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build in release mode
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

# Install required libraries (if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /usr/src/app/target/release/my_service /usr/local/bin/my_service

# Expose port
EXPOSE 3000

# Run the binary
CMD ["my_service"]
