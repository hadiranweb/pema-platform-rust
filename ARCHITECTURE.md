# 🏗️ PEMA Platform Architecture Documentation

## 📋 Table of Contents
1. [System Overview](#system-overview)
2. [Core Architecture](#core-architecture)
3. [Plugin System](#plugin-system)
4. [Multi-tenant System](#multi-tenant-system)
5. [Wallet System](#wallet-system)
6. [Event-driven Architecture](#event-driven-architecture)
7. [Database Design](#database-design)
8. [API Design](#api-design)
9. [Security Architecture](#security-architecture)
10. [Deployment Architecture](#deployment-architecture)

---

## 🎯 System Overview

PEMA Platform is a **modular, plugin-based, multi-tenant platform** built with Rust and WebAssembly. The platform provides a core system that can be extended through WASM plugins, supporting multiple tenants with isolated data and configurations.

### Key Characteristics:
- **Language**: Rust (Backend) + WebAssembly (Plugins)
- **Architecture Pattern**: Plugin-based Modular Architecture
- **Database**: PostgreSQL
- **Runtime**: Wasmtime for WASM execution
- **Web Framework**: Actix-web
- **Multi-tenancy**: Full tenant isolation

---

## 🏗️ Core Architecture

### System Components Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    PEMA Platform Core                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Plugin    │  │   Tenant    │  │   Event     │         │
│  │  Manager    │  │  Manager    │  │    Bus      │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Wallet    │  │    Auth     │  │  General    │         │
│  │  Service    │  │  Service    │  │  Services   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    Actix-web HTTP Server                    │
├─────────────────────────────────────────────────────────────┤
│                    PostgreSQL Database                      │
└─────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
backend-server/src/
├── main.rs                 # Application entry point
├── core/                   # Core system modules
│   ├── mod.rs             # Core module exports
│   ├── plugins/           # Plugin system
│   │   ├── mod.rs         # Plugin types and interfaces
│   │   ├── manager.rs     # Plugin lifecycle management
│   │   ├── loader.rs      # WASM module loading
│   │   ├── sandbox.rs     # WASM execution sandbox
│   │   └── registry.rs    # Plugin registration and discovery
│   ├── tenant/            # Multi-tenant system
│   │   ├── mod.rs         # Tenant types and models
│   │   └── manager.rs     # Tenant management
│   └── events/            # Event system
│       ├── mod.rs         # Event types and traits
│       └── bus.rs         # Event bus implementation
├── wallet/                # Wallet system
│   ├── mod.rs            # Wallet module exports
│   ├── models.rs         # Database models
│   ├── service.rs        # Business logic
│   ├── handlers.rs       # HTTP handlers
│   └── errors.rs         # Error types
├── auth/                  # Authentication system
│   ├── mod.rs            # Auth module exports
│   ├── middleware.rs     # JWT middleware
│   └── utils.rs          # Auth utilities
├── auth_routes.rs        # Authentication routes
├── general_routes.rs     # General API routes
└── db_pool_impl.rs       # Database connection pool
```

---

## 🔌 Plugin System

### Plugin Architecture

The plugin system is built on **WebAssembly (WASM)** for security, performance, and language-agnostic plugin development.

```
┌─────────────────────────────────────────────────────────────┐
│                    Plugin Ecosystem                         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Plugin    │  │   Plugin    │  │   Plugin    │         │
│  │      A      │  │      B      │  │      C      │         │
│  │   (WASM)    │  │   (WASM)    │  │   (WASM)    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                  Wasmtime Runtime                           │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Plugin    │  │   Plugin    │  │   Plugin    │         │
│  │  Manager    │  │   Loader    │  │  Registry   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    Core Platform                            │
└─────────────────────────────────────────────────────────────┘
```

### Plugin Components

#### 1. Plugin Manager (`core/plugins/manager.rs`)
- **Responsibility**: Plugin lifecycle management
- **Key Functions**:
  - `load_plugin()`: Load and initialize plugins
  - `execute_hook()`: Execute plugin hooks
  - `unload_plugin()`: Clean plugin shutdown
  - `hot_reload_plugin()`: Runtime plugin updates

#### 2. Plugin Loader (`core/plugins/loader.rs`)
- **Responsibility**: WASM module loading and validation
- **Key Functions**:
  - `load_module()`: Load WASM bytecode
  - `validate_wasm_module()`: Security validation
  - `precompile_module()`: Performance optimization

#### 3. Plugin Sandbox (`core/plugins/sandbox.rs`)
- **Responsibility**: Secure WASM execution environment
- **Security Features**:
  - Memory isolation
  - Resource limits
  - Host function restrictions
  - Execution timeouts

#### 4. Plugin Registry (`core/plugins/registry.rs`)
- **Responsibility**: Plugin discovery and metadata management
- **Key Functions**:
  - `register_plugin()`: Plugin registration
  - `get_plugins_for_hook()`: Hook-based plugin discovery
  - `check_plugin_permission()`: Access control

### Plugin Development

#### Plugin SDK (`shared/plugin-sdk/`)
```rust
// Plugin interface definition
pub trait Plugin {
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;
    fn execute_hook(&mut self, hook: PluginHook, data: &[u8]) -> Result<Vec<u8>>;
    fn cleanup(&mut self) -> Result<()>;
}

// Available hooks
pub enum PluginHook {
    BeforeRequest,
    AfterRequest,
    BeforeTransaction,
    AfterTransaction,
    UserRegistration,
    OrderProcessing,
}
```

#### Example Plugin (`plugins/discount_calculator/`)
```rust
use plugin_sdk::{Plugin, PluginHook, PluginContext};

pub struct DiscountCalculatorPlugin;

impl Plugin for DiscountCalculatorPlugin {
    fn execute_hook(&mut self, hook: PluginHook, data: &[u8]) -> Result<Vec<u8>> {
        match hook {
            PluginHook::OrderProcessing => {
                // Calculate discount logic
                let discount = calculate_discount(data)?;
                Ok(serde_json::to_vec(&discount)?)
            }
            _ => Ok(vec![])
        }
    }
}
```

---

## 🏢 Multi-tenant System

### Tenant Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Tenant A                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Users     │  │   Wallets   │  │   Plugins   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    Tenant B                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Users     │  │   Wallets   │  │   Plugins   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                  Tenant Manager                             │
├─────────────────────────────────────────────────────────────┤
│                  Shared Infrastructure                      │
└─────────────────────────────────────────────────────────────┘
```

### Tenant Components

#### 1. Tenant Model (`core/tenant/mod.rs`)
```rust
pub struct Tenant {
    pub id: Uuid,
    pub domain: String,
    pub name: String,
    pub status: TenantStatus,
    pub settings: TenantSettings,
    pub created_at: DateTime<Utc>,
}

pub struct TenantSettings {
    pub branding: BrandingSettings,
    pub database: DatabaseConfig,
    pub plugins: PluginConfig,
}
```

#### 2. Tenant Manager (`core/tenant/manager.rs`)
- **Responsibility**: Tenant lifecycle and resolution
- **Key Functions**:
  - `get_tenant()`: Resolve tenant from request
  - `add_tenant()`: Register new tenant
  - `update_tenant()`: Modify tenant settings

### Tenant Resolution Strategies

1. **Domain-based**: `tenant1.platform.com`
2. **Subdirectory**: `platform.com/tenant1`
3. **Header-based**: `X-Tenant-ID: tenant1`
4. **Database-driven**: Dynamic resolution

---

## 💰 Wallet System

### Wallet Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Wallet System                            │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Wallet    │  │Transaction  │  │  Purchase   │         │
│  │   Model     │  │   Model     │  │    Flow     │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Refund    │  │   Admin     │  │   Wallet    │         │
│  │  Request    │  │  Action     │  │  Service    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    HTTP Handlers                            │
└─────────────────────────────────────────────────────────────┘
```

### Wallet Components

#### 1. Wallet Models (`wallet/models.rs`)
```rust
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: Decimal,
    pub currency: String,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
}

pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
}
```

#### 2. Wallet Service (`wallet/service.rs`)
- **Business Logic Layer**
- **Key Functions**:
  - `create_wallet()`: Create user wallet
  - `create_transaction()`: Process transactions
  - `update_wallet_status()`: Wallet state management
  - `create_purchase_flow()`: Purchase workflow
  - `create_refund_request()`: Refund processing

#### 3. Wallet Handlers (`wallet/handlers.rs`)
- **HTTP API Layer**
- **Endpoints**:
  - `POST /wallet/create`: Create wallet
  - `GET /wallet/{id}`: Get wallet details
  - `GET /wallet/user/{user_id}`: Get user wallets
  - `POST /wallet/transaction`: Create transaction

### Transaction Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Request   │───▶│  Validate   │───▶│   Process   │
│ Transaction │    │   Input     │    │ Transaction │
└─────────────┘    └─────────────┘    └─────────────┘
                                              │
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Update    │◀───│   Record    │◀───│   Update    │
│   Balance   │    │ Transaction │    │   Wallet    │
└─────────────┘    └─────────────┘    └─────────────┘
```

---

## 📡 Event-driven Architecture

### Event System

```
┌─────────────────────────────────────────────────────────────┐
│                    Event Producers                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Wallet    │  │    Auth     │  │   Plugin    │         │
│  │  Service    │  │  Service    │  │  Manager    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                      Event Bus                              │
├─────────────────────────────────────────────────────────────┤
│                   Event Consumers                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Plugins   │  │  Analytics  │  │    Audit    │         │
│  │             │  │   Service   │  │     Log     │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

### Event Components

#### 1. Event Types (`core/events/mod.rs`)
```rust
pub enum EventType {
    UserRegistered,
    UserLoggedIn,
    WalletCreated,
    TransactionCreated,
    PluginLoaded,
    PluginExecuted,
}

pub struct Event {
    pub id: Uuid,
    pub event_type: EventType,
    pub tenant_id: String,
    pub user_id: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}
```

#### 2. Event Bus (`core/events/bus.rs`)
- **Async Event Broadcasting**
- **Key Functions**:
  - `publish()`: Broadcast events
  - `subscribe()`: Listen to events
  - `register_handler()`: Register event handlers

### Event Flow

```
Service ──publish──▶ Event Bus ──broadcast──▶ Handlers
   │                     │                        │
   │                     │                        ▼
   │                     │                   ┌─────────┐
   │                     │                   │ Plugin  │
   │                     │                   │ Handler │
   │                     │                   └─────────┘
   │                     │                        │
   │                     │                        ▼
   │                     │                   ┌─────────┐
   │                     │                   │ Audit   │
   │                     │                   │ Handler │
   │                     │                   └─────────┘
```

---

## 🗄️ Database Design

### Entity Relationship Diagram

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Tenants   │    │    Users    │    │   Wallets   │
│             │    │             │    │             │
│ id (PK)     │    │ id (PK)     │    │ id (PK)     │
│ domain      │    │ tenant_id   │    │ user_id (FK)│
│ name        │    │ email       │    │ balance     │
│ settings    │    │ password    │    │ currency    │
└─────────────┘    └─────────────┘    └─────────────┘
                           │                   │
                           └───────────────────┘
                                   │
                           ┌─────────────┐
                           │Transactions │
                           │             │
                           │ id (PK)     │
                           │ wallet_id   │
                           │ amount      │
                           │ type        │
                           │ status      │
                           └─────────────┘
```

### Database Tables

#### Core Tables
- **tenants**: Tenant configuration and settings
- **users**: User accounts per tenant
- **wallets**: User wallet information
- **transactions**: Financial transactions
- **purchase_flows**: Purchase workflow tracking
- **refund_requests**: Refund processing
- **admin_actions**: Administrative audit trail

#### Plugin Tables
- **plugins**: Registered plugin metadata
- **plugin_instances**: Plugin instances per tenant
- **plugin_executions**: Plugin execution logs

### Migration Strategy

```bash
# Database migrations located in:
backend-server/migrations/

# Migration files:
001_initial_schema.sql
002_wallet_system.sql
003_plugin_system.sql
004_tenant_system.sql
```

---

## 🌐 API Design

### RESTful API Structure

```
/api/v1/
├── /health                 # System health check
├── /auth/                  # Authentication endpoints
│   ├── POST /login        # User login
│   ├── POST /register     # User registration
│   └── POST /validate     # Token validation
├── /wallet/               # Wallet management
│   ├── GET /health        # Wallet system health
│   ├── POST /create       # Create wallet
│   ├── GET /{id}          # Get wallet by ID
│   ├── GET /user/{id}     # Get user wallets
│   └── POST /transaction  # Create transaction
├── /plugins/              # Plugin management
│   ├── GET /              # List plugins
│   ├── POST /upload       # Upload plugin
│   ├── DELETE /{id}       # Remove plugin
│   └── POST /{id}/reload  # Hot reload plugin
└── /orders/               # Order management
    └── GET /{id}          # Get order details
```

### API Response Format

```json
{
  "success": true,
  "data": {
    // Response data
  },
  "message": "Operation completed successfully",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "WALLET_NOT_FOUND",
    "message": "Wallet with ID 123 not found",
    "details": {}
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

---

## 🔐 Security Architecture

### Security Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │    Input    │  │    JWT      │  │    CORS     │         │
│  │ Validation  │  │    Auth     │  │   Policy    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    Plugin Sandbox                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Memory    │  │  Resource   │  │  Function   │         │
│  │ Isolation   │  │   Limits    │  │ Whitelist   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    Database Layer                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │    SQL      │  │   Tenant    │  │    Data     │         │
│  │ Injection   │  │ Isolation   │  │ Encryption  │         │
│  │ Protection  │  │             │  │             │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

### Security Features

#### 1. Authentication & Authorization
- **JWT Tokens**: Stateless authentication
- **Role-based Access**: User/Admin roles
- **Tenant Isolation**: Cross-tenant access prevention

#### 2. Plugin Security
- **WASM Sandbox**: Memory and resource isolation
- **Function Whitelist**: Limited host function access
- **Execution Limits**: CPU and memory constraints
- **Code Validation**: WASM bytecode verification

#### 3. Data Protection
- **SQL Injection**: Parameterized queries with SQLx
- **Input Validation**: Comprehensive input sanitization
- **Data Encryption**: Sensitive data encryption at rest
- **Audit Logging**: Complete action audit trail

---

## 🚀 Deployment Architecture

### Production Deployment

```
┌─────────────────────────────────────────────────────────────┐
│                    Load Balancer                            │
│                    (Nginx/HAProxy)                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   PEMA      │  │   PEMA      │  │   PEMA      │         │
│  │ Instance 1  │  │ Instance 2  │  │ Instance 3  │         │
│  │ (Port 8000) │  │ (Port 8001) │  │ (Port 8002) │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    PostgreSQL Cluster                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Primary   │  │  Replica 1  │  │  Replica 2  │         │
│  │  Database   │  │  (Read)     │  │  (Read)     │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

### Deployment Components

#### 1. Application Server
- **Runtime**: Native Rust binary
- **Process Manager**: systemd service
- **Monitoring**: Health check endpoints
- **Logging**: Structured logging with tracing

#### 2. Database
- **Primary**: PostgreSQL with write operations
- **Replicas**: Read-only replicas for scaling
- **Backup**: Automated backup strategy
- **Migration**: Automated schema migrations

#### 3. Reverse Proxy
- **SSL Termination**: TLS certificate management
- **Load Balancing**: Round-robin distribution
- **Static Assets**: Frontend asset serving
- **Rate Limiting**: API rate limiting

### Environment Configuration

#### Production Environment Variables
```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost/pema_platform
DATABASE_POOL_SIZE=20

# Security
JWT_SECRET=your-production-jwt-secret
CORS_ALLOWED_ORIGINS=https://yourdomain.com

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
RUST_LOG=info

# Plugin System
PLUGIN_MAX_MEMORY_MB=128
PLUGIN_MAX_EXECUTION_TIME_MS=5000
PLUGIN_STORAGE_PATH=/var/lib/pema/plugins
```

### Monitoring & Observability

#### Metrics Collection
- **Application Metrics**: Request latency, throughput
- **System Metrics**: CPU, memory, disk usage
- **Database Metrics**: Connection pool, query performance
- **Plugin Metrics**: Execution time, memory usage

#### Logging Strategy
- **Structured Logging**: JSON format with tracing
- **Log Levels**: Error, Warn, Info, Debug, Trace
- **Log Aggregation**: Centralized log collection
- **Alert Rules**: Automated alerting on errors

---

## 📈 Performance Considerations

### Optimization Strategies

#### 1. Database Optimization
- **Connection Pooling**: Efficient database connections
- **Query Optimization**: Indexed queries and prepared statements
- **Read Replicas**: Separate read/write operations
- **Caching**: Redis for frequently accessed data

#### 2. Plugin Performance
- **WASM Compilation**: Ahead-of-time compilation
- **Plugin Caching**: Compiled module caching
- **Resource Limits**: Memory and CPU constraints
- **Parallel Execution**: Concurrent plugin execution

#### 3. API Performance
- **Async Processing**: Non-blocking I/O operations
- **Response Caching**: HTTP response caching
- **Compression**: Gzip response compression
- **CDN Integration**: Static asset delivery

### Scalability Patterns

#### Horizontal Scaling
- **Stateless Design**: No server-side session state
- **Load Balancing**: Multiple application instances
- **Database Sharding**: Tenant-based data partitioning
- **Plugin Distribution**: Distributed plugin execution

#### Vertical Scaling
- **Resource Optimization**: Efficient memory usage
- **CPU Utilization**: Multi-threaded processing
- **I/O Optimization**: Async database operations
- **Memory Management**: Efficient data structures

---

## 🔄 Development Workflow

### Development Environment Setup

```bash
# 1. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install WASM target
rustup target add wasm32-unknown-unknown

# 3. Install development tools
cargo install sqlx-cli wasm-pack

# 4. Clone and setup project
git clone https://github.com/hadiranweb/pema-platform-rust.git
cd pema-platform-rust
git checkout mine

# 5. Setup database
createdb pema_platform
sqlx migrate run

# 6. Run development server
export SQLX_OFFLINE=true
cargo run
```

### Testing Strategy

#### Unit Tests
```bash
# Run unit tests
cargo test

# Run tests with coverage
cargo test --coverage
```

#### Integration Tests
```bash
# Run integration tests
cargo test --test integration

# Run plugin tests
cargo test --package plugin-sdk
```

#### Performance Tests
```bash
# Load testing
cargo bench

# Memory profiling
cargo run --release --features profiling
```

### CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      - name: Run tests
        run: cargo test --workspace
      
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Build release
        run: cargo build --release --workspace
        
  deploy:
    runs-on: ubuntu-latest
    needs: [test, build]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to production
        run: ./scripts/deploy.sh
```

---

## 📚 Additional Resources

### Documentation Links
- [Rust Documentation](https://doc.rust-lang.org/)
- [Actix-web Guide](https://actix.rs/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Wasmtime Guide](https://docs.wasmtime.dev/)

### Development Tools
- **IDE**: VS Code with rust-analyzer
- **Database**: PostgreSQL with pgAdmin
- **API Testing**: Postman or curl
- **Monitoring**: Prometheus + Grafana

### Community Resources
- [Rust Community](https://www.rust-lang.org/community)
- [WebAssembly Community](https://webassembly.org/)
- [PostgreSQL Community](https://www.postgresql.org/community/)

---

**Document Version**: 1.0  
**Last Updated**: 2024-01-15  
**Maintained By**: PEMA Platform Team  
**License**: MIT License