SHELL := /bin/bash
.PHONY: help setup dev build test clean migrate db-up db-down fmt clippy check platform plugins wasm-frontend deploy

# Plugin-Based Architecture Configuration
PLATFORM_DIR := backend-server
WASM_FRONTEND_DIR := wasm-frontend
PLUGINS_DIR := plugins

BUILD_PROFILE ?= release

help:
	@echo "🏗️  PEMA Platform - Plugin-Based Architecture"
	@echo "=============================================="
	@echo "  make setup               - Setup development environment"
	@echo "  make dev                 - Start platform in development mode"
	@echo "  make dev-frontend        - Start WASM frontend in development mode"
	@echo "  make build               - Build all components (platform + plugins + frontend)"
	@echo "  make platform            - Build core platform only"
	@echo "  make plugins             - Build all plugins"
	@echo "  make wasm-frontend       - Build WASM frontend"
	@echo "  make test                - Run all tests"
	@echo "  make clean               - Clean all build artifacts"
	@echo "  make migrate             - Run database migrations"
	@echo "  make db-up               - Start PostgreSQL database"
	@echo "  make db-down             - Stop PostgreSQL database"
	@echo "  make fmt                 - Format code"
	@echo "  make clippy              - Run clippy linter"
	@echo "  make check               - Run fmt, clippy, and tests"
	@echo "  make deploy              - Deploy to production"

setup:
	@echo "🚀 Setting up PEMA Platform..."
	@echo "Installing Rust WASM target and tools..."
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli || true
	cargo install sqlx-cli --no-default-features --features "postgres,runtime-tokio-rustls" || true
	@echo "Setting up environment..."
	cp .env.example .env || true
	make db-up
	@echo "⏳ Waiting for database..."
	sleep 10
	make migrate
	@echo "✅ Setup complete!"

dev:
	@echo "🚀 Starting PEMA Platform in development mode..."
	cargo run --bin backend-server

dev-frontend:
	@echo "🌐 Starting WASM Frontend in development mode..."
	cd $(WASM_FRONTEND_DIR) && trunk serve --port 3000

build: platform plugins wasm-frontend

platform:
	@echo "🏗️  Building PEMA Core Platform..."
	cargo build --$(BUILD_PROFILE) --bin backend-server

plugins:
	@echo "🔌 Building Plugins..."
	@for plugin in $(PLUGINS_DIR)/*/; do \
		if [ -f "$$plugin/Cargo.toml" ]; then \
			echo "Building plugin: $$plugin"; \
			cd "$$plugin" && cargo build --$(BUILD_PROFILE) --target wasm32-unknown-unknown; \
			cd ../..; \
		fi \
	done

wasm-frontend:
	@echo "🌐 Building WASM Frontend..."
	cd $(WASM_FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)

test:
	@echo "Running all tests..."
	cargo test --workspace

migrate:
	@echo "Running database migrations..."
	sqlx migrate run --database-url $$(grep DATABASE_URL .env | cut -d '=' -f2-)

clean:
	@echo "🧹 Cleaning all build artifacts..."
	cargo clean
	rm -rf $(WASM_FRONTEND_DIR)/dist
	find $(PLUGINS_DIR) -name target -type d -exec rm -rf {} + 2>/dev/null || true

db-up:
	@echo "🐘 Starting PostgreSQL database..."
	docker-compose -f docker-compose.db.yml up -d

db-down:
	@echo "🛑 Stopping PostgreSQL database..."
	docker-compose -f docker-compose.db.yml down

deploy: build
	@echo "🚀 Deploying PEMA Platform..."
	@./scripts/deploy.sh

fmt:
	@echo "Formatting code..."
	cargo fmt --all

clippy:
	@echo "Running clippy linter..."
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt clippy test



