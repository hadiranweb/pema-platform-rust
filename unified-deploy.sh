#!/bin/bash

# PEMA Platform Unified Deployment Script
# No SSL - Development/Testing Environment
# Priority: PostgreSQL -> Nginx -> Backend -> Frontend

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration Variables
PLATFORM_NAME="PEMA Platform"
PROJECT_ROOT="$(pwd)"
BACKEND_DIR="$PROJECT_ROOT/backend-server"
FRONTEND_DIR="$PROJECT_ROOT/wasm-frontend"
FRONTEND_DIST_DIR="/var/www/pema-platform"

# Database Configuration (No SSL)
DB_USER="pema_user"
DB_PASSWORD="F8s77@98"
DB_NAME="pema_db"

# Service Ports
BACKEND_PORT="8000"
FRONTEND_PORT="80"

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

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
    fi
}

# Step 1: Setup PostgreSQL (No SSL)
setup_postgresql() {
    log "🗄️  Step 1: Setting up PostgreSQL (No SSL)..."
    
    # Install PostgreSQL if not installed
    if ! command -v psql &> /dev/null; then
        log "Installing PostgreSQL..."
        apt update
        apt install -y postgresql postgresql-contrib
    fi
    
    # Start and enable PostgreSQL
    systemctl start postgresql
    systemctl enable postgresql
    
    # Configure PostgreSQL (No SSL)
    log "Configuring PostgreSQL database..."
    sudo -i -u postgres psql <<EOF
-- Drop existing database and user if they exist
DROP DATABASE IF EXISTS ${DB_NAME};
DROP USER IF EXISTS ${DB_USER};

-- Create new user and database
CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}';
CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};
GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};

-- Connect to database and grant schema permissions
\c ${DB_NAME};
GRANT ALL ON SCHEMA public TO ${DB_USER};
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${DB_USER};
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ${DB_USER};
EOF
    
    # Configure PostgreSQL to disable SSL
    PG_VERSION=$(sudo -u postgres psql -t -c "SELECT version();" | grep -oP '\d+\.\d+' | head -1)
    PG_CONFIG_DIR="/etc/postgresql/${PG_VERSION}/main"
    
    if [ -f "${PG_CONFIG_DIR}/postgresql.conf" ]; then
        log "Disabling SSL in PostgreSQL..."
        sed -i "s/#ssl = off/ssl = off/" "${PG_CONFIG_DIR}/postgresql.conf"
        sed -i "s/ssl = on/ssl = off/" "${PG_CONFIG_DIR}/postgresql.conf"
        
        # Restart PostgreSQL to apply changes
        systemctl restart postgresql
    fi
    
    success "✅ PostgreSQL setup completed (No SSL)"
}

# Step 2: Setup Nginx (No SSL, Port 80)
setup_nginx() {
    log "🌐 Step 2: Setting up Nginx (No SSL, Port 80)..."
    
    # Install Nginx if not installed
    if ! command -v nginx &> /dev/null; then
        log "Installing Nginx..."
        apt update
        apt install -y nginx
    fi
    
    # Create frontend directory
    mkdir -p "$FRONTEND_DIST_DIR"
    chown -R www-data:www-data "$FRONTEND_DIST_DIR"
    
    # Create Nginx configuration (No SSL)
    log "Creating Nginx configuration..."
    cat > "/etc/nginx/sites-available/pema-platform" << EOF
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    server_name _;
    
    # Frontend static files
    root $FRONTEND_DIST_DIR;
    index index.html;
    
    # Serve static files and handle SPA routing
    location / {
        try_files \$uri \$uri/ /index.html;
        add_header Cache-Control "no-cache, no-store, must-revalidate";
        add_header Pragma "no-cache";
        add_header Expires "0";
    }
    
    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot|wasm)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
        try_files \$uri =404;
    }
    
    # API routes - proxy to backend
    location /api/ {
        proxy_pass http://127.0.0.1:${BACKEND_PORT}/api/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_redirect off;
        
        # CORS headers for development
        add_header Access-Control-Allow-Origin "*" always;
        add_header Access-Control-Allow-Methods "GET, POST, PUT, DELETE, OPTIONS" always;
        add_header Access-Control-Allow-Headers "Authorization, Content-Type" always;
        add_header Access-Control-Allow-Credentials "true" always;
        
        if (\$request_method = OPTIONS) {
            return 204;
        }
    }
    
    # Health check endpoint
    location /health {
        proxy_pass http://127.0.0.1:${BACKEND_PORT}/health;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
    
    # Security headers (basic)
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    
    # Error pages
    error_page 404 /404.html;
    error_page 500 502 503 504 /50x.html;
}
EOF
    
    # Enable site and disable default
    ln -sf "/etc/nginx/sites-available/pema-platform" "/etc/nginx/sites-enabled/"
    rm -f "/etc/nginx/sites-enabled/default"
    
    # Test and reload Nginx
    if nginx -t; then
        systemctl enable nginx
        systemctl restart nginx
        success "✅ Nginx configured and started (Port 80, No SSL)"
    else
        error "❌ Nginx configuration test failed"
    fi
}

# Step 3: Build and Setup Backend Services
setup_backend() {
    log "⚙️  Step 3: Building and setting up Backend services..."
    
    cd "$PROJECT_ROOT"
    
    # Check requirements
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is not installed. Please install Rust first."
    fi
    
    # Create environment file
    log "Creating backend environment configuration..."
    cat > "$BACKEND_DIR/.env" << EOF
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}
JWT_SECRET=$(openssl rand -base64 32)
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=${BACKEND_PORT}
CORS_ALLOWED_ORIGINS=*
EOF
    
    # Set environment variables for build
    export SQLX_OFFLINE=true
    export RUST_LOG=info
    
    # Build the project
    log "Building backend server..."
    cargo build --release --package pema-backend-server
    
    if [ ! -f "./target/release/pema-backend-server" ]; then
        error "❌ Backend build failed - binary not found"
    fi
    
    # Create systemd service
    log "Creating systemd service..."
    cat > "/etc/systemd/system/pema-backend.service" << EOF
[Unit]
Description=PEMA Platform Backend Server
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=$BACKEND_DIR
EnvironmentFile=$BACKEND_DIR/.env
ExecStart=$PROJECT_ROOT/target/release/pema-backend-server
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$PROJECT_ROOT

[Install]
WantedBy=multi-user.target
EOF
    
    # Enable and start service
    systemctl daemon-reload
    systemctl enable pema-backend.service
    systemctl start pema-backend.service
    
    # Wait for service to start
    sleep 5
    
    if systemctl is-active --quiet pema-backend.service; then
        success "✅ Backend service started successfully (Port ${BACKEND_PORT})"
    else
        error "❌ Backend service failed to start"
    fi
}

# Step 4: Build and Deploy Frontend
setup_frontend() {
    log "🎨 Step 4: Building and deploying Frontend..."
    
    cd "$PROJECT_ROOT"
    
    # Check if Trunk is installed
    if ! command -v trunk &> /dev/null; then
        log "Installing Trunk..."
        cargo install trunk
    fi
    
    # Add WASM target if not present
    rustup target add wasm32-unknown-unknown
    
    # Create frontend environment
    if [ -d "$FRONTEND_DIR" ]; then
        log "Creating frontend environment configuration..."
        cat > "$FRONTEND_DIR/.env" << EOF
VITE_API_BASE_URL=http://localhost/api
VITE_APP_NAME=PEMA Platform
VITE_APP_VERSION=1.0.0
EOF
        
        # Build frontend
        log "Building frontend..."
        cd "$FRONTEND_DIR"
        trunk build --release
        
        # Deploy to Nginx directory
        if [ -d "./dist" ]; then
            log "Deploying frontend to Nginx..."
            cp -r ./dist/* "$FRONTEND_DIST_DIR/"
            chown -R www-data:www-data "$FRONTEND_DIST_DIR"
            success "✅ Frontend deployed successfully"
        else
            warning "⚠️ Frontend dist directory not found, creating placeholder..."
            echo "<h1>PEMA Platform</h1><p>Frontend build in progress...</p>" > "$FRONTEND_DIST_DIR/index.html"
        fi
    else
        warning "⚠️ Frontend directory not found, creating placeholder..."
        echo "<h1>PEMA Platform</h1><p>Backend API available at /api/</p>" > "$FRONTEND_DIST_DIR/index.html"
    fi
    
    cd "$PROJECT_ROOT"
}

# Health Check
perform_health_check() {
    log "🏥 Step 5: Performing health checks..."
    
    # Wait for services to be ready
    sleep 10
    
    # Check PostgreSQL
    if systemctl is-active --quiet postgresql; then
        success "✅ PostgreSQL is running"
    else
        error "❌ PostgreSQL is not running"
    fi
    
    # Check Nginx
    if systemctl is-active --quiet nginx; then
        success "✅ Nginx is running"
    else
        error "❌ Nginx is not running"
    fi
    
    # Check Backend
    if systemctl is-active --quiet pema-backend.service; then
        success "✅ Backend service is running"
    else
        error "❌ Backend service is not running"
    fi
    
    # Check if ports are listening
    if netstat -tuln | grep -q ":${BACKEND_PORT}"; then
        success "✅ Backend port ${BACKEND_PORT} is listening"
    else
        warning "⚠️ Backend port ${BACKEND_PORT} is not listening"
    fi
    
    if netstat -tuln | grep -q ":80"; then
        success "✅ Frontend port 80 is listening"
    else
        warning "⚠️ Frontend port 80 is not listening"
    fi
    
    # Test API endpoint
    sleep 5
    if curl -s http://localhost/health > /dev/null 2>&1; then
        success "✅ API health check passed"
    else
        warning "⚠️ API health check failed (service may still be starting)"
    fi
    
    # Test frontend
    if curl -s http://localhost/ > /dev/null 2>&1; then
        success "✅ Frontend is accessible"
    else
        warning "⚠️ Frontend is not accessible"
    fi
}

# Show deployment summary
show_summary() {
    log "📊 Deployment Summary:"
    echo ""
    success "🎉 PEMA Platform deployed successfully!"
    echo ""
    log "📍 Service URLs:"
    log "   Frontend: http://localhost/ (Port 80)"
    log "   Backend API: http://localhost/api/ (Port ${BACKEND_PORT})"
    log "   Health Check: http://localhost/health"
    echo ""
    log "🔧 Service Management:"
    log "   Backend Status: systemctl status pema-backend.service"
    log "   Backend Logs: journalctl -u pema-backend.service -f"
    log "   Nginx Status: systemctl status nginx"
    log "   PostgreSQL Status: systemctl status postgresql"
    echo ""
    log "🗄️ Database:"
    log "   Host: localhost:5432"
    log "   Database: ${DB_NAME}"
    log "   User: ${DB_USER}"
    log "   SSL: Disabled"
    echo ""
    log "📁 Directories:"
    log "   Project: ${PROJECT_ROOT}"
    log "   Frontend: ${FRONTEND_DIST_DIR}"
    log "   Backend Binary: ${PROJECT_ROOT}/target/release/pema-backend-server"
}

# Main deployment function
main() {
    log "🚀 Starting PEMA Platform Unified Deployment (No SSL)..."
    echo ""
    
    check_root
    setup_postgresql
    setup_nginx
    setup_backend
    setup_frontend
    perform_health_check
    show_summary
    
    success "🎯 Deployment completed successfully!"
}

# Show help
show_help() {
    echo "PEMA Platform Unified Deployment Script"
    echo ""
    echo "Usage: sudo $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  deploy    - Full deployment (default)"
    echo "  postgres  - Setup PostgreSQL only"
    echo "  nginx     - Setup Nginx only"
    echo "  backend   - Setup Backend only"
    echo "  frontend  - Setup Frontend only"
    echo "  health    - Health check only"
    echo "  help      - Show this help"
    echo ""
    echo "Examples:"
    echo "  sudo $0           # Full deployment"
    echo "  sudo $0 deploy    # Full deployment"
    echo "  sudo $0 postgres  # PostgreSQL setup only"
    echo "  sudo $0 health    # Health check only"
}

# Handle command line arguments
case "${1:-deploy}" in
    "deploy")
        main
        ;;
    "postgres")
        check_root
        setup_postgresql
        ;;
    "nginx")
        check_root
        setup_nginx
        ;;
    "backend")
        check_root
        setup_backend
        ;;
    "frontend")
        check_root
        setup_frontend
        ;;
    "health")
        perform_health_check
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        log "Unknown command: $1"
        show_help
        exit 1
        ;;
esac