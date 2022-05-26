# Use the latest Rust stable release as base image
FROM rust:1.61.0 AS builder

# Switch our working directory to `app` (same as `cd app)
# The `app` folder will be created for us by Docker in case 
# it does not exist already.
WORKDIR /app

# Install required system dependencies for our linking config
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .

# sqlx env var to use generated json data to check
# compile time SQL queries
ENV SQLX_OFFLINE true

# Build the binary
# Use release profile to make it speedy :)
RUN cargo build --release

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

# Set the APP_ENVIRONMENT to production
# Really doesn't need explaining but everything else
# had comments and this would have been lonely
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]