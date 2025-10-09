#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Configuration Variables ---
DOMAIN="pemalune.ir"
SERVER_IP="37.32.4.142"
DB_USER="pema_user"
DB_PASSWORD="F8s77@98"
DB_NAME="pema_db"
BACKEND_PORT="8080"
PROJECT_ROOT="/home/ubuntu/pema-platform-rust"

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

# 2. Install Nginx
log_info "Installing Nginx..."
sudo apt install nginx -y
sudo systemctl start nginx
sudo systemctl enable nginx
log_success "Nginx installed and started."

# 3. Install PostgreSQL
log_info "Installing PostgreSQL..."
sudo apt install postgresql postgresql-contrib -y
sudo systemctl start postgresql
sudo systemctl enable postgresql
log_success "PostgreSQL installed and started."

log_info "Configuring PostgreSQL database and user..."
sudo -i -u postgres psql <<EOF
CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}';
CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};
EOF
log_success "PostgreSQL database and user configured."

# 4. Install Certbot and obtain SSL certificates
log_info "Installing Certbot and obtaining SSL certificates..."
sudo apt install snapd -y
sudo snap install core || log_error "Failed to install snap core."
sudo snap refresh core
sudo apt remove certbot -y || true # Remove old certbot if exists
sudo snap install --classic certbot || log_error "Failed to install certbot snap."
sudo ln -s /snap/bin/certbot /usr/bin/certbot || log_error "Failed to create certbot symlink."

# Ensure Nginx config for domain exists before running certbot
log_info "Creating a temporary Nginx server block for Certbot validation..."
sudo bash -c "cat > /etc/nginx/sites-available/${DOMAIN}.conf <<EOL
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN} www.${DOMAIN};

    location / {
        root /var/www/html;
    }
}
EOL"
sudo ln -s /etc/nginx/sites-available/${DOMAIN}.conf /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx

log_info "Running Certbot to obtain SSL certificates. Follow prompts if any..."
sudo certbot --nginx -d ${DOMAIN} -d www.${DOMAIN} --non-interactive --agree-tos --email your_email@example.com || log_error "Certbot failed to obtain certificates. Check DNS and Nginx config."
log_success "SSL certificates obtained and Nginx configured for HTTPS."

# 5. Configure Nginx for PEMA Platform
log_info "Configuring Nginx for PEMA Platform..."
sudo bash -c "cat > /etc/nginx/sites-available/${DOMAIN}.conf <<EOL
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN} www.${DOMAIN};

    # Redirect HTTP to HTTPS
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name ${DOMAIN} www.${DOMAIN};

    # SSL configuration - Certbot will manage these paths
    ssl_certificate /etc/letsencrypt/live/${DOMAIN}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/${DOMAIN}/privkey.pem;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384';
    ssl_prefer_server_ciphers on;

    # Root for static frontend files
    root ${PROJECT_ROOT}/wasm-frontend/dist;
    index index.html;

    # Serve static files directly
    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # Proxy API requests to the main backend-server
    location /api/ {
        proxy_pass http://127.0.0.1:${BACKEND_PORT};
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

    # Proxy Auth requests to the main backend-server
    location /auth/ {
        proxy_pass http://127.0.0.1:${BACKEND_PORT};
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

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

# 6. Configure UFW (Uncomplicated Firewall)
log_info "Configuring UFW firewall..."
sudo ufw allow OpenSSH
sudo ufw allow 'Nginx Full'
sudo ufw --force enable
log_success "UFW configured and enabled."

# 7. Configure backend-server/.env
log_info "Configuring backend-server/.env file..."
cat > ${PROJECT_ROOT}/backend-server/.env <<EOL
SERVER_HOST=127.0.0.1
SERVER_PORT=${BACKEND_PORT}
DOMAIN=${DOMAIN}
BASE_URL=https://${DOMAIN}
DB_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}
JWT_SECRET=$(head /dev/urandom | tr -dc A-Za-z0-9_ | head -c 64)
EOL
log_success "backend-server/.env configured."

log_success "PEMA Platform server setup completed successfully!"
log_info "You can now build and run the PEMA Platform by navigating to ${PROJECT_ROOT} and running 'make all' followed by systemd service setup."

