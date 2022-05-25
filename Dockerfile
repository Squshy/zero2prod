# Use the latest Rust stable release as base image
FROM rust:1.61.0

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

# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]