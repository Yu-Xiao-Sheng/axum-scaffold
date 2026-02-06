// Validation utilities
//
// This module contains validation logic for project names and inputs.

use std::collections::HashSet;

/// Reserved Cargo keywords that cannot be used as project names
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
///
/// # Rules
/// - Cannot be empty
/// - Cannot exceed 100 characters
/// - Cannot start with a digit
/// - Cannot be a Rust reserved keyword
/// - Can only contain alphanumeric characters, hyphens, and underscores
/// - Cannot start with a hyphen or underscore
///
/// # Arguments
/// * `name` - The project name to validate
///
/// # Returns
/// * `Ok(())` if the name is valid
/// * `Err(String)` with an error message if the name is invalid
///
/// # Examples
/// ```
/// use create_axum_app::utils::validator::validate_project_name;
///
/// assert!(validate_project_name("my-app").is_ok());
/// assert!(validate_project_name("my_app").is_ok());
/// assert!(validate_project_name("123invalid").is_err());
/// assert!(validate_project_name("std").is_err()); // Reserved keyword
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(validate_project_name("my-app").is_ok());
        assert!(validate_project_name("my_app").is_ok());
        assert!(validate_project_name("myapp").is_ok());
        assert!(validate_project_name("my-app-123").is_ok());
        assert!(validate_project_name("a").is_ok());
    }

    #[test]
    fn test_empty_name() {
        let result = validate_project_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_too_long() {
        let long_name = "a".repeat(101);
        let result = validate_project_name(&long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too long"));
    }

    #[test]
    fn test_starts_with_digit() {
        let result = validate_project_name("123app");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot start with a digit"));
    }

    #[test]
    fn test_reserved_keywords() {
        assert!(validate_project_name("fn").is_err());
        assert!(validate_project_name("struct").is_err());
        assert!(validate_project_name("impl").is_err());
        assert!(validate_project_name("std").is_err());
    }

    #[test]
    fn test_invalid_characters() {
        assert!(validate_project_name("my app").is_err()); // space
        assert!(validate_project_name("my@app").is_err()); // @
        assert!(validate_project_name("my.app").is_err()); // .
        assert!(validate_project_name("my$app").is_err()); // $
    }

    #[test]
    fn test_starts_with_separator() {
        assert!(validate_project_name("-myapp").is_err());
        assert!(validate_project_name("_myapp").is_err());
    }
}
