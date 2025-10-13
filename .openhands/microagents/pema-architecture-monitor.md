---
name: PEMA Platform Architecture Monitor
type: knowledge
version: 1.0.0
agent: CodeActAgent
triggers:
  - architecture
  - structure
  - pema platform
  - directory structure
  - backend structure
  - frontend structure
  - module organization
  - file organization
  - project layout
---

# PEMA Platform Architecture Monitor

This microagent monitors and enforces the architectural standards for the PEMA Platform Rust project. It ensures that the project maintains its specified directory structure and reports any deviations from the established architecture.

## Expected Architecture

The PEMA Platform must maintain the following directory structure:

### Root Level Structure
```
pema-platform-rust/
│
├── backend/
├── frontend/
├── shared/
├── migrations/
├── scripts/
├── tests/
├── Cargo.toml
├── Makefile
└── docker-compose.yml
```

### Backend Structure
```
backend/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── config/
    │   ├── mod.rs
    │   ├── database.rs
    │   └── settings.rs
    │
    ├── core/                    # 🆕 Core platform features
    │   ├── mod.rs
    │   ├── tenant/              # Multi-tenancy
    │   │   ├── mod.rs
    │   │   ├── manager.rs
    │   │   ├── resolver.rs
    │   │   ├── context.rs
    │   │   └── middleware.rs
    │   │
    │   ├── plugins/             # 🆕 Plugin system
    │   │   ├── mod.rs
    │   │   ├── manager.rs
    │   │   ├── loader.rs
    │   │   ├── registry.rs
    │   │   └── interface.rs
    │   │
    │   └── events/              # 🆕 Event bus
    │       ├── mod.rs
    │       ├── bus.rs
    │       ├── publisher.rs
    │       └── subscriber.rs
    │
    ├── modules/
    │   ├── mod.rs
    │   ├── auth/
    │   │   ├── mod.rs
    │   │   ├── handlers.rs
    │   │   ├── service.rs
    │   │   ├── repository.rs
    │   │   └── middleware.rs
    │   │
    │   ├── products/
    │   │   ├── mod.rs
    │   │   ├── handlers.rs
    │   │   ├── service.rs
    │   │   └── repository.rs
    │   │
    │   ├── orders/
    │   ├── vendors/
    │   ├── wallet/
    │   ├── reviews/
    │   ├── shipping/
    │   ├── ai/                  # 🆕 AI engine
    │   │   ├── mod.rs
    │   │   ├── recommender/
    │   │   └── search/
    │   │
    │   ├── analytics/           # 🆕 Analytics
    │   ├── developer_api/       # 🆕 Developer API
    │   └── notifications/       # 🆕 Notifications
    │
    ├── middleware/
    │   ├── mod.rs
    │   ├── auth.rs
    │   ├── cors.rs
    │   └── logger.rs
    │
    └── utils/
        ├── mod.rs
        ├── error.rs
        └── response.rs
```

### Frontend Structure
```
frontend/
├── Cargo.toml
├── index.html
└── src/
    ├── main.rs
    ├── app.rs
    ├── pages/
    ├── components/
    ├── services/
    └── state/
```

### Multiple Frontends (New Addition)
```
frontends/                      # 🆕 Multiple frontends
├── customer-app/
├── vendor-portal/
└── admin-console/
```

### Shared Structure
```
shared/
├── models/
│   └── src/
│       ├── tenant.rs           # 🆕 Tenant model
│       ├── user.rs
│       ├── product.rs
│       └── ...
├── config/
├── utils/
└── plugin-sdk/                 # 🆕 Plugin SDK
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── plugin.rs
        ├── context.rs
        └── macros.rs
```

### Plugin Ecosystem
```
plugins/                         # 🆕 Plugin ecosystem
├── builtin/
│   ├── loyalty-program/
│   ├── subscription-box/
│   └── affiliate-system/
│
└── marketplace/
    └── ... (third-party)
```

### Migrations
```
migrations/
├── 001_create_tenants.sql      # 🆕 Tenant tables
├── 002_create_users.sql
└── ...
```

## Architecture Monitoring Functions

### 1. Structure Validation
When triggered, this microagent will:
- Scan the current directory structure
- Compare against the expected architecture
- Report any missing directories or files
- Identify unexpected files or directories
- Validate module organization within each component

### 2. Deviation Detection
The microagent detects and reports:
- **Missing Core Components**: Missing core/, tenant/, plugins/, events/ directories
- **Missing New Modules**: Missing ai/, analytics/, developer_api/, notifications/ modules
- **Incorrect Module Structure**: Modules missing required files (handlers.rs, service.rs, repository.rs)
- **Misplaced Files**: Files not following the established patterns
- **Missing Frontend Structure**: Missing multiple frontend applications
- **Plugin System Issues**: Missing plugin SDK or builtin plugins

### 3. Compliance Reporting
For each deviation found, the microagent provides:
- **Location**: Exact path where the issue was found
- **Type**: Category of architectural violation
- **Severity**: Critical, Warning, or Info level
- **Recommendation**: Specific action to resolve the issue
- **Impact**: How the deviation affects the overall architecture

## Usage Guidelines

### When to Use This Microagent
- Before major refactoring efforts
- During code reviews to ensure architectural compliance
- When adding new modules or components
- As part of CI/CD pipeline checks
- When onboarding new developers

### Architecture Enforcement Rules

1. **Core Directory Structure**: The core/ directory must contain tenant/, plugins/, and events/ subdirectories
2. **Module Consistency**: Each module should follow the handler-service-repository pattern
3. **Separation of Concerns**: Frontend, backend, and shared components must remain separate
4. **Plugin Architecture**: All plugins must use the shared plugin SDK
5. **Multi-tenancy**: Tenant-related code must be in the core/tenant/ directory
6. **Event-Driven Architecture**: Event handling must use the core/events/ system

### Reporting Format

When deviations are found, the microagent reports in this format:

```
🔍 PEMA Architecture Analysis Report
=====================================

✅ COMPLIANT COMPONENTS:
- backend/src/config/ - Complete
- backend/src/modules/auth/ - Complete
- shared/models/ - Complete

⚠️  DEVIATIONS FOUND:

[CRITICAL] Missing Core Component
Location: backend/src/core/tenant/
Issue: Multi-tenancy core component not found
Recommendation: Create tenant management system in backend/src/core/tenant/
Impact: Multi-tenant functionality unavailable

[WARNING] Incomplete Module Structure  
Location: backend/src/modules/products/
Issue: Missing repository.rs file
Recommendation: Add repository.rs following the established pattern
Impact: Data access layer incomplete for products module

[INFO] New Feature Missing
Location: backend/src/modules/ai/
Issue: AI engine module not implemented
Recommendation: Implement AI module with recommender/ and search/ subdirectories
Impact: AI-powered features unavailable
```

## Integration Points

This microagent integrates with:
- **File System Monitoring**: Watches for structural changes
- **Development Workflow**: Provides guidance during development
- **Code Review Process**: Validates architectural compliance
- **Documentation**: Maintains architectural documentation
- **CI/CD Pipeline**: Can be integrated for automated checks

## Limitations

- Does not validate code quality within files
- Does not check for circular dependencies
- Does not validate database schema compliance
- Does not monitor runtime architectural patterns
- Focuses on directory structure and file organization only

## Maintenance

This microagent should be updated when:
- New architectural patterns are introduced
- Directory structure requirements change
- New modules or components are added to the specification
- Plugin system architecture evolves
- Multi-tenancy requirements change

---

*This microagent ensures the PEMA Platform maintains its architectural integrity and provides clear guidance for developers working on the project.*