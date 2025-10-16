# 🛍️ PEMA Platform - پلتفرم جامع تجارت الکترونیک

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/postgresql-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Redis](https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

[![CI/CD](https://github.com/hadiranweb/pema-platform-rust/workflows/CI%2FCD/badge.svg)](https://github.com/hadiranweb/pema-platform-rust/actions)
[![Security](https://img.shields.io/badge/security-audited-green.svg)](https://github.com/hadiranweb/pema-platform-rust/security)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

یک پلتفرم جامع و پیشرفته تجارت الکترونیک ساخته شده با Rust، شامل سرویس‌های متعدد backend، اپلیکیشن‌های frontend، و فناوری‌های پیشرفته که از تمام شاخه‌های توسعه ادغام شده است.

## ✨ ویژگی‌های کلیدی

- 🚀 **عملکرد بالا**: ساخته شده با Rust برای حداکثر سرعت و ایمنی
- 🔒 **امنیت پیشرفته**: سیستم احراز هویت قوی با JWT و رمزنگاری
- 🌐 **WebAssembly**: اجزای WASM برای عملکرد بهینه در مرورگر
- 🔌 **معماری Plugin**: سیستم قابل گسترش برای منطق تجاری سفارشی
- 📱 **Responsive Design**: رابط کاربری مدرن و سازگار با موبایل
- 🌍 **چندزبانه**: پشتیبانی از فارسی و انگلیسی
- 💳 **پرداخت یکپارچه**: سیستم کیف پول و درگاه‌های پرداخت ایرانی
- 📊 **مانیتورینگ**: داشبورد جامع مانیتورینگ و آنالیتیکس

## 🏗️ Architecture Overview

This platform integrates technologies from multiple development branches:

### Frontend Applications
- **`frontend/`** - Modern Yew-based web frontend with comprehensive UI components
- **`wasm-frontend/`** - WebAssembly frontend with advanced i18n support

### Backend Services
- **`backend/`** - Modular backend architecture with comprehensive modules
- **`backend-server/`** - Main backend server with wallet functionality and JWT handling
- **`auth-server/`** - Dedicated authentication service
- **`wasm-auth-backend/`** - WebAssembly authentication backend
- **`wasm-general-backend/`** - WebAssembly general-purpose backend

### Shared Libraries
- **`shared/models/`** - Common data models
- **`shared/config/`** - Configuration management
- **`shared/dtos/`** - Data transfer objects
- **`shared/plugin-sdk/`** - Plugin development SDK

### Plugin System
- **`plugins/discount_calculator/`** - Example discount calculation plugin
- Extensible plugin architecture for custom business logic

### Infrastructure & Deployment
- **`nginx/`** - Production nginx configuration
- **`scripts/`** - Deployment and setup automation
- **`locales/`** - Internationalization support (English/Persian)

## 🚀 Features

### Core E-commerce Features
- **User Management**: Registration, authentication, profiles
- **Product Catalog**: Product management, categories, inventory
- **Order Processing**: Shopping cart, checkout, order tracking
- **Payment Integration**: Wallet system, payment gateways
- **Vendor Management**: Multi-vendor marketplace support
- **Review System**: Product reviews and ratings
- **Admin Dashboard**: Comprehensive admin interface

### Advanced Technologies
- **WebAssembly Integration**: High-performance WASM components
- **Plugin Architecture**: Extensible business logic system
- **Multi-tenant Support**: Tenant management and isolation
- **Event-driven Architecture**: Event bus for loose coupling
- **Internationalization**: Full i18n support with Persian/English
- **Microservices**: Modular service architecture
- **Database Integration**: SQLx with PostgreSQL support

### Development Features
- **Comprehensive Testing**: Unit and integration tests
- **CI/CD Pipeline**: GitHub Actions workflow
- **Docker Support**: Containerized deployment
- **Development Tools**: Setup scripts and automation
- **Documentation**: Comprehensive architecture documentation

## 🛠️ Technology Stack

### Backend
- **Rust** - Core language
- **Axum** - Web framework
- **SQLx** - Database toolkit
- **PostgreSQL** - Primary database
- **JWT** - Authentication tokens
- **Serde** - Serialization/deserialization

### Frontend
- **Yew** - Rust web framework
- **WebAssembly** - High-performance web components
- **CSS3** - Modern styling
- **JavaScript** - Browser integration

### Infrastructure
- **Docker** - Containerization
- **Nginx** - Reverse proxy and static serving
- **GitHub Actions** - CI/CD pipeline
- **Systemd** - Service management

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 13+
- Node.js 16+ (for frontend tooling)
- Docker (optional)

### Setup
1. **Clone and setup**:
   ```bash
   git clone <repository-url>
   cd pema-platform-rust
   chmod +x setup_server.sh
   ./setup_server.sh
   ```

2. **Database setup**:
   ```bash
   # Configure database connection in .env files
   cp .env.example .env
   # Run migrations
   cd backend-server && sqlx migrate run
   ```

3. **Build all components**:
   ```bash
   cargo build --workspace
   ```

4. **Run services**:
   ```bash
   # Backend server
   cargo run --bin backend-server
   
   # Auth server
   cargo run --bin auth-server
   
   # Frontend (in separate terminal)
   cd frontend && trunk serve
   ```

### Development Scripts
- `scripts/local-deploy.sh` - Local development deployment
- `scripts/deploy.sh` - Production deployment
- `scripts/validate-dependabot.sh` - Dependency validation

## 📁 Project Structure

```
pema-platform-rust/
├── frontend/                 # Yew-based web frontend
├── wasm-frontend/           # WebAssembly frontend with i18n
├── backend/                 # Modular backend architecture
├── backend-server/          # Main backend server
├── auth-server/            # Authentication service
├── wasm-auth-backend/      # WASM auth backend
├── wasm-general-backend/   # WASM general backend
├── shared/                 # Shared libraries
│   ├── models/            # Data models
│   ├── config/            # Configuration
│   ├── dtos/              # Data transfer objects
│   └── plugin-sdk/        # Plugin SDK
├── plugins/               # Plugin system
├── nginx/                 # Production configuration
├── scripts/               # Deployment scripts
├── locales/               # Internationalization
└── docs/                  # Documentation
```

## 🔧 Configuration

### Environment Variables
- **`.env`** - Main configuration
- **`.env.api`** - API service configuration
- **`.env.auth`** - Authentication service configuration

### Key Configuration Files
- **`Cargo.toml`** - Workspace configuration
- **`nginx/nginx.conf`** - Production web server
- **`Makefile`** - Build automation

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cargo test --package backend-server
cargo test --package frontend
```

## 📦 Deployment

### Local Development
```bash
./scripts/local-deploy.sh
```

### Production
```bash
./scripts/deploy.sh
```

### Docker
```bash
# Build containers
docker build -t pema-backend ./backend-server
docker build -t pema-frontend ./frontend

# Run with docker-compose (if available)
docker-compose up -d
```

## 🌐 API Documentation

The platform provides RESTful APIs for:
- Authentication (`/api/auth/*`)
- User management (`/api/users/*`)
- Products (`/api/products/*`)
- Orders (`/api/orders/*`)
- Payments (`/api/payments/*`)
- Admin functions (`/api/admin/*`)

## 🔌 Plugin Development

Create custom plugins using the Plugin SDK:

```rust
use shared_plugin_sdk::*;

#[plugin_main]
fn main() -> PluginResult<()> {
    // Your plugin logic here
    Ok(())
}
```

## 🌍 Internationalization

The platform supports multiple languages:
- English (`locales/en.json`)
- Persian/Farsi (`locales/fa.json`)

Add new languages by creating corresponding locale files.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For support and questions:
- Check the documentation in `docs/`
- Review architecture in `ARCHITECTURE.md`
- See setup instructions in `SETUP.md`

## 🎯 Roadmap

- [ ] Enhanced plugin system
- [ ] Advanced analytics
- [ ] Mobile app integration
- [ ] Advanced payment gateways
- [ ] AI-powered recommendations
- [ ] Advanced security features

---

**Built with ❤️ using Rust and modern web technologies**