# Tardigrade Git Module Dockerfile
# Multi-stage build for optimized production image

# Build stage
FROM rust:1.70-slim AS builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/tardigrade-git

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/
COPY migrations/ ./migrations/

# Build in release mode
RUN cargo build --release

# Production stage
FROM debian:12-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/tardigrade-git

# Copy binary from builder
COPY --from=builder /usr/src/tardigrade-git/target/release/tardigrade-git /usr/local/bin/

# Copy migrations
COPY --from=builder /usr/src/tardigrade-git/migrations/ ./migrations/

# Create non-root user
RUN useradd -r -s /bin/false tardigrade

# Change ownership
RUN chown -R tardigrade:tardigrade /usr/src/tardigrade-git

# Switch to non-root user
USER tardigrade

# Expose port
EXPOSE 3001

# Set environment variables
ENV DATABASE_URL=postgres://user:password@localhost:5432/tardigrade
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3001
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/health || exit 1

# Run the application
CMD ["tardigrade-git"]
