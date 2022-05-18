#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a custom user has been set
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # Increased maximum number of connections for testing purposes

# Ping Postgres until it isready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create