// Template context
//
// This module builds context data for template rendering.

#[allow(unused_imports)]
use crate::config::{DatabaseConfig, DatabaseOption, FeatureSet, ProjectConfig};
use serde::Serialize;

/// Template context data structure
///
/// This contains all variables that can be used in Handlebars templates.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    /// Project name (kebab-case)
    pub project_name: String,

    /// Project name (snake_case) for code
    pub project_name_snake: String,

    /// Project name (snake_case) alias for templates
    #[serde(rename = "project_snake_case")]
    pub project_name_snake_alias: String,

    /// Project name (PascalCase) for types
    pub project_name_pascal: String,

    /// Author name
    pub author_name: String,

    /// Project description
    pub description: String,

    /// Current year for copyright
    pub year: String,

    /// Feature flags
    #[serde(flatten)]
    pub features: FeaturesContext,

    /// Database configuration (if applicable)
    pub database: Option<DatabaseContext>,

    /// Authentication configuration (if applicable)
    pub authentication: Option<AuthContext>,

    /// Logging configuration (if applicable)
    pub logging: Option<LoggingContext>,

    /// Business error configuration (if applicable)
    pub biz_error: Option<BizErrorContext>,
}

/// Feature flags for template conditionals
#[derive(Debug, Clone, Serialize)]
pub struct FeaturesContext {
    /// Database support enabled
    pub has_database: bool,

    /// PostgreSQL support enabled
    pub has_postgresql: bool,

    /// SQLite support enabled
    pub has_sqlite: bool,

    /// Authentication support enabled
    pub has_auth: bool,

    /// Logging support enabled
    pub has_logging: bool,

    /// Business error support enabled
    pub has_biz_error: bool,
}

/// Database context for templates
#[derive(Debug, Clone, Serialize)]
pub struct DatabaseContext {
    /// Database type (PostgreSQL, SQLite, or Both)
    pub database_type: String,

    /// Default connection URL
    pub default_url: String,

    /// Maximum connections
    pub max_connections: u32,

    /// Minimum connections
    pub min_connections: u32,

    /// Enable migrations
    pub migrations: bool,

    /// Migration tool
    pub migration_tool: String,
}

/// Authentication context for templates
#[derive(Debug, Clone, Serialize)]
pub struct AuthContext {
    /// JWT secret (example only)
    pub example_secret: String,

    /// Token expiration in seconds
    pub expiration_seconds: u64,

    /// Token algorithm
    pub algorithm: String,

    /// Include user model
    pub include_user_model: bool,

    /// Include auth endpoints
    pub include_endpoints: bool,
}

/// Logging context for templates
#[derive(Debug, Clone, Serialize)]
pub struct LoggingContext {
    /// Default log level
    pub default_level: String,

    /// Available log levels (comma-separated)
    pub available_levels: String,

    /// Log format
    pub format: String,
}

/// Business error context for templates
#[derive(Debug, Clone, Serialize)]
pub struct BizErrorContext {
    /// Include examples
    pub include_examples: bool,

    /// Supported languages (comma-separated)
    pub languages: String,

    /// Path to error definitions
    pub error_def_path: String,

    /// Include middleware
    pub include_middleware: bool,
}

impl TemplateContext {
    /// Create template context from project configuration
    pub fn from_config(config: &ProjectConfig) -> Self {
        let project_name = &config.project_name;

        // Build features context
        let features = FeaturesContext {
            has_database: config.features.database.is_enabled(),
            has_postgresql: config.features.database.supports_postgresql(),
            has_sqlite: config.features.database.supports_sqlite(),
            has_auth: config.features.authentication,
            has_logging: config.features.logging,
            has_biz_error: config.features.biz_error,
        };

        // Build database context (if enabled)
        let database = if features.has_database {
            config.database.as_ref().map(|db| DatabaseContext {
                database_type: db.option.to_string(),
                default_url: db.default_url.clone(),
                max_connections: db.max_connections,
                min_connections: db.min_connections,
                migrations: db.migrations,
                migration_tool: db.migration_tool.clone(),
            })
        } else {
            None
        };

        // Build authentication context (if enabled)
        let authentication = if features.has_auth {
            config.authentication.as_ref().map(|auth| AuthContext {
                example_secret: auth.example_secret.clone(),
                expiration_seconds: auth.expiration_seconds,
                algorithm: auth.algorithm.clone(),
                include_user_model: auth.include_user_model,
                include_endpoints: auth.include_endpoints,
            })
        } else {
            None
        };

        // Build logging context (if enabled)
        let logging = if features.has_logging {
            config.logging.as_ref().map(|log| LoggingContext {
                default_level: log.default_level.clone(),
                available_levels: log.available_levels.join(", "),
                format: log.format.clone(),
            })
        } else {
            None
        };

        // Build biz-error context (if enabled)
        let biz_error = if features.has_biz_error {
            config.biz_error.as_ref().map(|be| BizErrorContext {
                include_examples: be.include_examples,
                languages: be.languages.join(", "),
                error_def_path: be.error_def_path.clone(),
                include_middleware: be.include_middleware,
            })
        } else {
            None
        };

        let project_name_snake = to_snake_case(project_name);

        Self {
            project_name: project_name.clone(),
            project_name_snake: project_name_snake.clone(),
            project_name_snake_alias: project_name_snake,
            project_name_pascal: to_pascal_case(project_name),
            author_name: config.author_name.clone().unwrap_or_else(|| {
                // Try to get from git config
                get_git_user_name().unwrap_or_else(|| "Anonymous".to_string())
            }),
            description: config
                .description
                .clone()
                .unwrap_or_else(|| "An Axum web application".to_string()),
            year: get_current_year(),
            features,
            database,
            authentication,
            logging,
            biz_error,
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
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
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

/// Get current year
fn get_current_year() -> String {
    use std::process::Command;

    // Try date command first (more reliable on Linux/macOS)
    if let Ok(output) = Command::new("date").arg("+%Y").output()
        && output.status.success()
        && let Some(year) = String::from_utf8_lossy(&output.stdout)
            .trim()
            .chars()
            .next()
        && year == '2'
    {
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    // Fallback to chrono (which we use in the CLI)
    // This is hardcoded to avoid chrono dependency for just the year
    "2025".to_string() // Update this when needed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseOption, FeatureSet};

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("my-app"), "my_app");
        assert_eq!(to_snake_case("my_axum_app"), "my_axum_app");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("my-app"), "MyApp");
        assert_eq!(to_pascal_case("axum-app-create"), "AxumAppCreate");
    }

    #[test]
    fn test_template_context_basic() {
        let config = ProjectConfig {
            project_name: "my-test-app".to_string(),
            ..Default::default()
        };

        let ctx = TemplateContext::from_config(&config);

        assert_eq!(ctx.project_name, "my-test-app");
        assert_eq!(ctx.project_name_snake, "my_test_app");
        assert_eq!(ctx.project_name_pascal, "MyTestApp");
    }

    #[test]
    fn test_template_context_with_database() {
        let config = ProjectConfig {
            project_name: "my-app".to_string(),
            features: FeatureSet {
                database: DatabaseOption::PostgreSQL,
                ..Default::default()
            },
            database: Some(DatabaseConfig::default()),
            ..Default::default()
        };

        let ctx = TemplateContext::from_config(&config);

        assert!(ctx.features.has_database);
        assert!(ctx.features.has_postgresql);
        assert!(!ctx.features.has_sqlite);
        assert!(ctx.database.is_some());
    }
}
