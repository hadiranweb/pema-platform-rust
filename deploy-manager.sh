#!/bin/bash

# PEMA Platform Deployment Manager
# Integrates all existing deployment scripts with proper priority
# Supports multiple deployment modes: development, production, unified

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
PLATFORM_NAME="PEMA Platform"
PROJECT_ROOT="$(pwd)"
SCRIPT_DIR="$PROJECT_ROOT/scripts"

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

info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

header() {
    echo -e "${PURPLE}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${PURPLE}║${NC}                    $1                    ${PURPLE}║${NC}"
    echo -e "${PURPLE}╚══════════════════════════════════════════════════════════════╝${NC}"
}

# Check available deployment scripts
check_available_scripts() {
    log "🔍 Checking available deployment scripts..."
    
    local scripts_found=0
    
    # Check unified deployment script
    if [ -f "$PROJECT_ROOT/unified-deploy.sh" ]; then
        success "✅ unified-deploy.sh (No SSL, Port 80)"
        scripts_found=$((scripts_found + 1))
    fi
    
    # Check production setup script
    if [ -f "$PROJECT_ROOT/setup_server.sh" ]; then
        success "✅ setup_server.sh (SSL Production)"
        scripts_found=$((scripts_found + 1))
    fi
    
    # Check plugin-based deployment
    if [ -f "$SCRIPT_DIR/deploy.sh" ]; then
        success "✅ scripts/deploy.sh (Plugin Architecture)"
        scripts_found=$((scripts_found + 1))
    fi
    
    # Check local development script
    if [ -f "$SCRIPT_DIR/local-deploy.sh" ]; then
        success "✅ scripts/local-deploy.sh (Development)"
        scripts_found=$((scripts_found + 1))
    fi
    
    # Check Makefile
    if [ -f "$PROJECT_ROOT/Makefile" ]; then
        success "✅ Makefile (Build System)"
        scripts_found=$((scripts_found + 1))
    fi
    
    # Check nginx configurations
    if [ -d "$PROJECT_ROOT/nginx" ]; then
        success "✅ nginx/ (Configuration Templates)"
        scripts_found=$((scripts_found + 1))
    fi
    
    if [ $scripts_found -eq 0 ]; then
        error "❌ No deployment scripts found!"
    else
        success "📊 Found $scripts_found deployment components"
    fi
}

# Development deployment (No SSL, Local)
deploy_development() {
    header "DEVELOPMENT DEPLOYMENT"
    log "🚀 Starting development deployment..."
    
    # Use local-deploy.sh if available, otherwise use unified
    if [ -f "$SCRIPT_DIR/local-deploy.sh" ]; then
        log "Using local development script..."
        bash "$SCRIPT_DIR/local-deploy.sh" build
        success "✅ Development build completed"
        
        log "Starting development server..."
        info "💡 Server will run on http://localhost:8000"
        info "💡 Press Ctrl+C to stop"
        bash "$SCRIPT_DIR/local-deploy.sh" run
    else
        warning "⚠️ Local deployment script not found, using unified deployment..."
        deploy_unified
    fi
}

# Production deployment (SSL, Full setup)
deploy_production() {
    header "PRODUCTION DEPLOYMENT"
    log "🚀 Starting production deployment..."
    
    # Check if running as root
    if [[ $EUID -ne 0 ]]; then
        error "Production deployment must be run as root (use sudo)"
    fi
    
    # Use setup_server.sh for full production setup
    if [ -f "$PROJECT_ROOT/setup_server.sh" ]; then
        log "Using production setup script..."
        bash "$PROJECT_ROOT/setup_server.sh"
        success "✅ Production deployment completed"
    else
        error "❌ Production setup script (setup_server.sh) not found!"
    fi
}

# Plugin-based deployment
deploy_plugin_architecture() {
    header "PLUGIN ARCHITECTURE DEPLOYMENT"
    log "🚀 Starting plugin-based deployment..."
    
    # Check if running as root
    if [[ $EUID -ne 0 ]]; then
        error "Plugin deployment must be run as root (use sudo)"
    fi
    
    if [ -f "$SCRIPT_DIR/deploy.sh" ]; then
        log "Using plugin architecture script..."
        bash "$SCRIPT_DIR/deploy.sh"
        success "✅ Plugin architecture deployment completed"
    else
        error "❌ Plugin deployment script not found!"
    fi
}

# Unified deployment (No SSL, Port 80)
deploy_unified() {
    header "UNIFIED DEPLOYMENT (NO SSL)"
    log "🚀 Starting unified deployment..."
    
    # Check if running as root
    if [[ $EUID -ne 0 ]]; then
        error "Unified deployment must be run as root (use sudo)"
    fi
    
    if [ -f "$PROJECT_ROOT/unified-deploy.sh" ]; then
        log "Using unified deployment script..."
        bash "$PROJECT_ROOT/unified-deploy.sh"
        success "✅ Unified deployment completed"
    else
        error "❌ Unified deployment script not found!"
    fi
}

# Build only (using Makefile)
build_only() {
    header "BUILD ONLY"
    log "🔨 Building project..."
    
    if [ -f "$PROJECT_ROOT/Makefile" ]; then
        log "Using Makefile for build..."
        make clean
        make all
        success "✅ Build completed using Makefile"
    else
        log "Using Cargo for build..."
        export SQLX_OFFLINE=true
        cargo build --release --workspace
        success "✅ Build completed using Cargo"
    fi
}

# Test compilation
test_compilation() {
    header "COMPILATION TEST"
    log "🧪 Testing compilation..."
    
    export SQLX_OFFLINE=true
    cargo check --workspace
    
    if [ $? -eq 0 ]; then
        success "✅ Compilation test passed"
    else
        error "❌ Compilation test failed"
    fi
}

# Health check for all services
health_check() {
    header "HEALTH CHECK"
    log "🏥 Performing comprehensive health check..."
    
    local issues=0
    
    # Check PostgreSQL
    if systemctl is-active --quiet postgresql 2>/dev/null; then
        success "✅ PostgreSQL is running"
    else
        warning "⚠️ PostgreSQL is not running"
        issues=$((issues + 1))
    fi
    
    # Check Nginx
    if systemctl is-active --quiet nginx 2>/dev/null; then
        success "✅ Nginx is running"
    else
        warning "⚠️ Nginx is not running"
        issues=$((issues + 1))
    fi
    
    # Check backend services
    for service in pema-backend pema-backend-server pema-platform; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            success "✅ $service is running"
            break
        fi
    done
    
    # Check common ports
    local ports=(80 8000 8080 8081 8082)
    for port in "${ports[@]}"; do
        if netstat -tuln 2>/dev/null | grep -q ":$port"; then
            success "✅ Port $port is listening"
        fi
    done
    
    # Test HTTP endpoints
    local endpoints=("http://localhost/" "http://localhost:8000/" "http://localhost/api/" "http://localhost/health")
    for endpoint in "${endpoints[@]}"; do
        if curl -s "$endpoint" > /dev/null 2>&1; then
            success "✅ $endpoint is accessible"
        fi
    done
    
    if [ $issues -eq 0 ]; then
        success "🎉 All health checks passed!"
    else
        warning "⚠️ Found $issues potential issues"
    fi
}

# Show service status
show_status() {
    header "SERVICE STATUS"
    log "📊 Current service status..."
    
    # System services
    local services=(postgresql nginx)
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            echo -e "${GREEN}●${NC} $service: active"
        else
            echo -e "${RED}●${NC} $service: inactive"
        fi
    done
    
    # PEMA services
    local pema_services=(pema-backend pema-backend-server pema-platform)
    for service in "${pema_services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            echo -e "${GREEN}●${NC} $service: active"
        fi
    done
    
    # Port status
    echo ""
    log "🔌 Port status:"
    local ports=(80 5432 8000 8080 8081 8082)
    for port in "${ports[@]}"; do
        if netstat -tuln 2>/dev/null | grep -q ":$port"; then
            echo -e "${GREEN}●${NC} Port $port: listening"
        else
            echo -e "${RED}●${NC} Port $port: not listening"
        fi
    done
}

# Show logs
show_logs() {
    header "SERVICE LOGS"
    log "📋 Recent service logs..."
    
    local services=(pema-backend pema-backend-server pema-platform nginx postgresql)
    
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            echo ""
            log "📄 Last 10 lines from $service:"
            journalctl -u "$service" -n 10 --no-pager 2>/dev/null || echo "No logs available"
        fi
    done
}

# Stop all services
stop_services() {
    header "STOPPING SERVICES"
    log "🛑 Stopping all PEMA services..."
    
    local services=(pema-backend pema-backend-server pema-platform)
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            log "Stopping $service..."
            systemctl stop "$service"
            success "✅ $service stopped"
        fi
    done
}

# Start all services
start_services() {
    header "STARTING SERVICES"
    log "▶️ Starting all PEMA services..."
    
    # Start system services first
    local system_services=(postgresql nginx)
    for service in "${system_services[@]}"; do
        if ! systemctl is-active --quiet "$service" 2>/dev/null; then
            log "Starting $service..."
            systemctl start "$service"
            success "✅ $service started"
        fi
    done
    
    # Start PEMA services
    local pema_services=(pema-backend pema-backend-server pema-platform)
    for service in "${pema_services[@]}"; do
        if systemctl list-unit-files | grep -q "$service.service"; then
            log "Starting $service..."
            systemctl start "$service"
            success "✅ $service started"
        fi
    done
}

# Show help
show_help() {
    header "PEMA PLATFORM DEPLOYMENT MANAGER"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "🚀 Deployment Commands:"
    echo "  dev         - Development deployment (No SSL, Local)"
    echo "  prod        - Production deployment (SSL, Full setup)"
    echo "  plugin      - Plugin architecture deployment"
    echo "  unified     - Unified deployment (No SSL, Port 80)"
    echo ""
    echo "🔨 Build Commands:"
    echo "  build       - Build project only"
    echo "  test        - Test compilation only"
    echo ""
    echo "🔧 Management Commands:"
    echo "  health      - Health check all services"
    echo "  status      - Show service status"
    echo "  logs        - Show service logs"
    echo "  start       - Start all services"
    echo "  stop        - Stop all services"
    echo "  restart     - Restart all services"
    echo ""
    echo "ℹ️  Information Commands:"
    echo "  check       - Check available scripts"
    echo "  help        - Show this help"
    echo ""
    echo "📝 Examples:"
    echo "  $0 dev              # Development deployment"
    echo "  sudo $0 prod        # Production deployment"
    echo "  sudo $0 unified     # Unified deployment"
    echo "  $0 build            # Build only"
    echo "  $0 health           # Health check"
    echo "  sudo $0 start       # Start services"
    echo ""
    echo "📋 Available Scripts Integration:"
    check_available_scripts
}

# Main function
main() {
    case "${1:-help}" in
        "dev"|"development")
            deploy_development
            ;;
        "prod"|"production")
            deploy_production
            ;;
        "plugin")
            deploy_plugin_architecture
            ;;
        "unified")
            deploy_unified
            ;;
        "build")
            build_only
            ;;
        "test")
            test_compilation
            ;;
        "health")
            health_check
            ;;
        "status")
            show_status
            ;;
        "logs")
            show_logs
            ;;
        "start")
            start_services
            ;;
        "stop")
            stop_services
            ;;
        "restart")
            stop_services
            sleep 2
            start_services
            ;;
        "check")
            check_available_scripts
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            error "Unknown command: $1"
            echo ""
            show_help
            ;;
    esac
}

# Run main function
main "$@"