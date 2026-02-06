// Configuration module
//
// This module contains project configuration structures and validation.

use serde::{Deserialize, Serialize};

/// Database option selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DatabaseOption {
    /// No database support
    #[default]
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

/// Feature set configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Business error configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Project configuration for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
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
