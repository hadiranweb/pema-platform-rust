SHELL := /bin/bash
.PHONY: all auth-backend general-backend frontend clean deploy-frontend

# Define paths
AUTH_BACKEND_DIR := wasm-auth-backend
GENERAL_BACKEND_DIR := wasm-general-backend
FRONTEND_DIR := wasm-frontend
SHARED_CONFIG_DIR := shared/config

# Default build profile
BUILD_PROFILE ?= release

all: auth-backend general-backend frontend

# Build the authentication backend (WASM library)
auth-backend:
	@echo "Building WASM Auth Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --features wasm --manifest-path $(AUTH_BACKEND_DIR)/Cargo.toml

# Build the general backend (WASM library)
general-backend:
	@echo "Building WASM General Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --features wasm --manifest-path $(GENERAL_BACKEND_DIR)/Cargo.toml

# Build the frontend (produces static files in wasm-frontend/dist)
frontend:
	@echo "Building WASM Frontend..."
	cd $(FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)

# Clean all build artifacts
clean:
	@echo "Cleaning all build artifacts..."
	cargo clean
	rm -rf $(FRONTEND_DIR)/dist

# Deploy frontend static files to Nginx web root
deploy-frontend:
	@echo "Deploying frontend static files..."
	sudo cp -r $(FRONTEND_DIR)/dist/* /var/www/pemalune.ir/
	@echo "Frontend files deployed to /var/www/pemalune.ir/"

# Note: Backend Rust servers (if any) should have their own build/run commands
# and are expected to be managed by systemd services as configured in Phase 3.
# The WASM libraries are built by 'auth-backend' and 'general-backend' targets.

