SHELL := /bin/bash
.PHONY: help setup dev build test clean migrate db-up db-down fmt clippy check

# Project Structure Configuration
AUTH_SERVER_DIR := auth-server
BACKEND_DIR := backend
FRONTEND_DIR := frontend
PLUGINS_DIR := plugins

BUILD_PROFILE ?= release

help:
	@echo "🏗️  PEMA Platform - Deployment & Development Commands"
	@echo "===================================================="
	@echo "  make setup               - Initial project setup (Docker Compose or manual DB)"
	@echo "  make dev                 - Start development servers (backend, auth-server, frontend)"
	@echo "  make build               - Build all production artifacts (backend, auth-server, frontend)"
	@echo "  make test                - Run all tests"
	@echo "  make clean               - Clean all build artifacts"
	@echo "  make migrate             - Run database migrations"
	@echo "  make db-up               - Start PostgreSQL database (manual or Docker)"
	@echo "  make db-down             - Stop PostgreSQL database (manual or Docker)"
	@echo "  make fmt                 - Format code"
	@echo "  make clippy              - Run clippy linter"
	@echo "  make check               - Run fmt, clippy, and tests"

setup:
	@echo "🚀 Setting up PEMA Platform..."
	@echo "Installing Rust WASM target and tools..."
	rustup target add wasm32-unknown-unknown || true
	cargo install trunk wasm-bindgen-cli || true
	cargo install sqlx-cli --no-default-features --features "postgres,runtime-tokio-rustls" || true
	@echo "Setting up environment files..."
	cp $(BACKEND_DIR)/.env.example $(BACKEND_DIR)/.env || true
	cp $(AUTH_SERVER_DIR)/.env.example $(AUTH_SERVER_DIR)/.env || true
	@echo "Starting database..."
	make db-up
	@echo "⏳ Waiting for database to be ready..."
	sleep 10
	make migrate
	@echo "✅ Setup complete!"

dev:
	@echo "🚀 Starting PEMA Platform development servers..."
	(cd $(BACKEND_DIR) && cargo watch -x 'run --release' --ignore 'target') & \
	(cd $(AUTH_SERVER_DIR) && cargo watch -x 'run --release' --ignore 'target') & \
	(cd $(FRONTEND_DIR) && trunk serve --port 3000) & \
	wait

build:
	@echo "🏗️  Building PEMA Platform production artifacts..."
	@echo "Building main backend..."
	cargo build --$(BUILD_PROFILE) --manifest-path $(BACKEND_DIR)/Cargo.toml
	@echo "Building authentication server..."
	cargo build --$(BUILD_PROFILE) --manifest-path $(AUTH_SERVER_DIR)/Cargo.toml
	@echo "Building frontend..."
	cd $(FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)
	@echo "Building plugins..."
	@for plugin in $(PLUGINS_DIR)/*/; do \
		if [ -f "$$plugin/Cargo.toml" ]; then \
			echo "Building plugin: $$plugin"; \
			cd "$$plugin" && cargo build --$(BUILD_PROFILE) --target wasm32-unknown-unknown; \
			cd ../..; \
		fi \
	done

test:
	@echo "Running all tests..."
	cargo test --workspace

migrate:
	@echo "Running database migrations..."
	sqlx migrate run --database-url $$(grep DATABASE_URL $(BACKEND_DIR)/.env | cut -d '=' -f2-)

clean:
	@echo "🧹 Cleaning all build artifacts..."
	cargo clean
	rm -rf $(FRONTEND_DIR)/dist
	find $(PLUGINS_DIR) -name target -type d -exec rm -rf {} + 2>/dev/null || true

db-up:
	@echo "🐘 Starting PostgreSQL database..."
	@echo "Attempting to start PostgreSQL via Docker Compose..."
	docker compose up -d db || \
	(echo "Docker Compose failed, attempting to start local PostgreSQL service..."; sudo systemctl start postgresql || echo "PostgreSQL service start failed - please install PostgreSQL first")

db-down:
	@echo "🛑 Stopping PostgreSQL database..."
	docker compose stop db || \
	(echo "Docker Compose failed, attempting to stop local PostgreSQL service..."; sudo systemctl stop postgresql || echo "PostgreSQL service stop failed")

fmt:
	@echo "Formatting code..."
	cargo fmt --all

clippy:
	@echo "Running clippy linter..."
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt clippy test

