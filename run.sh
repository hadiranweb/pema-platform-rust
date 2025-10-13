#!/bin/bash

# PEMA Platform Run Script
# Builds and runs the complete platform

set -e

echo "🚀 Starting PEMA Platform..."

# Load environment
source $HOME/.cargo/env 2>/dev/null || true

# Check if setup was run
if [ ! -f ".env" ]; then
    echo "❌ Setup not completed. Run './setup.sh' first"
    exit 1
fi

# Load configuration
source .env

echo "🔧 Building services..."

# Build auth server
echo "📊 Building auth server..."
cd auth-server
cargo build --release
cd ..

# Build backend server
echo "🔧 Building backend server..."
cd backend-server

# Run database migrations
echo "📊 Running database migrations..."
sqlx migrate run || echo "⚠️ Migration failed, continuing..."

cargo build --release
cd ..

echo "✅ Services built successfully"

# Start auth service
echo "🚀 Starting auth service..."
sudo tee /etc/systemd/system/pema-auth.service > /dev/null << EOF
[Unit]
Description=PEMA Platform Auth Server
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=www-data
WorkingDirectory=$(pwd)/auth-server
Environment=RUST_LOG=info
ExecStart=$(pwd)/auth-server/target/release/auth-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# Start backend service
echo "🚀 Starting backend service..."
sudo tee /etc/systemd/system/pema-backend.service > /dev/null << EOF
[Unit]
Description=PEMA Platform Backend Server
After=network.target postgresql.service pema-auth.service
Requires=postgresql.service
Wants=pema-auth.service

[Service]
Type=simple
User=www-data
WorkingDirectory=$(pwd)/backend-server
Environment=RUST_LOG=info
ExecStart=$(pwd)/backend-server/target/release/pema-backend-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable pema-auth pema-backend
sudo systemctl restart pema-auth
sleep 2
sudo systemctl restart pema-backend

echo "✅ Services started"

# Build WASM frontend
echo "🎨 Building WASM frontend..."
cd wasm-frontend

# Build frontend with Trunk
trunk build --release

echo "✅ WASM Frontend built successfully"

# Deploy frontend
echo "📦 Deploying frontend..."
sudo mkdir -p /var/www/pema-frontend
sudo cp -r dist/* /var/www/pema-frontend/
sudo chown -R www-data:www-data /var/www/pema-frontend

echo "✅ Frontend deployed"

# Restart Nginx
echo "🌐 Restarting Nginx..."
sudo systemctl restart nginx

# Check services
echo "🔍 Checking services..."
sleep 3

if systemctl is-active --quiet pema-auth; then
    echo "✅ Auth service is running"
else
    echo "❌ Auth service failed to start"
    sudo journalctl -u pema-auth --no-pager -n 10
fi

if systemctl is-active --quiet pema-backend; then
    echo "✅ Backend service is running"
else
    echo "❌ Backend service failed to start"
    sudo journalctl -u pema-backend --no-pager -n 10
fi

if systemctl is-active --quiet nginx; then
    echo "✅ Nginx is running"
else
    echo "❌ Nginx failed to start"
    sudo nginx -t
fi

if systemctl is-active --quiet postgresql; then
    echo "✅ PostgreSQL is running"
else
    echo "❌ PostgreSQL is not running"
fi

echo ""
echo "🎉 PEMA Platform is running!"
echo ""
echo "📋 Service Status:"
echo "  • Auth Server: http://localhost:8081"
echo "  • Backend Server: http://localhost:8080"
echo "  • Frontend: https://${DOMAIN}"
echo "  • Database: PostgreSQL (${DB_NAME})"
echo ""
echo "🔧 Useful Commands:"
echo "  • Auth logs: sudo journalctl -u pema-auth -f"
echo "  • Backend logs: sudo journalctl -u pema-backend -f"
echo "  • Restart services: sudo systemctl restart pema-auth pema-backend"
echo "  • Nginx logs: sudo tail -f /var/log/nginx/error.log"
echo "  • Test Auth API: curl http://localhost:8081/health"
echo "  • Test Backend API: curl http://localhost:8080/health"
echo ""
echo "🌐 Access your platform at: https://${DOMAIN}"