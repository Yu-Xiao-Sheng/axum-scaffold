// Embedded templates
//
// This module contains all embedded project templates.
//
// Templates are embedded at compile time using include_str! macro
// This allows the CLI tool to work offline after installation

use std::collections::HashMap;

/// Template file descriptor
#[derive(Debug, Clone)]
pub struct TemplateFile {
    /// Relative path in generated project
    pub path: &'static str,
    /// Template content (embedded via include_str!)
    pub content: &'static str,
    /// Is the file executable? (for scripts)
    pub executable: bool,
}

/// Get all templates for single-package mode
///
/// Returns a map of template paths to their contents
pub fn get_single_mode_templates() -> HashMap<&'static str, TemplateFile> {
    let mut templates = HashMap::new();

    // Cargo.toml
    templates.insert(
        "Cargo.toml",
        TemplateFile {
            path: "Cargo.toml",
            content: include_str!("single_mode/Cargo.toml.hbs"),
            executable: false,
        },
    );

    // src/main.rs
    templates.insert(
        "src/main.rs",
        TemplateFile {
            path: "src/main.rs",
            content: include_str!("single_mode/src/main.rs.hbs"),
            executable: false,
        },
    );

    // src/lib.rs
    templates.insert(
        "src/lib.rs",
        TemplateFile {
            path: "src/lib.rs",
            content: include_str!("single_mode/src/lib.rs.hbs"),
            executable: false,
        },
    );

    // src/config.rs
    templates.insert(
        "src/config.rs",
        TemplateFile {
            path: "src/config.rs",
            content: include_str!("single_mode/src/config.rs.hbs"),
            executable: false,
        },
    );

    // src/handlers/health.rs
    templates.insert(
        "src/handlers/health.rs",
        TemplateFile {
            path: "src/handlers/health.rs",
            content: include_str!("single_mode/src/handlers/health.rs.hbs"),
            executable: false,
        },
    );

    // src/handlers/mod.rs
    templates.insert(
        "src/handlers/mod.rs",
        TemplateFile {
            path: "src/handlers/mod.rs",
            content: include_str!("single_mode/src/handlers/mod.rs.hbs"),
            executable: false,
        },
    );

    // .env.example
    templates.insert(
        ".env.example",
        TemplateFile {
            path: ".env.example",
            content: include_str!("single_mode/.env.example"),
            executable: false,
        },
    );

    // .gitignore
    templates.insert(
        ".gitignore",
        TemplateFile {
            path: ".gitignore",
            content: include_str!("single_mode/.gitignore"),
            executable: false,
        },
    );

    // README.md
    templates.insert(
        "README.md",
        TemplateFile {
            path: "README.md",
            content: include_str!("single_mode/README.md.hbs"),
            executable: false,
        },
    );

    // Database feature templates (conditional based on {{#if has_database}})
    templates.insert(
        "src/db.rs",
        TemplateFile {
            path: "src/db.rs",
            content: include_str!("single_mode/src/db.rs.hbs"),
            executable: false,
        },
    );

    templates.insert(
        "migrations/001_initial.sql",
        TemplateFile {
            path: "migrations/001_initial.sql",
            content: include_str!("single_mode/migrations/001_initial.sql.hbs"),
            executable: false,
        },
    );

    // Authentication feature templates (conditional based on {{#if has_auth}})
    templates.insert(
        "src/handlers/auth.rs",
        TemplateFile {
            path: "src/handlers/auth.rs",
            content: include_str!("single_mode/src/handlers/auth.rs.hbs"),
            executable: false,
        },
    );

    // Biz-error feature templates (conditional based on {{#if has_biz_error}})
    templates.insert(
        "biz_errors.yaml",
        TemplateFile {
            path: "biz_errors.yaml",
            content: include_str!("single_mode/biz_errors.yaml.hbs"),
            executable: false,
        },
    );

    // build.rs (for biz-error code generation)
    templates.insert(
        "build.rs",
        TemplateFile {
            path: "build.rs",
            content: include_str!("single_mode/build.rs.hbs"),
            executable: false,
        },
    );

    // Dockerfile
    templates.insert(
        "Dockerfile",
        TemplateFile {
            path: "Dockerfile",
            content: include_str!("single_mode/Dockerfile.hbs"),
            executable: false,
        },
    );

    // .dockerignore
    templates.insert(
        ".dockerignore",
        TemplateFile {
            path: ".dockerignore",
            content: include_str!("single_mode/.dockerignore"),
            executable: false,
        },
    );

    templates
}
