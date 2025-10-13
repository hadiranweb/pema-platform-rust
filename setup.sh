#!/bin/bash

# PEMA Platform Setup Script
# This script sets up the complete PEMA platform with all dependencies and configurations

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Prompt user for input with default value
prompt_with_default() {
    local prompt="$1"
    local default="$2"
    local var_name="$3"
    
    echo -n "$prompt [$default]: "
    read -r input
    if [ -z "$input" ]; then
        eval "$var_name='$default'"
    else
        eval "$var_name='$input'"
    fi
}

# Prompt user for yes/no with default
prompt_yes_no() {
    local prompt="$1"
    local default="$2"
    local var_name="$3"
    
    while true; do
        echo -n "$prompt (y/n) [$default]: "
        read -r input
        if [ -z "$input" ]; then
            input="$default"
        fi
        case $input in
            [Yy]* ) eval "$var_name=true"; break;;
            [Nn]* ) eval "$var_name=false"; break;;
            * ) echo "Please answer yes (y) or no (n).";;
        esac
    done
}

# Main setup function
main() {
    log_info "🚀 Starting PEMA Platform Setup"
    echo "This script will help you set up the complete PEMA platform with all dependencies."
    echo ""

    # Check prerequisites
    log_info "📋 Checking prerequisites..."
    
    if ! command_exists "cargo"; then
        log_error "Rust/Cargo is not installed. Please install Rust first: https://rustup.rs/"
        exit 1
    fi
    
    if ! command_exists "node"; then
        log_warning "Node.js is not installed. Some frontend features may not work."
    fi
    
    if ! command_exists "docker"; then
        log_warning "Docker is not installed. Database setup will require manual PostgreSQL installation."
    fi

    log_success "Prerequisites check completed"
    echo ""

    # Configuration prompts
    log_info "⚙️  Configuration Setup"
    echo "Please provide the following configuration details:"
    echo ""

    # Server configuration
    log_info "🌐 Server Configuration"
    prompt_with_default "Server host" "0.0.0.0" "SERVER_HOST"
    prompt_with_default "Server port" "8080" "SERVER_PORT"
    prompt_with_default "Domain" "localhost" "DOMAIN"
    
    if [ "$DOMAIN" = "localhost" ]; then
        BASE_URL="http://localhost:$SERVER_PORT"
    else
        BASE_URL="https://$DOMAIN"
    fi
    
    echo ""

    # Database configuration
    log_info "🗄️  Database Configuration"
    prompt_yes_no "Use Docker for PostgreSQL database" "y" "USE_DOCKER_DB"
    
    if [ "$USE_DOCKER_DB" = true ]; then
        log_info "Docker database will be set up automatically"
        DB_HOST="localhost"
        DB_PORT="5432"
        DB_NAME="pema_db"
        DB_USER="pema_user"
        prompt_with_default "Database password" "secure_password_$(date +%s)" "DB_PASSWORD"
    else
        log_info "Manual database configuration"
        prompt_with_default "Database host" "localhost" "DB_HOST"
        prompt_with_default "Database port" "5432" "DB_PORT"
        prompt_with_default "Database name" "pema_db" "DB_NAME"
        prompt_with_default "Database user" "pema_user" "DB_USER"
        prompt_with_default "Database password" "" "DB_PASSWORD"
        
        if [ -z "$DB_PASSWORD" ]; then
            log_error "Database password is required"
            exit 1
        fi
    fi
    
    echo ""

    # Security configuration
    log_info "🔐 Security Configuration"
    prompt_with_default "JWT Secret (leave empty to generate)" "" "JWT_SECRET"
    
    if [ -z "$JWT_SECRET" ]; then
        JWT_SECRET=$(openssl rand -base64 64 | tr -d '\n' 2>/dev/null || head -c 64 /dev/urandom | base64 | tr -d '\n')
        log_info "Generated JWT secret"
    fi
    
    prompt_with_default "Session timeout (seconds)" "3600" "SESSION_TIMEOUT"
    
    echo ""

    # Environment configuration
    log_info "🌍 Environment Configuration"
    prompt_with_default "Environment (development/production)" "development" "ENVIRONMENT"
    
    if [ "$ENVIRONMENT" = "development" ]; then
        DEBUG="true"
        LOG_LEVEL="debug"
        CORS_ORIGINS="http://localhost:3000,http://localhost:8080"
    else
        DEBUG="false"
        LOG_LEVEL="info"
        CORS_ORIGINS="$BASE_URL"
    fi
    
    echo ""

    # Create .env file
    log_info "📝 Creating environment configuration..."
    
    cat > .env << EOF
# Server Configuration
SERVER_HOST=$SERVER_HOST
SERVER_PORT=$SERVER_PORT
DOMAIN=$DOMAIN
BASE_URL=$BASE_URL

# Database Configuration
DB_HOST=$DB_HOST
DB_PORT=$DB_PORT
DB_NAME=$DB_NAME
DB_USER=$DB_USER
DB_PASSWORD="$DB_PASSWORD"
DB_POOL_SIZE=10

# Application Configuration
APP_NAME="PEMA Platform"
ENVIRONMENT=$ENVIRONMENT
DEBUG=$DEBUG
LOG_LEVEL=$LOG_LEVEL

# Security Configuration
JWT_SECRET="$JWT_SECRET"
SESSION_TIMEOUT=$SESSION_TIMEOUT
CORS_ORIGINS="$CORS_ORIGINS"

# Database URL for SQLx
DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME

# SQLx Configuration
SQLX_OFFLINE=true

# Rust Configuration
RUST_LOG=$LOG_LEVEL
EOF

    log_success "Environment configuration created (.env)"
    echo ""

    # Database setup
    if [ "$USE_DOCKER_DB" = true ]; then
        log_info "🐳 Setting up PostgreSQL with Docker..."
        
        # Create docker-compose.yml for database
        cat > docker-compose.db.yml << EOF
version: '3.8'

services:
  postgres:
    image: postgres:15
    container_name: pema_postgres
    environment:
      POSTGRES_DB: $DB_NAME
      POSTGRES_USER: $DB_USER
      POSTGRES_PASSWORD: $DB_PASSWORD
    ports:
      - "$DB_PORT:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U $DB_USER -d $DB_NAME"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  postgres_data:
EOF

        log_info "Starting PostgreSQL container..."
        docker-compose -f docker-compose.db.yml up -d
        
        log_info "Waiting for database to be ready..."
        sleep 10
        
        # Wait for database to be ready
        for i in {1..30}; do
            if docker exec pema_postgres pg_isready -U "$DB_USER" -d "$DB_NAME" >/dev/null 2>&1; then
                log_success "Database is ready"
                break
            fi
            if [ $i -eq 30 ]; then
                log_error "Database failed to start within 5 minutes"
                exit 1
            fi
            sleep 10
        done
        
    else
        log_info "📋 Manual database setup required"
        echo "Please ensure PostgreSQL is installed and running with the following configuration:"
        echo "  Host: $DB_HOST"
        echo "  Port: $DB_PORT"
        echo "  Database: $DB_NAME"
        echo "  User: $DB_USER"
        echo "  Password: $DB_PASSWORD"
        echo ""
        echo "You can create the database and user with:"
        echo "  sudo -u postgres psql -c \"CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';\""
        echo "  sudo -u postgres psql -c \"CREATE DATABASE $DB_NAME OWNER $DB_USER;\""
        echo ""
        
        prompt_yes_no "Continue with setup (database ready)" "y" "DB_READY"
        if [ "$DB_READY" = false ]; then
            log_info "Setup paused. Please set up the database and run this script again."
            exit 0
        fi
    fi
    
    echo ""

    # Install Rust dependencies
    log_info "📦 Installing Rust dependencies..."
    
    # Install cargo tools if not present
    if ! command_exists "cargo-watch"; then
        log_info "Installing cargo-watch for development..."
        cargo install cargo-watch
    fi
    
    if ! command_exists "sqlx"; then
        log_info "Installing sqlx-cli for database migrations..."
        cargo install sqlx-cli --no-default-features --features postgres
    fi
    
    if ! command_exists "cargo-machete"; then
        log_info "Installing cargo-machete for unused dependency detection..."
        cargo install cargo-machete
    fi
    
    log_success "Rust tools installed"
    echo ""

    # Build the project
    log_info "🔨 Building the project..."
    
    # First, let's try to build without the problematic crates
    log_info "Building shared libraries..."
    cargo build --package models --package dtos --package shared-config --package pema_plugin_sdk
    
    log_info "Building backend-server..."
    cargo build --package pema-backend-server
    
    log_success "Project built successfully"
    echo ""

    # Database migrations
    log_info "🗄️  Setting up database schema..."
    
    # Create migrations directory if it doesn't exist
    mkdir -p migrations
    
    # Create initial migration if it doesn't exist
    if [ ! -f "migrations/001_initial.sql" ]; then
        log_info "Creating initial database schema..."
        cat > migrations/001_initial.sql << 'EOF'
-- Initial database schema for PEMA Platform

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    is_admin BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Vendors table
CREATE TABLE vendors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    contact_person VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    address TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Wallet status enum
CREATE TYPE wallet_status AS ENUM ('Active', 'Inactive', 'Suspended', 'Closed');

-- Wallets table
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    balance DECIMAL(15,2) DEFAULT 0.00,
    currency VARCHAR(3) DEFAULT 'USD',
    status wallet_status DEFAULT 'Active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Transaction type enum
CREATE TYPE transaction_type AS ENUM ('Deposit', 'Withdrawal', 'TransferIn', 'TransferOut', 'Purchase', 'Refund');

-- Transaction status enum
CREATE TYPE transaction_status AS ENUM ('Pending', 'Completed', 'Failed', 'Reversed');

-- Transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    status transaction_status DEFAULT 'Pending',
    description TEXT,
    reference_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- OTP codes table
CREATE TABLE otp_codes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code VARCHAR(6) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_transactions_wallet_id ON transactions(wallet_id);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_otp_codes_user_id ON otp_codes(user_id);
CREATE INDEX idx_otp_codes_expires_at ON otp_codes(expires_at);

-- Update triggers for updated_at columns
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vendors_updated_at BEFORE UPDATE ON vendors
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_wallets_updated_at BEFORE UPDATE ON wallets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
EOF
    fi
    
    # Run migrations
    log_info "Running database migrations..."
    if [ "$USE_DOCKER_DB" = true ]; then
        # Copy migration to container and run it
        docker exec -i pema_postgres psql -U "$DB_USER" -d "$DB_NAME" < migrations/001_initial.sql
    else
        # Run migration directly
        PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -f migrations/001_initial.sql
    fi
    
    log_success "Database schema created"
    echo ""

    # Create development scripts
    log_info "📜 Creating development scripts..."
    
    # Create start script
    cat > start-dev.sh << 'EOF'
#!/bin/bash

# Development startup script for PEMA Platform

echo "🚀 Starting PEMA Platform in development mode..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Start the backend server
echo "Starting backend server on $SERVER_HOST:$SERVER_PORT..."
cargo run --package pema-backend-server
EOF

    chmod +x start-dev.sh

    # Create database management script
    cat > manage-db.sh << 'EOF'
#!/bin/bash

# Database management script

case "$1" in
    start)
        echo "Starting database..."
        docker-compose -f docker-compose.db.yml up -d
        ;;
    stop)
        echo "Stopping database..."
        docker-compose -f docker-compose.db.yml down
        ;;
    restart)
        echo "Restarting database..."
        docker-compose -f docker-compose.db.yml restart
        ;;
    logs)
        echo "Showing database logs..."
        docker-compose -f docker-compose.db.yml logs -f
        ;;
    shell)
        echo "Opening database shell..."
        docker exec -it pema_postgres psql -U $DB_USER -d $DB_NAME
        ;;
    backup)
        echo "Creating database backup..."
        docker exec pema_postgres pg_dump -U $DB_USER $DB_NAME > backup_$(date +%Y%m%d_%H%M%S).sql
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|shell|backup}"
        exit 1
        ;;
esac
EOF

    chmod +x manage-db.sh

    # Create cleanup script
    cat > cleanup.sh << 'EOF'
#!/bin/bash

# Cleanup script for PEMA Platform

echo "🧹 Cleaning up PEMA Platform..."

# Clean Rust build artifacts
echo "Cleaning Rust build artifacts..."
cargo clean

# Stop and remove Docker containers
if [ -f docker-compose.db.yml ]; then
    echo "Stopping Docker containers..."
    docker-compose -f docker-compose.db.yml down -v
fi

# Remove generated files (but keep .env for reference)
echo "Removing generated files..."
rm -f docker-compose.db.yml
rm -f start-dev.sh
rm -f manage-db.sh

echo "✅ Cleanup completed"
EOF

    chmod +x cleanup.sh

    log_success "Development scripts created"
    echo ""

    # Final setup steps
    log_info "🎯 Final setup steps..."
    
    # Check for unused dependencies
    log_info "Checking for unused dependencies..."
    cargo machete --with-metadata || log_warning "Some unused dependencies detected. Run 'cargo machete' for details."
    
    echo ""

    # Setup complete
    log_success "🎉 PEMA Platform setup completed successfully!"
    echo ""
    echo "📋 Setup Summary:"
    echo "  • Environment: $ENVIRONMENT"
    echo "  • Server: $BASE_URL"
    echo "  • Database: PostgreSQL ($DB_HOST:$DB_PORT/$DB_NAME)"
    echo "  • Docker DB: $USE_DOCKER_DB"
    echo ""
    echo "🚀 Next Steps:"
    echo "  1. Start the platform: ./start-dev.sh"
    echo "  2. Access the API at: $BASE_URL"
    echo "  3. Manage database: ./manage-db.sh {start|stop|logs|shell}"
    echo "  4. View logs: cargo run --package pema-backend-server"
    echo ""
    echo "📚 Additional Commands:"
    echo "  • Run tests: cargo test"
    echo "  • Watch for changes: cargo watch -x run"
    echo "  • Check unused deps: cargo machete"
    echo "  • Database shell: ./manage-db.sh shell"
    echo "  • Create backup: ./manage-db.sh backup"
    echo ""
    echo "🔧 Configuration files created:"
    echo "  • .env (environment variables)"
    echo "  • docker-compose.db.yml (database setup)"
    echo "  • migrations/001_initial.sql (database schema)"
    echo "  • start-dev.sh (development startup)"
    echo "  • manage-db.sh (database management)"
    echo "  • cleanup.sh (cleanup script)"
    echo ""
    
    if [ "$ENVIRONMENT" = "production" ]; then
        log_warning "⚠️  Production Environment Notes:"
        echo "  • Ensure firewall is properly configured"
        echo "  • Use a reverse proxy (nginx/apache) for HTTPS"
        echo "  • Regularly backup your database"
        echo "  • Monitor logs and system resources"
        echo "  • Keep dependencies updated"
    fi
    
    echo ""
    log_info "Happy coding! 🎉"
}

# Run main function
main "$@"