SHELL := /bin/bash
.PHONY: all auth-backend general-backend frontend clean run-auth-backend run-general-backend run-backend run-frontend

AUTH_BACKEND_DIR := wasm-auth-backend
GENERAL_BACKEND_DIR := wasm-general-backend
FRONTEND_DIR := wasm-frontend
SHARED_CONFIG_DIR := shared/config
BACKEND_SERVER_DIR := backend-server

BUILD_PROFILE ?= release

all: auth-backend general-backend frontend

auth-backend:
	@echo "Building WASM Auth Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --features wasm --manifest-path $(AUTH_BACKEND_DIR)/Cargo.toml

general-backend:
	@echo "Building WASM General Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --features wasm --manifest-path $(GENERAL_BACKEND_DIR)/Cargo.toml

frontend:
	@echo "Building WASM Frontend..."
	cd $(FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)

clean:
	@echo "Cleaning all build artifacts..."
	cargo clean
	rm -rf $(FRONTEND_DIR)/dist

run-auth-backend:
	@echo "Starting Auth Backend Server..."
	cd $(BACKEND_SERVER_DIR) && dotenv -e ../.env.auth -- cargo run --release

run-general-backend:
	@echo "Starting General Backend Server..."
	cd $(BACKEND_SERVER_DIR) && dotenv -e ../.env.api -- cargo run --release

run-backend:
	@echo "Starting backend servers..."
	cd $(BACKEND_SERVER_DIR) && dotenv -e ../.env.auth -- cargo run --release & \
	cd $(BACKEND_SERVER_DIR) && dotenv -e ../.env.api -- cargo run --release &
	@echo "Backend servers running on ports 8081 and 8082"

run-frontend:
	@echo "Serving PEMA Frontend..."
	@echo "Ensure the frontend has been built using 'make frontend' first."
	cd $(FRONTEND_DIR) && trunk serve --port 3000

