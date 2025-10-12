# PEMA Platform Setup and Deployment Guide

This guide provides comprehensive instructions for setting up, deploying, and running the PEMA Platform. The architecture is designed to be modular, with a separate, dedicated service for authentication.

### 1. Project Architecture Overview

*   **`backend/`**: The main backend server (Actix-web) responsible for core business logic, product management, orders, etc.
*   **`auth-server/`**: A **separate, standalone authentication server** responsible for user registration, login, and JWT token management. **This is the default and recommended setup.**
*   **`frontend/`**: The Yew-based WebAssembly (WASM) frontend application.
*   **`shared/`**: Shared Rust crates for models, DTOs, and configuration.
*   **`migrations/`**: SQL database migrations.

### 2. Environment Configuration

Correct environment configuration is critical. The project uses separate `.env` files for each service.

**A. Main Backend (`backend/.env`)**

Copy the example file and configure it:

```bash
cp backend/.env.example backend/.env
```

**Key variables for `backend/.env`:**

*   `DATABASE_URL`: The connection string for your PostgreSQL database.
*   `SERVER_PORT`: The port the main backend will run on (e.g., `8000`).
*   `AUTH_SERVER_URL`: **Crucially, this must point to your running authentication service** (e.g., `http://127.0.0.1:8081`). The main backend delegates authentication tasks to this service.

**B. Authentication Server (`auth-server/.env`)**

Copy the example file and configure it:

```bash
cp auth-server/.env.example auth-server/.env
```

**Key variables for `auth-server/.env`:**

*   `DATABASE_URL`: The same database connection string used by the main backend.
*   `SERVER_PORT`: The port the authentication server will run on (e.g., `8081`). This must be different from the main backend's port.
*   `JWT_SECRET`: A strong, unique secret key used to sign JWTs. **This should not be shared with the main backend.**
*   `FRONTEND_URL`: The public URL of your frontend, for CORS configuration.

### 3. Deployment Steps

**Step 1: Prerequisites**

Ensure `Rust`, `Cargo`, `Node.js`, `Trunk`, `PostgreSQL`, and `Nginx` are installed as detailed in the previous guide.

**Step 2: Clone Repository & Setup Database**

Clone the repository and run the database migrations as previously described.

**Step 3: Build Both Services**

Build both the main backend and the authentication server in release mode:

```bash
# Build the main backend
cargo build --release --manifest-path backend/Cargo.toml

# Build the authentication server
cargo build --release --manifest-path auth-server/Cargo.toml
```

**Step 4: Run Services with `systemd`**

Create two separate `systemd` services to manage both processes.

**A. Main Backend Service (`/etc/systemd/system/pema-backend.service`)**

```ini
[Unit]
Description=PEMA Platform - Main Backend
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/opt/pema-platform-rust/backend
EnvironmentFile=/opt/pema-platform-rust/backend/.env
ExecStart=/opt/pema-platform-rust/target/release/backend
Restart=always

[Install]
WantedBy=multi-user.target
```

**B. Authentication Service (`/etc/systemd/system/pema-auth.service`)**

```ini
[Unit]
Description=PEMA Platform - Auth Server
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/opt/pema-platform-rust/auth-server
EnvironmentFile=/opt/pema-platform-rust/auth-server/.env
ExecStart=/opt/pema-platform-rust/target/release/auth-server
Restart=always

[Install]
WantedBy=multi-user.target
```

**Enable and start both services:**

```bash
sudo systemctl daemon-reload
sudo systemctl enable pema-backend pema-auth
sudo systemctl start pema-backend pema-auth
```

**Step 5: Build Frontend**

Build the frontend as previously described:

```bash
cd frontend
trunk build --release
```

**Step 6: Update Nginx Configuration**

Your Nginx configuration must be updated to route requests to the correct service. Requests to `/api/` go to the main backend, and requests to `/auth/` go to the authentication server.

```nginx
server {
    listen 443 ssl;
    server_name yourdomain.com;

    # SSL Config...
    ssl_certificate /path/to/your/fullchain.pem;
    ssl_certificate_key /path/to/your/privkey.pem;

    # Frontend Files
    location / {
        root /opt/pema-platform-rust/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # Route to Main Backend
    location /api/ {
        proxy_pass http://127.0.0.1:8000; # Port from backend/.env
        proxy_set_header Host $host;
        # Other headers...
    }

    # Route to Authentication Server
    location /auth/ {
        proxy_pass http://127.0.0.1:8081; # Port from auth-server/.env
        proxy_set_header Host $host;
        # Other headers...
    }
}
```

**Restart Nginx:**

```bash
sudo systemctl restart nginx
```

This corrected setup ensures proper separation of concerns, with a dedicated authentication service as the default, and provides clear instructions for environment configuration and deployment.
