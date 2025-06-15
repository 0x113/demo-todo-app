# Use the official Rust image as our base for building
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy manifest files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage using a smaller base image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies needed for PostgreSQL connections
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN useradd -m -u 1001 appuser

# Set working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/todo-app-rs ./todo-app-rs

# Change ownership to the non-root user
RUN chown appuser:appuser /app/todo-app-rs

# Switch to non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["./todo-app-rs"]