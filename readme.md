## PEMA Platform Installation Guide (Simplified with Makefile and Systemd)

This comprehensive guide provides step-by-step instructions for installing the PEMA Platform on your server, leveraging the `Makefile` for automation and `systemd` for service management. This assumes you have already cloned the repository to your server.

### 1. Project Architecture Overview

Understanding the project structure is crucial for a smooth installation:

*   **`backend-server/`**: This is the **main traditional Rust backend** (Actix-web server). It is responsible for loading the application configuration, serving the main API endpoints, and integrating with the WASM backend libraries.
*   **`backends/installer/`**: This is a **standalone Actix-web server** specifically designed for the initial web-based installation process. It generates the `config.toml` file.
*   **`backends/wasm-auth-backend/`**: This is a **WASM library** for authentication logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`backends/wasm-general-backend/`**: This is a **WASM library** for general business logic. It is *not* a standalone server but is integrated into the `backend-server/`.
*   **`wasm-frontend/`**: This is the Yew-based WebAssembly (WASM) frontend application.
*   **`shared/`**: Contains shared modules, including the `config` module used by backend components.

### 2. Server Prerequisites

Ensure your server is prepared with the necessary software and dependencies.

#### 2.1. System Updates

Keep your server's operating system up-to-date:

*   **Ubuntu/Debian:** `sudo apt update && sudo apt upgrade -y`
*   **Red Hat/CentOS:** `sudo yum update -y`

#### 2.2. Rust Toolchain

Install the Rust toolchain using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify the installation:

```bash
rustc --version
cargo --version
```

#### 2.3. Nginx (Recommended Reverse Proxy)

Install Nginx:

*   **Ubuntu/Debian:** `sudo apt install nginx -y`
*   **Red Hat/CentOS:** `sudo yum install nginx -y`

Start and enable Nginx:

```bash
sudo systemctl start nginx
sudo systemctl enable nginx
```

#### 2.4. PostgreSQL (Recommended Database)

Install PostgreSQL:

*   **Ubuntu/Debian:** `sudo apt install postgresql postgresql-contrib -y`
*   **Red Hat/CentOS:** `sudo yum install postgresql-server postgresql-contrib -y`

Initialize and start the database service if needed:

```bash
sudo postgresql-setup initdb
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

Create a database user and database for the PEMA Platform (remember these credentials for the installer):

```bash
sudo -i -u postgres psql
```

Inside the PostgreSQL prompt:

```sql
CREATE USER pema_user WITH PASSWORD 'your_strong_password';
CREATE DATABASE pema_db OWNER pema_user;
\q
```

### 3. Initial Project Setup

1.  **Navigate to the repository root:**

    ```bash
    cd pema-platform-rust
    ```

2.  **Prepare `.env` for the installer:**

    Copy the example environment file and edit it for your server. This file is crucial for the installer to run.

    ```bash
    cp .env.example backends/installer/.env
    nano backends/installer/.env
    ```

    **Key variables to configure in `backends/installer/.env`:**

    *   `SERVER_HOST`: `0.0.0.0` (to listen on all interfaces) or `127.0.0.1`.
    *   `SERVER_PORT`: The port the installer will listen on (default `8080`). **Ensure this port is free.**
    *   `DOMAIN`, `BASE_URL`: Your server's public domain or IP.
    *   `DB_*`: Your PostgreSQL credentials.
    *   `JWT_SECRET`: A strong, random string (min 32 chars).
    *   `CONFIG_PATH`: (Optional) Absolute path where `config.toml` should be saved (e.g., `/etc/pema/config.toml`). Ensure write permissions.

### 4. Build All Components

From the repository root (`pema-platform-rust/`), use the `Makefile` to build all parts of the application:

```bash
make all
```

This command will:
*   Build the `pema-installer` backend.
*   Build the `wasm-auth-backend` (WASM library).
*   Build the `wasm-general-backend` (WASM library).
*   Build the `wasm-frontend` (generating static files in `wasm-frontend/dist/`).

### 5. Run the Web Installer

1.  **Run the installer backend:**

    From the repository root, execute the `install-config` Makefile target:

    ```bash
    make install-config
    ```

    This will start the `pema-installer` server. You should see log messages indicating it's running, typically on `0.0.0.0:8080` (or your configured port).

    **Important:** If you have a firewall, open the installer's port (e.g., `sudo ufw allow 8080/tcp`).

2.  **Access the installer in your browser:**

    Open a web browser and navigate to `http://<your_server_ip>:<installer_port>/`.

3.  **Complete the installation form:**

    Fill in all the required details, ensuring accuracy for domain, database credentials, and security settings. Click **"Install & Configure"**.

4.  **Stop the installer:**

    Once the installation is successful (you'll see a success message in the browser), stop the `pema-installer` process by pressing `Ctrl+C` in your terminal.

### 6. Nginx Configuration

Configure Nginx as a reverse proxy to serve your frontend and proxy requests to your `backend-server`.

1.  **Create an Nginx configuration file:**

    ```bash
    sudo nano /etc/nginx/sites-available/pema-platform.conf
    ```

2.  **Add the server block configuration:**

    Paste the following content, making sure to replace placeholders:

    ```nginx
    server {
        listen 80;
        listen [::]:80;
        server_name your_domain.com www.your_domain.com; # Replace with your actual domain

        # Redirect HTTP to HTTPS (recommended for production)
        return 301 https://$host$request_uri;
    }

    server {
        listen 443 ssl http2;
        listen [::]:443 ssl http2;
        server_name your_domain.com www.your_domain.com; # Replace with your actual domain

        # SSL configuration (replace with your actual certificate paths)
        # Obtain these from Let's Encrypt (e.g., using Certbot)
        ssl_certificate /etc/letsencrypt/live/your_domain.com/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/your_domain.com/privkey.pem;
        ssl_session_cache shared:SSL:10m;
        ssl_session_timeout 10m;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384';
        ssl_prefer_server_ciphers on;

        # Root for static frontend files
        root /path/to/your/pema-platform-rust/wasm-frontend/dist; # IMPORTANT: Update this absolute path
        index index.html;

        # Serve static files directly
        location / {
            try_files $uri $uri/ /index.html;
        }

        # Proxy API requests to the main backend-server
        location /api/ {
            proxy_pass http://127.0.0.1:8080; # Assuming backend-server listens on 8080
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Proxy Auth requests to the main backend-server (if handled by it)
        location /auth/ {
            proxy_pass http://127.0.0.1:8080; # Assuming backend-server handles auth routes
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
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
    ```

    **Remember to update:**
    *   `server_name`
    *   `root` path to your `wasm-frontend/dist` directory
    *   `proxy_pass` ports for your `backend-server` (e.g., `8080`)
    *   `ssl_certificate` and `ssl_certificate_key` paths (use Certbot for free SSL).

3.  **Enable and test Nginx configuration:**

    ```bash
    sudo ln -s /etc/nginx/sites-available/pema-platform.conf /etc/nginx/sites-enabled/
    sudo nginx -t
    sudo systemctl reload nginx
    ```

### 7. Systemd Service Setup for `backend-server`

To ensure your main `backend-server` runs continuously and automatically, set it up as a `systemd` service.

1.  **Create a systemd service file:**

    ```bash
    sudo nano /etc/systemd/system/pema-backend.service
    ```

2.  **Add the following content:**

    ```ini
    [Unit]
    Description=PEMA Platform Backend Server
    After=network.target postgresql.service

    [Service]
    Type=simple
    User=pema # IMPORTANT: Create this user or use an existing non-root user
    WorkingDirectory=/path/to/pema-platform-rust/backend-server # IMPORTANT: Update this absolute path
    ExecStart=/path/to/pema-platform-rust/backend-server/target/release/pema-backend # IMPORTANT: Update this absolute path
    Restart=always
    RestartSec=10
    Environment="RUST_LOG=info" # Adjust log level as needed

    [Install]
    WantedBy=multi-user.target
    ```

    **Remember to update:**
    *   `User`: Create a dedicated system user (e.g., `sudo useradd -r -s /bin/false pema`) or use an existing non-root user.
    *   `WorkingDirectory`: The absolute path to your `pema-platform-rust/backend-server` directory.
    *   `ExecStart`: The absolute path to the compiled `pema-backend` executable within `target/release/`.

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

### 8. Troubleshooting

*   **Port Conflicts**: If a service fails to start, check `sudo netstat -tulnp` to see if its port is already in use. Adjust the port in the relevant configuration (e.g., `.env` for installer, `backend-server` code, Nginx config).
*   **Nginx `502 Bad Gateway`**: This usually means Nginx cannot connect to your backend server. Check:
    *   Is `pema-backend.service` running (`sudo systemctl status pema-backend`)?
    *   Is the `proxy_pass` URL in Nginx correct and does it match the port your `backend-server` is listening on?
    *   Are there any firewall rules blocking Nginx from connecting to the backend?
*   **File Permissions**: Ensure the user running the `pema-installer` and `pema-backend.service` has read/write permissions to necessary directories (e.g., where `config.toml` is stored, `wasm-frontend/dist`).
*   **Logs**: Always check `journalctl -u <service_name> -f` for detailed error messages from your services.

### Conclusion

By following this updated guide, you will have a fully installed and configured PEMA Platform. The `Makefile` streamlines the build process, the web installer simplifies initial configuration, and `systemd` ensures your main backend runs reliably. Nginx serves your frontend efficiently and securely, acting as a reverse proxy to your `backend-server`.

Your PEMA Platform is now ready for production use!


## Clone the repository

To clone the repository, use the following command:

```bash
git clone https://github.com/hadiranweb/pema-platform-rust.git
```

