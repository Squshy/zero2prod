FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef

WORKDIR /app

# Install required system dependencies for our linking config
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies, not the application
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if the dependency tree stays the same,
# all layers should be cached.

COPY . .

# sqlx env var to use generated json data to check
# compile time SQL queries
ENV SQLX_OFFLINE true

# Build the binary
# Use release profile to make it speedy :)
RUN cargo build --release --bin zero2prod

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder env to runtime env
COPY --from=builder /app/target/release/zero2prod zero2prod

# We need config file at runtime
COPY configuration configuration

ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]
