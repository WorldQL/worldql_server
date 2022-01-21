# syntax=docker/dockerfile:1.3

# ---
# Build Time
FROM rust AS builder

# Install build dependencies
WORKDIR /app
RUN apt-get update && \
  apt-get install \
    cmake \
  -y

# Build application
COPY ./.git ./.git
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./worldql_server ./worldql_server
RUN cargo install --path worldql_server

# ---
# Runtime
FROM debian:11-slim
WORKDIR /

# Setup non-root user
RUN \
  groupadd -g 1001 worldql && \
  useradd -mu 1001 -g worldql worldql

# Copy application
COPY --from=builder --chown=1001:1001 /usr/local/cargo/bin/worldql_server .

# Define repo label
ARG GIT_REPO
LABEL org.opencontainers.image.source=${GIT_REPO}

# Expose default ports
EXPOSE 5555
EXPOSE 8080
EXPOSE 8081

# Define user and entrypoint
USER worldql
ENTRYPOINT ["/worldql_server"]
