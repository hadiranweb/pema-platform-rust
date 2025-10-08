SHELL := /bin/bash
.PHONY: all installer auth-backend general-backend frontend clean install-config run-installer run-auth-backend run-general-backend run-frontend

# Define paths
INSTALLER_DIR := backends/installer
AUTH_BACKEND_DIR := backends/wasm-auth-backend
GENERAL_BACKEND_DIR := backends/wasm-general-backend
FRONTEND_DIR := wasm-frontend
SHARED_CONFIG_DIR := shared/config

# Default build profile
BUILD_PROFILE ?= release

all: installer auth-backend general-backend frontend

# Build the installer backend
installer: 
	@echo "Building PEMA Installer..."
	cargo build --$(BUILD_PROFILE) --manifest-path $(INSTALLER_DIR)/Cargo.toml

# Build the authentication backend (WASM library)
auth-backend: 
	@echo "Building WASM Auth Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --manifest-path $(AUTH_BACKEND_DIR)/Cargo.toml

# Build the general backend (WASM library)
general-backend: 
	@echo "Building WASM General Backend..."
	cargo build --target wasm32-unknown-unknown --$(BUILD_PROFILE) --manifest-path $(GENERAL_BACKEND_DIR)/Cargo.toml

# Build the frontend
frontend: 
	@echo "Building WASM Frontend..."
	cd $(FRONTEND_DIR) && trunk build --$(BUILD_PROFILE)

# Clean all build artifacts
clean:
	@echo "Cleaning all build artifacts..."
	cargo clean
	rm -rf $(FRONTEND_DIR)/dist

# --- Installation and Running --- 

# Run the installer to generate config.toml
install-config:
	@echo "Running PEMA Installer to generate config.toml..."
	@echo "Ensure you have configured $(INSTALLER_DIR)/.env before running."
	@echo "Access the installer at http://localhost:8080 (or configured port) in your browser."
	@echo "Press Ctrl+C after configuration is complete."
	cd $(INSTALLER_DIR) && RUST_LOG=info cargo run --package pema-installer

# Run the authentication backend (WASM library - typically run via a WASM runtime or integrated into a server)
run-auth-backend:
	@echo "WASM Auth Backend is a library. It needs a runtime or server to execute."
	@echo "You would typically integrate this into a server or use a WASM runtime."
	@echo "For testing, you might use wasm-bindgen-cli or a custom test runner."

# Run the general backend (WASM library - typically run via a WASM runtime or integrated into a server)
run-general-backend:
	@echo "WASM General Backend is a library. It needs a runtime or server to execute."
	@echo "You would typically integrate this into a server or use a WASM runtime."
	@echo "For testing, you might use wasm-bindgen-cli or a custom test runner."

# Serve the frontend (after building)
run-frontend:
	@echo "Serving PEMA Frontend..."
	@echo "Ensure the frontend has been built using 'make frontend' first."
	cd $(FRONTEND_DIR) && trunk serve --port 3000 --proxy-backend http://localhost:8081 --proxy-auth http://localhost:8082

# Helper for local development (runs installer, then frontend)
# Note: This is for local dev. For server, you'd run backends as services.
local-dev:
	@echo "Starting local development environment..."
	@echo "1. Run 'make install-config' in a separate terminal to generate config.toml."
	@echo "2. Once config.toml is generated, run 'make run-auth-backend' and 'make run-general-backend' (if they were traditional servers)."
	@echo "   (Note: Auth/General backends are WASM libraries, so they are not directly 'run' like this.)"
	@echo "3. Then run 'make run-frontend' in another terminal."
	@echo "This Makefile is primarily for building and initial config generation."

# Setup for deployment (e.g., systemd services, Docker Compose)
deploy-setup:
	@echo "Deployment setup involves creating systemd services or Docker Compose configurations."
	@echo "This Makefile focuses on build and initial config. Deployment scripts are separate."

