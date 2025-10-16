#!/bin/bash

# PEMA Platform Deployment Script
# نسخه: 1.0.0
# توضیحات: اسکریپت جامع برای نصب و راه‌اندازی پلتفرم PEMA

set -e  # خروج در صورت خطا

# رنگ‌ها برای نمایش بهتر
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# متغیرهای سراسری
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_NAME="pema-platform"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="/var/log/pema_deployment_${TIMESTAMP}.log"

# تابع لاگ‌گذاری
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}" | tee -a "$LOG_FILE"
    exit 1
}

warning() {
    echo -e "${YELLOW}[WARNING] $1${NC}" | tee -a "$LOG_FILE"
}

info() {
    echo -e "${BLUE}[INFO] $1${NC}" | tee -a "$LOG_FILE"
}

# بررسی دسترسی root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "این اسکریپت باید با دسترسی root اجرا شود. لطفاً از sudo استفاده کنید."
    fi
}

# تشخیص سیستم عامل
detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS=$NAME
        VER=$VERSION_ID
    else
        error "نمی‌توان سیستم عامل را تشخیص داد"
    fi
    log "سیستم عامل تشخیص داده شده: $OS $VER"
}

# دریافت اطلاعات از کاربر
get_user_input() {
    log "دریافت اطلاعات پیکربندی از کاربر..."
    
    # نام پلتفرم
    read -p "نام پلتفرم (پیش‌فرض: PEMA): " PLATFORM_NAME
    PLATFORM_NAME=${PLATFORM_NAME:-"PEMA"}
    
    # دامنه سایت
    read -p "دامنه سایت (مثال: example.com): " DOMAIN_NAME
    if [[ -z "$DOMAIN_NAME" ]]; then
        error "دامنه سایت الزامی است"
    fi
    
    # ایمیل ادمین
    read -p "ایمیل ادمین: " ADMIN_EMAIL
    if [[ -z "$ADMIN_EMAIL" ]]; then
        error "ایمیل ادمین الزامی است"
    fi
    
    # رمز عبور دیتابیس
    read -s -p "رمز عبور دیتابیس (خودکار تولید می‌شود اگر خالی باشد): " DB_PASSWORD
    echo
    if [[ -z "$DB_PASSWORD" ]]; then
        DB_PASSWORD=$(openssl rand -base64 32)
        log "رمز عبور دیتابیس خودکار تولید شد"
    fi
    
    # JWT Secret
    read -s -p "JWT Secret (خودکار تولید می‌شود اگر خالی باشد): " JWT_SECRET
    echo
    if [[ -z "$JWT_SECRET" ]]; then
        JWT_SECRET=$(openssl rand -base64 64)
        log "JWT Secret خودکار تولید شد"
    fi
    
    # پورت‌ها
    read -p "پورت backend (پیش‌فرض: 8081): " BACKEND_PORT
    BACKEND_PORT=${BACKEND_PORT:-8081}
    
    read -p "پورت frontend (پیش‌فرض: 3000): " FRONTEND_PORT
    FRONTEND_PORT=${FRONTEND_PORT:-3000}
    
    # تأیید اطلاعات
    echo
    info "اطلاعات وارد شده:"
    echo "نام پلتفرم: $PLATFORM_NAME"
    echo "دامنه: $DOMAIN_NAME"
    echo "ایمیل ادمین: $ADMIN_EMAIL"
    echo "پورت Backend: $BACKEND_PORT"
    echo "پورت Frontend: $FRONTEND_PORT"
    echo
    
    read -p "آیا اطلاعات صحیح است؟ (y/N): " CONFIRM
    if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
        error "عملیات لغو شد"
    fi
}

# تشخیص آدرس IP سرور
detect_server_info() {
    log "تشخیص اطلاعات سرور..."
    
    # آدرس IP عمومی
    PUBLIC_IP=$(curl -s ifconfig.me || curl -s ipinfo.io/ip || echo "unknown")
    
    # آدرس IP محلی
    LOCAL_IP=$(hostname -I | awk '{print $1}')
    
    # نام میزبان
    HOSTNAME=$(hostname)
    
    log "آدرس IP عمومی: $PUBLIC_IP"
    log "آدرس IP محلی: $LOCAL_IP"
    log "نام میزبان: $HOSTNAME"
}

# نصب وابستگی‌های سیستم
install_system_dependencies() {
    log "نصب وابستگی‌های سیستم..."
    
    if [[ "$OS" == *"Ubuntu"* ]] || [[ "$OS" == *"Debian"* ]]; then
        apt update
        apt install -y curl wget git build-essential pkg-config libssl-dev \
                      postgresql postgresql-contrib nginx certbot python3-certbot-nginx \
                      ufw fail2ban htop tree jq unzip
    elif [[ "$OS" == *"CentOS"* ]] || [[ "$OS" == *"Red Hat"* ]]; then
        yum update -y
        yum groupinstall -y "Development Tools"
        yum install -y curl wget git openssl-devel postgresql-server postgresql-contrib \
                      nginx certbot python3-certbot-nginx firewalld htop tree jq unzip
    else
        error "سیستم عامل پشتیبانی نمی‌شود: $OS"
    fi
}

# نصب Rust
install_rust() {
    log "بررسی و نصب Rust..."
    
    if ! command -v rustc &> /dev/null; then
        log "نصب Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        rustup default stable
        rustup component add clippy rustfmt
    else
        log "Rust از قبل نصب است"
        rustc --version
    fi
    
    # بررسی نسخه
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    log "نسخه Rust: $RUST_VERSION"
}

# نصب Node.js
install_nodejs() {
    log "بررسی و نصب Node.js..."
    
    if ! command -v node &> /dev/null; then
        log "نصب Node.js..."
        curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
        apt-get install -y nodejs
    else
        log "Node.js از قبل نصب است"
        node --version
        npm --version
    fi
}

# پیکربندی PostgreSQL
setup_postgresql() {
    log "پیکربندی PostgreSQL..."
    
    # شروع سرویس PostgreSQL
    systemctl start postgresql
    systemctl enable postgresql
    
    # بررسی وضعیت
    if ! systemctl is-active --quiet postgresql; then
        error "PostgreSQL راه‌اندازی نشد"
    fi
    
    # حذف دیتابیس و کاربر قبلی (اگر وجود دارد)
    sudo -u postgres psql -c "DROP DATABASE IF EXISTS pema_db;" || true
    sudo -u postgres psql -c "DROP USER IF EXISTS pema_user;" || true
    
    # ایجاد کاربر و دیتابیس جدید
    sudo -u postgres psql -c "CREATE USER pema_user WITH PASSWORD '$DB_PASSWORD';"
    sudo -u postgres psql -c "CREATE DATABASE pema_db OWNER pema_user;"
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE pema_db TO pema_user;"
    
    # تست اتصال
    if PGPASSWORD="$DB_PASSWORD" psql -h localhost -U pema_user -d pema_db -c "SELECT 1;" &> /dev/null; then
        log "اتصال به دیتابیس با موفقیت تست شد"
    else
        error "خطا در اتصال به دیتابیس"
    fi
}

# ایجاد فایل‌های محیطی
create_env_files() {
    log "ایجاد فایل‌های محیطی..."
    
    # فایل .env اصلی
    cat > "$SCRIPT_DIR/.env" << EOF
# PEMA Platform Configuration
# Generated on: $(date)

# Database Configuration
DATABASE_URL=postgres://pema_user:${DB_PASSWORD}@localhost:5432/pema_db

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=${BACKEND_PORT}
FRONTEND_PORT=${FRONTEND_PORT}

# Security
JWT_SECRET=${JWT_SECRET}
BCRYPT_COST=12

# Platform Information
PLATFORM_NAME=${PLATFORM_NAME}
DOMAIN_NAME=${DOMAIN_NAME}
ADMIN_EMAIL=${ADMIN_EMAIL}

# Server Information
PUBLIC_IP=${PUBLIC_IP}
LOCAL_IP=${LOCAL_IP}
HOSTNAME=${HOSTNAME}

# SSL Configuration
SSL_ENABLED=true
CERT_PATH=/etc/letsencrypt/live/${DOMAIN_NAME}/fullchain.pem
KEY_PATH=/etc/letsencrypt/live/${DOMAIN_NAME}/privkey.pem

# Email Configuration (SMTP)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=${ADMIN_EMAIL}
SMTP_PASSWORD=your_smtp_password_here

# SMS Configuration
SMS_PROVIDER=kavenegar
SMS_API_KEY=your_sms_api_key_here

# Redis Configuration (for caching and sessions)
REDIS_URL=redis://localhost:6379

# Logging
LOG_LEVEL=info
LOG_FILE=/var/log/pema/app.log

# Development/Production
ENVIRONMENT=production
DEBUG=false
EOF

    # کپی برای backend-server
    cp "$SCRIPT_DIR/.env" "$SCRIPT_DIR/backend-server/.env"
    
    # فایل .env برای frontend
    cat > "$SCRIPT_DIR/frontend/.env" << EOF
# Frontend Configuration
VITE_API_BASE_URL=https://${DOMAIN_NAME}/api
VITE_WS_URL=wss://${DOMAIN_NAME}/ws
VITE_PLATFORM_NAME=${PLATFORM_NAME}
VITE_DOMAIN=${DOMAIN_NAME}
EOF

    log "فایل‌های محیطی ایجاد شدند"
}

# تمیزکاری پورت‌ها
cleanup_ports() {
    log "تمیزکاری پورت‌های در حال استفاده..."
    
    # لیست پورت‌های مورد نیاز
    REQUIRED_PORTS=($BACKEND_PORT $FRONTEND_PORT 80 443 5432)
    
    for port in "${REQUIRED_PORTS[@]}"; do
        # پیدا کردن فرآیندهای در حال استفاده از پورت
        PIDS=$(lsof -ti:$port 2>/dev/null || true)
        
        if [[ -n "$PIDS" ]]; then
            warning "پورت $port در حال استفاده است. متوقف کردن فرآیندها..."
            echo "$PIDS" | xargs -r kill -9
            sleep 2
        fi
    done
    
    log "تمیزکاری پورت‌ها کامل شد"
}

# پیکربندی Nginx
setup_nginx() {
    log "پیکربندی Nginx..."
    
    # متوقف کردن Nginx
    systemctl stop nginx 2>/dev/null || true
    
    # حذف پیکربندی‌های قبلی
    rm -f /etc/nginx/sites-enabled/default
    rm -f /etc/nginx/sites-enabled/$DOMAIN_NAME
    rm -f /etc/nginx/sites-available/$DOMAIN_NAME
    
    # ایجاد پیکربندی جدید
    cat > "/etc/nginx/sites-available/$DOMAIN_NAME" << EOF
# PEMA Platform Nginx Configuration
# Generated on: $(date)

# Rate limiting
limit_req_zone \$binary_remote_addr zone=api:10m rate=10r/s;
limit_req_zone \$binary_remote_addr zone=login:10m rate=5r/m;

# Upstream servers
upstream backend {
    server 127.0.0.1:${BACKEND_PORT};
    keepalive 32;
}

upstream frontend {
    server 127.0.0.1:${FRONTEND_PORT};
    keepalive 32;
}

# HTTP to HTTPS redirect
server {
    listen 80;
    server_name ${DOMAIN_NAME} www.${DOMAIN_NAME};
    
    # Let's Encrypt challenge
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }
    
    # Redirect all other traffic to HTTPS
    location / {
        return 301 https://\$server_name\$request_uri;
    }
}

# HTTPS server
server {
    listen 443 ssl http2;
    server_name ${DOMAIN_NAME} www.${DOMAIN_NAME};
    
    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/${DOMAIN_NAME}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/${DOMAIN_NAME}/privkey.pem;
    ssl_session_timeout 1d;
    ssl_session_cache shared:MozTLS:10m;
    ssl_session_tickets off;
    
    # Modern configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    
    # HSTS
    add_header Strict-Transport-Security "max-age=63072000" always;
    
    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Referrer-Policy "strict-origin-when-cross-origin";
    
    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json;
    
    # API routes
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        
        proxy_pass http://backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
        
        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
    
    # Auth routes with stricter rate limiting
    location /api/auth/ {
        limit_req zone=login burst=5 nodelay;
        
        proxy_pass http://backend;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
    
    # WebSocket support
    location /ws/ {
        proxy_pass http://backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
    
    # Static files
    location /static/ {
        alias /var/www/${DOMAIN_NAME}/static/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
    
    # Frontend application
    location / {
        proxy_pass http://frontend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
        
        # Handle client-side routing
        try_files \$uri \$uri/ @fallback;
    }
    
    location @fallback {
        proxy_pass http://frontend;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
    
    # Health check
    location /health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
EOF

    # فعال‌سازی سایت
    ln -sf "/etc/nginx/sites-available/$DOMAIN_NAME" "/etc/nginx/sites-enabled/"
    
    # تست پیکربندی
    if nginx -t; then
        log "پیکربندی Nginx معتبر است"
    else
        error "خطا در پیکربندی Nginx"
    fi
}

# دریافت گواهی SSL
setup_ssl() {
    log "دریافت گواهی SSL..."
    
    # ایجاد دایرکتوری برای Let's Encrypt
    mkdir -p /var/www/html
    
    # شروع Nginx برای تأیید دامنه
    systemctl start nginx
    
    # دریافت گواهی
    if certbot --nginx -d "$DOMAIN_NAME" -d "www.$DOMAIN_NAME" --non-interactive --agree-tos --email "$ADMIN_EMAIL"; then
        log "گواهی SSL با موفقیت دریافت شد"
    else
        warning "خطا در دریافت گواهی SSL. ادامه بدون SSL..."
        # تغییر پیکربندی برای HTTP
        sed -i 's/listen 443 ssl http2;/listen 80;/' "/etc/nginx/sites-available/$DOMAIN_NAME"
        sed -i '/ssl_/d' "/etc/nginx/sites-available/$DOMAIN_NAME"
    fi
    
    # راه‌اندازی تجدید خودکار
    (crontab -l 2>/dev/null; echo "0 12 * * * /usr/bin/certbot renew --quiet") | crontab -
}

# کامپایل پروژه
build_project() {
    log "کامپایل پروژه..."
    
    cd "$SCRIPT_DIR"
    
    # بررسی وجود Cargo.toml
    if [[ ! -f "Cargo.toml" ]]; then
        error "فایل Cargo.toml یافت نشد"
    fi
    
    # پاک‌سازی build قبلی
    cargo clean
    
    # کامپایل در حالت production
    log "کامپایل backend..."
    cargo build --release --bin backend-server
    
    # کامپایل frontend
    log "کامپایل frontend..."
    cd frontend
    if [[ -f "package.json" ]]; then
        npm install
        npm run build
    else
        # اگر frontend Rust است
        cd ..
        cargo build --release --bin frontend
    fi
    
    cd "$SCRIPT_DIR"
    log "کامپایل کامل شد"
}

# ایجاد سرویس‌های systemd
create_systemd_services() {
    log "ایجاد سرویس‌های systemd..."
    
    # سرویس backend
    cat > "/etc/systemd/system/pema-backend.service" << EOF
[Unit]
Description=PEMA Platform Backend
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=${SCRIPT_DIR}
Environment=RUST_LOG=info
EnvironmentFile=${SCRIPT_DIR}/.env
ExecStart=${SCRIPT_DIR}/target/release/backend-server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=${SCRIPT_DIR} /var/log/pema /tmp

[Install]
WantedBy=multi-user.target
EOF

    # سرویس frontend (اگر جداگانه باشد)
    if [[ -f "${SCRIPT_DIR}/target/release/frontend" ]]; then
        cat > "/etc/systemd/system/pema-frontend.service" << EOF
[Unit]
Description=PEMA Platform Frontend
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=${SCRIPT_DIR}
EnvironmentFile=${SCRIPT_DIR}/frontend/.env
ExecStart=${SCRIPT_DIR}/target/release/frontend
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
    fi
    
    # بارگذاری مجدد systemd
    systemctl daemon-reload
    
    log "سرویس‌های systemd ایجاد شدند"
}

# پیکربندی فایروال
setup_firewall() {
    log "پیکربندی فایروال..."
    
    if command -v ufw &> /dev/null; then
        # Ubuntu/Debian
        ufw --force reset
        ufw default deny incoming
        ufw default allow outgoing
        
        # پورت‌های ضروری
        ufw allow ssh
        ufw allow 80/tcp
        ufw allow 443/tcp
        
        # فعال‌سازی
        ufw --force enable
        
    elif command -v firewall-cmd &> /dev/null; then
        # CentOS/RHEL
        systemctl start firewalld
        systemctl enable firewalld
        
        firewall-cmd --permanent --add-service=ssh
        firewall-cmd --permanent --add-service=http
        firewall-cmd --permanent --add-service=https
        firewall-cmd --reload
    fi
    
    log "فایروال پیکربندی شد"
}

# اجرای migration های دیتابیس
run_migrations() {
    log "اجرای migration های دیتابیس..."
    
    cd "$SCRIPT_DIR"
    
    # نصب sqlx-cli اگر وجود ندارد
    if ! command -v sqlx &> /dev/null; then
        cargo install sqlx-cli --no-default-features --features postgres
    fi
    
    # اجرای migration ها
    if [[ -d "migrations" ]]; then
        sqlx migrate run --database-url "postgres://pema_user:${DB_PASSWORD}@localhost:5432/pema_db"
        log "Migration ها با موفقیت اجرا شدند"
    else
        warning "پوشه migrations یافت نشد"
    fi
}

# شروع سرویس‌ها
start_services() {
    log "شروع سرویس‌ها..."
    
    # شروع و فعال‌سازی سرویس‌ها
    systemctl enable pema-backend
    systemctl start pema-backend
    
    if [[ -f "/etc/systemd/system/pema-frontend.service" ]]; then
        systemctl enable pema-frontend
        systemctl start pema-frontend
    fi
    
    # راه‌اندازی مجدد Nginx
    systemctl restart nginx
    systemctl enable nginx
    
    # بررسی وضعیت سرویس‌ها
    sleep 5
    
    if systemctl is-active --quiet pema-backend; then
        log "سرویس backend با موفقیت راه‌اندازی شد"
    else
        error "خطا در راه‌اندازی سرویس backend"
    fi
    
    if systemctl is-active --quiet nginx; then
        log "سرویس Nginx با موفقیت راه‌اندازی شد"
    else
        error "خطا در راه‌اندازی سرویس Nginx"
    fi
}

# تست نهایی
final_test() {
    log "تست نهایی سیستم..."
    
    # تست اتصال به backend
    if curl -f -s "http://localhost:${BACKEND_PORT}/health" > /dev/null; then
        log "Backend در دسترس است"
    else
        warning "Backend در دسترس نیست"
    fi
    
    # تست اتصال از طریق Nginx
    if curl -f -s "http://localhost/health" > /dev/null; then
        log "Nginx به درستی کار می‌کند"
    else
        warning "مشکل در پیکربندی Nginx"
    fi
    
    # نمایش اطلاعات نهایی
    echo
    log "=========================================="
    log "نصب با موفقیت کامل شد!"
    log "=========================================="
    echo
    info "اطلاعات دسترسی:"
    echo "URL سایت: https://$DOMAIN_NAME"
    echo "پنل ادمین: https://$DOMAIN_NAME/admin"
    echo "API: https://$DOMAIN_NAME/api"
    echo
    info "اطلاعات دیتابیس:"
    echo "نام دیتابیس: pema_db"
    echo "کاربر: pema_user"
    echo "رمز عبور: $DB_PASSWORD"
    echo
    info "فایل‌های مهم:"
    echo "تنظیمات: $SCRIPT_DIR/.env"
    echo "لاگ‌ها: $LOG_FILE"
    echo "Nginx config: /etc/nginx/sites-available/$DOMAIN_NAME"
    echo
    info "دستورات مفید:"
    echo "مشاهده لاگ backend: journalctl -u pema-backend -f"
    echo "مشاهده لاگ nginx: tail -f /var/log/nginx/error.log"
    echo "راه‌اندازی مجدد: systemctl restart pema-backend"
    echo
    log "=========================================="
}

# تابع اصلی
main() {
    log "شروع نصب پلتفرم PEMA..."
    
    # بررسی‌های اولیه
    check_root
    detect_os
    
    # دریافت اطلاعات
    get_user_input
    detect_server_info
    
    # نصب وابستگی‌ها
    install_system_dependencies
    install_rust
    install_nodejs
    
    # پیکربندی دیتابیس
    setup_postgresql
    
    # ایجاد فایل‌های محیطی
    create_env_files
    
    # تمیزکاری و پیکربندی
    cleanup_ports
    setup_nginx
    setup_ssl
    
    # کامپایل و راه‌اندازی
    build_project
    run_migrations
    create_systemd_services
    setup_firewall
    
    # شروع سرویس‌ها
    start_services
    
    # تست نهایی
    final_test
    
    log "نصب کامل شد!"
}

# اجرای اسکریپت
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi