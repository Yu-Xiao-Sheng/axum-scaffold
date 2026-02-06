// Validation utilities
//
// This module contains validation logic for project names and inputs.

use std::collections::HashSet;

/// Reserved Cargo keywords that cannot be used as project names
const RESERVED_KEYWORDS: &[&str] = &[
    "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure",
    "ref", "return", "self", "Self", "sizeof", "static", "struct", "super", "trait", "true",
    "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Validate project name according to Cargo naming conventions
///
/// # 命名规则 / Naming Rules
/// - 不能为空 / Cannot be empty
/// - 不能超过100个字符 / Cannot exceed 100 characters
/// - 不能以数字开头 / Cannot start with a digit
/// - 不能是Rust保留关键字 / Cannot be a Rust reserved keyword
/// - 只能包含字母数字、连字符和下划线 / Can only contain alphanumeric characters, hyphens, and underscores
/// - 不能以连字符或下划线开头 / Cannot start with a hyphen or underscore
///
/// # 参数 / Arguments
/// * `name` - 要验证的项目名称 / The project name to validate
///
/// # 返回 / Returns
/// * `Ok(())` 如果名称有效 / if the name is valid
/// * `Err(String)` 带有详细错误信息和修复建议 / with detailed error message and fix suggestions
///
/// # 示例 / Examples
/// ```
/// use axum_app_create::utils::validator::validate_project_name;
///
/// assert!(validate_project_name("my-app").is_ok());
/// assert!(validate_project_name("my_app").is_ok());
/// assert!(validate_project_name("123invalid").is_err());
/// assert!(validate_project_name("fn").is_err()); // Reserved keyword
/// ```
pub fn validate_project_name(name: &str) -> Result<(), String> {
    // Check if empty
    if name.is_empty() {
        return Err("❌ 项目名称不能为空 / Project name cannot be empty\n\n\
             💡 修复建议 / Fix: 提供一个有效的项目名称 / Provide a valid project name\n\
             ✅ 好的示例 / Good examples: my-app, my_app, awesome-project\n\n\
             📖 查看帮助 / View help: axum-app-create --help"
            .to_string());
    }

    // Check length (practical limit)
    if name.len() > 100 {
        let preview = if name.len() > 10 { &name[..10] } else { name };
        return Err(format!(
            "❌ 项目名称太长（{}个字符，最大100）/ Project name too long ({} chars, max 100)\n\n\
             💡 修复建议 / Fix: 使用更短的名称 / Use a shorter name\n\
             ✅ 好的示例 / Good example: {} (前10个字符 / first 10 chars)\n\n\
             📖 查看帮助 / View help: axum-app-create --help",
            name.len(),
            name.len(),
            preview
        ));
    }

    // Check if starts with digit
    if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        return Err(
            "❌ 项目名称不能以数字开头 / Project name cannot start with a digit\n\n\
             💡 修复建议 / Fix: 在数字前添加字母 / Add letters before the digit\n\
             ✅ 好的示例 / Good example: project123 → project123\n\n\
             📖 查看帮助 / View help: axum-app-create --help"
                .to_string(),
        );
    }

    // Check if reserved keyword
    let keywords: HashSet<_> = RESERVED_KEYWORDS.iter().cloned().collect();
    if keywords.contains(name) {
        return Err(format!(
            "❌ 项目名称不能是Rust关键字 / Project name cannot be a Rust keyword: '{}'\n\n\
             💡 修复建议 / Fix: 使用同义词或添加前缀/后缀 / Use a synonym or add prefix/suffix\n\
             ✅ 好的示例 / Good examples:\n\
              - '{}' → 'my_{}' 或 / or 'my-{}-cli'\n\
              - '{}' → 'setup_{}' 或 / or '{}-tool'\n\n\
             📖 查看帮助 / View help: axum-app-create --help",
            name, // The keyword itself
            name,
            name,
            name, // First example: {name} → my_{name} or my-{name}-cli
            name,
            name,
            name // Second example: {name} → setup_{name} or {name}-tool
        ));
    }

    // Check characters (alphanumeric, hyphens, underscores only)
    let invalid_chars: Vec<char> = name
        .chars()
        .filter(|c| !(c.is_alphanumeric() || *c == '-' || *c == '_'))
        .collect();

    if !invalid_chars.is_empty() {
        let invalid_str: String = invalid_chars.iter().collect();
        return Err(format!(
            "❌ 项目名称包含无效字符 / Project name contains invalid characters: '{}'\n\n\
             💡 修复建议 / Fix: 移除特殊字符，只使用字母、数字、连字符和下划线 / Remove special characters, use only letters, numbers, hyphens, and underscores\n\
             ❌ 无效字符 / Invalid characters found: {}\n\
             ✅ 好的示例 / Good example:\n\
              - 'my @ app' → 'my_at_app'\n\
              - 'my@project' → 'my_project'\n\
              - 'my.project' → 'my_project'\n\n\
             📖 查看帮助 / View help: axum-app-create --help",
            invalid_str, invalid_str
        ));
    }

    // Check if starts with hyphen or underscore
    if name.starts_with('-') || name.starts_with('_') {
        return Err(
            "❌ 项目名称不能以连字符或下划线开头 / Project name cannot start with a hyphen or underscore\n\n\
             💡 修复建议 / Fix: 在前面添加字母 / Add letters before the separator\n\
             ✅ 好的示例 / Good examples:\n\
              - '-project' → 'my-project'\n\
              - '_app' → 'my_app'\n\
              - '-123' → 'app-123'\n\n\
             📖 查看帮助 / View help: axum-app-create --help".to_string()
        );
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
        assert!(validate_project_name("match").is_err());
        assert!(validate_project_name("if").is_err());
        assert!(validate_project_name("else").is_err());
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
