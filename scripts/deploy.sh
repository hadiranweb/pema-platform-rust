#!/bin/bash

# PEMA Platform Deployment Script
# Plugin-Based Architecture Deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PLATFORM_NAME="PEMA Platform"
DEPLOY_DIR="/opt/pema-platform"
SERVICE_NAME="pema-platform"
BACKUP_DIR="/opt/pema-platform/backups"
LOG_FILE="/var/log/pema-platform-deploy.log"

# Functions
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
    exit 1
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
    fi
}

# Check system requirements
check_requirements() {
    log "Checking system requirements..."
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        error "Docker is not installed. Please install Docker first."
    fi
    
    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose is not installed. Please install Docker Compose first."
    fi
    
    # Check if PostgreSQL is accessible
    if ! docker-compose -f docker-compose.db.yml ps | grep -q "Up"; then
        warning "PostgreSQL container is not running. Starting it..."
        docker-compose -f docker-compose.db.yml up -d
        sleep 10
    fi
    
    success "System requirements check passed"
}

# Create backup
create_backup() {
    log "Creating backup..."
    
    if [ -d "$DEPLOY_DIR" ]; then
        BACKUP_NAME="pema-platform-backup-$(date +%Y%m%d-%H%M%S)"
        mkdir -p "$BACKUP_DIR"
        cp -r "$DEPLOY_DIR" "$BACKUP_DIR/$BACKUP_NAME"
        success "Backup created: $BACKUP_DIR/$BACKUP_NAME"
    else
        log "No existing deployment found, skipping backup"
    fi
}

# Stop existing service
stop_service() {
    log "Stopping existing service..."
    
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        systemctl stop "$SERVICE_NAME"
        success "Service stopped"
    else
        log "Service is not running"
    fi
}

# Deploy application
deploy_application() {
    log "Deploying $PLATFORM_NAME..."
    
    # Create deployment directory
    mkdir -p "$DEPLOY_DIR"
    
    # Copy built artifacts
    if [ -f "./target/release/backend-server" ]; then
        cp "./target/release/backend-server" "$DEPLOY_DIR/"
        chmod +x "$DEPLOY_DIR/backend-server"
        success "Platform binary deployed"
    else
        error "Platform binary not found. Run 'make build' first."
    fi
    
    # Copy WASM frontend
    if [ -d "./wasm-frontend/dist" ]; then
        cp -r "./wasm-frontend/dist" "$DEPLOY_DIR/frontend"
        success "WASM frontend deployed"
    else
        error "WASM frontend not found. Run 'make wasm-frontend' first."
    fi
    
    # Copy plugins
    if [ -d "./plugins" ]; then
        cp -r "./plugins" "$DEPLOY_DIR/"
        success "Plugins deployed"
    fi
    
    # Copy configuration
    cp ".env" "$DEPLOY_DIR/"
    cp "docker-compose.db.yml" "$DEPLOY_DIR/"
    
    success "Application deployed to $DEPLOY_DIR"
}

# Create systemd service
create_service() {
    log "Creating systemd service..."
    
    cat > "/etc/systemd/system/$SERVICE_NAME.service" << EOF
[Unit]
Description=PEMA Platform - Plugin-Based Architecture
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
WorkingDirectory=$DEPLOY_DIR
ExecStart=$DEPLOY_DIR/backend-server
Restart=always
RestartSec=5
Environment=RUST_LOG=info
EnvironmentFile=$DEPLOY_DIR/.env

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DEPLOY_DIR

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl enable "$SERVICE_NAME"
    success "Systemd service created and enabled"
}

# Start service
start_service() {
    log "Starting $PLATFORM_NAME service..."
    
    systemctl start "$SERVICE_NAME"
    sleep 5
    
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        success "Service started successfully"
    else
        error "Failed to start service. Check logs: journalctl -u $SERVICE_NAME"
    fi
}

# Setup reverse proxy (Nginx)
setup_nginx() {
    log "Setting up Nginx reverse proxy..."
    
    # Check if Nginx is installed
    if ! command -v nginx &> /dev/null; then
        log "Installing Nginx..."
        apt update
        apt install -y nginx
    fi
    
    # Create Nginx configuration
    cat > "/etc/nginx/sites-available/pema-platform" << EOF
server {
    listen 80;
    server_name pemalune.ir www.pemalune.ir;
    
    # Redirect HTTP to HTTPS
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name pemalune.ir www.pemalune.ir;
    
    # SSL configuration (you need to add your SSL certificates)
    # ssl_certificate /path/to/your/certificate.crt;
    # ssl_certificate_key /path/to/your/private.key;
    
    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    
    # Frontend static files
    location / {
        root $DEPLOY_DIR/frontend;
        try_files \$uri \$uri/ /index.html;
        
        # Cache static assets
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|wasm)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
    }
    
    # API proxy to backend
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
    }
    
    # WebSocket support
    location /ws/ {
        proxy_pass http://127.0.0.1:8080/ws/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

    # Enable site
    ln -sf "/etc/nginx/sites-available/pema-platform" "/etc/nginx/sites-enabled/"
    
    # Test Nginx configuration
    if nginx -t; then
        systemctl reload nginx
        success "Nginx configured and reloaded"
    else
        error "Nginx configuration test failed"
    fi
}

# Health check
health_check() {
    log "Performing health check..."
    
    # Wait for service to be ready
    sleep 10
    
    # Check if service is running
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        success "✅ Service is running"
    else
        error "❌ Service is not running"
    fi
    
    # Check if port is listening
    if netstat -tuln | grep -q ":8080"; then
        success "✅ Port 8080 is listening"
    else
        error "❌ Port 8080 is not listening"
    fi
    
    # Check database connection
    if docker-compose -f "$DEPLOY_DIR/docker-compose.db.yml" ps | grep -q "Up"; then
        success "✅ Database is running"
    else
        error "❌ Database is not running"
    fi
}

# Main deployment process
main() {
    log "🚀 Starting $PLATFORM_NAME deployment..."
    
    check_root
    check_requirements
    create_backup
    stop_service
    deploy_application
    create_service
    start_service
    setup_nginx
    health_check
    
    success "🎉 $PLATFORM_NAME deployed successfully!"
    log "📊 Deployment Summary:"
    log "   - Platform: $DEPLOY_DIR/backend-server"
    log "   - Frontend: $DEPLOY_DIR/frontend"
    log "   - Service: $SERVICE_NAME"
    log "   - Logs: journalctl -u $SERVICE_NAME -f"
    log "   - Status: systemctl status $SERVICE_NAME"
}

# Run main function
main "$@"