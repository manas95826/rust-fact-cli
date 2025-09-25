# Use the official Rust image as the base
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/fact-cli /usr/local/bin/fact-cli

# Make the binary executable
RUN chmod +x /usr/local/bin/fact-cli

# Set the working directory
WORKDIR /app

# Run the application
CMD ["fact-cli", "--telegram", "--category", "all"]
