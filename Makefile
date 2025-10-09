SHELL := /bin/bash
.PHONY: all auth-backend general-backend frontend clean run-auth-backend run-general-backend run-frontend

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


