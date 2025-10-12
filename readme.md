## PEMA Platform Setup Guide

This guide provides instructions for setting up the PEMA Platform on your server. It includes an automated setup script for server configurations and detailed steps for building and running the application.

### 1. Project Architecture Overview

Understanding the project structure is crucial for a smooth setup:

*   **`backend-server/`**: This is the **main traditional Rust backend** (Actix-web server). It is responsible for loading the application configuration, serving the main API endpoints, and integrating with the WASM backend libraries.
*   **`wasm-auth-backend/`**: This is a **WASM library** for authentication logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`wasm-general-backend/`**: This is a **WASM library** for general business logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`wasm-frontend/`**: This is the Yew-based WebAssembly (WASM) frontend application.
*   **`shared/`**: Contains shared modules, including the `config` module used by backend components.

### 2. Recent Changes and Fixes

This section summarizes recent structural and configuration improvements:

*   **Project Cleanup**: Removed numerous unnecessary files and directories, including old installation scripts, temporary build outputs, unused backend components (`auth-server`), forked dependencies (`ahash_fork`, `tempfile_fork`), and internal documentation/planning documents. This streamlines the project structure and reduces clutter.
*   **Configuration Management**: Standardized configuration loading for the `backend-server` by implementing a shared `AppConfig` module in `shared/config`. This ensures consistent handling of database and server settings across the application. The `AppConfig` now includes a `SecurityConfig` for `jwt_secret` and `session_timeout`.
*   **JWT and Type Handling Consistency**: Addressed inconsistencies in JWT (JSON Web Token) library usage and `user_id` type handling. The project now consistently uses the `jwt` crate with `hmac` and `sha2` for token generation and validation. `user_id` is primarily handled as `Uuid` in backend logic, with necessary conversions to `String` for JWT claims and WASM-bound functions to maintain compatibility.
*   **Dependency and Build Fixes**: Resolved various compilation errors related to `wasm-bindgen` and `jsonwebtoken` dependencies in `wasm-auth-backend`, `wasm-general-backend`, and `backend-server`. This included updating dependency versions and correcting `JsValue` handling in backend routes.
*   **New Wallet Creation Feature**: Implemented a comprehensive wallet creation feature, including:
    *   **Database Schema**: Added `wallets` and `transactions` tables, along with `WALLET_STATUS` and `TRANSACTION_TYPE` ENUMs, via a new migration file (`20251011000000_create_wallet_schema.sql`).
    *   **WASM General Backend**: Integrated wallet creation logic into `wasm-general-backend/src/service.rs` and exposed it via `create_new_wallet` function in `wasm-general-backend/src/lib.rs`.
    *   **Backend Server API**: Created a new API endpoint (`/wallet`) in `backend-server/src/wallet/routes.rs` and `backend-server/src/wallet/handlers.rs` that leverages the WASM general backend to create new wallets.
    *   **WASM Frontend Integration**: Added a wallet creation form to `wasm-frontend/src/pages/profile.rs` and corresponding models in `wasm-frontend/src/models/wallet.rs`, allowing users to create new wallets through the UI.

# 🚀 Deployment Guide

This guide provides step-by-step instructions for deploying the PEMA Platform on a Linux-based server (e.g., Ubuntu).

### **1. System Prerequisites**

Ensure the following prerequisites are installed on your server:

*   **Rust and Cargo:** For compiling the Rust projects.
    ```bash
    curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```
*   **Node.js and npm/yarn:** For frontend tooling.
    ```bash
    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
    sudo apt-get install -y nodejs
    ```
*   **Trunk:** The build tool for the Yew WASM frontend.
    ```bash
    cargo install trunk
    ```
*   **Docker and Docker Compose (Recommended):** For containerized deployment.
    ```bash
    # Follow the official Docker installation guide for your distribution.
    ```
*   **PostgreSQL Client:** For database interaction.
    ```bash
    sudo apt-get install -y postgresql-client
    ```
*   **Nginx:** As a web server and reverse proxy.
    ```bash
    sudo apt-get install -y nginx
    ```

### **2. Cloning the Repository**

Clone the repository onto your server:

```bash
cd /opt # Or any other suitable directory
git clone <URL_TO_YOUR_REPOSITORY>
cd pema-platform-rust
```

### **3. Environment Configuration**

Copy the example `.env` files and configure them with your environment-specific values:

```bash
cp .env.example .env
cp backend/.env.example backend/.env
```

**Key `.env` variables:**

*   `DATABASE_URL`: Connection string for your PostgreSQL database.
*   `JWT_SECRET`: A strong, secret key for JWT signing.
*   `FRONTEND_URL`: The public URL of your frontend.
*   `BACKEND_URL`: The public URL of your backend API.

### **4. Database Setup**

Create a PostgreSQL database and user, then run the migrations:

```bash
# Connect to PostgreSQL
sudo -u postgres psql

# In the psql shell:
CREATE USER pema_user WITH PASSWORD \'your_secure_password\';
CREATE DATABASE pema_db OWNER pema_user;
\q

# Install sqlx-cli and run migrations
cargo install sqlx-cli --no-default-features --features "postgres,runtime-tokio-rustls"
sqlx migrate run
```

### **5. Build and Deploy Backend**

Compile the backend for production:

```bash
cd backend
cargo build --release
```

The executable will be located at `target/release/backend`. It is recommended to run this as a `systemd` service for process management.

**Example `systemd` service file (`/etc/systemd/system/pema-backend.service`):**

```ini
[Unit]
Description=PEMA Platform Backend Service
After=network.target

[Service]
User=www-data
WorkingDirectory=/opt/pema-platform-rust/backend
EnvironmentFile=/opt/pema-platform-rust/backend/.env
ExecStart=/opt/pema-platform-rust/backend/target/release/backend
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Enable and start the service:**

```bash
sudo systemctl daemon-reload
sudo systemctl enable pema-backend
sudo systemctl start pema-backend
```

### **6. Build and Deploy Frontend**

Build the Yew WASM frontend using Trunk:

```bash
cd ../frontend
trunk build --release
```

The static files will be generated in the `dist` directory.

### **7. Configure Nginx**

Configure Nginx to serve the frontend\'s static files and act as a reverse proxy for the backend API. Create a new Nginx site configuration file (e.g., `/etc/nginx/sites-available/pema-platform`):

```nginx
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;

    # Redirect HTTP to HTTPS
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name yourdomain.com www.yourdomain.com;

    ssl_certificate /path/to/your/fullchain.pem;
    ssl_certificate_key /path/to/your/privkey.pem;

    location / {
        root /opt/pema-platform-rust/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://127.0.0.1:8000; # Backend address
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Enable the site and restart Nginx:**

```bash
sudo ln -s /etc/nginx/sites-available/pema-platform /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### **8. Monitoring**

Monitor the backend service and Nginx logs to ensure everything is running correctly:

```bash
# Backend logs
sudo journalctl -u pema-backend.service -f

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

### **Alternative: Docker Compose Deployment**

For a simplified deployment, you can use the provided `docker-compose.yml` file. Ensure Docker and Docker Compose are installed, then run:

```bash
docker compose build
docker compose up -d
```
This will build and run all services (database, backend, frontend, and Nginx) in a containerized environment.

