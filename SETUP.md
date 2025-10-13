# PEMA Platform Setup Guide

This guide will help you set up the PEMA Platform with all its dependencies and configurations.

## Quick Start

1. **Run the setup script:**
   ```bash
   ./setup.sh
   ```

2. **Follow the interactive prompts** to configure:
   - Server settings (host, port, domain)
   - Database configuration (Docker or manual PostgreSQL)
   - Security settings (JWT secret, session timeout)
   - Environment settings (development/production)

3. **Start the platform:**
   ```bash
   ./start-dev.sh
   ```

## Prerequisites

- **Rust** (latest stable) - Install from [rustup.rs](https://rustup.rs/)
- **Docker** (optional, for database) - Install from [docker.com](https://www.docker.com/)
- **PostgreSQL** (if not using Docker) - Install from [postgresql.org](https://www.postgresql.org/)
- **Node.js** (optional, for frontend development)

## Setup Options

### Database Setup

The setup script offers two database options:

#### Option 1: Docker PostgreSQL (Recommended)
- ✅ Automatic setup and configuration
- ✅ Isolated environment
- ✅ Easy backup and restore
- ✅ Consistent across different systems

#### Option 2: Manual PostgreSQL
- ⚙️ Requires manual PostgreSQL installation
- ⚙️ Manual database and user creation
- ⚙️ System-specific configuration

### Environment Configuration

#### Development Environment
- Debug logging enabled
- CORS configured for local development
- Detailed error messages
- Hot reloading support

#### Production Environment
- Optimized logging
- Secure CORS configuration
- Error message sanitization
- Performance optimizations

## Configuration Files

After running the setup script, you'll have:

### `.env` - Environment Variables
Contains all configuration settings:
- Server configuration
- Database connection details
- Security settings
- Application settings

### `docker-compose.db.yml` - Database Container
Docker Compose configuration for PostgreSQL database.

### `migrations/001_initial.sql` - Database Schema
Initial database schema with:
- Users and authentication
- Wallets and transactions
- Vendors and products
- Indexes and triggers

## Development Scripts

### `start-dev.sh` - Start Development Server
```bash
./start-dev.sh
```
Starts the backend server in development mode.

### `manage-db.sh` - Database Management
```bash
./manage-db.sh start    # Start database
./manage-db.sh stop     # Stop database
./manage-db.sh restart  # Restart database
./manage-db.sh logs     # View database logs
./manage-db.sh shell    # Open database shell
./manage-db.sh backup   # Create database backup
```

### `cleanup.sh` - Clean Up
```bash
./cleanup.sh
```
Removes build artifacts and Docker containers.

## Manual Setup (Alternative)

If you prefer manual setup or need to customize the process:

### 1. Environment Configuration
Copy and modify the example environment file:
```bash
cp .env.example .env
# Edit .env with your settings
```

### 2. Database Setup

#### With Docker:
```bash
docker run --name pema_postgres \
  -e POSTGRES_DB=pema_db \
  -e POSTGRES_USER=pema_user \
  -e POSTGRES_PASSWORD=your_password \
  -p 5432:5432 \
  -d postgres:15
```

#### Manual PostgreSQL:
```bash
sudo -u postgres psql -c "CREATE USER pema_user WITH PASSWORD 'your_password';"
sudo -u postgres psql -c "CREATE DATABASE pema_db OWNER pema_user;"
```

### 3. Install Dependencies
```bash
# Install Rust tools
cargo install sqlx-cli --no-default-features --features postgres
cargo install cargo-watch
cargo install cargo-machete

# Build the project
cargo build --workspace --exclude frontend
```

### 4. Database Migration
```bash
# Run the initial migration
psql -h localhost -U pema_user -d pema_db -f migrations/001_initial.sql
```

### 5. Start the Server
```bash
cargo run --package pema-backend-server
```

## Troubleshooting

### Common Issues

#### Database Connection Failed
- Ensure PostgreSQL is running
- Check database credentials in `.env`
- Verify database exists and user has permissions

#### Compilation Errors
- Update Rust: `rustup update`
- Clean build cache: `cargo clean`
- Check for missing dependencies

#### Port Already in Use
- Change `SERVER_PORT` in `.env`
- Kill existing processes: `lsof -ti:8080 | xargs kill`

#### Docker Issues
- Ensure Docker is running
- Check container status: `docker ps`
- View container logs: `docker logs pema_postgres`

### SQLx Compile-Time Verification

The project uses SQLx with compile-time verification. If you encounter SQLx errors:

1. **Ensure database is running** and accessible
2. **Set offline mode** (already configured):
   ```bash
   export SQLX_OFFLINE=true
   ```
3. **Prepare query cache** (when database is ready):
   ```bash
   cargo sqlx prepare
   ```

### Performance Optimization

For production deployments:

1. **Build with optimizations:**
   ```bash
   cargo build --release --workspace --exclude frontend
   ```

2. **Configure reverse proxy** (nginx example):
   ```nginx
   server {
       listen 80;
       server_name your-domain.com;
       
       location / {
           proxy_pass http://127.0.0.1:8080;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
   }
   ```

3. **Set up SSL/TLS** with Let's Encrypt or your certificate provider

4. **Configure monitoring** and logging

## Development Workflow

### Daily Development
```bash
# Start database
./manage-db.sh start

# Start development server with hot reload
cargo watch -x "run --package pema-backend-server"

# In another terminal, run tests
cargo test

# Check for unused dependencies
cargo machete
```

### Before Committing
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run all tests
cargo test --workspace

# Check for unused dependencies
cargo machete
```

## API Documentation

Once the server is running, you can access:

- **Health Check:** `GET http://localhost:8080/health`
- **API Documentation:** Available in the code comments and handlers

## Support

For issues and questions:

1. Check this setup guide
2. Review the troubleshooting section
3. Check the project's issue tracker
4. Review Rust and PostgreSQL documentation

## Security Notes

### Development
- Default passwords are generated automatically
- CORS is configured for local development
- Debug logging may expose sensitive information

### Production
- Use strong, unique passwords
- Configure CORS for your specific domain
- Use HTTPS with proper SSL certificates
- Regularly update dependencies
- Monitor logs for security issues
- Set up proper firewall rules

## Next Steps

After successful setup:

1. **Explore the API** endpoints
2. **Run the test suite** to ensure everything works
3. **Review the code structure** to understand the architecture
4. **Start developing** your features
5. **Set up CI/CD** for automated testing and deployment

Happy coding! 🚀