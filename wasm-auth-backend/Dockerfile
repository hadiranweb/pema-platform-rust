FROM rust:slim-buster AS builder

WORKDIR /app

# Copy the root Cargo.toml and Cargo.lock for workspace context
COPY Cargo.toml Cargo.lock ./

# Copy the specific backend's Cargo.toml and src
COPY wasm-auth-backend/Cargo.toml ./wasm-auth-backend/Cargo.toml
COPY wasm-auth-backend/src ./wasm-auth-backend/src

# Copy shared crates
COPY shared/models ./shared/models
COPY shared/config ./shared/config

# Build the backend
RUN cargo build --release --package wasm-auth-backend

# --- Final stage ---
FROM debian:buster-slim

WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/wasm-auth-backend .

# Expose the port the service will run on (assuming 8080 as a common default)
EXPOSE 8080

CMD ["./wasm-auth-backend"]
