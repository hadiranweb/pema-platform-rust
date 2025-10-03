FROM rust:slim-buster AS builder

WORKDIR /app

# Copy the root Cargo.toml and Cargo.lock for workspace context
COPY Cargo.toml Cargo.lock ./

# Copy the specific backend's Cargo.toml and src
COPY wasm-general-backend/Cargo.toml ./wasm-general-backend/Cargo.toml
COPY wasm-general-backend/src ./wasm-general-backend/src

# Copy shared crates
COPY shared/models ./shared/models
COPY shared/config ./shared/config

# Build the backend
RUN cargo build --release --package wasm-general-backend

# --- Final stage ---
FROM debian:buster-slim

WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/wasm-general-backend .

# Expose the port the service will run on (assuming 8080 as a common default)
EXPOSE 8080

CMD ["./wasm-general-backend"]
