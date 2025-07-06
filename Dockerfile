FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
# Install cross-compilation targets for amd64 and aarch64
RUN rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
# Build for the target architecture (set by Docker Buildx)
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
      cargo build --release --bin server --target x86_64-unknown-linux-gnu && \
      mv target/x86_64-unknown-linux-gnu/release/server /app/server; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
      cargo build --release --bin server --target aarch64-unknown-linux-gnu && \
      mv target/aarch64-unknown-linux-gnu/release/server /app/server; \
    else \
      echo "Unsupported architecture: $TARGETARCH" && exit 1; \
    fi

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/server /usr/local/bin/server
EXPOSE 8080
CMD ["/usr/local/bin/server"]
