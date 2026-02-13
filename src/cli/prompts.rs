// Interactive prompts
//
// This module contains interactive prompt logic using inquire.

use crate::config::{DatabaseOption, FeatureSet, Preset, ProjectConfig, ProjectMode};
use crate::utils::validator::validate_project_name;
use inquire::{Confirm, Select, Text};

/// CLI overrides for non-interactive mode
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub database: Option<DatabaseOption>,
    pub auth: Option<bool>,
    pub biz_error: Option<bool>,
    pub log_level: Option<String>,
    pub author: Option<String>,
    pub mode: Option<ProjectMode>,
    pub preset: Option<Preset>,
    pub ci: Option<bool>,
}

/// Prompt for project name
///
/// Returns None if in non-interactive mode and no name provided
pub fn prompt_project_name(interactive: bool, default_name: Option<String>) -> Option<String> {
    if !interactive {
        return default_name;
    }

    let name = Text::new("Project name?")
        .with_placeholder("my-axum-app")
        .with_help_message("Use kebab-case (e.g., my-app, my-awesome-project)")
        .prompt()
        .ok()?;

    Some(name)
}

/// Prompt for author name
///
/// Returns None if not provided (will be detected from git)
pub fn prompt_author_name(interactive: bool) -> Option<String> {
    if !interactive {
        return None;
    }

    Text::new("Author name (optional)?")
        .with_help_message("Leave empty to use git config")
        .prompt()
        .ok()
        .filter(|s| !s.is_empty())
}

/// Prompt for project description
pub fn prompt_description(interactive: bool) -> Option<String> {
    if !interactive {
        return Some("An Axum web application".to_string());
    }

    Text::new("Project description (optional)?")
        .with_default("An Axum web application")
        .prompt()
        .ok()
}

/// Prompt for database selection
pub fn prompt_database(interactive: bool) -> DatabaseOption {
    if !interactive {
        return DatabaseOption::None;
    }

    let options = vec![
        "None - No database support",
        "PostgreSQL - Production-ready, requires external setup",
        "SQLite - Development-friendly, embedded",
        "Both - PostgreSQL + SQLite (environment-based)",
    ];

    let ans = Select::new("Select database support:", options)
        .prompt()
        .unwrap();

    match ans {
        "None - No database support" => DatabaseOption::None,
        "PostgreSQL - Production-ready, requires external setup" => DatabaseOption::PostgreSQL,
        "SQLite - Development-friendly, embedded" => DatabaseOption::SQLite,
        "Both - PostgreSQL + SQLite (environment-based)" => DatabaseOption::Both,
        _ => DatabaseOption::None,
    }
}

/// Prompt for authentication
pub fn prompt_authentication(interactive: bool) -> bool {
    if !interactive {
        return false;
    }

    Confirm::new("Enable JWT authentication?")
        .with_default(false)
        .with_help_message("Include login/register endpoints and JWT middleware")
        .prompt()
        .unwrap_or(false)
}

/// Prompt for biz-error support
pub fn prompt_biz_error(interactive: bool) -> bool {
    if !interactive {
        return false;
    }

    Confirm::new("Enable biz-error integration?")
        .with_default(false)
        .with_help_message("Standardized error responses with i18n support")
        .prompt()
        .unwrap_or(false)
}

/// Prompt for log level selection
pub fn prompt_log_level(interactive: bool) -> String {
    if !interactive {
        return "info".to_string();
    }

    let options = vec![
        "trace - Very detailed diagnostic information",
        "debug - Detailed diagnostic information",
        "info - General informational messages (default)",
        "warn - Warning messages",
        "error - Error messages only",
    ];

    let ans = Select::new("Select default log level:", options)
        .prompt()
        .unwrap_or("info - General informational messages (default)");

    // Extract just the level name
    ans.split(" - ").next().unwrap_or("info").to_string()
}

/// Prompt for project mode selection
pub fn prompt_project_mode(interactive: bool) -> ProjectMode {
    if !interactive {
        return ProjectMode::Single;
    }

    let options = vec![
        "Single package - 单包模式（推荐新手）/ Single-package mode (recommended for beginners)",
        "Workspace - 工作区模式（多 crate 分层架构）/ Multi-crate workspace (Clean Architecture)",
    ];

    let default_option = options[0];

    let ans = Select::new("选择项目模式 / Select project mode:", options)
        .prompt()
        .unwrap_or(default_option);

    if ans.starts_with("Workspace") {
        ProjectMode::Workspace
    } else {
        ProjectMode::Single
    }
}

/// Prompt for preset selection
/// Returns None if user chooses "Custom"
pub fn prompt_preset(interactive: bool) -> Option<Preset> {
    if !interactive {
        return None;
    }

    let options = vec![
        "Minimal - 最小配置 / No optional features",
        "API - API 开发 / PostgreSQL + Auth + Biz-error",
        "Fullstack - 全栈开发 / Both DBs + Auth + Biz-error",
        "Custom - 自定义 / Choose features individually",
    ];

    let default_option = options[3];

    let ans = Select::new("选择配置预设 / Select configuration preset:", options)
        .prompt()
        .unwrap_or(default_option);

    match ans {
        s if s.starts_with("Minimal") => Some(Preset::Minimal),
        s if s.starts_with("API") => Some(Preset::Api),
        s if s.starts_with("Fullstack") => Some(Preset::Fullstack),
        _ => None, // Custom
    }
}

/// Prompt for CI/CD workflow generation
pub fn prompt_ci(interactive: bool) -> bool {
    if !interactive {
        return false;
    }

    Confirm::new("生成 GitHub Actions CI 工作流？/ Generate GitHub Actions CI workflow?")
        .with_default(true)
        .with_help_message(
            "包含 check/test/fmt/clippy 四个 job / Includes check, test, fmt, clippy jobs",
        )
        .prompt()
        .unwrap_or(false)
}

/// Resolve features from preset + CLI overrides
///
/// Priority: CLI flags > preset values > interactive prompts > defaults
pub fn resolve_features(
    preset: Option<Preset>,
    overrides: &CliOverrides,
    interactive: bool,
) -> (FeatureSet, String) {
    let base = match preset {
        Some(p) => p.to_feature_set(),
        None => FeatureSet::default(),
    };

    let database = overrides.database.unwrap_or_else(|| {
        if preset.is_some() {
            base.database
        } else {
            prompt_database(interactive)
        }
    });
    let authentication = overrides.auth.unwrap_or_else(|| {
        if preset.is_some() {
            base.authentication
        } else {
            prompt_authentication(interactive)
        }
    });
    let biz_error = overrides.biz_error.unwrap_or_else(|| {
        if preset.is_some() {
            base.biz_error
        } else {
            prompt_biz_error(interactive)
        }
    });
    let log_level = overrides.log_level.clone().unwrap_or_else(|| {
        if preset.is_some() {
            "info".to_string()
        } else {
            prompt_log_level(interactive)
        }
    });

    let features = FeatureSet {
        database,
        authentication,
        logging: true,
        biz_error,
    };

    (features, log_level)
}

/// Build complete ProjectConfig from interactive prompts
///
/// This is the main entry point for collecting user input.
/// CLI overrides take precedence over interactive prompts.
pub fn prompt_project_config(
    interactive: bool,
    default_name: Option<String>,
    overrides: Option<CliOverrides>,
) -> Result<ProjectConfig, String> {
    let overrides = overrides.unwrap_or_default();

    // Get project name
    let project_name = if let Some(name) = default_name {
        validate_project_name(&name).map_err(|e| format!("Invalid project name: {}", e))?;
        name
    } else if let Some(name) = prompt_project_name(interactive, None) {
        validate_project_name(&name).map_err(|e| format!("Invalid project name: {}", e))?;
        name
    } else {
        return Err("Project name is required in non-interactive mode".to_string());
    };

    // Get author name (CLI override > prompt > git detection)
    let author_name = if overrides.author.is_some() {
        overrides.author.clone()
    } else {
        prompt_author_name(interactive)
    };

    let description = prompt_description(interactive);

    // Get project mode (CLI override > prompt > default)
    let mode = overrides
        .mode
        .unwrap_or_else(|| prompt_project_mode(interactive));

    // Get preset (CLI override > prompt > None)
    let preset = if overrides.preset.is_some() {
        overrides.preset
    } else {
        prompt_preset(interactive)
    };

    // Resolve features from preset + overrides
    let (features, log_level) = resolve_features(preset, &overrides, interactive);

    // Get CI option (CLI override > prompt > default)
    let ci = overrides.ci.unwrap_or_else(|| prompt_ci(interactive));

    // Build logging config with selected log level
    let logging = Some(crate::config::LoggingConfig {
        default_level: log_level,
        ..Default::default()
    });

    Ok(ProjectConfig {
        project_name,
        features,
        author_name,
        description,
        logging,
        mode,
        preset,
        ci,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseOption, Preset};
    use proptest::prelude::*;

    fn arb_preset() -> impl Strategy<Value = Preset> {
        prop_oneof![
            Just(Preset::Minimal),
            Just(Preset::Api),
            Just(Preset::Fullstack),
        ]
    }

    fn arb_database_option() -> impl Strategy<Value = DatabaseOption> {
        prop_oneof![
            Just(DatabaseOption::None),
            Just(DatabaseOption::PostgreSQL),
            Just(DatabaseOption::SQLite),
            Just(DatabaseOption::Both),
        ]
    }

    // Property 2: CLI flags override preset values
    // For any Preset and any CLI flag combination, the resolved FeatureSet
    // should use the flag value for overridden fields and preset value for others.
    // Validates: Requirements 2.5, 6.1, 6.2
    proptest! {
        #[test]
        fn prop_cli_flags_override_preset(
            preset in arb_preset(),
            db_override in proptest::option::of(arb_database_option()),
            auth_override in proptest::option::of(proptest::bool::ANY),
            biz_override in proptest::option::of(proptest::bool::ANY),
        ) {
            let overrides = CliOverrides {
                database: db_override,
                auth: auth_override,
                biz_error: biz_override,
                ..Default::default()
            };

            let (features, _) = resolve_features(Some(preset), &overrides, false);
            let preset_features = preset.to_feature_set();

            // Overridden fields should match the override value
            if let Some(db) = db_override {
                prop_assert_eq!(features.database, db);
            } else {
                prop_assert_eq!(features.database, preset_features.database);
            }

            if let Some(auth) = auth_override {
                prop_assert_eq!(features.authentication, auth);
            } else {
                prop_assert_eq!(features.authentication, preset_features.authentication);
            }

            if let Some(biz) = biz_override {
                prop_assert_eq!(features.biz_error, biz);
            } else {
                prop_assert_eq!(features.biz_error, preset_features.biz_error);
            }
        }
    }

    #[test]
    fn test_prompt_database() {
        // Test that prompt_database returns a valid option
        // We can't test interactive prompts in unit tests
        // But we can verify the function exists and compiles
    }
}
