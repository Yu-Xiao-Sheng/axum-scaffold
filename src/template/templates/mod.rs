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

/// Get all templates for workspace mode (multi-crate)
///
/// Returns a map of template paths to their contents
pub fn get_workspace_mode_templates() -> HashMap<&'static str, TemplateFile> {
    let mut templates = HashMap::new();

    // Root files
    templates.insert(
        "Cargo.toml",
        TemplateFile {
            path: "Cargo.toml",
            content: include_str!("workspace_mode/root/Cargo.toml.hbs"),
            executable: false,
        },
    );
    templates.insert(
        ".env.example",
        TemplateFile {
            path: ".env.example",
            content: include_str!("workspace_mode/root/.env.example.hbs"),
            executable: false,
        },
    );
    templates.insert(
        ".gitignore",
        TemplateFile {
            path: ".gitignore",
            content: include_str!("workspace_mode/root/.gitignore"),
            executable: false,
        },
    );
    templates.insert(
        ".dockerignore",
        TemplateFile {
            path: ".dockerignore",
            content: include_str!("workspace_mode/root/.dockerignore"),
            executable: false,
        },
    );
    templates.insert(
        "README.md",
        TemplateFile {
            path: "README.md",
            content: include_str!("workspace_mode/root/README.md.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "Dockerfile",
        TemplateFile {
            path: "Dockerfile",
            content: include_str!("workspace_mode/root/Dockerfile.hbs"),
            executable: false,
        },
    );

    // api crate
    templates.insert(
        "api/Cargo.toml",
        TemplateFile {
            path: "api/Cargo.toml",
            content: include_str!("workspace_mode/api/Cargo.toml.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/main.rs",
        TemplateFile {
            path: "api/src/main.rs",
            content: include_str!("workspace_mode/api/src/main.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/lib.rs",
        TemplateFile {
            path: "api/src/lib.rs",
            content: include_str!("workspace_mode/api/src/lib.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/config.rs",
        TemplateFile {
            path: "api/src/config.rs",
            content: include_str!("workspace_mode/api/src/config.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/handlers/mod.rs",
        TemplateFile {
            path: "api/src/handlers/mod.rs",
            content: include_str!("workspace_mode/api/src/handlers/mod.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/handlers/health.rs",
        TemplateFile {
            path: "api/src/handlers/health.rs",
            content: include_str!("workspace_mode/api/src/handlers/health.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/handlers/auth.rs",
        TemplateFile {
            path: "api/src/handlers/auth.rs",
            content: include_str!("workspace_mode/api/src/handlers/auth.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "api/src/middleware/mod.rs",
        TemplateFile {
            path: "api/src/middleware/mod.rs",
            content: include_str!("workspace_mode/api/src/middleware/mod.rs.hbs"),
            executable: false,
        },
    );

    // domain crate
    templates.insert(
        "domain/Cargo.toml",
        TemplateFile {
            path: "domain/Cargo.toml",
            content: include_str!("workspace_mode/domain/Cargo.toml.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "domain/src/lib.rs",
        TemplateFile {
            path: "domain/src/lib.rs",
            content: include_str!("workspace_mode/domain/src/lib.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "domain/src/entities/mod.rs",
        TemplateFile {
            path: "domain/src/entities/mod.rs",
            content: include_str!("workspace_mode/domain/src/entities/mod.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "domain/src/traits/mod.rs",
        TemplateFile {
            path: "domain/src/traits/mod.rs",
            content: include_str!("workspace_mode/domain/src/traits/mod.rs.hbs"),
            executable: false,
        },
    );

    // infrastructure crate
    templates.insert(
        "infrastructure/Cargo.toml",
        TemplateFile {
            path: "infrastructure/Cargo.toml",
            content: include_str!("workspace_mode/infrastructure/Cargo.toml.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "infrastructure/src/lib.rs",
        TemplateFile {
            path: "infrastructure/src/lib.rs",
            content: include_str!("workspace_mode/infrastructure/src/lib.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "infrastructure/src/db.rs",
        TemplateFile {
            path: "infrastructure/src/db.rs",
            content: include_str!("workspace_mode/infrastructure/src/db.rs.hbs"),
            executable: false,
        },
    );

    // common crate
    templates.insert(
        "common/Cargo.toml",
        TemplateFile {
            path: "common/Cargo.toml",
            content: include_str!("workspace_mode/common/Cargo.toml.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "common/src/lib.rs",
        TemplateFile {
            path: "common/src/lib.rs",
            content: include_str!("workspace_mode/common/src/lib.rs.hbs"),
            executable: false,
        },
    );
    templates.insert(
        "common/src/error.rs",
        TemplateFile {
            path: "common/src/error.rs",
            content: include_str!("workspace_mode/common/src/error.rs.hbs"),
            executable: false,
        },
    );

    templates
}

/// Get CI/CD templates
///
/// Returns CI workflow templates that can be appended to any mode's template set
pub fn get_ci_templates() -> HashMap<&'static str, TemplateFile> {
    let mut templates = HashMap::new();

    templates.insert(
        ".github/workflows/ci.yml",
        TemplateFile {
            path: ".github/workflows/ci.yml",
            content: include_str!("ci/.github/workflows/ci.yml.hbs"),
            executable: false,
        },
    );

    templates
}
