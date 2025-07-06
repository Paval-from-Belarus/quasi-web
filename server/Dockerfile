FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --bin server --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/quasi-web-server /usr/local/bin/server
EXPOSE 8080
CMD ["/usr/local/bin/server"]
