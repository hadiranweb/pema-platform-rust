SHELL := /bin/bash
.PHONY: help setup dev build test clean migrate docker-up docker-down fmt clippy check

AUTH_SERVER_DIR := auth-server
BACKEND_DIR := backend
FRONTEND_DIR := frontend

BUILD_PROFILE ?= release

help:
	@echo "Available commands:"
	@echo "  make setup        - Initial project setup (Docker Compose)"
	@echo "  make dev          - Start development servers (backend, auth-server, frontend)"
	@echo "  make build        - Build production artifacts (backend, auth-server, frontend)"
	@echo "  make test         - Run all tests"
	@echo "  make clean        - Clean all build artifacts"
	@echo "  make migrate      - Run database migrations"
	@echo "  make docker-up    - Start Docker services"
	@echo "  make docker-down  - Stop Docker services"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Run clippy linter"
	@echo "  make check        - Run fmt, clippy, and tests"

setup:
	@echo "🚀 Setting up PEMA Platform with Docker Compose..."
	@echo "Ensure Docker and Docker Compose are installed."
	@echo "Copying .env files..."
	cp .env.example .env || true
	cp $(BACKEND_DIR)/.env.example $(BACKEND_DIR)/.env || true
	cp $(AUTH_SERVER_DIR)/.env.example $(AUTH_SERVER_DIR)/.env || true
	make docker-up
	@echo "⏳ Waiting for database..."
	sleep 10
	make migrate
	@echo "✅ Setup complete!"

dev:
	@echo "Starting development servers..."
	(cd $(BACKEND_DIR) && cargo watch -x 'run --release' --ignore 'target') & \
	(cd $(AUTH_SERVER_DIR) && cargo watch -x 'run --release' --ignore 'target') & \
	(cd $(FRONTEND_DIR) && trunk serve --port 3000) & \
	wait

build:
	@echo "Building production artifacts..."
	@echo "Building backend..."
	cargo build --$(BUILD_PROFILE) --manifest-path $(BACKEND_DIR)/Cargo.toml
	@echo "Building auth-server..."
	cargo build --$(BUILD_PROFILE) --manifest-path $(AUTH_SERVER_DIR)/Cargo.toml
	@echo "Building frontend..."
	cd $(FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)

test:
	@echo "Running all tests..."
	cargo test --workspace

migrate:
	@echo "Running database migrations..."
	cargo install sqlx-cli --no-default-features --features "postgres,runtime-tokio-rustls" || true
	sqlx migrate run --database-url $$(grep DATABASE_URL $(BACKEND_DIR)/.env | cut -d '=' -f2-)

clean:
	@echo "Cleaning all build artifacts..."
	cargo clean
	rm -rf $(FRONTEND_DIR)/dist

docker-up:
	@echo "Starting Docker services..."
	docker compose up -d

docker-down:
	@echo "Stopping Docker services..."
	docker compose down

fmt:
	@echo "Formatting code..."
	cargo fmt --all

clippy:
	@echo "Running clippy linter..."
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt clippy test

