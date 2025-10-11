## PEMA Platform Setup Guide

This guide provides instructions for setting up the PEMA Platform on your server. It includes an automated setup script for server configurations and detailed steps for building and running the application.

### 1. Project Architecture Overview

Understanding the project structure is crucial for a smooth setup:

*   **`backend-server/`**: This is the **main traditional Rust backend** (Actix-web server). It is responsible for loading the application configuration, serving the main API endpoints, and integrating with the WASM backend libraries.
*   **`wasm-auth-backend/`**: This is a **WASM library** for authentication logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`wasm-general-backend/`**: This is a **WASM library** for general business logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`wasm-frontend/`**: This is the Yew-based WebAssembly (WASM) frontend application.
*   **`shared/`**: Contains shared modules, including the `config` module used by backend components.

### 2. Initial Server Setup with `setup_server.sh`

To automate the installation of Nginx, PostgreSQL, Certbot, and UFW, and to configure them for your domain `pemalune.ir` and server IP `37.32.4.142`, use the provided `setup_server.sh` script.

**Before running the script:**
*   Ensure your server is a fresh Ubuntu/Debian installation.
*   Make sure your domain `pemalune.ir` is pointing to your server's IP `37.32.4.142`.
*   **Important:** Edit the `setup_server.sh` script to replace `your_email@example.com` with your actual email address for Certbot registration.

1.  **Clone the project (if you haven't already):**

    ```bash
    git clone https://github.com/hadiranweb/pema-platform-rust.git
    cd pema-platform-rust
    ```

2.  **Make the setup script executable and run it:**

    ```bash
    chmod +x setup_server.sh
    sudo ./setup_server.sh
    ```

    This script will:
    *   Update system packages.
    *   Install and configure Nginx.
    *   Install and configure PostgreSQL, creating the `pema_user` and `pema_db` with password `F8s77@98`.
*   Apply database migrations to initialize the database schema.
    *   Install Certbot and obtain SSL certificates for `pemalune.ir` and `www.pemalune.ir`.
    *   Configure UFW (firewall) to allow SSH and Nginx traffic.
    *   Create and configure the `backend-server/.env` file with `SERVER_HOST=127.0.0.1`, `SERVER_PORT=8080`, `DOMAIN=pemalune.ir`, `BASE_URL=https://pemalune.ir`, and the database credentials. It will also generate a random `JWT_SECRET`.

### 3. Rust Toolchain and Project Build

After the server setup is complete, install the Rust toolchain and build the project components.

1.  **Install Rust Toolchain (if not already installed by the script):**

    ```bash
    curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    rustup target add wasm32-unknown-unknown
    cargo install trunk
    ```

2.  **Build All Components:**

    From the repository root (`/home/ubuntu/pema-platform-rust/`), use the `Makefile` to build all parts of the application:

    ```bash
    make all
    ```

    This command will:
    *   Build `wasm-auth-backend` (WASM library).
    *   Build `wasm-general-backend` (WASM library).
    *   Build `wasm-frontend` (generating static files in `wasm-frontend/dist/`).
    *   Build `backend-server` (executable in `backend-server/target/release/pema-backend`).

### 4. Systemd Service Setup for `backend-server`

To ensure your main `backend-server` runs continuously and automatically, set it up as a `systemd` service.

1.  **Create a systemd service file:**

    ```bash
    sudo nano /etc/systemd/system/pema-backend.service
    ```

2.  **Add the following content:**

    Copy and paste the content below into the `pema-backend.service` file. **Do not change anything.**

    ```ini
    [Unit]
    Description=PEMA Platform Backend Server
    After=network.target postgresql.service

    [Service]
    Type=simple
    User=ubuntu # This uses your current user. Adjust if you have a dedicated user.
    WorkingDirectory=/home/ubuntu/pema-platform-rust/backend-server
    ExecStart=/home/ubuntu/pema-platform-rust/backend-server/target/release/pema-backend
    Restart=always
    RestartSec=10
    Environment="RUST_LOG=info"

    [Install]
    WantedBy=multi-user.target
    ```

3.  **Reload systemd, enable, and start the service:**

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl enable pema-backend
    sudo systemctl start pema-backend
    ```

4.  **Check service status and logs:**

    ```bash
    sudo systemctl status pema-backend
    journalctl -u pema-backend -f # View live logs
    ```

### 5. Troubleshooting

*   **Nginx `502 Bad Gateway`**: This usually means Nginx cannot connect to your backend server. Check:
    *   Is `pema-backend.service` running (`sudo systemctl status pema-backend`)?
    *   Are there any firewall rules blocking Nginx from connecting to the backend? (The `setup_server.sh` script configures UFW to allow Nginx traffic).
*   **File Permissions**: Ensure the user running the `pema-backend.service` has read/write permissions to necessary directories (e.g., `wasm-frontend/dist`).
*   **Logs**: Always check `journalctl -u <service_name> -f` for detailed error messages from your services.

### 6. Recent Changes and Fixes

This section summarizes recent structural and configuration improvements:

*   **Project Cleanup**: Removed numerous unnecessary files and directories, including old installation scripts, temporary build outputs, unused backend components (`auth-server`), forked dependencies (`ahash_fork`, `tempfile_fork`), and internal documentation/planning documents. This streamlines the project structure and reduces clutter.
*   **Configuration Management**: Standardized configuration loading for the `backend-server` by implementing a shared `AppConfig` module in `shared/config`. This ensures consistent handling of database and server settings across the application. The `AppConfig` now includes a `SecurityConfig` for `jwt_secret` and `session_timeout`.
*   **JWT and Type Handling Consistency**: Addressed inconsistencies in JWT (JSON Web Token) library usage and `user_id` type handling. The project now consistently uses the `jwt` crate with `hmac` and `sha2` for token generation and validation. `user_id` is primarily handled as `Uuid` in backend logic, with necessary conversions to `String` for JWT claims and WASM-bound functions to maintain compatibility.
*   **Dependency and Build Fixes**: Resolved various compilation errors related to `wasm-bindgen` and `jsonwebtoken` dependencies in `wasm-auth-backend`, `wasm-general-backend`, and `backend-server`. This included updating dependency versions and correcting `JsValue` handling in backend routes.

### Conclusion

By following this guide and using the `setup_server.sh` script, your PEMA Platform will be fully installed and configured. Nginx serves your frontend efficiently and securely via `https://pemalune.ir`, acting as a reverse proxy to your `backend-server` which listens internally. Mobile applications can connect to `https://pemalune.ir/api/` and `https://pemalune.ir/auth/`.

Your PEMA Platform is now ready for production use!

