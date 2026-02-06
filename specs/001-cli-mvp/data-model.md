# Data Model: create-axum-app CLI Tool - Phase 1 MVP

**Date**: 2025-02-05
**Purpose**: Define data structures and entities for the CLI tool and generated projects
**Status**: Complete

## Overview

This document defines the data model for the create-axum-app CLI tool. Since this is a code generation tool rather than a data-driven application, the "data model" primarily consists of configuration structures, template contexts, and the schemas for generated projects when optional features are enabled.

## 1. CLI Tool Data Structures

### 1.1 Project Configuration

**Purpose**: Represents user choices and project metadata for generation.

```rust
pub struct ProjectConfig {
    /// Project name (validated)
    pub project_name: String,

    /// Optional features selected
    pub features: FeatureSet,

    /// Author name (optional, from Git config or default)
    pub author_name: Option<String>,

    /// Project description (optional, user-provided or default)
    pub description: Option<String>,

    /// Database configuration (if database feature enabled)
    pub database: Option<DatabaseConfig>,

    /// Authentication configuration (if auth feature enabled)
    pub authentication: Option<AuthConfig>,

    /// Logging configuration (if logging feature enabled)
    pub logging: Option<LoggingConfig>,

    /// Business error handling configuration (if biz-error feature enabled)
    pub biz_error: Option<BizErrorConfig>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            project_name: "my-axum-app".to_string(),
            features: FeatureSet::default(),
            author_name: None, // Will try to detect from git
            description: Some("An Axum web application".to_string()),
            database: None,
            authentication: None,
            logging: Some(LoggingConfig::default()),
            biz_error: None,
        }
    }
}
```

### 1.2 Feature Set

**Purpose**: Represents which optional features are enabled.

```rust
#[derive(Debug, Clone, Default)]
pub struct FeatureSet {
    /// Database support (none, postgresql, sqlite, or both)
    pub database: DatabaseOption,

    /// Authentication support
    pub authentication: bool,

    /// Logging configuration
    pub logging: bool,

    /// Business error handling integration
    pub biz_error: bool,
}

impl Default for FeatureSet {
    fn default() -> Self {
        Self {
            database: DatabaseOption::None,
            authentication: false,
            logging: true, // Logging enabled by default
            biz_error: false,
        }
    }
}
```

### 1.3 Database Options

**Purpose**: Represents database type selection.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseOption {
    /// No database support
    None,

    /// PostgreSQL only (production-focused)
    PostgreSQL,

    /// SQLite only (development-focused)
    SQLite,

    /// Both PostgreSQL and SQLite (environment-based switching)
    Both,
}

impl DatabaseOption {
    /// Returns true if any database support is enabled
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::PostgreSQL | Self::SQLite | Self::Both)
    }

    /// Returns true if PostgreSQL is supported
    pub fn supports_postgresql(&self) -> bool {
        matches!(self, Self::PostgreSQL | Self::Both)
    }

    /// Returns true if SQLite is supported
    pub fn supports_sqlite(&self) -> bool {
        matches!(self, Self::SQLite | Self::Both)
    }
}

impl std::fmt::Display for DatabaseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::PostgreSQL => write!(f, "PostgreSQL"),
            Self::SQLite => write!(f, "SQLite"),
            Self::Both => write!(f, "PostgreSQL + SQLite"),
        }
    }
}
```

### 1.4 Database Configuration

**Purpose**: Database-specific settings for generated project.

```rust
pub struct DatabaseConfig {
    /// Database type selected
    pub option: DatabaseOption,

    /// Default connection string format (for .env.example)
    pub default_url: String,

    /// Maximum pool size
    pub max_connections: u32,

    /// Minimum pool size
    pub min_connections: u32,

    /// Enable migration support
    pub migrations: bool,

    /// Migration tool (sqlx-cli recommended)
    pub migration_tool: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            option: DatabaseOption::Both,
            default_url: "postgresql://postgres:password@localhost/mydb".to_string(),
            max_connections: 10,
            min_connections: 1,
            migrations: true,
            migration_tool: "sqlx-cli".to_string(),
        }
    }
}
```

### 1.5 Authentication Configuration

**Purpose**: JWT authentication settings for generated project.

```rust
pub struct AuthConfig {
    /// JWT secret (for .env.example only, not real secret)
    pub example_secret: String,

    /// Token expiration time (in seconds)
    pub expiration_seconds: u64,

    /// Token algorithm (HS256 recommended)
    pub algorithm: String,

    /// Include user model in generated project
    pub include_user_model: bool,

    /// Include login/logout endpoints
    pub include_endpoints: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            example_secret: "your-secret-key-min-32-chars".to_string(),
            expiration_seconds: 24 * 60 * 60, // 24 hours
            algorithm: "HS256".to_string(),
            include_user_model: true,
            include_endpoints: true,
        }
    }
}
```

### 1.6 Logging Configuration

**Purpose**: Logging settings for generated project.

```rust
pub struct LoggingConfig {
    /// Default log level
    pub default_level: String,

    /// Available log levels
    pub available_levels: Vec<String>,

    /// Log format (json, pretty, compact)
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            default_level: "info".to_string(),
            available_levels: vec![
                "trace".to_string(),
                "debug".to_string(),
                "info".to_string(),
                "warn".to_string(),
                "error".to_string(),
            ],
            format: "compact".to_string(),
        }
    }
}
```

### 1.7 Business Error Configuration

**Purpose**: biz-error integration settings for generated project.

```rust
pub struct BizErrorConfig {
    /// Include example error codes
    pub include_examples: bool,

    /// Supported languages
    pub languages: Vec<String>,

    /// Path to error definition YAML
    pub error_def_path: String,

    /// Include error handler middleware
    pub include_middleware: bool,
}

impl Default for BizErrorConfig {
    fn default() -> Self {
        Self {
            include_examples: true,
            languages: vec!["en".to_string(), "zh".to_string()],
            error_def_path: "biz_errors.yaml".to_string(),
            include_middleware: true,
        }
    }
}
```

### 1.8 Template Context

**Purpose**: Data structure passed to Handlebars templates during rendering.

```rust
pub struct TemplateContext {
    /// Project name (kebab-case)
    pub project_name: String,

    /// Project name (snake_case) for code
    pub project_name_snake: String,

    /// Project name (PascalCase) for types
    pub project_name_pascal: String,

    /// Author name
    pub author_name: String,

    /// Project description
    pub description: String,

    /// Current year for copyright
    pub year: String,

    /// Feature flags
    pub features: FeatureSet,

    /// Database configuration (if applicable)
    pub database: Option<DatabaseConfig>,

    /// Authentication configuration (if applicable)
    pub authentication: Option<AuthConfig>,

    /// Logging configuration (if applicable)
    pub logging: Option<LoggingConfig>,

    /// Business error configuration (if applicable)
    pub biz_error: Option<BizErrorConfig>,
}

impl TemplateContext {
    /// Create template context from project configuration
    pub fn from_config(config: &ProjectConfig) -> Self {
        let project_name = &config.project_name;

        Self {
            project_name: project_name.clone(),
            project_name_snake: to_snake_case(project_name),
            project_name_pascal: to_pascal_case(project_name),
            author_name: config.author_name.clone().unwrap_or_else(|| {
                // Try to get from git config
                get_git_user_name().unwrap_or_else(|| "Anonymous".to_string())
            }),
            description: config.description.clone().unwrap_or_else(||
                "An Axum web application".to_string()
            ),
            year: chrono::Utc::now().format("%Y").to_string(),
            features: config.features.clone(),
            database: config.database.clone(),
            authentication: config.authentication.clone(),
            logging: config.logging.clone(),
            biz_error: config.biz_error.clone(),
        }
    }
}

/// Convert kebab-case to snake_case
fn to_snake_case(name: &str) -> String {
    name.replace('-', "_")
}

/// Convert kebab-case to PascalCase
fn to_pascal_case(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect()
}

/// Get user name from git config
fn get_git_user_name() -> Option<String> {
    use std::process::Command;

    Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|output| {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if name.is_empty() { None } else { Some(name) }
        })
}
```

## 2. Generated Project Data Models

### 2.1 User Model (When Authentication Enabled)

**Purpose**: User entity for authentication.

```rust
// In generated project: src/models/user.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Primary key
    pub id: i64,

    /// Username (unique)
    pub username: String,

    /// Email address (unique)
    pub email: String,

    /// Password hash (bcrypt)
    pub password_hash: String,

    /// Account creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User creation request
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// User response (without password)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
```

### 2.2 Claims Model (When Authentication Enabled)

**Purpose**: JWT token claims.

```rust
// In generated project: src/models/auth.rs

use serde::{Deserialize, Serialize};

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID
    pub sub: i64,

    /// Username
    pub username: String,

    /// Token expiration time
    pub exp: usize,

    /// Token issuance time
    pub iat: usize,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
```

## 3. Database Schema (When Database Enabled)

### 3.1 Users Table (PostgreSQL/SQLite)

**Purpose**: Store user accounts for authentication.

```sql
-- migrations/001_create_users.sql

-- For PostgreSQL
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- For SQLite
-- CREATE TABLE IF NOT EXISTS users (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     username TEXT NOT NULL UNIQUE,
--     email TEXT NOT NULL UNIQUE,
--     password_hash TEXT NOT NULL,
--     created_at TEXT NOT NULL DEFAULT (datetime('now')),
--     updated_at TEXT NOT NULL DEFAULT (datetime('now'))
-- );

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
```

## 4. Business Error Data Model (When biz-error Enabled)

### 4.1 Error Definition YAML

**Purpose**: Define error codes with multilingual messages.

```yaml
# biz_errors.yaml

error_codes:
  # Validation errors (4xx)
  - name: VALIDATION_ERROR
    code: 1001
    status: 400
    messages:
      en: "Validation failed"
      zh: "验证失败"

  - name: INVALID_INPUT
    code: 1002
    status: 400
    messages:
      en: "Invalid input data"
      zh: "输入数据无效"

  - name: MISSING_REQUIRED_FIELD
    code: 1003
    status: 400
    messages:
      en: "Missing required field: {field}"
      zh: "缺少必填字段：{field}"

  # Resource not found errors (404)
  - name: RESOURCE_NOT_FOUND
    code: 2001
    status: 404
    messages:
      en: "Resource not found: {resource}"
      zh: "资源未找到：{resource}"

  - name: USER_NOT_FOUND
    code: 2002
    status: 404
    messages:
      en: "User not found"
      zh: "用户未找到"

  # Authentication/Authorization errors (401/403)
  - name: UNAUTHORIZED
    code: 3001
    status: 401
    messages:
      en: "Unauthorized access"
      zh: "未授权访问"

  - name: INVALID_CREDENTIALS
    code: 3002
    status: 401
    messages:
      en: "Invalid username or password"
      zh: "用户名或密码无效"

  - name: FORBIDDEN
    code: 3003
    status: 403
    messages:
      en: "Access forbidden"
      zh: "禁止访问"

  # Database errors (500)
  - name: DATABASE_ERROR
    code: 5001
    status: 500
    messages:
      en: "Database operation failed"
      zh: "数据库操作失败"

  - name: CONNECTION_ERROR
    code: 5002
    status: 500
    messages:
      en: "Database connection failed"
      zh: "数据库连接失败"

  # Internal server errors (500)
  - name: INTERNAL_ERROR
    code: 9000
    status: 500
    messages:
      en: "Internal server error"
      zh: "服务器内部错误"
```

### 4.2 Error Response Format

**Purpose**: Standard JSON error response structure.

```rust
// In generated project (when biz-error enabled)

use serde::{Deserialize, Serialize};
use biz_error::BizError;

/// Standard error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code (numeric)
    pub code: u32,

    /// Error message (localized)
    pub msg: String,

    /// Optional additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Example usage
impl BizError for MyError {
    fn to_response(&self) -> ErrorResponse {
        match self {
            MyError::UserNotFound { user_id } => ErrorResponse {
                code: 2002,
                msg: "User not found".to_string(),
                data: Some(serde_json::json!({ "user_id": user_id })),
            },
            // ... other error variants
        }
    }
}
```

## 5. Configuration Data Model

### 5.1 Application Configuration

**Purpose**: Runtime configuration for generated projects.

```rust
// In generated project: src/config.rs

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Server host
    pub host: String,

    /// Server port
    pub port: u16,

    /// Database URL (if database enabled)
    pub database_url: Option<String>,

    /// JWT secret (if auth enabled)
    pub jwt_secret: Option<String>,

    /// Log level
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL").ok(),
            jwt_secret: env::var("JWT_SECRET").ok(),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }

    /// Get server bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
```

## 6. Validation Rules

### 6.1 Project Name Validation

**Purpose**: Ensure project names comply with Cargo naming conventions.

```rust
use std::collections::HashSet;

/// Reserved Cargo keywords
const RESERVED_KEYWORDS: &[&str] = &[
    "abstract", "alignof", "as", "become", "box", "break", "const",
    "continue", "crate", "do", "else", "enum", "extern", "false", "final",
    "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match",
    "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub",
    "pure", "ref", "return", "self", "Self", "sizeof", "static", "struct",
    "super", "trait", "true", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while", "yield",
];

/// Validate project name according to Cargo naming conventions
pub fn validate_project_name(name: &str) -> Result<(), String> {
    // Check if empty
    if name.is_empty() {
        return Err("Project name cannot be empty".to_string());
    }

    // Check length (practical limit)
    if name.len() > 100 {
        return Err("Project name too long (max 100 characters)".to_string());
    }

    // Check if starts with digit
    if name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        return Err("Project name cannot start with a digit".to_string());
    }

    // Check if reserved keyword
    let keywords: HashSet<_> = RESERVED_KEYWORDS.iter().cloned().collect();
    if keywords.contains(name) {
        return Err(format!(
            "Project name cannot be a Rust keyword: '{}'",
            name
        ));
    }

    // Check characters (alphanumeric, hyphens, underscores only)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(
            "Project name can only contain alphanumeric characters, hyphens, and underscores".to_string()
        );
    }

    // Check if starts with hyphen or underscore
    if name.starts_with('-') || name.starts_with('_') {
        return Err("Project name cannot start with a hyphen or underscore".to_string());
    }

    Ok(())
}
```

## 7. Template Data Model

### 7.1 Template Metadata

**Purpose**: Template file metadata and organization.

```rust
/// Template file descriptor
pub struct TemplateFile {
    /// Relative path in generated project
    pub path: &'static str,

    /// Template content (embedded via include_str!)
    pub content: &'static str,

    /// Feature requirement (if any)
    pub requires_feature: Option<Feature>,

    /// Is the file executable? (for scripts)
    pub executable: bool,
}

/// Feature requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    Database,
    Authentication,
    Logging,
    BizError,
}

/// All template files
pub const TEMPLATES: &[TemplateFile] = &[
    TemplateFile {
        path: "Cargo.toml",
        content: include_str!("templates/single_mode/Cargo.toml.hbs"),
        requires_feature: None,
        executable: false,
    },
    TemplateFile {
        path: "src/main.rs",
        content: include_str!("templates/single_mode/src/main.rs.hbs"),
        requires_feature: None,
        executable: false,
    },
    // ... more templates
];
```

## Summary

The data model for create-axum-app consists of:

1. **CLI Tool Configuration**: `ProjectConfig`, `FeatureSet`, and feature-specific configs
2. **Template Context**: `TemplateContext` for variable substitution in templates
3. **Generated Project Models**: User, Claims, and Configuration structures
4. **Database Schemas**: SQL migrations for user management
5. **Business Error Model**: YAML-based error definitions with multilingual support
6. **Validation Rules**: Project name validation and Cargo compliance

All structures are designed to be:
- **Serializable**: Compatible with serde for persistence/transmission
- **Validated**: Input validation at entry points
- **Documented**: Clear field purposes and constraints
- **Extensible**: Easy to add new fields and features

---

**Next Steps**: Generate API contracts documentation
