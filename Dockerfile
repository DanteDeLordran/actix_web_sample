# Stage 1: Build Stage
FROM rust:1.72 as builder

# Set working directory
WORKDIR /usr/src/app

# Copy the Cargo manifest and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime Stage
FROM debian:buster-slim

# Install required libraries for Rust binaries
RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV RUST_LOG=info \
    APP_ENV=production

# Set working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/actix-web-app /app/app

RUN chmod 744 /app/app

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./app"]
