#!/bin/bash

# PEMA Platform Setup Script
# Server: 37.32.4.142 | Domain: pemalune.ir

set -e

SERVER_IP="37.32.4.142"
DOMAIN="pemalune.ir"
DB_NAME="pema_platform"
DB_USER="pema_user"
DB_PASSWORD="pema_$(date +%s)"

echo "🚀 Setting up PEMA Platform..."

# Install prerequisites
echo "📦 Installing prerequisites..."
sudo apt update -y
sudo apt install -y curl wget git build-essential pkg-config libssl-dev postgresql postgresql-contrib nginx certbot python3-certbot-nginx

# Install Rust
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    rustup target add wasm32-unknown-unknown
fi

# Install Trunk for frontend
if ! command -v trunk &> /dev/null; then
    cargo install trunk
fi

# Setup PostgreSQL
echo "🗄️ Setting up database..."
sudo systemctl start postgresql
sudo systemctl enable postgresql

sudo -u postgres psql << EOF
CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}';
CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};
GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};
\q
EOF

# Create environment files
echo "⚙️ Creating configuration..."

# Backend server environment
cat > backend-server/.env << EOF
DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}
JWT_SECRET=$(openssl rand -hex 32)
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
DOMAIN=${DOMAIN}
BASE_URL=https://${DOMAIN}
CORS_ALLOWED_ORIGINS=https://${DOMAIN}
AUTH_SERVER_URL=http://127.0.0.1:8081
EOF

# Auth server environment
cat > auth-server/.env << EOF
DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}
JWT_SECRET=$(openssl rand -hex 32)
SERVER_HOST=0.0.0.0
SERVER_PORT=8081
RUST_LOG=info
DOMAIN=${DOMAIN}
EOF

# WASM Frontend environment
cat > wasm-frontend/.env << EOF
VITE_API_BASE_URL=https://${DOMAIN}/api
VITE_AUTH_URL=https://${DOMAIN}/auth
VITE_WS_URL=wss://${DOMAIN}/ws
VITE_DOMAIN=${DOMAIN}
EOF

# Setup Nginx
echo "🌐 Configuring Nginx..."
sudo tee /etc/nginx/sites-available/${DOMAIN} > /dev/null << 'EOF'
server {
    listen 80;
    server_name 37.32.4.142 pemalune.ir www.pemalune.ir;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name pemalune.ir www.pemalune.ir;
    
    ssl_certificate /etc/letsencrypt/live/pemalune.ir/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/pemalune.ir/privkey.pem;
    
    # Frontend
    location / {
        root /var/www/pema-frontend;
        try_files $uri $uri/ /index.html;
    }
    
    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # Auth API
    location /auth/ {
        proxy_pass http://127.0.0.1:8081/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # WebSocket
    location /ws/ {
        proxy_pass http://127.0.0.1:8080/ws/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
EOF

sudo ln -sf /etc/nginx/sites-available/${DOMAIN} /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default
sudo nginx -t
sudo systemctl enable nginx

# Setup SSL
echo "🔒 Setting up SSL..."
sudo certbot --nginx -d ${DOMAIN} --non-interactive --agree-tos --email admin@${DOMAIN} || echo "SSL setup failed"

# Setup firewall
echo "🔥 Configuring firewall..."
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw --force enable

echo "✅ Setup complete!"
echo "Database: ${DB_NAME} (user: ${DB_USER})"
echo "Domain: https://${DOMAIN}"
echo "Run './run.sh' to start the platform"