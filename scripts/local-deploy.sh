#!/bin/bash

# PEMA Platform Local Deployment Script
# Simple deployment for development and testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PLATFORM_NAME="PEMA Platform"
PROJECT_DIR="$(pwd)"
BACKEND_DIR="$PROJECT_DIR/backend-server"

# Functions
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check requirements
check_requirements() {
    log "Checking requirements..."
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ] || [ ! -d "backend-server" ]; then
        error "Please run this script from the project root directory"
    fi
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is not installed. Please install Rust first."
    fi
    
    success "Requirements check passed"
}

# Setup environment
setup_environment() {
    log "Setting up environment..."
    
    # Create .env file if it doesn't exist
    if [ ! -f "$BACKEND_DIR/.env" ]; then
        log "Creating .env file..."
        cat > "$BACKEND_DIR/.env" << EOF
DATABASE_URL=postgresql://pema_user:pema_password@localhost/pema_platform
JWT_SECRET=your-jwt-secret-key-change-this-in-production
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
CORS_ALLOWED_ORIGINS=*
EOF
        success ".env file created"
    else
        log ".env file already exists"
    fi
    
    # Set environment variables
    export SQLX_OFFLINE=true
    export RUST_LOG=info
    
    success "Environment setup complete"
}

# Build project
build_project() {
    log "Building $PLATFORM_NAME..."
    
    # Build the entire workspace
    cargo build --release --workspace
    
    if [ $? -eq 0 ]; then
        success "Build completed successfully"
    else
        error "Build failed"
    fi
}

# Test compilation
test_compilation() {
    log "Testing compilation..."
    
    export SQLX_OFFLINE=true
    cargo check --workspace
    
    if [ $? -eq 0 ]; then
        success "Compilation test passed"
    else
        error "Compilation test failed"
    fi
}

# Run server
run_server() {
    log "Starting $PLATFORM_NAME server..."
    
    cd "$BACKEND_DIR"
    
    # Check if binary exists
    if [ ! -f "../target/release/pema-backend-server" ]; then
        error "Binary not found. Build failed or binary name incorrect."
    fi
    
    log "Server starting on http://localhost:8000"
    log "Health check: http://localhost:8000/health"
    log "Press Ctrl+C to stop the server"
    
    # Run the server
    ../target/release/pema-backend-server
}

# Quick test
quick_test() {
    log "Running quick test..."
    
    # Test compilation only
    export SQLX_OFFLINE=true
    cargo check --workspace
    
    if [ $? -eq 0 ]; then
        success "✅ Project compiles successfully"
        log "✅ All systems ready for deployment"
    else
        error "❌ Compilation failed"
    fi
}

# Show help
show_help() {
    echo "PEMA Platform Local Deployment Script"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build     - Build the project"
    echo "  run       - Build and run the server"
    echo "  test      - Test compilation only"
    echo "  setup     - Setup environment only"
    echo "  help      - Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 build     # Build the project"
    echo "  $0 run       # Build and run server"
    echo "  $0 test      # Quick compilation test"
}

# Main function
main() {
    case "${1:-run}" in
        "build")
            check_requirements
            setup_environment
            build_project
            ;;
        "run")
            check_requirements
            setup_environment
            build_project
            run_server
            ;;
        "test")
            check_requirements
            quick_test
            ;;
        "setup")
            check_requirements
            setup_environment
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            log "🚀 Starting $PLATFORM_NAME local deployment..."
            check_requirements
            setup_environment
            build_project
            run_server
            ;;
    esac
}

# Run main function
main "$@"