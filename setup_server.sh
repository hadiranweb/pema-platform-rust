#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Configuration Variables ---
DOMAIN="pemalune.ir"
SERVER_IP="37.32.4.142"
DB_USER="pema_user"
DB_PASSWORD="F8s77@98"
DB_NAME="pema_db"
AUTH_BACKEND_PORT="8082"
GENERAL_BACKEND_PORT="8081"
PROJECT_ROOT="/home/ubuntu/pema-platform-rust"
USER="ubuntu"
FRONTEND_DIR="${PROJECT_ROOT}/wasm-frontend"
FRONTEND_DIST_DIR="/var/www/pemalune.ir"
CERTBOT_EMAIL="hadiranweb@gmail.com"

# --- Functions ---
log_info() {
    echo "[INFO] $1"
}

log_success() {
    echo "[SUCCESS] $1"
}

log_error() {
    echo "[ERROR] $1"
    exit 1
}

# --- Main Script ---
log_info "Starting PEMA Platform server setup..."

# 1. Update System
log_info "Updating system packages..."
sudo apt update
sudo apt upgrade -y
log_success "System packages updated."

# 2. Install Required Packages
log_info "Installing required packages..."
sudo apt install nginx postgresql postgresql-contrib snapd curl build-essential pkg-config libssl-dev -y
log_success "Required packages installed."

# 3. Install Rust and WASM tools
log_info "Installing Rust and WASM tools..."
curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
log_success "Rust and WASM tools installed."

# 4. Install Trunk for WASM frontend
log_info "Installing Trunk..."
cargo install trunk
log_success "Trunk installed."

# 5. Install Node.js and npm for frontend dependencies (if needed)
log_info "Installing Node.js..."
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
log_success "Node.js installed."

# 6. Start and enable services
log_info "Starting services..."
sudo systemctl start nginx
sudo systemctl enable nginx
sudo systemctl start postgresql
sudo systemctl enable postgresql
log_success "Services started and enabled."

# 7. Configure PostgreSQL
log_info "Configuring PostgreSQL database and user..."
sudo -i -u postgres psql <<EOF
CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}';
CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};
GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};
\c ${DB_NAME};
GRANT ALL ON SCHEMA public TO ${DB_USER};
EOF

log_info "Applying database migrations..."
# For SQLx, typically you would run `sqlx migrate run` from the backend-server directory.
# However, since this is a setup script, we\'ll execute the SQL directly.
# This assumes migration files are applied in order.

# Apply the initial wallet schema migration
if [ -f "${PROJECT_ROOT}/backend-server/migrations/20251008234714_create_wallet_schema.sql" ]; then
    log_info "Applying 20251008234714_create_wallet_schema.sql..."
    sudo -i -u postgres psql -d ${DB_NAME} -f "${PROJECT_ROOT}/backend-server/migrations/20251008234714_create_wallet_schema.sql" || log_error "Failed to apply 20251008234714_create_wallet_schema.sql"
    log_success "20251008234714_create_wallet_schema.sql applied."
else
    log_info "Migration file 20251008234714_create_wallet_schema.sql not found. Skipping."
fi

# Apply the new wallet schema migration
if [ -f "${PROJECT_ROOT}/backend-server/migrations/20251011000000_create_wallet_schema.sql" ]; then
    log_info "Applying 20251011000000_create_wallet_schema.sql..."
    sudo -i -u postgres psql -d ${DB_NAME} -f "${PROJECT_ROOT}/backend-server/migrations/20251011000000_create_wallet_schema.sql" || log_error "Failed to apply 20251011000000_create_wallet_schema.sql"
    log_success "20251011000000_create_wallet_schema.sql applied."
else
    log_error "New wallet schema migration file 20251011000000_create_wallet_schema.sql not found. This is unexpected as it should have been created."
fi
log_success "PostgreSQL database and user configured with migrations."


# 8. Create frontend distribution directory
log_info "Setting up frontend directories..."
sudo mkdir -p ${FRONTEND_DIST_DIR}
sudo chown -R ${USER}:${USER} ${FRONTEND_DIST_DIR}
sudo chown -R ${USER}:${USER} ${PROJECT_ROOT}
log_success "Frontend directories configured."

# 9. Install Certbot and obtain SSL certificates
log_info "Installing Certbot and obtaining SSL certificates..."
sudo snap install core
sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot

# Create temporary Nginx config for Certbot
log_info "Creating temporary Nginx configuration for Certbot..."
sudo bash -c "cat > /etc/nginx/sites-available/${DOMAIN}.conf <<EOL
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN} www.${DOMAIN};
    root ${FRONTEND_DIST_DIR};
    index index.html;

    location / {
        try_files \$uri \$uri/ /index.html;
    }
}
EOL"

sudo ln -sf /etc/nginx/sites-available/${DOMAIN}.conf /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default
sudo nginx -t && sudo systemctl reload nginx

log_info "Running Certbot to obtain SSL certificates..."
sudo certbot --nginx -d ${DOMAIN} -d www.${DOMAIN} --non-interactive --agree-tos --email ${CERTBOT_EMAIL} || log_error "Certbot failed to obtain certificates."
log_success "SSL certificates obtained."

# 10. Configure Nginx for PEMA Platform
log_info "Configuring Nginx for PEMA Platform..."
sudo bash -c "cat > /etc/nginx/sites-available/${DOMAIN}.conf <<EOL
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN} www.${DOMAIN};
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name ${DOMAIN} www.${DOMAIN};

    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/${DOMAIN}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/${DOMAIN}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    # Frontend - serve from wasm-frontend/dist
    root ${FRONTEND_DIST_DIR};
    index index.html;

    # Serve static files and handle SPA routing
    location / {
        try_files \$uri \$uri/ /index.html;
        add_header Cache-Control \"no-cache, no-store, must-revalidate\";
        add_header Pragma \"no-cache\";
        add_header Expires \"0\";
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control \"public, immutable\";
        try_files \$uri =404;
    }

    # API routes - proxy to backend servers
    location /api/auth/ {
        proxy_pass http://127.0.0.1:${AUTH_BACKEND_PORT}/api/auth/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_redirect off;
        
        # CORS headers
        add_header Access-Control-Allow-Origin \"https://${DOMAIN}\" always;
        add_header Access-Control-Allow-Methods \"GET, POST, PUT, DELETE, OPTIONS\" always;
        add_header Access-Control-Allow-Headers \"Authorization, Content-Type\" always;
        add_header Access-Control-Allow-Credentials \"true\" always;
        
        if (\$request_method = OPTIONS) {
            return 204;
        }
    }

    location /api/general/ {
        proxy_pass http://127.0.0.1:${GENERAL_BACKEND_PORT}/api/general/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_redirect off;
        
        # CORS headers
        add_header Access-Control-Allow-Origin \"https://${DOMAIN}\" always;
        add_header Access-Control-Allow-Methods \"GET, POST, PUT, DELETE, OPTIONS\" always;
        add_header Access-Control-Allow-Headers \"Authorization, Content-Type\" always;
        add_header Access-Control-Allow-Credentials \"true\" always;
        
        if (\$request_method = OPTIONS) {
            return 204;
        }
    }

    # Security headers
    add_header X-Frame-Options \"SAMEORIGIN\" always;
    add_header X-Content-Type-Options \"nosniff\" always;
    add_header X-XSS-Protection \"1; mode=block\" always;
    add_header Referrer-Policy \"strict-origin-when-cross-origin\" always;

    # Error pages
    error_page 404 /404.html;
    location = /404.html {
        internal;
    }

    error_page 500 502 503 504 /50x.html;
    location = /50x.html {
        internal;
    }
}
EOL"

sudo nginx -t && sudo systemctl reload nginx
log_success "Nginx configured for PEMA Platform."

# 11. Configure environment files
log_info "Configuring environment files..."

# Generate a random JWT secret
JWT_SECRET=$(openssl rand -base64 32)

# .env (for general backend-server configuration if needed, using API port as default)
cat > ${PROJECT_ROOT}/.env <<EOL
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}
JWT_SECRET=${JWT_SECRET}
SERVER_HOST=0.0.0.0
SERVER_PORT=${GENERAL_BACKEND_PORT}
DOMAIN=${DOMAIN}
EOL
log_success ".env configured."

# .env.auth
cat > ${PROJECT_ROOT}/.env.auth <<EOL
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}
JWT_SECRET=${JWT_SECRET}
SERVER_HOST=0.0.0.0
SERVER_PORT=${AUTH_BACKEND_PORT}
DOMAIN=${DOMAIN}
EOL
log_success ".env.auth configured."

# .env.api
cat > ${PROJECT_ROOT}/.env.api <<EOL
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}
JWT_SECRET=${JWT_SECRET}
SERVER_HOST=0.0.0.0
SERVER_PORT=${GENERAL_BACKEND_PORT}
DOMAIN=${DOMAIN}
EOL
log_success ".env.api configured."

# Frontend environment (if needed)
cat > ${FRONTEND_DIR}/.env <<EOL
VITE_API_BASE_URL=https://${DOMAIN}/api
VITE_AUTH_BASE_URL=https://${DOMAIN}/api/auth
VITE_APP_NAME=PEMA Platform
VITE_APP_VERSION=1.0.0
EOL
log_success "Frontend environment configured."

# 12. Configure systemd services
log_info "Configuring systemd services..."

# Backend server service
sudo bash -c "cat > /etc/systemd/system/pema-backend-server.service <<EOL
[Unit]
Description=PEMA Backend Server
After=network.target postgresql.service

[Service]
Type=simple
User=${USER}
Group=${USER}
WorkingDirectory=${PROJECT_ROOT}/backend-server
EnvironmentFile=${PROJECT_ROOT}/.env
ExecStart=${PROJECT_ROOT}/target/release/pema-backend-server
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOL"

# Frontend service (optional - if you want to serve via trunk in production)
sudo bash -c "cat > /etc/systemd/system/pema-frontend.service <<EOL
[Unit]
Description=PEMA Frontend Server
After=network.target

[Service]
Type=simple
User=${USER}
Group=${USER}
WorkingDirectory=${FRONTEND_DIR}
ExecStart=/home/${USER}/.cargo/bin/trunk serve --port 3000 --dist-dir ${FRONTEND_DIST_DIR}
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOL"

sudo systemctl daemon-reload
log_success "Systemd services configured."

# 13. Configure UFW firewall
log_info "Configuring UFW firewall..."
sudo ufw allow OpenSSH
sudo ufw allow 'Nginx Full'
sudo ufw --force enable
log_success "UFW configured and enabled."

# 14. Build and deploy frontend
log_info "Building and deploying frontend..."
cd ${PROJECT_ROOT}

# Build the project
log_info "Building the project..."
make clean
make all

# Deploy frontend to Nginx directory
log_info "Deploying frontend to Nginx..."
make deploy-frontend

log_success "Frontend built and deployed."

# 15. Create deploy script
log_info "Creating deployment script..."
cat > ${PROJECT_ROOT}/deploy.sh <<'EOL'
#!/bin/bash
set -e

echo "Starting deployment..."

cd /home/ubuntu/pema-platform-rust

# Pull latest changes (if using git)
# git pull origin main

# Build the project
make clean
make all

# Deploy frontend
make deploy-frontend

# Restart backend service
sudo systemctl restart pema-backend-server.service

# Reload nginx
sudo systemctl reload nginx

echo "Deployment completed successfully!"
EOL

chmod +x ${PROJECT_ROOT}/deploy.sh
log_success "Deployment script created."

# 16. Start backend service
log_info "Starting backend service..."
sudo systemctl enable pema-backend-server.service
sudo systemctl start pema-backend-server.service
log_success "Backend service started."

# 17. Set up SSL auto-renewal
log_info "Setting up SSL certificate auto-renewal..."
sudo crontab -l | { cat; echo "0 12 * * * /usr/bin/certbot renew --quiet"; } | sudo crontab -
log_success "SSL auto-renewal configured."

log_success "PEMA Platform server setup completed successfully!"
echo ""
log_info "=== SETUP COMPLETE ==="
log_info "Frontend URL: https://${DOMAIN}"
log_info "Backend API: https://${DOMAIN}/api"
log_info "Auth API: https://${DOMAIN}/api/auth"
echo ""
log_info "Next steps:"
log_info "1. Check backend status: sudo systemctl status pema-backend-server.service"
log_info "2. Check Nginx status: sudo systemctl status nginx"
log_info "3. View backend logs: sudo journalctl -u pema-backend-server.service -f"
log_info "4. Test the website: curl -I https://${DOMAIN}"
echo ""
log_info "For future deployments, run: ${PROJECT_ROOT}/deploy.sh"

