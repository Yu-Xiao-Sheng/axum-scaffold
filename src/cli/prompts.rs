// Interactive prompts
//
// This module contains interactive prompt logic using inquire.

use crate::config::{DatabaseOption, FeatureSet, ProjectConfig};
use crate::utils::validator::validate_project_name;
use inquire::{Confirm, Select, Text};

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

/// Build complete ProjectConfig from interactive prompts
///
/// This is the main entry point for collecting user input
pub fn prompt_project_config(
    interactive: bool,
    default_name: Option<String>,
) -> Result<ProjectConfig, String> {
    // Get project name
    let project_name = if let Some(name) = default_name {
        // Validate provided name
        validate_project_name(&name).map_err(|e| format!("Invalid project name: {}", e))?;
        name
    } else if let Some(name) = prompt_project_name(interactive, None) {
        // Validate prompted name
        validate_project_name(&name).map_err(|e| format!("Invalid project name: {}", e))?;
        name
    } else {
        return Err("Project name is required in non-interactive mode".to_string());
    };

    // Get other optional information
    let author_name = prompt_author_name(interactive);
    let description = prompt_description(interactive);

    // Collect feature selections
    let database_option = prompt_database(interactive);
    let authentication = prompt_authentication(interactive);
    let biz_error = prompt_biz_error(interactive);
    let log_level = prompt_log_level(interactive);

    // Build feature set
    let features = FeatureSet {
        database: database_option,
        authentication,
        logging: true, // Always enabled for now
        biz_error,
    };

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
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prompt_database() {
        // Test that prompt_database returns a valid option
        // We can't test interactive prompts in unit tests
        // But we can verify the function exists and compiles
    }
}
