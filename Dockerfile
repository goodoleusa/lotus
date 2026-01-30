# ============================================
# Lotus OSINT Platform - Production Dockerfile
# ============================================

# Build stage
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    libluajit-5.1-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy source
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies and OSINT tools
RUN apt-get update && apt-get install -y \
    libssl3 \
    libluajit-5.1-2 \
    ca-certificates \
    curl \
    git \
    python3 \
    python3-pip \
    golang-go \
    && rm -rf /var/lib/apt/lists/*

# Set Go path
ENV GOPATH=/root/go
ENV PATH=$PATH:/root/go/bin:/root/.local/bin

# Install Python OSINT tools
RUN pip3 install --break-system-packages --no-cache-dir \
    bbot \
    theHarvester \
    shodan

# Install Go OSINT tools
RUN go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest && \
    go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest && \
    go install github.com/projectdiscovery/httpx/cmd/httpx@latest && \
    go install github.com/gitleaks/gitleaks/v8@latest

# Copy Lotus binary
COPY --from=builder /app/target/release/lotus /usr/local/bin/lotus

# Copy example scripts
COPY examples /app/examples
COPY docs /app/docs

WORKDIR /app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Default command - start web UI
CMD ["lotus", "serve", "--host", "0.0.0.0", "--port", "8080"]
